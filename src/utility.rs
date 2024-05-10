use std::{io, thread};
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read};
use crossterm::{cursor, ExecutableCommand, execute, terminal};
use num_derive::FromPrimitive;

use crate::point::{Point, translate_by, translate_to_screen};

#[derive(FromPrimitive)]
pub enum Keys{
    Up = 1,
    Right = 2,
    Left = 3,
    Abort = 4
}

pub fn spawn_stdin_channel() -> Receiver<u16> {
    let (tx, rx) = mpsc::channel::<u16>();
    thread::spawn(move || loop {
        match read().unwrap() {
            Event::Key(KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, .. }) => tx.send(Keys::Up as u16).unwrap(),
            Event::Key(KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, .. }) => tx.send(Keys::Right as u16).unwrap(),
            Event::Key(KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, .. }) => tx.send(Keys::Left as u16).unwrap(),
            Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, kind: KeyEventKind::Release, .. }) => {
                tx.send(Keys::Abort as u16).unwrap();
                break;
            },
            _ => continue
        }
    });

    return rx
}

pub fn get_screen_translated_vertices(vertices : &Vec<Point>, master_node_position : &Point) -> Vec<Point>{
    let mut translated : Vec<Point> = vec![];

    for point in vertices.iter(){
        let new_point = translate_to_screen(point, &master_node_position);
        translated.push(new_point);
    }

    return translated;
}

pub fn get_translated_vertices(vertices : &Vec<Point>, master_node_position : &Point) -> Vec<Point>{
    let mut translated : Vec<Point> = vec![];

    for point in vertices.iter(){
        let mut new_point = point.clone();
        translate_by(&mut new_point, &master_node_position);
        translated.push(new_point);
    }

    return translated;
}

pub fn shorten_interval(interval : u128) -> u128{
    let new_interval : f32 = (interval as f32) * (0.999f32);
    return new_interval.floor() as u128;
}

pub fn setup_console(){
    execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::DisableBlinking,
        cursor::Hide
    ).expect("terminal::Clear or Cursor::DisableBlinking or Cursor::Hide failed");
}


pub fn move_to_and_write(mut x : i32, mut y : i32, msg : &str){
    let mut console = io::stdout();

    if y < 0{
        y = 0;
    }

    if x < 0{
        x = 0;
    }

    console.execute(MoveTo(x as u16, y as u16)).expect("MoveTo failed");
    console.write(msg.as_ref()).unwrap();
}

pub fn clear_console(){
    let mut console = io::stdout();

    execute!(
        console,
        terminal::Clear(terminal::ClearType::All)
    ).expect("Terminal::Clear failed");
}