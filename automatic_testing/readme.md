# Automatic Testing
Beispiel zur automatischen Testdurchführung. Tests müssen annotiert werden. Im *Tests*-Ordner liegen eizelne Testfälle, welche u.a. assert-Statements durchführen und negative Testfälle abdecken. Im *src*-Ordner gibt es einen **Dokumententest**, welcher die Beispiele in den *Doc-Comments* durchführt und testet.
Kommentare können ausführbare Beispeile enthalten und werden mit *markdown* formatiert bei Bedarf in HTML-Dokumentationen umgewandelt.

Die Ausführung der Tests erfolgt über den Befehl `cargo test` auf der Konsole.

Die Grundversion des Programmes wurde einer [externen Quelle](https://rust-lang-de.github.io/rustbook-de/book/Testen.html) entnommen.


Beispielausgabe der Testausführung:

        $ cargo test
           Compiling automatic_testing v0.1.0 (...)
             Running target/debug/automatic_testing-c2c27b9c2cae4bcb

        running 1 test
        test tests::documentation_testing ... ok

        test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

             Running target/debug/lib-88a3d74c34f9fca2

        running 6 tests
        test ignored_test ... ignored
        test function_without_doing ... ok
        test assert_should_panic ... ok
        test expected_panic_failed_assertion ... ok
        test test_uses_non_testing_function::it_works ... ok
        test wrong_assert ... FAILED

        failures:

        ---- wrong_assert stdout ----
        	thread 'wrong_assert' panicked at 'assertion failed: false', tests/lib.rs:7


        failures:
            wrong_assert

        test result: FAILED. 4 passed; 1 failed; 1 ignored; 0 measured
