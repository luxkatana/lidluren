# Lidl uren

Ik werk sindskort bij de Lidl, als practice en nuttigheid wil ik een programma bouwen waarmee ik de uren kan noteren dat ik heb gewerkt in een bepaalde "maand".


## Wat dit project inhoudt
Het gebeurt soms dat bedrijven fouten maken met het noteren van uren die je hebt gewerkt (denk bv aan extra uren die je hebt gewerkt, of je bent vergeten om te uitklokken). Het zou immers handig zijn als je je eigen log kunt vergelijken met je loonstrook. Een app zou wel handig zijn.

## Wat dit project niet gaat bevatten
Dit project focust zich niet op security zoals authentication, dit project is ook niet bedoeld voor professioneel-gebruik. Het is alleen voor mij bedoeld.

## Eisen
- Must have: De app moet uren kunnen noteren
- Should have: Loonstrookjes kunnen parsen, een web-interface ook hebben
- Could have: Dayforce API gebruiken om eigen uren te vergelijken met die van dayforce
- Won't have: Authentication, commericeel gebruik 
## Punten
- Het project moet bestendig zijn om gebruikt te worden op een android-telefoon
- Het project moet uren opslaan met een database
- Het project moet intelligent errors oppakken en aantonen als er wat fout gaat
- Het project moet niet echt bugs hebben met het tellen het van aantal uren die ik heb gewerkt, anders bestaat er verwarring

## Architectuur

- Database: MySQL, ik heb tabellen nodig
Extraatje: Een ORM (Object relational mapping) framework wordt gebruikt om handig met de MySQL te praten en dingen te gaan doen.
```
+----------------------------+-------------------------+------------------------+
| Aantal minuten pauze (int) | Begintijd (UTC iso8601) | Eindtijd (UTC iso8601) |
+----------------------------+-------------------------+------------------------+
|                         30 | 2026-07-11T16:00:00Z    | 2026-07-11T21:00:00Z   |
+----------------------------+-------------------------+------------------------+
```
- Frontend/interface: React-native TSX met UI libraries (make-up)
- Backend: Rust met een http framework
> Eisen voor de HTTP framework:
    - Rust
    - Versturen en ophalen van JSON moet gemakkelijk zijn (JSON validation)
    - GET, POST, UPDATE en DELETE moeten mogelijk zijn
    - Globale variabel thread-safe delen moet mogelijk zijn (Mutex<Arc<T>>)


    

- Communicatie voor het fetchen, bijwerken en verwijderen van data gaat dmv REST


## Milestones
### M1: Startup (Initialiseren) 
- [x] README.md maken
- [x] cargo new
- [x] MySQL installatie 
- [x] Schema design - **shiftID, begintime, endtime and dayforceplanned (boolean)**


### M2: Rust webserver & onderzoek
- [x] Zoeken naar een HTTP webserver framework **Actix web**
- [x] Zoeken naar een ORM framework **SeaORM**
- [x] Noteren endpoints die handig zijn
> http.http bestand representeert endpoints die handig kunnen zijn voor de shifts.

- [ ] CRUD implementeren

### M3: Frontend
- [ ] React native initialiseren
- [ ] Verdiepen in React native
- [ ] UI Framework vinden
- [ ] Gebruik van de APP vaststellen
- [ ] Uren toevoegen
- [ ] Uren verwijderen
- [ ] Uren updaten
- [ ] Uren laten zien

### M4: Could have's implementeren
- [ ] Dayforce API integreren


