
```shell
cargo run < test-cases/test_1.json > output.json
```

## Rules

- Zeitbasiert
    - Datumsbereich
        - Von/Bis
        - Zeitbereich (Uhrzeit)
    - Sonntag
    - Feiertag
        - Idee 1: Angabe durch Datumsangabe
        - Idee 2: Feirtage werden je Bundesland importiert
    - Nachtbereich
- Geobasiert (Geofence)
    - Flughafen ist teuer (Bereiche werden eingegrenzt)
    - Innenstatt ist teuer
    - Start und Ziel
    - Start oder Ziel im Ausland
    - Km in bestimmtem Bereich
- Fahrzeugbasiert (nicht MVP)
    - Fahrzeugtyp
    - Fahrzeuggruppe
    - Fahrzeugstandort
- Personenmengen
- Fahrtdauer/Distanz
    - Bsp.: Länger als x Stunden
    - Bsp.: Länger als x Km
    - Distanzregel: Streckenteil (Abhohlort bis Start, Fahrt, Abhohlort bis Ziel)


- Rule processing
    - continue/final
- Anwendbarkeit von Regeln
    - Fahrt arten: AB/ABA/ABBA
- **Calculation on Request**
    - Fahrt konnte nicht automatisch berechnet werden

- Preisveränderungen
    - Increment/Decrese by Percentage
    - Festpreis

- **Interessant für Steffan**:
    - Incremental Pricing by Capacity
    - Preis nach Auslastung/Umsatz/Km-Leistung (**confirmed status**)

- Unterscheiden zwischen Angebot/Confirmed

- Grundpreis
    - Initial (grundpreis, Fahrer dreht schlüssel um, für Fahrzeug)
    - Delivery (von Basis zu Pickup)
    - Preis pro Distanz
    - Preis pro Zeit
        - per hour return
        - per hour return new ride
        - extra time (bsp.: stop im cafe)

- transfer
- hin und rückfahrt
- 

- Geofences
- netto,mwst ja/nein