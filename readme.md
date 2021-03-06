# Allgemeines
In diesem Repository befinden sich Beispiele, geschrieben mit der Sprache **[Rust](https://www.rust-lang.org/)**. Sie dienen dem Verständnis der Sprache bzw. der Demonstration dessen Möglichkeiten. Diese Beispiele sind entstanden im Rahmen des Master-Moduls *Fortgeschrittenes Software Engineering* an der *[FH Münster](https://www.fh-muenster.de/)*.

Die hier bereitgestellten Beispiele erheben keinen Anspruch auf maximal effizienten Code bzw. bestmögliche Code-Eleganz. Die Beispiele dienen dazu, verschiedene Programmierkonstrukte von Rust darzustellen und zu verdeutlichen sowie der Lösungsdarstellung algorithmischer Problemstellungen.

Projekte können über das Terminal / die Konsole gestartet werden. Hierfür ist der Ordner der Anwendung zu öffnen, z.B. `cd guessing_game`. Das Projekt kann mit dem Befehl `cargo run` gestartet werden. Bedingung ist selbstverständlich die [Installation von Rust](https://www.rust-lang.org/downloads.html) auf dem Device.

**Wichtig:** Die Beispiel-Programme wurden mit [Atom](https://atom.io/) bearbeitet und über die Konsole / das Terminal erstellt. Ein direkter Import in *Eclipse* ist zum jetzigen Zeitpunkt **nicht** ohne Mehraufwand möglich. Folgender Workaround steht zur Verfügung: Anlegen eines neuen Rust-Projektes in Eclipse und über *Import->Filesystem* das Beispiel-Projekt in das gerade angelegte Eclipse-Rust-Projekt importieren.

Soll ein neues Projekt über das Terminal / die Konsole erstellt werden, so kann `cargo new myProject --bin` verwendet werden. Neben einer grundsätzlichen Projektstruktur und der Anlage von der *main.rs* sowie der *cargo.toml* wird auch ein *Git-Repository* initialisiert, sofern Git auf dem Device verfügbar ist.  

Ein Release-Build kann erstellt werden mit `cargo build --release`. Hierbei dauert der Compilevorgang länger, da *Optimierungen* durchgeführt werden. Das binary-file ist unter `cd target/release` zu finden.

---

# Enthaltene Beispiele

- *advanced_documentation*: Aufzeigen von Möglichkeiten der RustDoc-Generierung
- *automatic_testing*: Durchführung automatischer Tests in Rust
- *c-integration*: Einbindung einer C-Bibliothek
- *concurrency*: Nebenläufiges Programm
- *guessing_game*: Ein Projekt bei dem eine Zahl zwischen 1 und 100 geraten werden soll
- *hash_map*: Programm zur exemplarischen Nutzung einer HashMap und eines Arrays
- *lambda*: Beispiel für die Verwendung von Lambda-Expressions
- *logging*: Ausgabe von Log-Meldungen durch externes Crate
- *multi_file*: Exemplarische Implementierung des Aufrufs von Funktionen aus mehreren Dateien
- *n-queens_problem*: Ein Algorithmus zur Lösung des n-Damen-Problems
- *parsing_main_arguments*: Übergabe von Startparametern
- *pi_concurrency*: Umsetzung eines Multi-Threading-Algorithmus zur Berechnung von Pi
- *recursion-ecample*: Simpler rekursiver Aufruf
- *schwartzian_transform*: Implementierung der Schwartzian-Transformation mit Rust
- *shadowing*: Exemplarische Darstellung der shadowing-Eigenschaft
- *struct_trait*: Beispiel für die Verwendung von Structs und Traits
- *towers_of_hanoi*: Algorithmus zur Lösung der Tower-of-Hanoi-Aufgabe
- *webserver*: Ein in Rust geschriebener beispielhafter Webserver
- *xml_counter*: Programm für eine exemplarische Verarbeitung eines XML-Dokumentes

---

# Einige Rust-Begriffe

| Begriff | Bedeutung |
| ---|---|
| crates | Vergleichbar mit Libraries. |
| macro | Vergleichbar mit Funktionen - gekennzeichnet durch abschließendes ! - allerdings wird beim Kompilieren keine Sprungadresse sondern der Quellcode eingefügt. |
| moustaches | Bracelets: {} z.B. bei Verwendung als Platzhalter bei Ausgaben. |
| panic! | Beendet mit Nachricht den Thread und löst alle Resourcenbelegungen, bei 1-Thread: Programmende. |
| Rustaceans | Bezeichnung für Rust-Entwickler. |
| Shadowing | Wiederverwendung eines Variablennamens in einem inneren Scope ohne die äußere Definition zu verändern. |
| snake_case | Im Gegensatz zum CamelCase (Höcker für immer auftretende Großbuchstaben zur Wortabgrenzung) wird bei snake_case alles in Kleinbuchstaben, getrennt durch Unterstrichen (_) geschrieben. Ist der Standard bei Rust, der Compiler warnt bei Nichteinhaltung. Lediglich bei Structs und Traits wird CamelCase gefordert. |
| Struct | Komplexer Datentyp mit der Möglichkeit einer Methodenimplementierung. |
| Trait | Garantieren Methodenimplementierung eines Structs, Vergleichbar mit Java-Interfaces. |
| _ | Else-Zweig einer If-Anweisung, eines match-Statements oder eines Result-Types, z.B. *Err(_) => ...*. |

---

# Links

Nachfolgend finden sich Links zum Thema Rust.

## Doku & Tutorials

- [Offizielle Website](https://www.rust-lang.org/)
- [Rust@Github](https://github.com/rust-lang/rust)
- [Rust Release Notes](https://github.com/rust-lang/rust/blob/stable/RELEASES.md)
- [Cargo.io - Crate host](https://crates.io/)
- [Rust book](https://doc.rust-lang.org/book/)
- [Rust book auf deutsch](https://rust-lang-de.github.io/rustbook-de/index.html)
- [Rust by example](http://rustbyexample.com/)
- [Cargo guide](http://doc.crates.io/guide.html)
- [Rust Syntax index](https://doc.rust-lang.org/book/syntax-index.html)
- [Rust error handling](http://blog.burntsushi.net/rust-error-handling/)
- [Rust Compiler Error Index](https://doc.rust-lang.org/error-index.html)
- [Rustonomicon - Advanced Programming with Rust](https://doc.rust-lang.org/nomicon/)
- [Rust-Rosetta examples](https://github.com/Hoverbear/rust-rosetta/tree/master/src)
- [Rust Standard-Library](https://doc.rust-lang.org/std/)
- [Rust Reference](https://doc.rust-lang.org/reference.html)

## Referenzprojekte

- [Rustc - Rust Compiler](https://manishearth.github.io/rust-internals-docs/rustc/)
- [Rustdoc - Rust Documentation Generator](https://manishearth.github.io/rust-internals-docs/rustdoc/index.html)
- [Cargo.io - Crate host](https://crates.io/)
- [Google-XI - Editor with rust-Backend](https://github.com/google/xi-editor)
- [Google hat-backup - Backend-agnostic snapshotting backup system](https://github.com/google/hat-backup)
- [Servo - Parallel Browser Engine](https://servo.org/)
- [Skylight - Ein Performance-Test-Framework für Ruby](https://www.skylight.io/)
- [Nickel - Web Application Framework](http://nickel.rs/)
- [Piston - Game Engine](https://github.com/PistonDevelopers/piston)
- [Redox - Betriebssystem](https://github.com/redox-os/redox)
- [CGMath - Linear Algebra Tool](https://github.com/bjz/cgmath)
- [iota - Text editor](https://github.com/gchp/iota)
- [Rust-Q3 - Quake3 like Game](https://github.com/jeaye/q3)
- [intellij-Rust - Intellij-based IDE for Rust](https://github.com/intellij-rust/intellij-rust)
- [Serde-RS - Serialized Rust Data Structues](https://github.com/serde-rs/serde)
- [lettre - SMTP library](https://github.com/lettre/lettre)
- [rust-FTP - FTP Client](https://github.com/mattnenterprise/rust-ftp)
- [trust-DNS - DNS Server](https://github.com/bluejekyll/trust-dns)
- [mysql-simple - mySQL - Client](https://github.com/blackbeam/rust-mysql-simple)

## Rust-Projektsammlungen

- [Awesome-Rust](https://github.com/kud1ing/awesome-rust)
- [GitHub Trending: Rust](https://github.com/trending/rust)
- [Crates.io](https://crates.io/)
- [Rustkit](http://rustkit.io/)
