use std::env;
use std::fs;


extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
use pest::iterators::Pair;
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
    println!(r#"<rect width="0.75in" height="0.5in" fill="none" stroke="black"/>"#);
}

fn emit_circle() {
    println!(r#"<circle r="0.25in" fill="none" stroke="black"/>"#);
}

fn emit_ellipse() {
    println!(r#"<ellipse rx="0.375in" ry="0.25in" fill="none" stroke="black"/>"#);
}
fn emit_primitive(primitive: Pair<Rule>){
    match primitive.as_rule() {
        Rule::rect => emit_box(),
        Rule::circle => emit_circle(),
        Rule::ellipse => emit_ellipse(),
        _ => unreachable!()
    }
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
            Rule::primitive => {
                println!("<!--{}-->", elem.as_str());
                emit_primitive(elem.into_inner().next().unwrap());
            },
            _ => panic!()
        }
    }
    emit_footer();
}
