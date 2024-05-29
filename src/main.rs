use lexer::{Lexer, Token};

mod lexer;

fn main() {
    let input = "
CREATE TABLE accounts(id int primary key, name varchar(255) not null);

UPDATE accounts SET id = 5423 WHERE LEN(name) > 15;
";

    let mut lexer = Lexer::new(input.chars().peekable());

    println!("{}", input);

    let mut token = lexer.next_token();
    while token != Token::Eof {
        println!("{}", token);
        token = lexer.next_token();
    }
}
