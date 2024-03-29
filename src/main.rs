use anyhow::{bail, Context, Result};
use rlox::{ast_printer::AstPrinter, parser::Parser, scanner, visitor::walk_expr};
use std::{
    env,
    io::{BufRead, Write},
};

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();

    match args.len() {
        1 => run_repl()?,
        2 => run_file(&args[1])?,
        _ => bail!("Usage: rlox [script]"),
    }

    Ok(())
}

fn run_repl() -> Result<()> {
    let print_prompt = || -> Result<()> {
        print!("> ");
        std::io::stdout().flush()?;
        Ok(())
    };
    let stdin = std::io::stdin().lock();

    print_prompt()?;

    for line in stdin.lines() {
        run(line?);

        print_prompt()?;
    }

    Ok(())
}

fn run_file(file_name: &str) -> Result<()> {
    let script = std::fs::read_to_string(file_name)
        .context(format!("Cannot read script from: {file_name}"))?;

    run(script);

    Ok(())
}

fn run(source: String) {
    let tokens = scanner::scan_tokens(&source);

    let mut parser = Parser::new(&tokens);
    let expr = parser.parse();
    println!("{}", walk_expr(&mut AstPrinter, &expr));
}
