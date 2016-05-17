# Schwartzian Transform

Dies ist eine Implementierung der Schwartzian Tranformation mit Rust.

Es wird ein Mitarbeiter mit Personalnummer und Salery generiert. Ein Vergleich findet statt zwischen der Rust-eigenen *sort_by* Methode und der implementierten Schwartzian Transformation. Standardmäßig werden 50.000 Mitarbeiter generiert. In der gewählten Implementierung der *Schwartzian Transformation* finden zwei Kopiervorgänge statt.

- Eingehender Vector mit Struct-Objekten wird in einen Vector aus Tuplen mit Struct und Saleryangabe überführt
- Sortierter Tuple-Vector wird in einen Rückgabevector ohne Saleryangabe überführt

Aus diesem Grunde gibt es mehrere Messungen, jeweils für Attribut- und Methoden-Berechnungen:

- Messung der Rust-nativen *sort_by* Sortierung
- Messung der Schwartzian Transformation in Summe
- Messung ausschließlich des ersten Kopiervorgangs der Schwartzian Transformation
- Messung ausschließlich des reinen Sortiervorgangs des Tuple-Vectors innerhalb der Schwartzian Transformation
- Ausgabe der Summe aus ersten Kopiervorgang und des Sortiervorgangs innerhalb der Schwartzian Transformation

Der Unterschied zwischen Attribut- und Methoden-Berechnungen liegt in dem Vorliegen der Daten. Bei den Attributen liegen die Sortierkriterien direkt als Attribut vor. Bei den Methoden-Berechnungen muss das Sortier-Kriterium jeweils aus einem Attribut nach einer Berechnungsvorschrift erzeugt werden. Die Berechnungsvorschrift lautet: `((salery * pi) * pi).sqrt().ln().exp()`. Dabei ist *salery* das vorhandene Attribut. Die Funktion *exp()* setzt die vorher berechnete Zahl als Exponent zu *e* ein. Bei den Berechnungen finden zudem *Cast*-Vorgänge statt.

Beispielhafte Ausgabeergebnisse:

        $ cargo run
             Running `target/debug/schwartzian_transform`
        Compare of Schwartzian Transoformation and Rust Sort by Attribut and by Method (with calculation
        Attribut: Runtime Schwartzian Transformation (Copy-to-tuple): PT0.011221491S
        Attribut: Runtime Schwartzian Transformation (Inner-Sort): PT0.057658665S
        Attribut: Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): PT0.068880156S
        Attribut: Runtime Schwartzian Transformation (Complete): PT0.084805962S
        Attribut: Runtime Rust Sort: PT0.078285134S
        Method: Runtime Schwartzian Transformation (Copy-to-tuple): PT0.014257219S
        Method: Runtime Schwartzian Transformation (Inner-Sort): PT0.059524846S
        Method: Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): PT0.073782065S
        Method: Runtime Schwartzian Transformation (Complete): PT0.085176302S
        Method: Runtime Rust Sort: PT0.196385185S

        $ cargo run
             Running `target/debug/schwartzian_transform`
        Compare of Schwartzian Transoformation and Rust Sort by Attribut and by Method (with calculation)
        Attribut: Runtime Schwartzian Transformation (Copy-to-tuple): PT0.015436704S
        Attribut: Runtime Schwartzian Transformation (Inner-Sort): PT0.059299850S
        Attribut: Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): PT0.074736554S
        Attribut: Runtime Schwartzian Transformation (Complete): PT0.089479936S
        Attribut: Runtime Rust Sort: PT0.079105180S
        Method: Runtime Schwartzian Transformation (Copy-to-tuple): PT0.015189650S
        Method: Runtime Schwartzian Transformation (Inner-Sort): PT0.060955657S
        Method: Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): PT0.076145307S
        Method: Runtime Schwartzian Transformation (Complete): PT0.090725483S
        Method: Runtime Rust Sort: PT0.221055194S

        $ cargo run
             Running `target/debug/schwartzian_transform`
        Compare of Schwartzian Transoformation and Rust Sort by Attribut and by Method (with calculation)
        Attribut: Runtime Schwartzian Transformation (Copy-to-tuple): PT0.009624766S
        Attribut: Runtime Schwartzian Transformation (Inner-Sort): PT0.064531386S
        Attribut: Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): PT0.074156152S
        Attribut: Runtime Schwartzian Transformation (Complete): PT0.087052338S
        Attribut: Runtime Rust Sort: PT0.069171239S
        Method: Runtime Schwartzian Transformation (Copy-to-tuple): PT0.013358203S
        Method: Runtime Schwartzian Transformation (Inner-Sort): PT0.066107548S
        Method: Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): PT0.079465751S
        Method: Runtime Schwartzian Transformation (Complete): PT0.090564200S
        Method: Runtime Rust Sort: PT0.206303583S

        $ cargo run
             Running `target/debug/schwartzian_transform`
        Compare of Schwartzian Transoformation and Rust Sort by Attribut and by Method (with calculation)
        Attribut: Runtime Schwartzian Transformation (Copy-to-tuple): PT0.013865268S
        Attribut: Runtime Schwartzian Transformation (Inner-Sort): PT0.065100204S
        Attribut: Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): PT0.078965472S
        Attribut: Runtime Schwartzian Transformation (Complete): PT0.092685264S
        Attribut: Runtime Rust Sort: PT0.072440492S
        Method: Runtime Schwartzian Transformation (Copy-to-tuple): PT0.010782779S
        Method: Runtime Schwartzian Transformation (Inner-Sort): PT0.057915547S
        Method: Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): PT0.068698326S
        Method: Runtime Schwartzian Transformation (Complete): PT0.079665651S
        Method: Runtime Rust Sort: PT0.192230944S

Es ist bei diesen attributbasierten Testläufen deutlich zu erkennen, dass der reine Sortiervorgang der Schwartzian Transformation schneller als der *sort_by*-Algorithmus ist. Allerdings entsteht ein Overhead durch die Kopiervorgänge. Bei den erzeugten 50.000 Mitarbeitern ist der *Inner-Sort + Copy-to-tuple* etwa gleich schnell wie der Rust-Sort-Algorithmus.

Wird die Anzahl der erzeugten Mitarbeiter zu einem größeren Wert verändert, werden Geschwindigkeitsvorteile der Schwartzian Transformation sichtbarer. Wird der Wert verkleinert, so wirkt der *sort-by*-Algorithmus effizienter.

Werden Berechnungen durchgeführt, so werden beim *sort-by* Algorithmus teure Aufrufe mehrfach durchgeführt. An dieser Stelle macht sich die Schwartzian Transformation "bezahlt".
