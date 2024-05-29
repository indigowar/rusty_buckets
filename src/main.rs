use lexer::Lexer;

mod lexer;

fn main() {
    let input = "
CREATE TABLE accounts(id int primary key, name varchar(255) not null);

UPDATE accounts SET id = 5423 WHERE LEN(name) > 15;
";

    println!("{}", input);
    for token in Lexer::read_all(input.chars().peekable()) {
        println!("{}", token);
    }
}
