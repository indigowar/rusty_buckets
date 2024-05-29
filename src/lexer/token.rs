use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Create,
    Drop,
    Alter,

    Table,
    View,
    Index,

    Select,
    Insert,
    Delete,
    Update,

    Where,
    From,
    Set,
    Is,
    Group,
    Having,
    Order,
    By,

    Commit,
    Rollback,

    Join,
    Inner,
    Left,
    Right,
    Full,

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
    In,
    Between,
    Like,
    Null,

    Case,
    Then,
    Else,
    End,
    As,
    On,

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
            Token::Alter => "alter",

            Token::Table => "table",
            Token::View => "view",
            Token::Index => "index",

            Token::Select => "select",
            Token::Insert => "insert",
            Token::Delete => "delete",
            Token::Update => "update",

            Token::Where => "where",
            Token::From => "from",
            Token::Set => "set",
            Token::Is => "is",
            Token::Group => "group",
            Token::Having => "having",
            Token::Order => "order",
            Token::By => "by",

            Token::Commit => "commit",
            Token::Rollback => "rollback",

            Token::Join => "join",
            Token::Inner => "inner",
            Token::Left => "left",
            Token::Right => "right",
            Token::Full => "full",

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
            Token::In => "in",
            Token::Between => "between",
            Token::Like => "like",
            Token::Null => "null",

            Token::Case => "case",
            Token::Then => "then",
            Token::Else => "else",
            Token::End => "end",
            Token::As => "as",
            Token::On => "on",

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
            "alter" => Token::Alter,

            "table" => Token::Table,
            "view" => Token::View,
            "index" => Token::Index,

            "select" => Token::Select,
            "insert" => Token::Insert,
            "delete" => Token::Delete,
            "update" => Token::Update,

            "where" => Token::Where,
            "from" => Token::From,
            "set" => Token::Set,
            "is" => Token::Is,
            "group" => Token::Group,
            "having" => Token::Having,
            "order" => Token::Order,
            "by" => Token::By,

            "commit" => Token::Commit,
            "rollback" => Token::Rollback,

            "join" => Token::Join,
            "inner" => Token::Inner,
            "left" => Token::Left,
            "right" => Token::Right,
            "full" => Token::Full,

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
            "in" => Token::In,
            "between" => Token::Between,
            "like" => Token::Like,
            "null" => Token::Null,

            "case" => Token::Case,
            "then" => Token::Then,
            "else" => Token::Else,
            "end" => Token::End,
            "as" => Token::As,
            "on" => Token::On,

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
