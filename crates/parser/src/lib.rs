#[derive(pest_derive::Parser)]
#[grammar = "./fastly-vcl.pest"]
pub struct VclParser;
