use std::io;
use crossterm::{ExecutableCommand, style,};
use crossterm::style::Color;
use rand::distributions::{Distribution, Standard};
use rand::{random, Rng};

use crate::brick::BrickOrientation::{Horizontal, Vertical};
use crate::point::{Point};
use crate::utility::{clear_console, get_screen_translated_vertices, move_to_and_write};

#[derive(Copy, Clone, PartialEq)]
pub enum BrickOrientation { //every rotation happens clockwise
    Horizontal,
    Vertical,
}
#[derive(Clone)]
pub struct Brick{
    pub(crate) vertices : Vec<Point>,  //points are bottom-left corners of tile that builds the brick
    pub(crate) width_horizontal : u16,
    pub(crate) height_horizontal: u16,
    pub(crate) orientation: BrickOrientation,
    pub(crate) color : Color,
}

#[derive(Clone)]
pub struct DeadBrick {
    pub(crate) vertices : Vec<Point>, //already translated to screen coordinates
    pub(crate) color : Color,
}
pub(crate) enum BrickShapes {
    S,
    L,
    Box,
    I,
    Castle,
}

impl Distribution<BrickShapes> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BrickShapes {
        match rng.gen_range(0..5) {
            0 => BrickShapes::I,
            1 => BrickShapes::S,
            2 => BrickShapes::L,
            3 => BrickShapes::Castle,
            _ => BrickShapes::Box,
        }
    }
}

pub fn generate_brick() -> Brick{
    let version = random::<BrickShapes>();
    let vertices : Vec<Point> =
        match version{
            BrickShapes::I => vec!{Point{x : 0, y : 0},
                                   Point{x : 0, y : 1},
                                   Point{x : 0, y : 2},
            },

            BrickShapes::S => vec!{Point{x : 0, y : 0},
                                   Point{x : 1, y : 0},
                                   Point{x : 1, y : 1},
                                   Point{x : 2, y : 1},
            },

            BrickShapes::Box => vec!{Point{x : 0, y : 0},
                                     Point{x : 0, y : 1},
                                     Point{x : 1, y : 1},
                                     Point{x : 1, y : 0},
            },

            BrickShapes::L => vec!{Point{x : 0, y : 0},
                                   Point{x : 1, y : 0},
                                   Point{x : 0, y : 1},
            },

            BrickShapes::Castle => vec!{Point{x : 0, y : 0},
                                        Point{x : 1, y : 1},
                                        Point{x : 1, y : 0},
                                        Point{x : 2, y : 0},
            },
        };

    return build_brick(vertices);
}

pub fn build_brick(vertices : Vec<Point>) -> Brick {
    let width = calculate_width(&vertices);
    let height = calculate_height(&vertices);

    let color =
    match rand::thread_rng().gen_range(0..5) {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Blue,
        3 => Color::Grey,
        _ => Color::DarkYellow,
    };

    return Brick{vertices, width_horizontal: width, height_horizontal: height, orientation: Horizontal, color};
}

pub fn get_height(brick : &Brick) -> u16{
    if brick.orientation == Horizontal{
        brick.height_horizontal
    }
    else{
       brick.width_horizontal
    }
}

pub fn get_width(brick : &Brick) -> u16{
    if brick.orientation == Horizontal{
        brick.width_horizontal
    }
    else{
        brick.height_horizontal
    }
}

fn calculate_width(vertices : &Vec<Point>) -> u16{
    let mut min_x = 100;
    let mut max_x = -100;

    for Point{x, y: _} in vertices.iter(){
        if x > &max_x {
            max_x = *x;
        }

        if x < &min_x {
            min_x = *x;
        }
    }

    return (max_x - min_x + 1) as u16;
}

fn calculate_height(vertices : &Vec<Point>) -> u16{
    let mut min_y = 100;
    let mut max_y = -100;

    for Point{x: _, y} in vertices.iter(){
        if y > &max_y {
            max_y = *y;
        }

        if y < &min_y {
            min_y = *y;
        }
    }

    return (max_y - min_y + 1) as u16;
}

pub fn rotate(brick: &mut Brick){
    let master_node = find_future_master_node(&brick.vertices);
    for vertex in brick.vertices.iter_mut(){
        let old_x = vertex.x;

        vertex.x = vertex.y;
        vertex.y = master_node.x - (old_x + 1);
    }

    if brick.orientation == Horizontal {
        brick.orientation = Vertical;
    }
    else {
        brick.orientation = Horizontal;
    }
}

fn find_future_master_node(vertices : &Vec<Point>) -> Point{
    //max old x & min old y will become new master node coordinates after the bricks rotation
    //master node is in the bottom left corner of a rectangle drawn over the brick

    let mut p: Point = vertices[0];

    for vertex in vertices.iter() {
        if vertex.x > p.x {
            p.x = vertex.x;
        }

        if vertex.y < p.y {
            p.y = vertex.y;
        }
    }

    p.x += 1;

    return p;
}

pub fn find_dead_master_node(vertices : &Vec<Point>) -> Point{
    //master node is in the bottom left corner of a rectangle drawn over the brick
    let mut p: Point = vertices[0];

    for vertex in vertices.iter() {
        if vertex.x < p.x {
            p.x = vertex.x;
        }

        if vertex.y > p.y {
            p.y = vertex.y;
        }
    }

    return p;
}

pub fn print_brick(brick : &Brick, master_node_position : &Point){
    clear_console();

    let vertices = get_screen_translated_vertices(&brick.vertices, master_node_position);

    print_dead_brick(&DeadBrick{vertices, color: brick.color});
}

pub fn print_dead_brick(brick: &DeadBrick){
    let mut console = io::stdout();

    console.execute(style::SetForegroundColor(brick.color)).unwrap();

    for point in brick.vertices.iter(){
        move_to_and_write(point.x, point.y, "*");
    }

    console.execute(style::SetForegroundColor(Color::White)).unwrap();
}
