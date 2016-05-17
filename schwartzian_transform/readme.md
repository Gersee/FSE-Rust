# Schwartzian Transform

Dies ist eine Implementierung der Schwartzian Tranformation mit Rust.

Es wird ein Mitarbeiter mit Personalnummer und Salery generiert. Ein Vergleich findet statt zwischen der Rust-eigenen *sort_by* Methode und der implementierten Schwartzian Transformation. Standardmäßig werden 50.000 Mitarbeiter generiert. In der gewählten Implementierung der *Schwartzian Transformation* finden zwei Kopiervorgänge statt.

- Eingehender Vector mit Struct-Objekten wird in einen Vector aus Tuplen mit Struct und Saleryangabe überführt
- Sortierter Tuple-Vector wird in einen Rückgabevector ohne Saleryangabe überführt

Aus diesem Grunde gibt es mehrere Messungen:

- Messung der Rust-nativen *sort_by* Sortierung
- Messung der Schwartzian Transformation in Summe
- Messung ausschließlich des ersten Kopiervorgangs der Schwartzian Transformation
- Messung ausschließlich des reinen Sortiervorgangs des Tuple-Vectors innerhalb der Schwartzian Transformation
- Ausgabe der Summe aus ersten Kopiervorgang und des Sortiervorgangs innerhalb der Schwartzian Transformation

Beispielhafte Ausgabeergebnisse:

        $ cargo run
             Running `target/debug/schwartzian_transform`
        Compare of Schwartzian Transoformation and Rust Sort
        Runtime Schwartzian Transformation (Copy-to-tuple): PT0.011353355S
        Runtime Schwartzian Transformation (Inner-Sort): PT0.063243163S
        Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): PT0.074596518S
        Runtime Schwartzian Transformation (Complete): PT0.087531174S
        Runtime Rust Sort: PT0.074163726S

        $ cargo run
             Running `target/debug/schwartzian_transform`
        Compare of Schwartzian Transoformation and Rust Sort
        Runtime Schwartzian Transformation (Copy-to-tuple): PT0.012413382S
        Runtime Schwartzian Transformation (Inner-Sort): PT0.058601131S
        Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): PT0.071014513S
        Runtime Schwartzian Transformation (Complete): PT0.093673215S
        Runtime Rust Sort: PT0.069313076S

        $ cargo run
             Running `target/debug/schwartzian_transform`
        Compare of Schwartzian Transoformation and Rust Sort
        Runtime Schwartzian Transformation (Copy-to-tuple): PT0.011878340S
        Runtime Schwartzian Transformation (Inner-Sort): PT0.062821830S
        Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): PT0.074700170S
        Runtime Schwartzian Transformation (Complete): PT0.087366034S
        Runtime Rust Sort: PT0.079702741S

Es ist bei diesen Testläufen deutlich zu erkennen, dass der reine Sortiervorgang der Schwartzian Transformation schneller als der *sort_by*-Algorithmus ist. Allerdings entsteht ein Overhead durch die Kopiervorgänge. Bei den erzeugten 50.000 Mitarbeitern ist der *Inner-Sort + Copy-to-tuple* etwa gleich schnell wie der Rust-Sort-Algorithmus.

Wird die Anzahl der erzeugten Mitarbeiter zu einem größeren Wert verändert, werden Geschwindigkeitsvorteile der Schwartzian Transformation sichtbarer. Wird der Wert verkleinert, so wirkt der *sort-by*-Algorithmus effizienter.

In diesem Falle liegt der Wert als Attribut vor. Sollte dieser jedes mal berechnet werden, dürfte die Schwartzian Transformation auch bei kleineren Größenordnungen deutlich besser abschneiden.
