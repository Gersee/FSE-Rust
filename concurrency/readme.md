# Concurrency
Ein Beispiel der nebenläufigen Verarbeitung unter Berücksichtigung von Zugriffssicherheiten und der Verwendung eines *Mutex*. Zur veranschaulichung des konkurrierenden Zugriffs wird nicht nur ein *Vector* von mehreren Threads verarbeitet sondern auch jeder Vector-Index von mehreren Threads geändert.

Die Grundversion des Programmes wurde einer [externen Quelle](https://doc.rust-lang.org/book/concurrency.html) entnommen.
