# Parsing Main Arguments
Die anderen Beispielprojekte enthalten keine Argumentenübergabe beim Start der Anwendung. Die *main*-Function enthält bei Rust keine Parameter (Java-Vergleich: *public static void main(String [] args)*). Dennoch können Parameter als Liste angegeben und ausgelesen werden.

Die Grundversion des Programmes wurde einer [externen Quelle](http://rustbyexample.com/std_misc/arg.html) entnommen.

Der Aufruf erfolgt mit `cargo run -- <param 1> <param 2> <param ...> <param n>`. Demnach wird der `cargo run`-Befehl von einem ` -- ` getrennt und mit führendem Leerzeichen eine leerzeichen-Separierte Parameter-Liste angegeben.

Eine Beispielausgabe sieht wie folgt aus:

        $ cargo run -- 1 2 3
             Running `target/debug/parsing_main_arguments 1 2 3`
        My path is target/debug/parsing_main_arguments.
        I got 3 arguments: ["1", "2", "3"].
