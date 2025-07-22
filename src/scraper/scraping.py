import requests
from bs4 import BeautifulSoup
from collections import defaultdict
import re
from datetime import datetime, timedelta
import psycopg2  # für PostgreSQL
from dotenv import load_dotenv
import os


# Verbindung zur PostgreSQL-Datenbank herstellen
# Lade die .env-Datei
load_dotenv()

SUBDIRS = ["f1", "f2"]
BASE = os.getenv('SCRAPING_BASE')

print("running on base: ", BASE)

vertretungenheute = defaultdict(list)
vertretungenmorgen = defaultdict(list)

vertretungen = [vertretungenheute, vertretungenmorgen]


def split_klasse(klasse_ges) -> (int, str):
    stufe = 0
    klasse = ""
    match = re.search(r'(\d+)(.*)', klasse_ges)
    if match:
        stufe = match.group(0)
        klasse = match.group(1)
    return (stufe, klasse)


def split_text(text):
    if '?' in text:
        teile_text = text.split('?')
        teil_1, teil_2 = [teil.strip() for teil in teile_text if teil]
        ausgabe = (teil_1, teil_2)
    else:
        ausgabe = (text, "")
    return ausgabe


def scrape():
    for index in range(len(SUBDIRS)):
        i = 1
        update_datum_str = None
        expected_date = datetime.now() + timedelta(days=index)

        while True:
            filename = f"subst_{i:03}.htm"
            url = f"{BASE}/{SUBDIRS[index]}/{filename}"

            r = requests.get(url)
            if r.status_code != 200:
                break

            r.encoding = 'utf-8'
            soup = BeautifulSoup(r.text, 'html.parser')
            page_text = soup.get_text()

            # Stand-Datum extrahieren
            if update_datum_str is None:
                stand_match = re.search(
                    r"Stand:\s*(\d{1,2}\.\d{1,2}\.\d{4})", page_text)
                if stand_match:
                    update_datum_str = stand_match.group(1)

            # Seitendatum extrahieren
            datum_match = re.search(
                r"(\d{1,2}\.\d{1,2})(?:\.(\d{4}))?", page_text)
            if not datum_match:
                break

            tag_monat = datum_match.group(1)
            jahr = datum_match.group(2) if datum_match.group(
                2) else str(expected_date.year)
            seitendatum = f"{tag_monat}.{jahr}"

            tables = soup.find_all("table")
            table = None
            for t in tables:
                headers = [th.get_text(strip=True).lower()
                           for th in t.find_all("th")]
                if headers and ("klasse" in headers and "stunde" in headers and "fach" in headers):
                    table = t
                    break

            if not table:
                break  # keine passende Tabelle gefunden

            if seitendatum == update_datum_str:
                rows = table.find_all("tr")
                for row in rows[1:]:
                    cells = row.find_all("td")
                    if len(cells) < 5:
                        continue

                    stunde = cells[1].text.strip()
                    klasse_ges = cells[0].text.strip()
                    fach_ges = cells[2].text.strip()
                    raum_ges = cells[3].text.strip()
                    text = cells[4].text.strip()
                    lehrer_ges = ""

                    (fach, fach_neu) = split_text(fach_ges)
                    (raum, raum_neu) = split_text(raum_ges)
                    (stufe, klasse) = split_klasse(klasse_ges)
                    (lehrer, lehrer_neu) = split_text(lehrer_ges)

                    vert = {
                        "klasse": klasse,
                        "stufe": stufe,
                        "stunde": stunde,
                        "fach": fach,
                        "fach_neu": fach_neu,
                        "text": text,
                        "raum": raum,
                        "raum_neu": raum_neu,
                        "lehrer": lehrer,
                        "lehrer_neu": lehrer_neu,
                    }
                    vertretungen[index][klasse].append(vert)

            i += 1

    # Ergebnisse ausgeben
    for i in range(2):
        number = 0
        if i == 0:
            print("Heute")
        else:
            print("Morgen")
        for klasse in sorted(vertretungen[i]):
            print(f"\n=== {klasse} ===")
            for v in vertretungen[i][klasse]:
                number += 1
                print(f"\tStunde {v['stunde']}: {
                      v['fach']} — {v['raum']} / {v['text']}")
        print(str(i+1) + " :" + str(number))
        print("")

    return vertretungen


print("connecting...")
# Verbindung zur PostgreSQL-Datenbank herstellen, mit den Werten aus der .env-Datei
conn = psycopg2.connect(
    host=os.getenv('DB_HOST'),  # liest den Wert von DB_HOST aus der .env-Datei
    # liest den Wert von DB_USER aus der .env-Datei
    user=os.getenv('DB_USERNAME'),
    # liest den Wert von DB_PASSWORT aus der .env-Datei
    password=os.getenv('DB_PASSWORD'),
    # liest den Wert von DB_NAME aus der .env-Datei
    database=os.getenv('DB_NAME')
)
# conn = psycopg2.connect(
#   host='DEIN_DB_HOST',
#  user='DEIN_DB_USER',
#    password='DEIN_DB_PASSWORT',
#    database='DEINE_DATENBANK'
# )

cursor = conn.cursor()


def speichere_in_db(vertretungen, datum):
    for klasse in vertretungen:
        for v in vertretungen[klasse]:
            cursor.execute('''
                           INSERT INTO vertretungen (datum, stufe, klasse, stunde,
                           fach, fach_neu, raum, raum_neu, lehrer, lehrer_neu, text)
                           VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)''', (
                datum,
                v["stufe"],
                v["klasse"],
                v["stunde"],
                v["fach"],
                v["fach_neu"],
                v["raum"],
                v["raum_neu"],
                v["lehrer"],
                v["lehrer_neu"],
                v["text"],
            ))
    conn.commit()


# --- Hauptteil ---
print("scraping...")
vertretungenheute, vertretungenmorgen = scrape()

heute = datetime.today().date()
morgen = heute + timedelta(days=1)

print("saving in db...")
speichere_in_db(vertretungenheute, heute)
speichere_in_db(vertretungenmorgen, morgen)

print("finishing...")
cursor.close()
conn.close()
# print("Einträge aktualisiert.")
