# Rost

<p align="center"><img src="logo.png" alt="Rost Oktoberfest emblem" width="640"></p>

Aren't you _müde_ from writing Rust programs in English? Do you like saying
"scheiße" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some German touch to your
programs?

**Rost** (German for _Rust_) is here to save your day. It allows you to
write Rust programs in German, using German keywords, German function names,
German idioms.

You don't feel at ease using only German words? Don't worry!
German Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Rost targets **Rust 2024** and requires Rust 1.85 or newer. It is a playful
experiment, but ordinary Rust and German Rust remain fully interoperable.

## Schnellstart

Add Rost to your project:

```toml
[dependencies]
rost = { git = "https://github.com/michidk/rost" }
```

Then wrap German Rust in the `rost!` macro:

```rust
rost::rost! {
    fk haupt() {
        lass nachricht = "Hallo, Welt!";
        ausgabe!("{nachricht}");
    }
}
```

Here is a larger example:

## struct and impl (aka Konvention und Umsetzung)

```rust
rost::rost! {
    benutze std::sammlungen::Wörterbuch als Wöbu;

    eigenschaft SchlüsselWert {
        fk schreibe(&selbst, schlsl: Zeichenkette, wert: Zeichenkette);
        fk lese(&selbst, schlsl: &str) -> Ergebnis<Möglichkeit<Zeichenkette>, Zeichenkette>;
    }

    benutze std::sync::{LazyLock, Mutex};

    statisch WÖRTERBUCH: LazyLock<Mutex<Wöbu<Zeichenkette, Zeichenkette>>> =
        LazyLock::new(|| Mutex::new(Wöbu::new()));

    struktur Konkret;

    umstz SchlüsselWert für Konkret {

        fk schreibe(&selbst, schlsl: Zeichenkette, wert: Zeichenkette) {
            lass änd wöbu = WÖRTERBUCH.lock().erwarte("Wörterbuchsperre wurde vergiftet");
            wöbu.einfügen(schlsl, wert);
        }

        fk lese(&selbst, schlsl: &str) -> Ergebnis<Möglichkeit<Zeichenkette>, Zeichenkette> {
            lass wöbu = WÖRTERBUCH.lock()
                .map_err(|_| Zeichenkette::von("Wörterbuchsperre wurde vergiftet"))?;
            Gut(wöbu.hole(schlsl).cloned())
        }
    }
}
```

## Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Gut so!

## Aber warum?

The [French](https://github.com/bnjbvr/rouille) and
[Dutch](https://github.com/jeroenhd/roest) can do it, so we can as well! See
Rouille's [Other languages](https://github.com/bnjbvr/rouille#other-languages)
section for the full collection.

## Mitwirken

First of all, _vielen Dank_ for considering participating to this joke, the
German government will thank you later! Feel free to add a few identifiers
here and there, and open a pull request against the `hauptzweig` (German for
`main branch`). The initial translation was made by [Shemnei](https://github.com/Shemnei/) and [michidk](https://github.com/michidk/).

## Die Lizenzbestimmungen

[WTFPL](http://www.wtfpl.net/). The logo was
[created with ChatGPT](https://chatgpt.com/s/m_6ac0faf830488191b9bda58c2c03f9e4)
and does not fall under this license.
