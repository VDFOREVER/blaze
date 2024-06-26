use crate::db::create_db;
use crate::scripting::tokens::Token;
use crate::scripting::{lexer, parser};
use crate::server::server_bz;
use std::io::{self, Result};

pub fn handle_command_arguments() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_help_section();
        return Ok(());
    };
    match args[1].as_str() {
        "create" => create_db_with_console()?,
        "run" => server_bz::server_run(args)?,
        "lexer" => {
            let text = input_text()?;
            analyze_lexically(text)?;
        }
        "parser" => {
            let text = input_text()?;
            analyze_syntatically(text)?;
        }
        _ => {
            eprintln!("Invalid arguments");
            std::process::exit(1);
        }
    }
    Ok(())
}

fn print_help_section() {
    let help_list = r#"Blaze Db 0.0.1a - available commands:
    Database management
        create  - create a new datablaze
    Blaze Language
        lexer   - get to see how the code is subjected to lexical analysis under the hood
        parser  - try the first version of a parser
        run     - start server"#;

    println!("{}", help_list);
}

pub fn create_db_with_console() -> Result<()> {
    let mut path = String::new();
    println!("Specify a path to a datablaze");
    io::stdin().read_line(&mut path)?;
    create_db::create_db_structure(path.trim())?;

    Ok(())
}

fn analyze_lexically(code_to_parse: String) -> Result<Vec<Token>> {
    let mut code_lexer = lexer::Lexer::new(code_to_parse);
    code_lexer
        .get_context()
        .set_code_source("Shell".to_string());
    Ok(code_lexer.analyze().unwrap())
}

pub fn analyze_syntatically(code: String) -> Result<()> {
    let tokens = analyze_lexically(code)?;
    let mut code_parser = parser::Parser::new(tokens);
    code_parser
        .get_context()
        .set_code_source("Shell".to_string());
    let nodes = code_parser.parse()?.nodes;
    if !nodes.is_empty() {
        println!(
            "Parsing successfully completed! Nodes Count: {}",
            nodes.len()
        );
    }
    Ok(())
}

fn input_text() -> io::Result<String> {
    let mut code_to_parse = String::new();
    std::io::stdin().read_line(&mut code_to_parse)?;
    Ok(code_to_parse)
}
