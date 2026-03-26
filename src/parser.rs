
// Suppress all warnings from generated code
#[allow(warnings)]
pub mod binwordslexer {
    include!(concat!(env!("OUT_DIR"), "/binwordslexer.rs"));
}

#[allow(warnings)]
pub mod binwordsparser {
    include!(concat!(env!("OUT_DIR"), "/binwordsparser.rs"));
}

#[allow(warnings)]
pub mod binwordslistener {
    include!(concat!(env!("OUT_DIR"), "/binwordslistener.rs"));
}
