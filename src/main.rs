use std::fs;

pub mod lexer;

const FILEPATH: &str = "./hw.c";

fn main() {
    use lexer as clex;
    
    let source_code: String = match fs::read_to_string(FILEPATH) {
        Ok(content) => content,
        Err(e) => {
            panic!("{e}");
        },
    };

    let mut lexer: clex::Lexer = clex::Lexer::new(source_code, FILEPATH.to_string());
    
    loop {
        let token = lexer.get_token().unwrap();
        if token == clex::Token::EOF { break; }
        println!("{token:?}");
    }
}
