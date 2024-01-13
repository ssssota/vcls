use vcls_ast::*;
use vcls_parser::parse;

#[test]
fn penaltybox() {
    assert_eq!(
        parse("penaltybox test { }").unwrap(),
        Vcl {
            declarations: vec![Declaration::PenaltyBox(PenaltyBoxDeclaration {
                name: "test".to_string(),
                span: Span(0, 19)
            })]
        }
    );
}
