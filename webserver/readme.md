# Webserver
Dies ist eine Mini-Webserver-Anwendung, geschrieben in der Sprache **Rust**. Als Framework wird **[nickel](http://nickel.rs/)** eingesetzt, welches ebenfalls komplett in Rust geschrieben wurde. Es wird kein Fehlerhandling o.ä. umgesetzt.

Der Webserver wird durch Cargo gestartet:

        cargo run

Die Webseite ist anschließend unter der Adresse `127.0.0.1:8080` zu erreichen.


# Beispieloutput
Ein Beispielhafter Output im Terminal.

        $ cargo run
        Compiling nickel_example v0.1.0 (file://.........../FSE-Rust/webserver)
         Running `target/debug/nickel_example`
        Listening on http://127.0.0.1:8080
        Ctrl-C to shutdown server
        Requested url: /
        Requested url: /sendData
        Got parameters: 'name=Meier'
        ^C
        $
