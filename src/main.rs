use std::env;
use std::fs;


extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
#[derive(Parser)]
#[grammar = "pic.pest"]
pub struct PicParser;

fn emit_header() {
    println!("<?xml version=\"1.0\" encoding=\"utf-8\"?>");
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"8.5in\" height=\"11in\">");
}

fn emit_footer(){
    println!("</svg>");
}

fn emit_box() {
    println!("<rect width=\"0.75in\" height=\"0.5in\" fill=\"none\" stroke=\"black\"/>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = &args[1];
    let program = fs::read_to_string(progname)
        .expect("Could not read input file");
    let program = PicParser::parse(Rule::pic, &program).expect("Parse error")
        .next().unwrap();
    emit_header();
    for elem in program.into_inner() {
        match elem.as_rule() {
            Rule::element => {
                println!("<!--{}-->", elem.as_str());
                emit_box();
            },
            _ => panic!()
        }
    }
    emit_footer();
}
