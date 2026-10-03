rost::rost! {
    benutze std::sammlungen::Wörterbuch;

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
            *ergebnis.entry(*wert).or_insert(0) += 1;
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
    let person = Person {
        name: "Ada".to_owned(),
    };

    assert_eq!(beschreibe(Some(person)), Ok("Ada".to_owned()));
    assert_eq!(beschreibe(None), Err("keine Person".to_owned()));
}

#[test]
fn translates_nested_groups_and_unicode_identifiers() {
    let counts = häufigkeiten(&[1, 2, 1, 3, 1]);

    assert_eq!(counts.get(&1), Some(&3));
    assert_eq!(counts.get(&2), Some(&1));
}

#[test]
fn translates_rust_2024_weak_keywords() {
    let wert = 7;

    rost::rost! {
        lass zeiger = &roh konstante wert;
        behaupte_gleich!(zeiger, &wert als *const i32);
        behaupte_gleich!(begrüßung!(), "Hallo");
        behaupte_gleich!(std::mem::size_of::<Zahl>(), 4);
        behaupte_gleich!(abs(-3), 3);
    }
}
