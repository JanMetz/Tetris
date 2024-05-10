use crate::brick::{Brick, DeadBrick, find_dead_master_node, print_dead_brick};
use crate::point::{are_touching, Point, translate_by};
use crate::utility::{clear_console, move_to_and_write, get_translated_vertices, get_screen_translated_vertices};

#[derive(Clone)]
pub struct Field{
    pub(crate) bricks : Vec<DeadBrick>,
    pub(crate) height : u16,
    pub(crate) width : u16
}

pub fn create_field(width : u16, height: u16) -> Field{
    Field{bricks: vec![], height, width}
}

pub fn will_have_collision(current_vertices: &Vec<Point>, future_vertices: &Vec<Point>, field: &Field) -> bool {
    for brick in field.bricks.iter(){
        if brick.vertices != *current_vertices {
            for dead_ver in brick.vertices.iter() {
                for new_ver in future_vertices.iter() {
                    if dead_ver == new_ver {
                        return true;
                    }
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

pub fn can_descend_brick(vertices: &Vec<Point>, field: &Field) -> bool{
    let lowered_vertices = get_translated_vertices(&vertices, &Point{x: 0, y: 1});
    let master_node_pos = find_dead_master_node(&lowered_vertices);

    return (master_node_pos.y < field.height as i32) && (!will_have_collision(&vertices, &lowered_vertices, field));
}

fn lower_dead_brick(brick : &mut DeadBrick, field: &Field){
    while can_descend_brick(&brick.vertices, field){
        for point in brick.vertices.iter_mut(){
            translate_by(point , &Point{x: 0, y: 1});
        }
    }
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

    for brick in bricks_copy.into_iter(){
        for my_vertex in brick.vertices.iter() {
            let mut touches_any_neighbour= false;
            for other_vertex in brick.vertices.iter() {
                if my_vertex == other_vertex {
                    continue;
                }

                touches_any_neighbour |= are_touching(my_vertex, other_vertex);
            }

            if touches_any_neighbour == false && brick.vertices.len() > 1 {
                remove_vertex(field, &my_vertex);
                field.bricks.push(DeadBrick{vertices: vec![*my_vertex], color: brick.color });
            }
        }
    }
}

fn remove_vertex(field: &mut Field, vertex: &Point) {
    for brick in field.bricks.iter_mut(){
        let position = brick.vertices.iter().position(|point| point == vertex);

        if position != None {
            brick.vertices.remove(position.unwrap());
            if brick.vertices.len() == 0 {
                remove_empty_bricks(field);
            }

            break;
        }
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

    let field_immut = field.clone();

    for brick in field.bricks.iter_mut(){
        lower_dead_brick(brick, &field_immut);
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