> Hinweis: Das japanische PORTING.md im Wurzelverzeichnis ist das Original. Diese Fassung wurde von einer KI übersetzt und von einer KI auf muttersprachlich natürliche Formulierung geprüft; ein Mensch mit dieser Muttersprache hat sie noch nicht geprüft.

# pl2pl – Übergabe
(Ältere, ausführliche Notizen stehen in `docs/HISTORY-ja.md`, auf Japanisch.)

## Wo wir stehen (2026-11-19)
- Version 0.1.0 ist veröffentlicht, mit Installern für Windows, Linux und Mac sowie Tests in der CI.
- PHP → FastAPI ist mit echtem FastAPI durchgängig getestet: Routen, Eloquent, Query Builder, Blade, Middleware, Beziehungen und die `@can`-Familie.
- `pl2pl web` erkennt jetzt auch PHP.
- `pl2pl watch` findet neue Versionen, testet sie, behebt Probleme und schickt eine Warnung, wenn nichts hilft. Auch auf GitHub wird jeden Abend und jeden Morgen mit den neuesten Versionen getestet.
- `pl2pl ir` (Testversion) bringt die Routen von PHP, Python und Java (Spring, JAX-RS, Servlet, Spark) in eine gemeinsame Form und schreibt JSON, OpenAPI oder ein FastAPI-Gerüst. Die IR trägt auch typisierte Parameter, Funktionsinhalte, Modelle, Middleware und Vorlagen, und `pl2pl php` schreibt seine Ausgabe jetzt über den gemeinsamen Ausgabeteil in `src/ir_emit.rs` (Ausgabe unverändert gegenüber vorher; 49 End-to-End-Prüfungen bestehen).
- Die Seite mit Infos und Downloads ist online: <https://easy-web.tokyo/pl2pl/> (auf dem VPS, statisch). Auch auf der Startseite von easy-web.tokyo gibt es eine Karte dafür.
- COBOL_to und Java_to sind in `converters/` übernommen.

## Noch zu tun (Wichtigstes zuerst)
1. **Prüfen**: Kommt eine echte E-Mail an (`version-control --test-notify`; dieser PC hat keine SMTP-Einstellungen)? Funktioniert die automatische Reparatur über version-control `--loop`? Test mit dem neuesten fastapi (der nächtliche Lauf auf GitHub prüft das).
2. Typen und Funktionsinhalte in die IR bringen. Die jetzige FastAPI-Ausgabe (`pl2pl php`) über die IR laufen lassen. Wenn die Routensuche etwas übersieht, tree-sitter in Betracht ziehen.
3. `watch` wurde mit einer echten neuen Version ausprobiert: Mit fastapi 0.165.0 (starlette 1.10) bestand `tests/run-php.sh` 49 Prüfungen, daher rückte die getestete Version in der Registry auf 0.165. Offen: Die neuen laravel 13 / django 6.1 / spring-boot 4.1 bleiben „ungeprüft“, weil die End-to-End-Tests die echten Frameworks nicht nutzen (nur Meldung, bis es solche Tests gibt).
4. Die nächtlichen und morgendlichen Läufe auf einem PC einrichten (noch nicht getan: Der Prüfbefehl läuft auf diesem PC nicht und würde Fehlalarme geben; ein Beispiel steht oben in `scripts/pl2pl-watch.cmd`).
5. (Erledigt) Eine pl2pl-Karte steht auf der Startseite von easy-web.tokyo und ist produktiv online (open-easy-web `95cb1e7`; die alte Binärdatei ist `server/target/release/open-easy-web-server.before-pl2pl-card-20261119`).
6. PHP (erledigt, mit Grenzen): Blade-`@php` wandelt jetzt Zuweisungen und `++` `--` `+=` `-=` `*=` `/=` `.=` in `{% set %}` um; `$attributes` funktioniert als Ausgabe, außerdem `->merge` (Textwerte oder umwandelbare Ausdrücke), `->class([...])` (eine Liste von Texten), `->only` und `->except`; ebenso `morphToMany` (Typspalte in der Zwischentabelle) und Blade-Komponenten als Klassen (`app/View/Components/*.php`: Konstruktorargumente werden zu Standardwerten und `_props`, und ein `view('x.y')` in `render()` wird verfolgt) (55 End-to-End-Prüfungen, 41 Unit-Tests). Noch offen: öffentliche Methoden einer Komponentenklasse (ein `{{ $method() }}` in ihrer Vorlage bleibt ein TODO, und der Bericht zählt sie), `->class()` mit Schlüsseln (bedingt) sowie `@php` mit if/foreach oder Funktionsaufrufen (bleibt ein TODO).
7. Anbindung an aruaru-llm (erledigt): Am 2026-11-19 haben wir aruaru-llm v0.2.5 übernommen und ein echtes `pl2pl llm setup` ausgeführt. Die automatische Wahl, 3B, scheiterte, weil ihre Gewichte in Teildateien (Shards) aufgeteilt sind; deshalb weicht die Einrichtung jetzt auf das nächstkleinere Modell aus (nicht, wenn man eines fest vorgibt). `GenerateQwen` liefert Text mit qwen2.5-1.5b-instruct (rund 2,9 GB, auf der CPU). **`pl2pl php --explain`** (optional; `--explain-max N`, Standard 3) fügt unter jedem TODO-Punkt des Berichts eine KI-Notiz hinzu. Gemessen: etwa 2 Minuten pro Punkt auf der CPU, und **die Qualität ist schlecht und manchmal falsch** (1 von 2 lag daneben), deshalb ist es standardmäßig aus. Ein neu gestartetes aruaru-llm hat kein Modell gewählt (400), daher wählt es zuerst ein installiertes. **Ein größeres Modell (rund 12 GB)**: Das größte im Katalog von aruaru-llm ist 3B (rund 6,2 GB), und 3B und 7B werden als mehrere Shard-Dateien geliefert, die der jetzige Installer nicht holen kann. Zuerst kommen Shard-Unterstützung (in aruaru-llm) und die CPU-Geschwindigkeit (schon bei 1.5B so langsam). Kokkos auf der GPU und Kernel auf open-cuda / open-directx sind noch ungeprüft. **GPU (geprüft am 2026-11-19)**: Auf einer GeForce GT 730 (2 GB, sm_35, Treiber 475 = Klasse CUDA 11.4) haben wir den CUDA-11.8-nvcc ohne Administratorrechte über micromamba (NVIDIAs conda-Kanal) installiert, einen einfachen CUDA-Kernel und die Ausgabe von `pl2pl gpu --target open-cuda` (`vec_loop12.cu`) auf der echten GPU ausgeführt und 0 Abweichungen gegenüber der CPU gesehen (`tests/gpu-check.ps1`). Stolpersteine: MSVC 14.44 braucht `-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH` und `-allow-unsupported-compiler`; PyPI hat kein nvcc für Windows; CUDA 12 kann sm_35 nicht als Ziel nehmen. **Noch ungeprüft**: das Kokkos-CUDA-Backend sowie der Lauf auf open-cuda (der Rust-Umsetzung) und open-directx.
8. Die ursprünglichen COBOL_to und Java_to löschen (**nicht rückgängig zu machen, deshalb vorher die endgültige Zustimmung einholen**).
9. `docs/DESIGN.md` schreibt der Eigentümer. Wir haben `docs/LANGUAGES.md` am 2026-11-19 neu gelesen und nichts gefunden, was nicht zum heutigen Stand passt (für Java und COBOL gibt es Konverter). Es ist weiterhin eine Liste aus Vorwissen und nicht gegen eine frische Websuche geprüft.

## Tests
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP durchgängig) / `cargo test --release`

## Nachricht zum Weitermachen
„F:\pl2pl fortsetzen. ‚Noch zu tun‘ in PORTING.md von oben abarbeiten. Was sich nicht umschreiben lässt, nie verstecken, sondern als TODO stehen lassen. Erst durch echtes Ausführen prüfen, dann Erledigung melden. Antworten nur auf Japanisch. Bei jedem Schritt committen und pushen, auch den VPS aktualisieren.“

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

## 2026-11-25 Nachtrag: c2rust – Arrays, break/continue

`pl2pl c2rust` beherrscht jetzt Arrays fester Länge (`int a[8] = {...};`, `double w[4];`, fehlende Startwerte werden mit 0 aufgefüllt), lesende und schreibende Indexzugriffe (`a[j + 1]` wird zu `a[(j + 1) as usize]`), `break;`, `continue;` sowie `while (1)` als `loop`. Ein Beispiel mit Bubblesort und `while (1)` wurde in WSL mit gcc und rustc ausgeführt, und die Ausgaben stimmen vollständig überein (`1 2 3 4 5 7 8 9` / `6.00 12.00 9`). 56 Unit-Tests bestehen. Noch nicht: Zeiger, Strukturen, zweidimensionale Arrays, Arrays als Funktionsargumente und implizite Umwandlung zwischen Ganz- und Kommazahlen (`k * 1.5` muss als `(double)k` geschrieben werden).
