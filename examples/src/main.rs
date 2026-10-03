rost::rost! {
    benutze standardbibliothek::sammlungen::Wörterbuch als Wöbu;
    benutze standardbibliothek::synchronisierung::{VerzögerteSperre, Sperre};

    eigenschaft SchlüsselWert {
        fk schreibe(&selbst, schlsl: Zeichenkette, wert: Zeichenkette);
        fk lese(&selbst, schlsl: &Zeichenstrang) -> Ergebnis<Möglichkeit<Zeichenkette>, Zeichenkette>;
    }

    statisch WÖRTERBUCH: VerzögerteSperre<Sperre<Wöbu<Zeichenkette, Zeichenkette>>> =
        VerzögerteSperre::neu(|| Sperre::neu(Wöbu::neu()));

    struktur Konkret;

    umstz SchlüsselWert für Konkret {

        fk schreibe(&selbst, schlsl: Zeichenkette, wert: Zeichenkette) {
            lass änd wöbu = WÖRTERBUCH.sperren().erwarte("Wörterbuchsperre wurde vergiftet");
            wöbu.einfügen(schlsl, wert);
        }

        fk lese(&selbst, schlsl: &Zeichenstrang) -> Ergebnis<Möglichkeit<Zeichenkette>, Zeichenkette> {
            lass wöbu = WÖRTERBUCH.sperren()
                .ordne_fehler_zu(|_| Zeichenkette::von("Wörterbuchsperre wurde vergiftet"))?;
            Gut(wöbu.hole(schlsl).geklont())
        }
    }

    öffentlich(kiste) fk vielleicht(i: u32) -> Möglichkeit<Ergebnis<u32, Zeichenkette>> {
        wenn i % 2 == 1 {
            wenn i == 42 {
                Etwas(Fehler(Zeichenkette::von("Scheiße")))
            } anderenfalls {
                Etwas(Gut(33))
            }
        } anderenfalls {
            Nichts
        }
    }

    öffentlich asynchron fk beispiel() {
    }

    öffentlich asynchron fk beispiel2() {
        beispiel().abwarten;
    }

    fk einstieg() {
        lass speicher = Konkret;
        speicher.schreibe("servus".hinein(), "welt".hinein());
        behaupte_gleich!(speicher.lese("servus").entpacken(), Etwas("welt".hinein()));

        lass änd x = 31;

        entspreche x {
            42 => {
                ausgabe!("Wienerschnitzel")
            }
            _ => ausgabe!("Na geht doch")
        }

        für i in 0..10 {
            lass änd val = 0;
            schleife {
                wenn val >= i {
                    abbruch;
                }
                val += 1;
            }

            während keins x < val {
                x += 1;
            }

            x = wenn lass Etwas(ergebnis) = vielleicht(i) {
                ergebnis.entpacken()
            } anderenfalls {
                12
            };
        }

        benutze standardbibliothek::vgl::Ordnung;
        lass _mod7 = vektor![0; 100].wieder()
            .nehme(50)
            .zuordnen(|nummer| nummer %  7)
            .sammeln::<Vektor<i32>>()
            .zu_wieder()
            .falte(0, |a, nummer| entspreche nummer.vgl(&a) {
                Ordnung::Mehr => a - nummer,
                Ordnung::Weniger => a + nummer,
                Ordnung::Gleich => a,
            });
    }
}
