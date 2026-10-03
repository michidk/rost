//! A tiny procedural macro for writing Rust with German keywords and common
//! standard-library names.

#![forbid(unsafe_code)]

use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn translate_ident(ident: Ident) -> Option<Ident> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Fehler" => "Err",
        "Gut" => "Ok",
        "Zeichenkette" => "String",
        "Wörterbuch" | "Woerterbuch" => "HashMap",
        "Standard" => "Default",
        "Fehlfunktion" => "Error",
        "Möglichkeit" | "Moeglichkeit" => "Option",
        "Etwas" => "Some",
        "Nichts" => "None",
        "Ergebnis" => "Result",
        "Selbst" => "Self",
        "Vektor" => "Vec",
        "Satz" | "Menge" => "Set",
        "Kopieren" => "Copy",
        "Klonen" => "Clone",
        "Debuggen" => "Debug",
        "ableiten" => "derive",
        "Gleichheit" => "Eq",
        "PartialGleichheit" => "PartialEq",
        "PartialOrdnung" => "PartialOrd",
        "sammlungen" => "collections",
        "ausgabe" => "println",
        "schreibe" => "writeln",
        "abbruch" => "break",
        "weiter" => "continue",
        "asynchron" => "async",
        "abwarten" => "await",
        "schleife" => "loop",
        "schiebe" | "verschiebe" | "bewege" => "move",
        "buchstabe" => "char",
        "scheibe" => "slice",
        "niemals" | "nie" => "never",
        "kiste" => "crate",
        "Schachtel" => "Box",
        "unerreichbarer_code" => "unreachable_code",
        "fallenlassen" => "drop",
        "als" => "as",
        "konstante" => "const",
        "eigenschaft" => "trait",
        "typ" => "type",
        "gefährlich" | "gefaehrlich" => "unsafe",
        "in" => "in",
        "von" => "from",
        "auswerten" => "parse",
        "dynamisch" => "dyn",
        "entpacken" | "auspacken" => "unwrap",
        "standard" => "default",
        "als_ref" => "as_ref",
        "ea" => "io",
        "stdein" => "stdin",
        "stdaus" => "stdout",
        "extern" => "extern",
        "falsch" => "false",
        "funktion" | "fk" => "fn",
        "übergeordnet" | "uebergeordnet" => "super",
        "einfügen" | "einfuegen" => "insert",

        // iterator funktionen
        "wieder" => "iter",
        "zu_wieder" => "into_iter",
        "zuordnen" | "ordne_zu" => "map",
        "ausbreiten" | "ausbreite_aus" => "flat_map",
        "falte" => "fold",
        "leeren" => "drain",
        "sammeln" => "collect",
        "finde" => "find",
        "nehme" | "nimm" => "take",
        "produkt" => "product",
        "summe" => "sum",
        "zeilen" => "lines",

        // ordering
        "vgl" => "cmp",
        "Ordnung" => "Ordering",
        "Mehr" => "Greater",
        "Weniger" => "Less",
        "Gleich" => "Equal",
        "hole" => "get",
        "drücke" | "druecke" => "push",
        "erlaube" => "allow",
        "panik" | "scheiße" | "mist" | "ups" => "panic",
        "modul" => "mod",
        "änd" | "aend" => "mut",
        "neu" => "new",
        "wo" => "where",
        "für" | "fuer" => "for",
        "hole_oder_füge_ein_mit" | "hole_oder_fuege_ein_mit" => "get_or_insert_with",
        "einstieg" | "haupt" => "main",
        "öffentlich" | "oeffentlich" => "pub",
        "keins" => None?,
        "zurückgebe" | "zurueckgebe" => "return",
        "umstz" => "impl",
        "ref" => "ref",
        "entspreche" => "match",
        "wenn" | "falls" | "sofern" | "insoweit" => "if",
        "anderenfalls" | "ansonsten" => "else",
        "selbst" => "self",
        "lass" => "let",
        "statisch" => "static",
        "struktur" => "struct",
        "entspricht" => "matches",
        "erwarte" => "expect",
        "behaupte" => "assert",
        "behaupte_gleich" => "assert_eq",
        "behaupte_ungleich" => "assert_ne",
        "unerreichbar" => "unreachable",
        "während" | "waehrend" | "solange" => "while",
        "benutze" | "nutze" => "use",
        "hinein" => "into",
        "wahr" => "true",
        "aufzählung" | "aufzaehlung" => "enum",

        _ => &ident_str,
    };

    Some(Ident::new(new_str, ident.span()))
}

fn translate_tree(token: TokenTree) -> Option<TokenTree> {
    match token {
        TokenTree::Group(group) => {
            let mut translated = Group::new(group.delimiter(), translate_stream(group.stream()));
            translated.set_span(group.span());
            Some(TokenTree::Group(translated))
        }
        TokenTree::Ident(ident) => translate_ident(ident).map(TokenTree::Ident),
        TokenTree::Punct(..) | TokenTree::Literal(..) => Some(token),
    }
}

fn translate_stream(stream: TokenStream) -> TokenStream {
    stream.into_iter().filter_map(translate_tree).collect()
}

/// Translates German Rust syntax into regular Rust syntax.
#[proc_macro]
pub fn rost(item: TokenStream) -> TokenStream {
    translate_stream(item)
}
