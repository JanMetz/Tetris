use crate::brick::{Brick, DeadBrick, find_dead_master_node, print_dead_brick};
use crate::point::{are_touching, Point, translate_by};
use crate::utility::{clear_console, move_to_and_write, get_screen_translated_vertices, get_translated_vertices};

#[derive(Clone)]
pub struct Field{
    pub(crate) bricks : Vec<DeadBrick>,
    pub(crate) height : u16,
    pub(crate) width : u16
}

pub fn create_field(width : u16, height: u16) -> Field{
    Field{bricks: vec![], height, width}
}

pub fn will_have_collision(vertices : &Vec<Point>, field: &Field) -> bool {
    for dead_brick in field.bricks.iter(){
        for dead_ver in dead_brick.vertices.iter() {
            for new_ver in vertices.iter() {
                if dead_ver == new_ver {
                    return true;
                }
            }
        }
    }

    return false;
}

pub fn land_brick(brick: &Brick, master_node_position : &Point, field: &mut Field){
    let vertices = get_screen_translated_vertices(&brick.vertices, master_node_position);

    field.bricks.push(DeadBrick { vertices, color: brick.color });

    remove_full_rows(field);
}

pub fn find_full_rows(field : &Field) -> Vec<i32>{
    let mut rows : Vec<i32> = vec![];
    let mut fills;

    for row in (0..field.height).rev() {
        fills = true;
        for col in 1..(field.width - 1) {
            fills = does_vertex_exist(field, row as i32, col as i32);

            if !fills {
                break;
            }
        }

        if fills {
            rows.push(row as i32);
        }
    }

    return rows;
}

fn does_vertex_exist(field: &Field, row: i32, col: i32) -> bool {
    for brick in field.bricks.iter() {
        for vertex in brick.vertices.iter(){
            if vertex.x == col && vertex.y == row {
                return true;
            }
        }
    }

    return false;
}

pub fn can_descend_brick(brick : &Brick, master_node_pos : &Point, field: &Field) -> bool{
    let translated_vertices = get_screen_translated_vertices(&brick.vertices, &master_node_pos);
    let lowered_translated_vertices = get_translated_vertices(&translated_vertices, &Point{x: 0, y: 1});

    return ((master_node_pos.y + 1) < field.height as i32) && (!will_have_collision(&lowered_translated_vertices, field));
}

pub fn can_descend_dead_brick(field: &Field, idx : usize) -> bool{
    let brick_to_be_lowered = field.bricks.get(idx).unwrap();
    let master_node_pos = find_dead_master_node(&brick_to_be_lowered.vertices);

    if (master_node_pos.y + 1) >= field.height as i32{
        return false;
    }

    for i in 0..field.bricks.len() {
        if i == idx {
            continue;
        }

        let brick = field.bricks.get(i).unwrap();

        for ver in brick.vertices.iter() {
            for new_ver in brick_to_be_lowered.vertices.iter() {
                if ver.x == new_ver.x && ver.y == (new_ver.y + 1) {
                    return false;
                }
            }
        }
    }

    return true;
}

fn did_lower_dead_brick(field: &mut Field, idx : usize) -> bool{
    let mut was_lowered = false;

    while can_descend_dead_brick(field, idx){
        was_lowered = true;
        for point in field.bricks.get_mut(idx).unwrap().vertices.iter_mut(){
            translate_by(point , &Point{x: 0, y: 1});
        }
    }

    return was_lowered;
}

fn remove_empty_bricks(field: &mut Field){
    loop {
        let pos = field.bricks.iter().position(|dead| dead.vertices.len() == 0);

        if pos != None {
            field.bricks.remove(pos.unwrap());
        }
        else {
            break;
        }
    }
}

fn split_disconnected_subbricks(field: &mut Field){
    let bricks_copy = field.bricks.clone();

    for brick_idx in 0..bricks_copy.len(){
        let brick = bricks_copy.get(brick_idx).unwrap();

        for vertex_idx in 0..brick.vertices.len() {
            let my_vertex= brick.vertices.get(vertex_idx).unwrap();
            let mut touches_any_neighbour= false;

            for other_vertex in brick.vertices.iter() {
                if my_vertex == other_vertex {
                    continue;
                }

                touches_any_neighbour |= are_touching(my_vertex, other_vertex);
            }

            if touches_any_neighbour == false && brick.vertices.len() > 1 {
                remove_vertex(field, brick_idx, vertex_idx);
                field.bricks.push(DeadBrick{vertices: vec![*my_vertex], color: brick.color });
            }
        }
    }
}

fn remove_vertex(field: &mut Field, brick_idx : usize, vertex_idx : usize) {
    let mut brick = field.bricks.get_mut(brick_idx).unwrap();
    brick.vertices.remove(vertex_idx);

    if brick.vertices.len() == 0 {
        remove_empty_bricks(field);
    }
}

pub fn remove_full_rows(field: &mut Field){
    let full_rows = find_full_rows(field);

    if full_rows.len() == 0 {
        return;
    }

    for row in full_rows.iter(){
        for brick in field.bricks.iter_mut(){
            let mut new_vertices : Vec<Point> = vec![];
            for vertex in brick.vertices.iter(){
                if vertex.y != *row {
                    new_vertices.push(*vertex);
                }
            }

            brick.vertices = new_vertices;
        }
    }

    remove_empty_bricks(field);
    split_disconnected_subbricks(field);

    loop { //to make sure no bricks are blocking virtually
        let mut any_brick_lowered = false;

        for i in 0..field.bricks.len() {
            any_brick_lowered |= did_lower_dead_brick(field, i);
        }

        if !any_brick_lowered {
            break;
        }
    }

    remove_full_rows(field);

    clear_console();
}

fn print_frame(field: &Field) {
    //left border
    for row in 0..field.height{
        move_to_and_write(0, row as i32, "+");
    }

    //bottom
    for col in 0..(field.width + 1){
        move_to_and_write(col as i32, field.height as i32, "+");
    }

    //right border
    for row in 0..field.height{
        move_to_and_write(field.width as i32, row as i32, "+");
    }
}

pub fn print_field(field: &Field){
    print_frame(field);

    for brick in field.bricks.iter() {
        print_dead_brick(&brick);
    }
}
