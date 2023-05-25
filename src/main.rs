mod lexer;

fn main() {
    let source_code = String::from("let my_var = 10;");
    let src: Vec<char> = source_code.chars().collect();
    let mut lexer = lexer::Lexer::new(src);
    lexer.tokenize();

    for x in lexer.tokens {
        println!("{:?}", x);
    }

}

