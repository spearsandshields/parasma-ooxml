use ooxml_codegen::{lex_rnc, parse_rnc};

mod common;

#[test]
fn test_parse_sml() {
    let Some(input) = common::read_schema("sml.rnc") else {
        return;
    };

    // First, let's see the tokens around the error position
    let tokens = lex_rnc(&input).expect("failed to lex sml.rnc");
    println!("Total tokens: {}", tokens.len());
    println!("\nTokens 15585-15605:");
    for (i, tok) in tokens.iter().enumerate().skip(15585).take(25) {
        println!("  {}: {:?}", i, tok);
    }

    let schema = parse_rnc(&input).expect("failed to parse sml.rnc");

    println!(
        "Parsed {} namespaces, {} definitions",
        schema.namespaces.len(),
        schema.definitions.len()
    );

    // Print first few definitions
    for def in schema.definitions.iter().take(10) {
        println!("  {}", def.name);
    }
}
