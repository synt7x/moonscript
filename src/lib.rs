pub mod lex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shebang() {
        let state = lex::init("#!/usr/bin/env moon".to_string()).unwrap();
        assert_eq!(state.position, 19);
    }

    #[test]
    fn empty_file() {
        let state = lex::init("".to_string());
        assert!(state.is_none());
    }

    #[test]
    fn open_bracket() {
        let mut state = lex::init("[[hello]]".to_string()).unwrap();
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::String);
        assert_eq!(token.lexeme, "hello");
    }

    #[test]
    fn open_bracket_with_separators() {
        let mut state = lex::init("[=[hello]=]".to_string()).unwrap();
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::String);
        assert_eq!(token.lexeme, "hello");
    }

    #[test]
    fn open_bracket_with_separators_and_newlines() {
        let mut state = lex::init("[=[hello\nworld]=]".to_string()).unwrap();
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::String);
        assert_eq!(token.lexeme, "hello\nworld");
    }

    #[test]
    fn test_and_or_not_arithmetic() {
        let mut state = lex::init("and or not + - * / %".to_string()).unwrap();
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::Arithmetic);
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::Arithmetic);
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::Arithmetic);
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::Arithmetic);
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::Arithmetic);
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::Arithmetic);
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::Arithmetic);
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::Arithmetic);
    }

    #[test]
    fn test_strings() {
        let mut state = lex::init("\"hello\"".to_string()).unwrap();
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::String);
        assert_eq!(token.lexeme, "hello");
    }

    #[test]
    fn test_strings_with_escapes() {
        let mut state = lex::init("\"hello\\nworld\"".to_string()).unwrap();
        let token = lex::lex(&mut state).unwrap();
        assert_eq!(token.kind, lex::TokenKind::String);
        assert_eq!(token.lexeme, "hello\nworld");
    }
}
