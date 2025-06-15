use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
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

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rost(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
