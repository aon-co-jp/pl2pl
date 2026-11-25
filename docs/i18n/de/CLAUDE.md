> Hinweis: Das japanische CLAUDE.md im Wurzelverzeichnis ist das Original. Diese Fassung wurde von einer KI übersetzt und von einer KI auf muttersprachlich natürliche Formulierung geprüft; ein Mensch mit dieser Muttersprache hat sie noch nicht geprüft.

# pl2pl – Entwicklungsnotizen
- **Aufgabe**: ein Werkzeug, das Code zwischen Sprachen, Frameworks und Betriebssystemen umschreibt (in Rust gebaut, in `src/`). Quellsprachen stehen in `docs/LANGUAGES.md`, Kombinationen in `docs/COMBINATIONS.md`. Den Entwurf schreibt der Eigentümer nach und nach in `docs/DESIGN.md`. **Nie die Spezifikation für ihn erfinden.**
- **Richtung**: „Viele zu viele“ zwischen bekannten Sprachen umschreiben. Nicht alles auf einmal bauen, sondern mit dem Ausgewählten anfangen. Die Website bietet nur Anleitungen und Downloads. Umgeschrieben wird in der PC-App.
- **Regeln**: Was sich nicht umschreiben lässt, bleibt als `TODO` stehen und wird gemeldet (nie verstecken). Vorsichtig sein (bei möglichen Abhängigkeiten oder Nebenwirkungen nichts ändern). Erst wirklich ausführen, dann „fertig“ sagen. Keine neuen REST-Schnittstellen. Antworten nur auf Japanisch.
- **Gemeinsame Form (IR)**: `src/ir.rs` (Typen, Funktionsinhalte, Modelle, Middleware, Vorlagen) und die Schreibseite in `src/ir_emit.rs`. Eingabeseite (PHP, Python, Java) und Ausgabeseite (JSON, OpenAPI, FastAPI) sind getrennt, damit neue Paare leicht dazukommen. `pl2pl php` läuft ebenfalls über dieselbe Schreibseite; bei einer Änderung, die die Ausgabe nicht verändern darf, beweist man das mit einem leeren Diff der Ausgabe vorher und nachher. Dateien ohne gefundene Routen werden gemeldet. Die Ausgabe wirklich prüfen, zum Beispiel mit `py_compile`.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. Durchgängiger Test: `tests/run-php.sh` (echtes FastAPI, SQLite, Jinja2). Unit-Tests: `cargo test --release`. In WSL: `wsl -d Ubuntu -- bash <file>`.
- **Automatisches Nachziehen**: `pl2pl watch` (`src/watch.rs`, `registry/ecosystem.toml`). Einträge ohne Prüfbefehl werden nur als „ungeprüft“ angezeigt und nie von selbst angehoben. Warnungen laufen über `--notify-file` von version-control.
- **Bereitstellung**: Die Infoseite ist `webpage/index.html`. Auf dem VPS: `/root/easy-web.tokyo/pl2pl` (Aktualisieren mit `git pull`; statisch, also kein Neustart), `pl2pl-web.service` auf Port 8142, bereits in `domains.toml` eingetragen.
- **Stolperfallen**: Dateien mit Japanisch mit Python `-X utf8` schreiben. Shell-Skripte in LF halten. Git Bash ändert `/mnt/...`-Pfade. Für Text mit vielen Anführungszeichen oder `\` die Werkzeuge Write / Edit nutzen, kein Heredoc.
- **COBOL_to / Java_to**: schon in `converters/` übernommen. Das Löschen der ursprünglichen Repositorys lässt sich nicht rückgängig machen, deshalb vorher die endgültige Zustimmung einholen.
- **Übersetzungssystem** (menschliche Sprache) bleibt getrennt von den Konvertern. Die Konverter verlassen sich nicht auf ein LLM.
- **Sprachen der Dokumente**: Die japanischen Dateien sind die Originale. Die englischen Dateien wurden in einfachen Worten neu geschrieben. Die übrigen Sprachen in `docs/i18n/` sind KI-Übersetzungen (noch nicht von Muttersprachlern geprüft). Ältere Geschichte: `docs/HISTORY-ja.md`.

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

## 2026-11-25 Stand und offene Punkte

Stand: ein Werkzeug, das alte Programme (COBOL, C, C++, Fortran, Java u. a.) in die gewählte Sprache und das gewählte Framework umschreibt. Zuletzt wurde die PetClinic-Unterstützung von Java_to (ResponseEntity, Mapper, übergeordnete Datenklassen) übernommen und die mehrsprachige Dokumentation abgeglichen. Offen: Prüfung auf einem echten Mac und iPhone (solche Geräte haben wir hier nicht). Übersetzungen: KI-übersetzt (außer Japanisch und Englisch), noch nicht von Muttersprachlern geprüft.

## 2026-11-25 Nachtrag: Abgleich der Konverter

In `converters/cobol_to` wurde von COBOL_to das Umsetzen von EXEC CICS LINK/XCTL in CALL übernommen (`7d67bbf`), in `converters/java_to` von Java_to der ternäre Operator (in Rust-Rümpfen, `e9ac300`). Tests: Java_to 39, COBOL_to 25, pl2pl 55, alle bestanden. Offen: echte Mac- und iPhone-Geräte sowie der Start des erzeugten FastAPI mit einer echten Datenbank.

## 2026-11-25 Nachtrag: Auswahl bei mehreren Implementierungen per Test festgeschrieben

Hat eine Repository-Schnittstelle mehrere Implementierungen, bevorzugt der Generator eine, die im Speicher läuft (ohne Jdbc oder Jpa im Namen), und nimmt sonst die erste; das war bereits umgesetzt (der frühere Hinweis „nicht unterstützt“ war veraltet). Jetzt ist es durch den Unit-Test `multiple_impls_prefer_non_db_one` festgeschrieben (Java_to: 40 bestanden). Offen: komplexe MapStruct-Mapper und der Start mit einer echten Datenbank.

## 2026-11-25 Nachtrag: MapStruct-@Mapping unterstützt

`@Mapping(target=..., source="a.b")` (verschachtelte Quellattribute), `ignore=true` und `constant="..."` an Mapper-Methoden werden jetzt an das erzeugte Python `MapperBase._to(cls, src, rename, ignore, const)` weitergegeben. Abgedeckt durch den Unit-Test `mapstruct_mapping_annotations` (Java_to: 41 bestanden) und eine Prüfung zur Laufzeit (`owner.name` zu `fullName`, ignore, Konstante). Offen: die verschachtelte Schreibweise `@Mappings({...})`, `expression=`, `@MappingTarget` und der Start mit einer echten Datenbank.

<!-- directive-20261125-1130 -->
## 2026-11-25 Anweisung (bis 11:30 keine Rückfrage nötig)
Bis 2026-11-25 11:30 wird Unerledigtes ohne Rückfrage automatisch bis zur Fertigstellung vorangetrieben. Auch wenn die Nachfrage gering oder die Notwendigkeit niedrig erscheint oder alles von Grund auf neu gebaut werden müsste, weichen wir nicht auf ein anderes Verfahren aus, sondern bauen es richtig neu. Gemeinsam entwickelt werden: COBOL_to, Java_to, MFastAPI, 2MFastAPI (von anderen Sprachen und Frameworks zu MFastAPI-kompatiblem Code; Node.js und TypeScript sind eine optionale Ausnahme, und man kann auch beides nicht wählen), RMojo, RPoRea (Integration von RPoem und Reaxt), 2RPoRea (von anderen Sprachen und Frameworks zu RPoRea) und Reaxt (Integration von React, Next.js, Nuxt.js und Nest.js). Ändert sich eines davon, werden die anderen geprüft, und README, CLAUDE, PORTING und POLICY werden am selben Tag auf Japanisch und Englisch aktualisiert.

<!-- directive-20261125-mojo -->
## 2026-11-25 Nachtrag: MFastAPI muss aus Mojo nutzbar sein
MFastAPI behält die **Vorgabe, aus Mojo nutzbar zu sein** (Python-Syntax, Rust-Implementierung, Mojo-Unterstützung; unter Windows wird das echte Mojo in WSL über `compat/wsl-bridge` von RMojo angesteuert). Auch das Ergebnis von 2MFastAPI muss aus Mojo nutzbar sein. Aktueller Stand: `mojo/` ist eine Referenzimplementierung, und die Server der Stufen B und C sind experimentell. Als „geprüft“ bezeichnen wir nur, was wirklich ausgeführt wurde. Bis 11:30 geht es ohne Rückfrage automatisch weiter.

## Versionless-API-Richtlinie (2026-11-25)
Die APIs dieses Repositorys sind **Versionless**: keine `v1` o. Ä. in URLs oder Paketnamen; sie entwickeln sich nur durch Hinzufügen weiter, ohne die Kompatibilität zu brechen. Felder werden nur hinzugefügt, neue Felder sind optional, unbekannte Felder werden ignoriert, und eine Umbenennung geschieht, indem man den neuen Namen hinzufügt und den alten als veraltet kennzeichnet (Frist standardmäßig mindestens 6 Monate; gelöscht wird erst, wenn nachweislich nichts mehr den alten Namen nutzt). Enums müssen unbekannte Werte tolerieren. Ausnahme sind dringende Sicherheitskorrekturen. Bestehende `.v1`-Namen bleiben vorerst, damit Aufrufer nicht brechen; es wird ein versionsloser Alias hinzugefügt, und `.v1` wird schrittweise abgeschafft. Einzelheiten und Quellen: am Ende von [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md). Die Anwendung folgt der Reihe nach.

## API-Richtlinie: REST abschaffen, auf gRPC / Connect umstellen
Dieses Repository legt **keine neuen REST-APIs** an. APIs, die vom Browser oder mit `curl` aufgerufen werden, nutzen **einfache Connect-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`); zwischen Diensten wird **gRPC / gRPC-web** verwendet. GraphQL ist nur lesend. Bestehendes REST wird umgestellt, sobald man es anfasst, und die alte Route wird danach entfernt. JSON wird mit RS-JSON gelesen (`from_str_secure` / `parse_secure`). Ausnahmen sind nur Formen, die ein fremder Standard vorgibt (ACME, OAuth/OIDC-Callbacks, Webhooks von Dritten, Health-Checks, statische Dateien, OTLP). Im Zweifel Connect. Maßgebliche Quelle: [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md).
