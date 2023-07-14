use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Fehler" | "Fehla" | "Fehlerle" => "Err",
        "Gut" | "Fei" | "Fain"  => "Ok",
        "Zeichenkette" | "Gliedrkättschl" | "Zeichnkättschn" | "Schorlakädshn" | "Weisjakäddsl" => "String",
        "Wörterbuch" | "Wuerterschbuuch" |  "Wöadbuch" | "Wörtaboich" | "Wöadaubuch" | "Wöaddschbuuch" => "HashMap",
        "Standard" | "Schtandard" | "Stanna" | "Stammord" => "Default",
        "Fehlfunktion" | "Fählfunkzion" => "Error",
        "Möglichkeit" | "Gleegenheide" | "Gleegescheite" |  "Gleegäweêssn" | "Gleeschentleite" | "Gleeschentlugg" => "Option",
        "Etwas" | "Äs" | "Ebbes" => "Some",
        "Nichts" | "Nüscht" | "Laridari" => "None",
        "Ergebnis" | "Ärgäbnis" | "Ärgämbs" => "Result",
        "Selbst" | "Selbschd" | "Sälbschtn" | "Selm" | "Selber" | "Sälwer" | "Säl" | "Egscht" => "Self",
        "sammlungen" | "sammlunk" | "zammlungen" | "zommelsches"=> "collections",
        "ausgabe" | "uusgahm" | "schenken" | "uushänn"=> "println",
        "abbruch" => "break",
        "asynchron" => "async",
        "abwarten" => "await",
        "schleife" => "loop",
        "schiebe" => "move",
        "kiste" => "crate",
        "Schachtel" => "Box",
        "unerreichbarer_code" => "unreachable_code",
        "als" => "as",
        "konstante" => "const",
        "eigenschaft" => "trait",
        "typ" => "type",
        "gefährlich" => "unsafe",
        "in" => "in",
        "von" => "from",
        "dynamisch" => "dyn",
        "entpacken" => "unwrap",
        "standard" => "default",
        "als_ref" => "as_ref",
        "ea" => "io",
        "extern" => "extern",
        "falsch" => "false",
        "funktion" => "fn",
        "übergeordnet" => "super",
        "einfügen" => "insert",

        // iterator funktionen
        "wieder" => "iter",
        "zu_wieder" => "into_iter",
        "zuordnen" => "map",
        "ausbreiten" => "flat_map",
        "falte" => "fold",
        "leeren" => "drain",
        "sammeln" => "collect",
        "finde" => "find",
        "nehme" => "take", 
        "produkt" => "product",

        // ordering
        "vgl" => "cmp",
        "Ordnung" => "Ordering",
        "Mehr" => "Greater",
        "Weniger" => "Less",
        "Gleich" => "Equal",
        "hole" => "get",
        "erlaube" => "allow",
        "panik" | "scheiße" | "mist" | "ups" => "panic",
        "modul" => "mod",
        "änd" => "mut",
        "neu" => "new",
        "wo" => "where",
        "für" => "for",
        "hole_oder_füge_ein_mit" => "get_or_insert_with",
        "einstieg" => "main",
        "öffentlich" => "pub",
        "keins" => None?,
        "zurückgebe" => "return",
        "umstz" => "impl",
        "ref" => "ref",
        "entspreche" => "match",
        "wenn" => "if",
        "anderenfalls" => "else",
        "selbst" => "self",
        "lass" => "let",
        "statisch" => "static",
        "struktur" => "struct",
        "erwarte" => "expect",
        "während" => "while",
        "benutze" => "use",
        "hinein" => "into",
        "wahr" => "true",
        "aufzählung" => "enum",

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
