#[derive(pest_derive::Parser)]
#[grammar = "./vcl.pest"]
pub struct VclParser;
