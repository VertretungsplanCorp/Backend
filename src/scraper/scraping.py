import re
import os
from datetime import datetime, timedelta
from dataclasses import dataclass

import requests
from bs4 import BeautifulSoup
import psycopg2
from dotenv import load_dotenv


@dataclass
class Vertretung:
    stufe: int
    klasse: str
    stunde: int
    fach: str
    fach_neu: str
    lehrer: str
    lehrer_neu: str
    text: str
    raum: str
    raum_neu: str


def split_klasse(klasse_ges: str) -> (int, str):
    stufe = 0
    klasse = ""
    match = re.search(r'(\d+)(.*)', klasse_ges)
    if match:
        stufe = match.group(1)
        klasse = match.group(2)
    return (stufe, klasse)


def split_stunden(stunden_ges: str) -> range:
    stunden = range(0)
    match = re.search(r'(\d+) ?-? ?(\d+)?', stunden_ges)
    if match:
        von_match = match.group(1)
        bis_match = match.group(2)

        von = int(von_match)
        if not bis_match:
            stunden = range(von, von + 1)
        else:
            bis = int(bis_match)
            stunden = range(von, bis + 1)

    return stunden


def split_text(text):
    if '?' in text:
        teile_text = text.split('?')
        teil_1, teil_2 = [teil.strip() for teil in teile_text if teil]
        ausgabe = (teil_1, teil_2)
    else:
        ausgabe = ("", text)
    return ausgabe


def scrape(base, subdirs):
    vertretungen = {}
    for index, subdir in enumerate(subdirs):
        i = 1
        update_datum_str = None
        expected_date = datetime.now() + timedelta(days=index)

        if expected_date not in vertretungen:
            vertretungen[expected_date] = []

        while True:
            filename = f"subst_{i:03}.htm"
            url = f"{base}/{subdir}/{filename}"

            r = requests.get(url)
            if r.status_code != 200:
                break

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

                    stunden_ges = cells[1].text.strip()
                    klasse_ges = cells[0].text.strip()
                    fach_ges = cells[2].text.strip()
                    raum_ges = cells[3].text.strip()
                    text = cells[4].text.strip()
                    lehrer_ges = ""

                    (fach, fach_neu) = split_text(fach_ges)
                    (raum, raum_neu) = split_text(raum_ges)
                    (stufe, klasse) = split_klasse(klasse_ges)
                    (lehrer, lehrer_neu) = split_text(lehrer_ges)
                    stunden = split_stunden(stunden_ges)

                    for stunde in stunden:
                        vert = Vertretung(stufe, klasse, stunde, fach, fach_neu,
                                          lehrer, lehrer_neu, text, raum,
                                          raum_neu)
                        vertretungen[expected_date].append(vert)
            i += 1
    return vertretungen


def speichere_in_db(connection, vertretungen):
    cursor = connection.cursor()
    cursor.execute('''DELETE FROM vertretungen''')
    for datum, datum_vtr in vertretungen.items():
        for v in datum_vtr:
            if v.stufe == 0:
                continue
            if v.klasse == '':
                continue
            if v.stunde == 0:
                continue
            if v.raum_neu == '---':
                v.raum_neu = None
            cursor.execute('''
                               INSERT INTO vertretungen (datum, stufe, klasse, stunde,
                               fach, fach_neu, raum, raum_neu, lehrer, lehrer_neu, text)
                               VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)''',
                           (
                               datum,
                               v.stufe,
                               v.klasse,
                               v.stunde,
                               v.fach or None,
                               v.fach_neu or None,
                               v.raum or None,
                               v.raum_neu or None,
                               v.lehrer or None,
                               v.lehrer_neu or None,
                               v.text or None,
                           ))
    cursor.close()
    connection.commit()


def main():
    load_dotenv()

    SUBDIRS = ["f1", "f2"]
    BASE = os.getenv('SCRAPING_BASE')

    print("running on base: ", BASE)

    print("scraping...")
    vertretungen = scrape(BASE, SUBDIRS)

    print("connecting...")
    connection = psycopg2.connect(
        host=os.getenv('DB_HOST'),
        user=os.getenv('DB_USERNAME'),
        password=os.getenv('DB_PASSWORD'),
        database=os.getenv('DB_NAME')
    )

    print("saving in db...")
    speichere_in_db(connection, vertretungen)

    print("finishing...")
    connection.close()


if __name__ == "__main__":
    main()
