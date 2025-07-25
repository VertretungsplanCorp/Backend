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


def split_klasse(klasse_ges) -> (int, str):
    stufe = 0
    klassen = []
    match = re.search(r'(\d+)(?:(\w\d?) ?- ?(\w\d?)|(\w+\d?))?',
                      klasse_ges.text.strip())
    if match:
        stufe = match.group(1)
        klasse_von = match.group(2)
        klasse_bis = match.group(3)
        klassen_ges = match.group(4)

        if klassen_ges:
            klassen = list(klassen_ges)
        else:
            klassen = [chr(i) for i in range(ord(klasse_von[0]),
                                             ord(klasse_bis[0])+1)]
    return (stufe, klassen)


def split_stunden(stunden_ges) -> range:
    stunden = range(0)
    match = re.search(r'(\d+) ?-? ?(\d+)?', stunden_ges.text.strip())
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


def split_text(text_ges):
    text = ""
    text_neu = ""
    match = re.search(r'^(\w+)\??(\w*?)$', text_ges.text.strip())
    if match:
        if text_ges.s:  # durchgestrichen
            text = match.group(1)
            text_neu = match.group(2)
        else:
            text = match.group(2)
            text_neu = match.group(1)
    return (text, text_neu)


def scrape(base, subdirs):
    vertretungen = {}
    for subdir in subdirs:
        for i in range(1, 4):
            filename = f"subst_00{i}.htm"
            url = f"{base}/{subdir}/{filename}"

            r = requests.get(url)
            if r.status_code != 200:
                break

            soup = BeautifulSoup(r.text, 'html.parser')

            mon_title = soup.find_all("div", class_="mon_title")

            if not mon_title:
                continue

            datum_split = mon_title[0].text.split()

            if not datum_split:
                continue

            datum_str = datum_split[0]

            datum = datetime.strptime(datum_str, "%d.%m.%Y")

            if not datum:
                continue

            if datum not in vertretungen:
                vertretungen[datum] = []

            tables = soup.find_all("table")
            table = None
            for t in tables:
                headers = [th.get_text(strip=True).lower()
                           for th in t.find_all("th")]
                if headers and ("klasse" in headers and "stunde" in headers and "fach" in headers):
                    table = t
                    break

            if not table:
                break

            rows = table.find_all("tr")
            for row in rows[1:]:
                cells = row.find_all("td")
                if len(cells) < 5:
                    continue

                (stufe, klassen) = split_klasse(cells[0])
                stunden = split_stunden(cells[1])
                (fach, fach_neu) = split_text(cells[2])
                (raum, raum_neu) = split_text(cells[3])
                (lehrer, lehrer_neu) = ("", "")  # todo: add cell
                text = cells[4].text.strip()

                for stunde in stunden:
                    for klasse in klassen:
                        vert = Vertretung(stufe, klasse, stunde, fach, fach_neu,
                                          lehrer, lehrer_neu, text, raum,
                                          raum_neu)
                        vertretungen[datum].append(vert)
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
