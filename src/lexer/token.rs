use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    Create,
    Drop,

    Table,
    View,

    Select,
    Insert,
    Delete,
    Update,

    Where,
    From,
    Set,
    Is,

    Primary,
    Foreign,
    Key,
    References,

    Unique,
    Check,
    Constraint,
    Default,

    And,
    Or,
    Not,

    Equal,
    NotEqual,
    Bigger,
    Less,
    BiggerEqual,
    LessEqual,

    Plus,
    Minus,
    Slash,
    Percent,

    Dot,
    Comma,
    Semicolon,

    RightParenteses,
    LeftParentheses,

    Identifier(String),
    StringValue(String),
    Number(String),
    FloatNumber(String),

    Eof,

    Illegal,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Token::Create => "create",
            Token::Drop => "drop",

            Token::Table => "table",
            Token::View => "view",

            Token::Select => "select",
            Token::Insert => "insert",
            Token::Delete => "delete",
            Token::Update => "update",

            Token::Where => "where",
            Token::From => "from",
            Token::Set => "set",
            Token::Is => "is",

            Token::Primary => "primary",
            Token::Foreign => "foreign",
            Token::Key => "key",
            Token::References => "references",

            Token::Unique => "unique",
            Token::Check => "check",
            Token::Constraint => "constraint",
            Token::Default => "default",

            Token::And => "and",
            Token::Or => "or",
            Token::Not => "not",

            Token::Equal => "=",
            Token::NotEqual => "<>",
            Token::Bigger => ">",
            Token::Less => "<",
            Token::BiggerEqual => ">=",
            Token::LessEqual => "<=",

            Token::Plus => "+",
            Token::Minus => "-",
            Token::Slash => "/",
            Token::Percent => "%",

            Token::Dot => ".",
            Token::Comma => ",",
            Token::Semicolon => ";",

            Token::RightParenteses => ")",
            Token::LeftParentheses => "(",

            Token::Identifier(v) => v.as_str(),
            Token::StringValue(v) => v.as_str(),
            Token::Number(v) => v.as_str(),
            Token::FloatNumber(v) => v.as_str(),

            Token::Eof => todo!(),

            Self::Illegal => "[illegal]",
        };

        write!(f, "{}", output)
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        if value.starts_with('\'') && value.ends_with('\'') {
            if value.len() == 2 {
                return Token::StringValue("".into());
            }
            let content = &value[1..value.len() - 1];
            return Token::StringValue(content.to_string());
        }

        if value.chars().all(|c| c.is_ascii_digit()) {
            return Token::Number(value);
        }

        if contains_float_number(&value) {
            return Token::FloatNumber(value);
        }

        let value = value.to_lowercase();
        match value.as_str() {
            "create" => Token::Create,
            "drop" => Token::Drop,

            "table" => Token::Table,
            "view" => Token::View,

            "select" => Token::Select,
            "insert" => Token::Insert,
            "delete" => Token::Delete,
            "update" => Token::Update,

            "where" => Token::Where,
            "from" => Token::From,
            "set" => Token::Set,
            "is" => Token::Is,

            "primary" => Token::Primary,
            "foreign" => Token::Foreign,
            "key" => Token::Key,
            "references" => Token::References,

            "unique" => Token::Unique,
            "check" => Token::Check,
            "constraint" => Token::Constraint,
            "default" => Token::Default,

            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,

            "<>" => Token::NotEqual,
            ">=" => Token::BiggerEqual,
            "<=" => Token::LessEqual,

            _ => {
                if value
                    .chars()
                    .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_')
                {
                    Token::Identifier(value)
                } else {
                    Token::Illegal
                }
            }
        }
    }
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '=' => Token::Equal,
            '>' => Token::Bigger,
            '<' => Token::Less,

            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::Slash,
            '%' => Token::Percent,

            '.' => Token::Dot,
            ',' => Token::Comma,
            ';' => Token::Semicolon,

            '(' => Token::LeftParentheses,
            ')' => Token::RightParenteses,

            '\0' => Token::Eof,

            _ => Token::Illegal,
        }
    }
}

fn contains_float_number(value: &String) -> bool {
    let mut dot_count = 0;

    for c in value.chars() {
        if c.is_digit(10) {
            continue;
        } else if c == '.' {
            dot_count += 1;
            if dot_count > 1 {
                return false;
            }
        } else {
            return false;
        }
    }

    dot_count == 1
}
