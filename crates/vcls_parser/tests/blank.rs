use vcls_ast::*;
use vcls_parser::parse;

#[test]
fn blank() {
    assert_eq!(
        parse("").unwrap(),
        Vcl {
            declarations: vec![]
        }
    );
}
