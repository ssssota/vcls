use vcls_ast::*;
use vcls_parser::parse;

#[test]
fn include() {
    assert_eq!(
        parse(r#"include "foo.vcl";"#).unwrap(),
        Vcl {
            declarations: vec![Declaration::Include(IncludeDeclaration {
                path: StringToken {
                    value: "foo.vcl".to_string(),
                    span: Span(8, 17),
                },
                span: Span(0, 18),
            })],
        },
    );
}
