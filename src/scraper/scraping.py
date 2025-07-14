import requests
from bs4 import BeautifulSoup
from collections import defaultdict
from datetime import datetime, timedelta
import re

BASE = "https://tafel.gymb.de/V-public"
SUBDIRS = ["f1", "f2"]

vertretungenheute = defaultdict(list)
vertretungenmorgen = defaultdict(list)

vertretungen = [vertretungenheute, vertretungenmorgen]

def scrape():
	# Jede Unterseite und jedes Unterverzeichnis durchgehen
	for index in range(len(SUBDIRS)):
		i = 1
		update_datum_str = None  # wird später aus "Stand:" gezogen

		while True:
			filename = f"subst_{i:03}.htm"
			url = f"{BASE}/{SUBDIRS[index]}/{filename}" if SUBDIRS[index] else f"{BASE}/{filename}"

			r = requests.get(url)
			if r.status_code != 200:
				break  # keine Seite mehr vorhanden

			r.encoding = 'utf-8'
			soup = BeautifulSoup(r.text, 'html.parser')

			page_text = soup.get_text()

			# Stand-Datum extrahieren (z. B. „Stand: 08.07.2025, 06:45 Uhr“)
			if update_datum_str is None:
				stand_match = re.search(r"Stand:\s*(\d{1,2}\.\d{1,2}\.\d{4})", page_text)
				if stand_match:
					update_datum_str = stand_match.group(1)

			# Datum auf der Seite suchen (Eintragsdatum)
			datum_match = re.search(r"(\d{1,2}\.\d{1,2})(?:\.(\d{4}))?", page_text)
			if not datum_match:
				break  # kein Datum → vermutlich ungültige Seite

			# Datum zusammenbauen
			tag_monat = datum_match.group(1)
			jahr = datum_match.group(2) if datum_match.group(2) else str(expected_date.year)
			seitendatum = f"{tag_monat}.{jahr}"

			table = soup.find("table")
			if not table:
				break  # keine Tabelle → vermutlich das Ende

			# Entscheidung: heute oder morgen
			if seitendatum == update_datum_str:

				rows = table.find_all("tr")
				for row in rows[1:]:
					cells = row.find_all("td")
					if len(cells) < 5:
						continue
	
					stunde = cells[1].text.strip()
					klasse = cells[0].text.strip()
					fach = cells[2].text.strip()
					raum = cells[3].text.strip()
					text = cells[4].text.strip()
	
					is_frei = ("---" in raum) or ("entfällt" in text.lower()) or ("---" in fach)
	
					vert = {
						"Stunde": stunde,
						"Fach": fach,
						"Raum/Text": f"{raum} / {text}",
						"Freistunde": is_frei
					}
					vertretungen[index][klasse].append(vert)

			i += 1  # nächste Seite

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
				if v["Freistunde"]:
					print(f"\t[FREISTUNDE] Stunde {v['Stunde']}: {v['Fach']} — {v['Raum/Text']}")
				else:
					print(f"\tStunde {v['Stunde']}: {v['Fach']} — {v['Raum/Text']}")
		print(str(i+1) + " :" + str(number))
		print("")

	return (vertretungenheute, vertretungenmorgen)

scrape()
