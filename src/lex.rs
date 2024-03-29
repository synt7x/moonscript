#[derive(Debug)]
pub struct LexState {
    pub input: Vec<char>,
    pub position: usize,
    pub line: usize,
    pub current: char,
}

impl LexState {
    pub fn new(input: String) -> LexState {
        LexState {
            input: input.chars().collect(),
            position: 0,
            line: 0,
            current: '\0',
        }
    }

    fn next(&mut self) {
        self.position += 1;

        if self.position >= self.input.len() {
            self.current = '\0';
            return;
        }

        self.current = self.input[self.position];
    }

    fn increment_line(&mut self) {
        self.line += 1;
        self.next();
    }
}

macro_rules! peek {
    ($state:expr) => {
        $state.input[$state.position + 1]
    }
}

macro_rules! line_token {
    ($state:expr) => {
        {
            $state.increment_line();
            return Some(
                Token::new(TokenKind::Whitespace, $state.line)
            );
        }
    }
}

macro_rules! one_token {
    ($kind:expr, $state:expr) => {
        {
            $state.next();
            Some(Token::new($kind, $state.line))
        }
    }
}

macro_rules! token {
    ($kind:expr, $lexeme:expr, $state:expr) => {
        Token {
            kind: $kind,
            lexeme: $lexeme,
            line: $state.line,
        }
    }

}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize) -> Token {
        Token {
            kind,
            lexeme: "".to_string(),
            line,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Whitespace,
    Comment,
    Identifier,
    Number,
    String,
    Equal,
    Arithmetic,
    Comparison,
    Infix,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Period,
    Comma,
    Dots,
    Bang,
    Arrow
}

fn read_identifier(state: &mut LexState) -> Option<Token> {
    let mut token = Token::new(TokenKind::Identifier, state.line);
    token.lexeme.push(state.current);
    state.next();

    while state.position < state.input.len() && (state.current.is_alphanumeric() || state.current == '_') {
        token.lexeme.push(state.current);
        state.next();
    }

    return Some(token);
}

fn read_shebang(state: &mut LexState) {
    if state.input.len() > 1 && state.current == '#' && peek!(state) == '!' {        
        while state.position < state.input.len() && state.current != '\n' {
            state.next();
        }
    }
}

fn read_comment(state: &mut LexState) -> Option<Token> {
    state.next();

    if state.current == '[' {
        let mut sep = 0;
        state.next();

        while state.position < state.input.len() && state.current == '=' {
            state.next();
            sep += 1;
        }

        if state.current == '[' {
            while state.position < state.input.len() {
                state.next();

                if state.current == ']' {
                    let mut end_sep = 0;

                    while state.position < state.input.len() && state.current == '=' {
                        state.next();
                        end_sep += 1;
                    }

                    if end_sep == sep && state.current == ']' {
                        state.next();
                        return Some(
                            Token::new(TokenKind::Comment, state.line)
                        );
                    }
                }
            }
        }
    } else {
        while state.position < state.input.len() && state.current != '\n' {
            state.next();
        }
    }

    return Some(
        Token::new(TokenKind::Comment, state.line)
    );
}

fn read_dash(state: &mut LexState) -> Option<Token> {
    state.next();

    match state.current {
        '-' => read_comment(&mut *state),
        '=' => {
            state.next();
            Some(
                token!(TokenKind::Infix, "-=".to_string(), state)
            )
        },
        '>' => {
            state.next();
            Some(
                token!(TokenKind::Arrow, "->".to_string(), state)
            )
        },
        _ => Some(
            token!(TokenKind::Arithmetic, "-".to_string(), state)
        )
    }
}

fn read_open_bracket(state: &mut LexState) -> Option<Token> {
    let mut sep = 0;
    state.next();

    while state.position < state.input.len() && state.current == '=' {
        state.next();
        sep += 1;
    }

    if state.current == '[' {
        let mut lexeme = String::new();
        while state.position < state.input.len() {
            state.next();

            if state.current == ']' {
                let mut end_sep = 0;
                state.next();

                while state.position < state.input.len() && state.current == '=' {
                    state.next();
                    end_sep += 1;
                }

                if end_sep == sep && state.current == ']' {
                    state.next();
                    
                    return Some(
                        token!(TokenKind::String, lexeme, state)
                    );
                }
            }

            lexeme.push(state.current);
        }
    } else {
        state.position -= sep;
        state.current = state.input[state.position];
    }

    return Some(
        Token::new(TokenKind::LeftBracket, state.line)
    );
}

fn read_equal(state: &mut LexState) -> Option<Token> {
    state.next();

    if state.current == '=' {
        state.next();
        return Some(
            token!(TokenKind::Comparison, "==".to_string(), state)
        );
    } else if state.current == '>' {
        state.next();
        return Some(
            token!(TokenKind::Arrow, "=>".to_string(), state)
        );
    }

    return Some(
        token!(TokenKind::Equal, "=".to_string(), state)
    );
}

fn read_comparison(state: &mut LexState) -> Option<Token> {
    let mut token = Token::new(TokenKind::Comparison, state.line);
    token.lexeme.push(state.current);
    state.next();

    if state.current == '=' {
        token.lexeme.push(state.current);
        state.next();

        return Some(token);
    }

    return Some(token);
}

fn read_slash(state: &mut LexState) -> Option<Token> {
    state.next();

    if state.current == '/' {
        state.next();
        
        return Some(
            token!(TokenKind::Arithmetic, "//".to_string(), state)
        )
    } else if state.current == '=' {
        state.next();
        return Some(
            token!(TokenKind::Infix, "/=".to_string(), state)
        )
    }
    
    return Some(
        token!(TokenKind::Arithmetic, "/".to_string(), state)
    );
}

fn read_not_equal(state: &mut LexState) -> Option<Token> {
    state.next();

    if state.current == '=' {
        state.next();
        return Some(
            token!(TokenKind::Comparison, "~=".to_string(), state)
        )
    }

    return Some(
        token!(TokenKind::Arithmetic, "~".to_string(), state)
    );
}

fn read_arithmetic(state: &mut LexState) -> Option<Token> {
    let mut token = Token::new(TokenKind::Arithmetic, state.line);
    token.lexeme.push(state.current);
    state.next();

    if state.current == '=' {
        token.lexeme.push(state.current);
        token.kind = TokenKind::Infix;
        state.next();

        return Some(token);
    }

    return Some(token);
}

fn read_arithmetic_word(state: &mut LexState) -> Option<Token> {
    let token = read_identifier(&mut *state);
    
    let mut token = token.expect("Failed to read arithmetic word");
    let lexeme = token.lexeme.as_str();

    match lexeme {
        "and" => token.kind = TokenKind::Arithmetic,
        "or" => token.kind = TokenKind::Arithmetic,
        "not" => token.kind = TokenKind::Arithmetic,
        _ => token.kind = TokenKind::Identifier
    }

    if state.current == '=' {
        token.lexeme.push(state.current);
        token.kind = TokenKind::Infix;
        state.next();
    }

    return Some(token);
}

fn read_dot(state: &mut LexState) -> Option<Token> {
    state.next();

    if state.current == '.' {
        state.next();

        if state.current == '.' {
            state.next();
            return Some(
                token!(TokenKind::Dots, "...".to_string(), state)
            )
        }

        return Some(
            token!(TokenKind::Arithmetic, "..".to_string(), state)
        )
    }

    return Some(
        token!(TokenKind::Period, ".".to_string(), state)
    );
}

fn read_bang(state: &mut LexState) -> Option<Token> {
    state.next();

    if state.current == '=' {
        state.next();
        return Some(
            token!(TokenKind::Comparison, "~=".to_string(), state)
        )
    }

    return Some(
        Token::new(TokenKind::Bang, state.line)
    );
}

fn read_brace(state: &mut LexState) -> Option<Token> {
    state.next();

    return Some(
        Token::new(
            match state.current {
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                ']' => TokenKind::RightBracket,
                _ => TokenKind::Arithmetic
            }, state.line
        )
    )
}

pub fn init(input: String) -> Option<LexState> {
    let mut state = LexState::new(input);
    if state.input.len() > 0 {
        state.current = state.input[0];
    } else {
        return None;
    }

    read_shebang(&mut state);
    return Some(state);
}

pub fn lex(state: &mut LexState) -> Option<Token> {
    let token = match state.current {
        '\n' | '\r' => line_token!(&mut *state),
        ' ' | '\t' => one_token!(TokenKind::Whitespace, state),
        '-' => read_dash(&mut *state),
        '[' => read_open_bracket(&mut *state),
        '=' => read_equal(&mut *state),
        '<' | '>' => read_comparison(&mut *state),
        '/' => read_slash(&mut *state),
        '~' => read_not_equal(&mut *state),
        '+' | '*' | '%' => read_arithmetic(&mut *state),
        'a' | 'o' | 'n' => read_arithmetic_word(&mut *state),
        '.' => read_dot(&mut *state),
        ',' => one_token!(TokenKind::Comma, state),
        '!' => read_bang(&mut *state),
        '{' | '}' | '(' | ')' | ']' => read_brace(&mut *state),
        'a'..='z' | 'A'..='Z' | '_' => read_identifier(&mut *state),
        //'0'..='9' => read_number(&mut *state),
        _ => None
    };

    match token {
        Some(token) => match token.kind {
            TokenKind::Whitespace | TokenKind::Comment => lex(&mut *state),
            _ => Some(token)
        },
        None => None
    }
}