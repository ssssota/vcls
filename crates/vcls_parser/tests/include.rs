use vcls_ast::*;
use vcls_parser::parse;

#[test]
fn include() {
    assert_eq!(
        parse(r#"include "foo.vcl";"#).unwrap(),
        Vcl {
            declarations: vec![Declaration::Include(IncludeDeclaration {
                path: "foo.vcl".to_string()
            })]
        }
    );
}
