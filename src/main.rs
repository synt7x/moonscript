use moonscript as moon;
use moon::lex;

fn main() {
    // constant string
    let input = include_str!(
        "../tests/lex.moon"
    );

    let mut state = lex::init(input.to_string()).expect("Empty input");

    while let Some(token) = lex::lex(&mut state) {
        println!("{:?}", token);
    }
}