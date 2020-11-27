mod ast;
mod parser;
mod token;
mod tokenizer;

@@ -12,9 +14,8 @@ fn main() -> Result<(), Box<dyn std::error::Error>> {
        tokenizer::tokenize(args.pop().unwrap())?
    };

    for (idx, t) in tokens.iter().enumerate() {
        eprintln!("tokens[{}]: {:?}", idx, t);
    }
    let root_node = parser::parse(tokens)?;
    eprintln!("{:?}", root_node);

    Ok(())
}
