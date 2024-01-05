use vcls_ast::Vcl;

#[derive(pest_derive::Parser)]
#[grammar = "./fastly-vcl.pest"]
pub struct VclParser;

pub fn parse(src: &str) -> Result<Vcl, ()> {
    Err(())
}
