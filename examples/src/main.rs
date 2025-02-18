rost::rost! {
    benutze std::sammlungen::Woerterbuch als Woebu;

    eigenschaft SchluesselWert {
        funktion schreibe(&selbst, schlsl: Zeichenkette, wert: Zeichenkette);
        funktion lese(&selbst, schlsl: Zeichenkette) -> Ergebnis<Moeglichkeit<&Zeichenkette>, Zeichenkette>;
    }

    statisch aend WOERTERBUCH: Moeglichkeit<Woebu<Zeichenkette, Zeichenkette>> = Nichts;

    struktur Konkret;

    umstz SchluesselWert fuer Konkret {

        funktion schreibe(&selbst, schlsl: Zeichenkette, wert: Zeichenkette) {
            lass woebu = gefaehrlich {
                WOERTERBUCH.hole_oder_fuege_ein_mit(Standard::standard)
            };
            woebu.einfuegen(schlsl, wert);
        }

        funktion lese(&selbst, schlsl: Zeichenkette) -> Ergebnis<Moeglichkeit<&Zeichenkette>, Zeichenkette> {
            wenn lass Etwas(woebu) = gefaehrlich { WOERTERBUCH.als_ref() } {
                Gut(woebu.hole(&schlsl))
            } anderenfalls {
                Fehler("Holt das Woerterbuch".hinein())
            }
        }
    }

    oeffentlich(kiste) funktion vielleicht(i: u32) -> Moeglichkeit<Ergebnis<u32, Zeichenkette>> {
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

    asynchron funktion beispiel() {
    }

    asynchron funktion beispiel2() {
        beispiel().abwarten;
    }

    funktion einstieg() {
        lass aend x = 31;

        entspreche x {
            42 => {
                ausgabe!("Wienerschnitzel")
            }
            _ => ausgabe!("Na geht doch")
        }

        fuer i in 0..10 {
            lass val = schleife {
                abbruch i;
            };

            waehrend keins x < val {
                x += 1;
            }

            x = wenn lass Etwas(ergebnis) = vielleicht(i) {
                ergebnis.entpacken()
            } anderenfalls {
                12
            };
        }

        benutze std::vgl::Ordnung;
        let _mod7 = vec![0; 100].wieder()
            .nehme(50)
            .zuordnen(|nummer| nummer %  7)
            .sammeln::<Vec<i32>>()
            .zu_wieder()
            .falte(0, |a, nummer| match nummer.vgl(&a) {
                Ordnung::Mehr => a - nummer,
                Ordnung::Weniger => a + nummer,
                Ordnung::Gleich => a,
            });
    }
}

