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
    height: i32,
    width: i32,
}

struct Move {
    vertical: i32,
    horizontal: i32,
}

static SPACE: char = ' ';
static FOOD: char = '*';
static BODY: char = 'o';
static HEAD: char = 'c';


fn startGame(screen : &mut Screen, content :&mut Vec<String>){

    content.clear();
    
    //generate board
    let mut s = "                   \n".to_string();
    content.push(s);
    s = "                   \n".to_string();
    content.push(s);
    s = format!("{}{}{}{}{}{}{}{}\n",SPACE,BODY,BODY,BODY,HEAD,SPACE,SPACE,FOOD);
    content.push(s);
    s = "                   \n".to_string();
    content.push(s);
    s = "                   \n".to_string();
    content.push(s);
}

fn addIntroToScreen(screen : &mut Screen, content :&mut Vec<String>){

    content.clear();
    let mut s = "         ____\n".to_string();
    content.push(s);
    s = "         SNAKE\n".to_string();
    content.push(s);
    s = "         _____\n".to_string();
    content.push(s);
    s = "              \n".to_string();
    content.push(s);
    s = "Press p to start \n".to_string();
    content.push(s);
}


//randomise food and iterate snake movement
fn iterateScreen(screen :&mut Screen, content :&mut Vec<String>, window :&Window){

    //loop through columns then rows

    //place food

    //check food eaten

    //grow snake if needed

    //move snake

}

fn initialiseSnakeAndFood(screen :&mut Screen, content :&mut Vec<String>, window :&Window){

    //place first food

}


fn drawScreen(screen :&mut Screen, content :&mut Vec<String>, window :&Window){

    window.clear();

    //loop through columns then rows

    for row in content.iter() {

        window.printw(row);
    }

    window.refresh();
}

fn main() {

    let mut content = Vec::new();

    let mut foodLocation = Point { x: 0, y: 0 };
    let mut sc = Screen{ height : 100, width : 100, };

    let window = initscr();
    
    window.refresh();
    noecho();

    addIntroToScreen(&mut sc, &mut content);
    
    loop {
        match window.getch() {
            Some(Input::Character(keyin)) => {
                
                //quit game
                if keyin == 'q' {
                    break;
                }

                //start game
                if keyin == 'p' {
                    startGame(&mut sc, &mut content);
                    initialiseSnakeAndFood(&mut sc, &mut content, &window);
                }

                

                let prnt = format!("KEYIN: {}", keyin);
                window.printw(&prnt);
            },
            _ => (),
        }

        //randomise food and iterate snake movement
        iterateScreen(&mut sc, &mut content, &window);

        drawScreen(&mut sc, &mut content, &window);
    }
    endwin();
}