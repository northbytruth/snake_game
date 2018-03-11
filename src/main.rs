extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho, Window};

struct Point {
    x: i32,
    y: i32,
}

struct Snake {
    head: Point,
    body: Vec<Point>,
}

struct Screen {
    content: Vec<String>,
    height: i32,
    width: i32,
}

static SPACE: char = ' ';
static FOOD: char = '*';
static BODY: char = 'o';

fn drawScreen(screenToDisplay :&Screen, window :&Window){

    window.clear();

    window.printw("jnyrgdb\n");
    window.printw("GEGE\n");
    window.printw("GG\n");
    window.printw("BB\n");

    window.refresh();
}

fn main() {

    let sc = Screen{ gg: 5 };
    let window = initscr();
    window.printw("Welcome to Snake\n");
    window.refresh();
    noecho();
    
    loop {
        match window.getch() {
            Some(Input::Character(keyin)) => {
                if keyin == 'q' {
                    break;
                }

                if keyin == 'g' {
                    drawScreen(&sc, &window);
                }

                let origin = Point { x: 0, y: 0 }; // origin: Point

                let prnt = format!("KEYIN: {}", keyin);
                window.printw(&prnt);
            },
            _ => (),
        }
    }
    endwin();
}