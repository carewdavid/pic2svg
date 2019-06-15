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

mod shape;
use shape::*;

struct Pic {
    direction: Direction,
    here: Point,
    move_distance: f32,
    objects: Vec<Box<Primitive>>
}

impl Pic {
    fn new() -> Pic {
        Pic {
            direction: Direction::Right,
            here: Point(0.0, 0.0),
            move_distance: 0.5,
            objects: Vec::new()
        }
    }

    fn move_point(&mut self) {
        let Point(x, y) = self.here;
        match self.direction {
            Direction::Left => self.here = Point(x - self.move_distance, y),
            Direction::Right => self.here = Point(x + self.move_distance, y),
            Direction::Down => self.here = Point(x, y + self.move_distance),
            Direction::Up => self.here = Point(x, y - self.move_distance),
        }
    }

    fn current_location(&self) -> Point {
        self.here
    }

    fn emit(&self) {
        for obj in &self.objects {
            obj.emit();
        }
    }

    //Add a primitive object to the image.
    //Places said object adjacent to the previous one (if it exists)
    fn place_object(&mut self, mut obj: Box<Primitive>){
        //Get the point for the next object to connect to. We need to save it here
        //so it's not affected by direction changes.
        self.here = match self.direction {
            Direction::Left => {
                let offset = Point::distance(obj.center(), obj.east());
                obj.set_location(Point(self.here.0 - offset, self.here.1));
                obj.west()
            },
            Direction::Right => {
                let offset = Point::distance(obj.center(), obj.west());
                obj.set_location(Point(self.here.0 + offset, self.here.1));
                obj.east()
            }, 
            Direction::Down => {
                let offset = Point::distance(obj.center(), obj.north());
                obj.set_location(Point(self.here.0, self.here.1 + offset));
                obj.south()
            }, 
            Direction::Up => {
                let offset = Point::distance(obj.center(), obj.south());
                obj.set_location(Point(self.here.0, self.here.1 - offset));
                obj.north()
            } 
        };
        self.objects.push(obj);
    } 
}

fn emit_header() {
    println!("<?xml version=\"1.0\" encoding=\"utf-8\"?>");
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"8.5in\" height=\"11in\">");
}

fn emit_footer(){
    println!("</svg>");
}

fn emit_primitive(pic: &mut Pic, primitive: Pair<Rule>){
    //Location for the next object to be placed
    let here = pic.here;
    let dir = pic.direction;
    match primitive.as_rule() {
        Rule::rect => pic.place_object(Box::new(Rect::new(here))),
        Rule::circle => pic.place_object(Box::new(Circle::new(here))),
        Rule::ellipse => pic.place_object(Box::new(Ellipse::new(here))),
        Rule::line => pic.place_object(Box::new(Line::new(here, dir))),
        _ => unreachable!()
    }
}

fn do_command(pic: &mut Pic, command: Pair<Rule>) {
    match command.as_rule() {
        Rule::movement => pic.move_point(),
        Rule::left => pic.direction = Direction::Left,
        Rule::right => pic.direction = Direction::Right,
        Rule::down => pic.direction = Direction::Down,
        Rule::up => pic.direction = Direction::Up,
        //_ =>println!("{}", command)
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
    let mut pic = Pic::new();
    emit_header();
    for elem in program.into_inner() {
        match elem.as_rule() {
            Rule::primitive => {
                println!("<!--{}-->", elem.as_str());
                emit_primitive(&mut pic, elem.into_inner().next().unwrap());
            },
            Rule::command => do_command(&mut pic, elem.into_inner().next().unwrap()),
            _ => panic!()
        }
    }
    pic.emit();
    emit_footer();
}
