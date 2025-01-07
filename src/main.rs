use anyhow::Result;
use bnf::Grammar;
use lambda_calculus::lexer::grammar::GRAMMAR;

fn main() -> Result<()> {

    let grammar: Grammar = GRAMMAR.parse()?;
    println!("{grammar:#?}");
    
    let sentence = r"
      lv1.v1
    "
    .to_string();
    
    let mut parse_trees = grammar.parse_input(&sentence);
    match parse_trees.next() {
        Some(parse_tree) => println!("{parse_tree}"),
        _ => println!("Grammar could not parse sentence"),
    }

    Ok(())
}
