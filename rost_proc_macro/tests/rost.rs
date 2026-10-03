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
