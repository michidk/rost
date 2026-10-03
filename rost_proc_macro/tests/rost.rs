rost::rost! {
    benutze standardbibliothek::sammlungen::{Satz, Wörterbuch};
    benutze standardbibliothek::synchronisierung::{VerzögerteSperre, Sperre};

    statisch ZAHL: VerzögerteSperre<Sperre<i32>> =
        VerzögerteSperre::neu(|| Sperre::neu(7));

    #[ableiten(Debuggen, PartialGleichheit)]
    struktur Person {
        name: Zeichenkette,
    }

    fk beschreibe(person: Möglichkeit<Person>) -> Ergebnis<Zeichenkette, Zeichenkette> {
        entspreche person {
            Etwas(Person { name }) => Gut(name),
            Nichts => Fehler("keine Person".hinein()),
        }
    }

    fk häufigkeiten(werte: &[i32]) -> Wörterbuch<i32, usize> {
        lass änd ergebnis = Wörterbuch::neu();
        für wert in werte {
            *ergebnis.eintrag(*wert).oder_einfügen(0) += 1;
        }
        ergebnis
    }

    fk eindeutige_werte(werte: &[i32]) -> Satz<i32> {
        lass änd ergebnis = Satz::neu();
        für wert in werte {
            ergebnis.einfügen(*wert);
        }
        ergebnis
    }

    makro_regeln! begrüßung {
        () => { "Hallo" };
    }

    öffentlich vereinigung Zahl {
        öffentlich ganz: u32,
        öffentlich gleitend: f32,
    }

    gefährlich extern "C" {
        sicher fk abs(eingabe: i32) -> i32;
    }
}

#[test]
fn translates_keywords_and_standard_types() {
    rost::rost! {
        lass person = Person {
            name: "Ada".zu_eigen(),
        };

        behaupte_gleich!(beschreibe(Etwas(person)), Gut("Ada".zu_eigen()));
        behaupte_gleich!(beschreibe(Nichts), Fehler("keine Person".zu_eigen()));
    }
}

#[test]
fn translates_nested_groups_and_unicode_identifiers() {
    rost::rost! {
        lass häufigkeiten = häufigkeiten(&[1, 2, 1, 3, 1]);
        lass werte = eindeutige_werte(&[1, 2, 1, 3, 1]);

        behaupte_gleich!(häufigkeiten.hole(&1), Etwas(&3));
        behaupte_gleich!(häufigkeiten.hole(&2), Etwas(&1));
        behaupte_gleich!(werte.länge(), 3);
    }
}

#[test]
fn translates_rust_2024_weak_keywords() {
    rost::rost! {
        lass wert = 7;
        lass zeiger = &roh konstante wert;
        behaupte_gleich!(zeiger, &wert als *const i32);
        behaupte_gleich!(begrüßung!(), "Hallo");
        behaupte_gleich!(standardbibliothek::speicher::größe_von::<Zahl>(), 4);
        behaupte_gleich!(abs(-3), 3);
    }
}

#[test]
fn translates_synchronization_types_and_methods() {
    rost::rost! {
        lass zahl = ZAHL.sperren().erwarte("Zahlensperre wurde vergiftet");
        behaupte_gleich!(*zahl, 7);
    }
}
