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


fn startGame(screen : &mut Screen, content :&mut Vec<String>, snake :&mut Snake){

    //place first food
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

    let mut s = "\n\n\n".to_string();
    content.push(s);
    s="    _/_/_/    _/    _/    _/_/_/  _/_/_/_/_/  \n".to_string();
    content.push(s);
    s="   _/    _/  _/    _/  _/            _/       \n".to_string();
    content.push(s);
    s="  _/_/_/    _/    _/    _/_/        _/        \n".to_string();
    content.push(s);
    s=" _/    _/  _/    _/        _/      _/         \n".to_string();
    content.push(s);
    s="_/    _/    _/_/    _/_/_/        _/          \n".to_string();
    content.push(s);

    s = "\n\n".to_string();
    content.push(s);

    s="       _/_/_/  _/      _/    _/_/    _/    _/  _/_/_/_/      _/ \n".to_string();
    content.push(s);
    s="    _/        _/_/    _/  _/    _/  _/  _/    _/            _/  \n".to_string();
    content.push(s);
    s="     _/_/    _/  _/  _/  _/_/_/_/  _/_/      _/_/_/        _/   \n".to_string(); 
    content.push(s);
    s="        _/  _/    _/_/  _/    _/  _/  _/    _/                  \n".to_string();
    content.push(s);
    s=" _/_/_/    _/      _/  _/    _/  _/    _/  _/_/_/_/      _/     \n".to_string();
    content.push(s);
    s = "\n\n".to_string();
    content.push(s);
    s = "      Press p to start \n\n".to_string();
    content.push(s);
}


//randomise food and iterate snake movement
fn iterateScreen(screen :&mut Screen, content :&mut Vec<String>, key_in : char){

    //check key to change direction

    //if 
    //loop through columns then rows

    //make diff only changes
    //loop through screen to generate
    for i in 0..screen.height {

        let mut line: String = "".to_owned();
        
        for j in 0..screen.width {
            //line.push_str()

        }
    }

    //place food

    //check food eaten

    //grow snake if needed

    //move snake

}


fn drawScreen(content :&mut Vec<String>, window :&Window){

    window.clear();

    for row in content.iter() {
        window.printw(row);
    }

    window.refresh();
}

fn main() {

    let mut gameRunning: bool = false;
    let mut content = Vec::new();
    
    let mut foodLocation = Point { x: 50, y: 50 };
    let mut sc = Screen{ height : 100, width : 100, };

    let mut body_snake = Vec::new();
    body_snake.push(Point {x: 39, y: 40});
    let mut snake = Snake { head: Point { x: 40, y: 40 }, body: body_snake };

    let window = initscr();
    
    window.refresh();
    noecho();
    
    addIntroToScreen(&mut sc, &mut content);
    drawScreen(&mut content, &window);

    loop {
        match window.getch() {
            Some(Input::Character(keyin)) => {

                //quit game
                if keyin == 'q' {
                    //have confirmation here
                    break;
                }

                //start game
                if keyin == 'p' && gameRunning == false {
                    gameRunning = true;
                    startGame(&mut sc, &mut content, &mut snake);
                }

                iterateScreen(&mut sc, &mut content, keyin);
                drawScreen(&mut content, &window);
            },
            _ => (),
        }

    }
    endwin();
}