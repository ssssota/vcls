use vcls_ast::*;
use vcls_parser::parse;

#[test]
fn import() {
    assert_eq!(
        parse("import foo;").unwrap(),
        Vcl {
            declarations: vec![Declaration::Import(ImportDeclaration {
                ident: "foo".to_string()
            })]
        }
    );
}
