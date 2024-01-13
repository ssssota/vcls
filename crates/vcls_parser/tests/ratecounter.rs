use vcls_ast::*;
use vcls_parser::parse;

#[test]
fn ratecounter() {
    assert_eq!(
        parse("ratecounter test { }").unwrap(),
        Vcl {
            declarations: vec![Declaration::RateCounter(RateCounterDeclaration {
                name: "test".to_string()
            })]
        }
    );
}
