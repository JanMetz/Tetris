mod brick;
mod field;
mod utility;
mod point;

use std::io;
use crate::brick::{generate_brick, print_brick, Brick, get_height, get_width, rotate, };
use crate::field::{can_descend_brick, create_field, Field, will_have_collision, land_brick, print_field};
use crate::point::{Point, translate_by};
use crate::utility::{Keys, move_to_and_write, setup_console, shorten_interval, spawn_stdin_channel, get_screen_translated_vertices};

use crate::State::{BrickGen, Descend, Stop, Touchdown};

extern crate crossterm;

use std::sync::mpsc::TryRecvError;
use std::time::Instant;
use num_traits::FromPrimitive;


#[derive(Copy, Clone)]
enum State { //state of the game
    Touchdown,
    BrickGen,
    Descend,
    Stop
}

fn rotate_brick(brick: &mut Brick, master_node_position : &mut Point, field: &Field) {
    let future_width = get_height(&brick) as i32;
    let fits_in_field = master_node_position.x + future_width < (field.width as i32);
    let mut requested_brick = brick.clone();
    rotate(&mut requested_brick);

    let mut future_mn_pos = master_node_position.clone();
    translate_by(&mut future_mn_pos, &Point { x: 0, y: 1 });

    let future_vertices = get_screen_translated_vertices(&requested_brick.vertices, &future_mn_pos);
    let current_vertices = get_screen_translated_vertices(&brick.vertices, &master_node_position);

    let has_collision = will_have_collision(&current_vertices, &future_vertices, &field);

    if fits_in_field && (!has_collision) {
        rotate(brick);
        print_brick(&brick, &master_node_position);
        print_field(&field);
    }
}

fn move_brick_right(brick: &mut Brick, master_node_position : &mut Point, field: &Field) {
    let mut future_mn_pos = master_node_position.clone();
    translate_by(&mut future_mn_pos, &Point { x: 1, y: 0 });
    let width = get_width(&brick) as i32;
    let fits_in_field = future_mn_pos.x + width < (field.width as i32);

    let future_vertices = get_screen_translated_vertices(&brick.vertices, &future_mn_pos);
    let current_vertices = get_screen_translated_vertices(&brick.vertices, &master_node_position);

    if fits_in_field && (!will_have_collision(&current_vertices, &future_vertices, &field)){
        translate_by(master_node_position, &Point { x: 1, y: 0 });
        print_brick(&brick, &master_node_position);
        print_field(&field);
    }
}

fn move_brick_left(brick: &mut Brick, master_node_position : &mut Point, field: &Field) {
    let mut future_mn_pos = master_node_position.clone();
    translate_by(&mut future_mn_pos, &Point { x: -1, y: 0 });
    let fits_in_field = future_mn_pos.x > 0;

    let future_vertices = get_screen_translated_vertices(&brick.vertices, &future_mn_pos);
    let current_vertices = get_screen_translated_vertices(&brick.vertices, &master_node_position);

    if fits_in_field && (!will_have_collision(&current_vertices, &future_vertices, &field)){
        translate_by(master_node_position, &Point { x: -1, y: 0 });
        print_brick(&brick, &master_node_position);
        print_field(&field);
    }
}

fn reset_master_node(brick: &Brick, master_node_position : &mut Point, field: &Field){
    master_node_position.y = 0;

    if master_node_position.x + brick.width_horizontal as i32 >= field.width as i32 {
        master_node_position.x = field.width as i32 - brick.width_horizontal as i32 - 1;
    }
}


fn main() {
    let field_width = 7; //including frame
    let field_height = 10; //including frame

    let mut state : State = BrickGen;
    let mut brick : Brick = generate_brick();
    let mut field : Field = create_field(field_width, field_height);
    let mut master_node_position = Point{x: ((field_width - 2) / 2) as i32, y: 0}; //master node is in the bottom left corner of a rectangle drawn over the brick
    let start = Instant::now();
    let mut interval = 1000;
    let stdin_channel = spawn_stdin_channel();

    setup_console();

    loop {
        // brick handling
        match state {
            Touchdown => {
                land_brick(&brick, &master_node_position, &mut field);
                print_field(&field);

                state = BrickGen;
            },

            BrickGen => {
                brick = generate_brick();
                reset_master_node(&brick, &mut master_node_position, &field);

                let translated_vertices = get_screen_translated_vertices(&brick.vertices, &master_node_position);
                if can_descend_brick(&translated_vertices, &field) {
                    state = Descend;
                }
                else {
                    state = Stop;
                }

            },

            Descend => {
                if start.elapsed().as_millis() % interval == 1 {
                    let translated_vertices = get_screen_translated_vertices(&brick.vertices, &master_node_position);

                    if can_descend_brick(&translated_vertices, &field) {
                        translate_by(&mut master_node_position, &Point { x: 0, y: 1 });
                        print_brick(&brick, &master_node_position);
                        print_field(&field);
                    }
                    else {
                        state = Touchdown;
                    }

                    interval = shorten_interval(interval);
                }
            },

            Stop => {
                move_to_and_write(field.width as i32 / 2 - 5, field_height as i32 + 2, "Game over!\nPress any key to continue...");
                let mut ignored = String::new();
                io::stdin().read_line(&mut ignored).unwrap();
                break;
            }
        }

        // steering handling
        match stdin_channel.try_recv() {
            Ok(key) => {
                match FromPrimitive::from_u16(key) {
                    Some(Keys::Up) => rotate_brick(&mut brick, &mut master_node_position, &field),

                    Some(Keys::Right) => move_brick_right(&mut brick, &mut master_node_position, &field),

                    Some(Keys::Left) => move_brick_left(&mut brick, &mut master_node_position, &field),

                    Some(Keys::Abort) =>  {
                        move_to_and_write(field.width as i32 / 2 - 5, field_height as i32 + 2, "Received abort");
                        break;
                    },

                    _ => continue
                }
            },

            Err(TryRecvError::Empty) => {
                continue;
            },

            Err(TryRecvError::Disconnected) => {
                panic!("Channel disconnected");
            },
        }
    }
}
