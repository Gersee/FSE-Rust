# Allgemeines
In diesem Repository befinden sich Beispiele, geschrieben mit der Sprache **[Rust](https://www.rust-lang.org/)**. Sie dienen dem Verständnis der Sprache bzw. der Demonstration dessen Möglichkeiten. Diese Beispiele sind entstanden im Rahmen des Master-Moduls *Fortgeschrittenes Software Engineering* an der *[FH Münster](https://www.fh-muenster.de/)*.

---

# Enthaltene Beispiele

- *guessing_game*: Ein Projekt wo eine Zahl zwischen 1 und 100 geraten werden soll
- *hash_map*: Programm zur exemplarischen Nutzung einer HashMap mit Nutzung der Iteration über ein Array
- *lambda*: Beispiel für die Verwendung von Lambda-Expressions
- *multi_file*: Exemplarische Implementierung des Aufrufs von Funktionen aus mehreren Dateien
- *n-queens_problem*: Ein Algorithmus zur Lösung des n-Damen-Problems
- *pi_concurrency*: Umsetzung eines Multi-Threading-Algorithmus zur Berechnung von Pi
- *recursion-ecample*: Simpler rekursiver Aufruf
- *struct_trait*: Beispiel für die Verwendung von Structs und Traits
- *towers_of_hanoi*: Ein Algorithmus zur Lösung der Tower-of-Hanoi-Aufgabe
- *webserver*: Ein in Rust geschriebener beispielhafter Webserver
- *xml_counter*: Ein Programm für eine exemplarische Verarbeitung eines XML-Dokumentes

---

# Einige Rust-Begriffe

| Begriff | Bedeutung |
| ---|---|
| crates | Vergleichbar mit Libraries. |
| macro | Vergleichbar mit Funktionen - gekennzeichnet durch abschließendes ! - allerdings wird beim Kompilieren keine Sprungadresse sondern der Quellcode wird an der Stelle eingefügt. |
| moustaches | Bracelets: {} z.B. bei Verwendung als Platzhalter bei Ausgaben. |
| panic! | Beendet mit Nachricht den Thread und löst alle Resourcenbelegungen, bei 1-Thread: Programmende. |
| Rustaceans | Bezeichnung für Rust-Entwickler. |
| Shadowing | Wiederverwendung einer Varaible auch mit anderem Typ sobald die Ursprungsvariable den Scope verlassen hat. |
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
