#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calculator1);

#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator1::TermParser::new().parse("((22)").is_err());
}

lalrpop_mod!(pub calculator3);

#[cfg_attr(not(test), allow(unused_macros))]
macro_rules! test3 {
    ($expr:expr) => {
        println!("parsing {}", stringify!($expr));
        assert_eq!(
            calculator3::ExprParser::new()
                .parse(stringify!($expr))
                .unwrap(),
            $expr
        );
    };
}

#[test]
fn calculator3() {
    test3!(22 + 44);
    test3!(22 - 44 - 66);
    test3!(22 * 44 + 66);
    test3!(22 * 44 + 66 / 3);
    test3!(22 * (44 + 66) / 3);
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
