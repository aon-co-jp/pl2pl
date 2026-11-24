> Hinweis: Das japanische README.md im Wurzelverzeichnis ist das Original. Diese Fassung wurde von einer KI übersetzt und von einer KI auf muttersprachlich natürliche Formulierung geprüft; ein Mensch mit dieser Muttersprache hat sie noch nicht geprüft.

# pl2pl

## Was ist pl2pl?
Programme sind in vielen „Sprachen“ (Programmiersprachen) geschrieben.
**pl2pl ist ein Werkzeug, das ein Programm aus einer Sprache in eine andere umschreibt.**
Zum Beispiel kann es eine Website aus PHP (Laravel) in eine Website aus Python (FastAPI) verwandeln.
Es läuft unter Windows, Linux und Mac (Version 0.1.0, noch im Bau).

- Man installiert es auf dem eigenen Computer. Auf der Website gibt es nur Anleitungen und Downloads → <https://easy-web.tokyo/pl2pl/>
- Was pl2pl nicht umschreiben kann, versteckt es nicht. Es lässt ein `TODO` (eine Notiz für später) stehen und nennt den Grund im Bericht.
- Unser Ziel: zwischen möglichst vielen bekannten Sprachen umschreiben zu können.

## Was heute schon geht
| Befehl | Was er tut |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. Mehr in [PHP.md](../../PHP.md) (Japanisch) |
| `pl2pl web` | Schaut in einen Ordner, erkennt Java, COBOL, PHP und Flask/Django und gibt jedes an das passende Werkzeug |
| `pl2pl ir` | (Testversion) Bringt die „Routen“ von PHP, Python und Java in eine gemeinsame Form und erzeugt JSON, OpenAPI oder ein FastAPI-Gerüst. Auch die PHP-Umwandlung legt Typen, Funktionsinhalte, Modelle, Middleware und Vorlagen in diese Form, bevor sie schreibt |
| `pl2pl watch` | Findet neue Versionen von Sprachen und Werkzeugen und zieht von selbst nach |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Macht C/C++-Schleifen schneller (gpu ist eine Testversion) |
| `pl2pl cport` | Macht C-Code für UNIX unter Windows und Linux lauffähig |
| `pl2pl pyhpc` | Macht Python-Rechnungen mit Numba schneller |
| `pl2pl llm setup` | Installiert den KI-Helfer (aruaru-llm). Die echte Installation haben wir noch nicht geprüft |

## Neue Versionen von selbst nachziehen
Wir lassen `pl2pl watch` jeden Abend und jeden Morgen laufen.
1. Es prüft, ob es neue Versionen von Sprachen und Werkzeugen gibt (die Liste steht in `registry/ecosystem.toml`).
2. Es testet mit der neuen Version. Bestehen die Tests, trägt es die „unterstützte Version“ selbst nach.
3. Schlagen sie fehl, versucht version-control (mit Hilfe von KI) das Problem zu beheben.
4. Gelingt auch das nicht, schickt es eine Nachricht an die gespeicherten E-Mail-Adressen und an LINE.

Bitte beachten: Bei laravel, symfony, flask, django und einigen anderen meldet es bisher nur „Es gibt eine neue Version“.
Auf GitHub laufen außerdem jeden Abend und jeden Morgen Tests mit den neuesten Versionen von selbst.

## So benutzt man es
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## Gut zu wissen
- pl2pl ist vorsichtig. Was riskant aussieht, lässt es unverändert und nennt den Grund. Geschwindigkeit können wir nicht versprechen.
- Wir haben bisher nur kleine Beispiele getestet. Große, echte Programme haben wir noch nicht ausprobiert.

## Wie es weitergeht
- Auch Typen und Funktionsinhalte in die gemeinsame Form bringen und die FastAPI-Ausgabe darüber laufen lassen.
- PHP: Blade-Bausteine, die als Klassen geschrieben sind.
- Echte E-Mail-Warnungen und die automatische Reparatur durch KI prüfen.
- Prüfen, ob es auf der GPU läuft (open-cuda / open-directx).

## Neu in v0.2.0
- Frei kombinieren: `pl2pl web <dir> --backend <rust-axum|rust-poem|fastapi|mfastapi|rpoem|rporea> [--frontend bootstrap-ts|rust-wasm|none] [--db none|mysql|mariadb|postgres|sqlite] --out <dir>`. COBOL wird umgeschrieben; C, C++ und Fortran laufen unverändert und werden vom Web aus aufgerufen.
- `pl2pl c2rust` (einfaches C und C++) und `pl2pl f2rust` (einfaches Fortran) erzeugen Rust. `pl2pl cobol --os <linux|macos|unix|windows>` erstellt ein Build-Skript für GnuCOBOL.

## Neu in v0.2.1 (2026-11-20)
- Wählt man bei `pl2pl web --backend` `rpoem` / `rporea`, erscheinen Hinweise (Japanisch und Englisch): RPoem enthält schon Tauri- und WunderGraph-Cosmo-kompatible Funktionen, tokio und WASM, also nicht extra wählen; HTML5 und CSS3 werden genutzt, aber knapp gehalten; JavaScript, TypeScript, Node.js und REST werden vom Entwurf her nicht genutzt. Was man mit `--also` hinzufügt und schon enthalten ist, wird abgelehnt (Exit-Code 2); was enthalten, aber noch nicht ausgereift ist, bleibt mit Warnhinweis wählbar.
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` beurteilt diese Reife mit Testläufen (Liste: `registry/maturity.toml`). Die KI-Meinung von aruaru-llm ändert das Urteil nie allein.
- Bei Stacks außer RPoem / RPoRea wird `--also nodejs` (oder `next.js`) mit Hinweis akzeptiert: Wir zwingen unsere Regel „kein Node.js“ niemandem auf, empfehlen es aber nicht. Alternativen für die gewählte Sprache werden aufgelistet (RPoem / RPoRea + Reaxt, FastAPI + Jinja2, RS-Node.js und weitere, die wir nicht geprüft haben).
- Schwester-Repositorys, die gemeinsam entwickelt werden: Reaxt, RPoRea, RPoem, RMojo, open-cosmo, RS-TypeScript, RS-JavaScript, RS-Node.js und pl2pl. Ändert sich eines, werden die anderen am selben Tag geprüft.
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` und `webpage/info.html` erklären die Entwurfsprinzipien von RPoRea / RPoem mit Vor- und Nachteilen in 11 Sprachen.
- Die ursprünglichen `COBOL_to` / `Java_to` bleiben erhalten (nicht gelöscht); die zusammengeführten Kopien in `converters/` lieferten identische Ausgabe.

## Neu (2026-11-22 bis 2026-11-23)
- COBOL (`converters/cobol_to`): neu sind `SORT` (`USING` / `GIVING`, `INPUT` / `OUTPUT PROCEDURE`, `RELEASE`, `RETURN`), `MERGE`, `SEARCH` / `SEARCH ALL` und `INSPECT ... LEADING` / `CHARACTERS`. Die Angaben `COMP-3` und `COMP` werden jetzt gelesen.
- Eingebettetes SQL: `EXEC SQL` (Cursor, `FETCH`, `SELECT ... INTO`), `WHENEVER`, Indikatorvariablen (auch in der Schreibweise mit dem Schlüsselwort `INDICATOR`), `SQLSTATE`, `EXECUTE IMMEDIATE` sowie `PREPARE` / `EXECUTE ... USING`. Das erzeugte Python wurde wirklich ausgeführt; die Rust-Seite ist nur mit einem Schein-Handler geprüft.
- `EXEC CICS` (minimal): `SEND TEXT` zeigt Text an, `RETURN` / `ABEND` beenden das Programm. Andere Befehle bleiben als `TODO` stehen. BMS-Masken werden nicht nachgebildet.
- Java (`converters/java_to`): Methodenreferenzen, `Comparator`, `Collectors` und anonyme Klassen (alle Methoden bleiben erhalten). Auf der Rust-Seite funktionieren List-Operationen (`Vec`, `push`, Indexzugriff, `for`); das erzeugte Rust besteht `cargo check` für spring, jaxrs, servlet und spark.
- Versuch mit Spring PetClinic REST (ein Projekt, das von OpenAPI ausgeht): Die gefundenen Routen stiegen von 1 auf 37; das erzeugte Python hat 0 Syntaxfehler; abgeleitete Abfragen (`Or`, `Like`, Vergleiche, `OrderBy` usw.) laufen auf einem Speicher-Repository; die App lässt sich importieren (41 Routen); die Namen von `this.field` stimmen jetzt überein, sodass ein Aufruf Controller → Service → Repository durchläuft.
- Noch offen: Erzeugung von Repository und MapStruct-Mapper (Aufrufe bleiben an einem `TODO` stehen), eine echte Datenbank, ein echter Lauf des erzeugten FastAPI, CICS-Masken und -Dateien sowie Rust-Rümpfe mit Map oder Stream. pl2pl-Tests: 55 bestanden.

## Mehr erfahren
[LANGUAGES.md](../../LANGUAGES.md) (Liste der Sprachen) / [COMBINATIONS.md](../../COMBINATIONS.md) (Kombinationen) / [DESIGN.md](../../DESIGN.md) (der Entwurf, vom Eigentümer geschrieben) / [components/README.md](../../../components/README.md) (mitgelieferte Teile). Diese Dateien sind auf Japanisch.
