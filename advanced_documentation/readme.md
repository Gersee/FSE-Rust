# Advanced Documentation
Beispielhafte Anwendung, welche verschiedene Möglichkeiten der Generierung von Dokumentationen aus den Rust-Kommentaren ermöglicht. Doc-Kommentare beginnen mit drei Slashes /// und können Markdown-Styles enthalten. Der gewünschte Aufbau ist im [RFC505](https://github.com/rust-lang/rfcs/blob/master/text/0505-api-comment-conventions.md) vorgegeben.

Die Doc-Kommentare können *Code-Beispiele* enthalten. Ein Cloud: Mit `cargo test` werden diese getestet, sofern die Datei *lib.rs* heißt, also als Library gekennzeichnet ist. In diesem Repo muss die Datei *comment-lib.rs* dafür umbenannt werden. Alternativ ist im Projekt **automatic_testing** dieses Repositories ein Dokumentations-Test enthalten.

Eine HTML-Documentation kann mit `cargo doc` erstellt werden. Anschließend ist diese zu finden im Pfad

        <projectName>/target/doc/<projectName>/index.html
        In diesem Beispiel:
        advanced_documentation/target/doc/advanced_documentation/index.html
