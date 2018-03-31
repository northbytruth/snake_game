extern crate pancurses;
extern crate rand;

use pancurses::{initscr, endwin, Input, noecho, Window, resize_term, half_delay};
use rand::{Rng, thread_rng};

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

static SPACE: &'static str = " ";
static FOOD: &'static str = "*";
static BODY: &'static str = "O";
static HEAD: &'static str = "0";
static BORDER: &'static str = "^";

fn startGame(screen : &mut Screen, snake :&mut Snake, food :&mut Point){


    
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
fn iterateScreen(screen :&mut Screen, content :&mut Vec<String>, key_in : &String, snake : &mut Snake, food :&mut Point, direction :&mut Move) -> i32{

    content.clear();
    
    //loop through screen to generate
    for i in 0..screen.height {

        let mut line = "".to_string();
        let mut changed: bool = false;
        for j in 0..screen.width {

            if  i==0 || j==0 || i==screen.height-1 || j==screen.width-1
            {
                line = line + &BORDER;
                changed = true;
            }
            

            if snake.head.y == i && snake.head.x == j {
                line = line + &HEAD;
                changed = true;
            }

            for p in 0..snake.body.len(){
                if snake.body[p].y == i && snake.body[p].x == j {
                    line = line + &BODY;
                    changed = true;
                    break;
                }
            }

            if food.y == i && food.x == j {
                line = line + &FOOD;
                changed = true;
            }

            if !changed {
                line = line + &SPACE;
            }

            changed = false;
        }
        content.push(line);
    }

    //check key to change direction
    if(key_in == "s"){
        direction.vertical = 1;
        direction.horizontal = 0;
    }
    if(key_in == "w"){
        direction.vertical = -1;
        direction.horizontal = 0;
    }
    if(key_in == "d"){
        direction.vertical = 0;
        direction.horizontal = 1;
    }
    if(key_in == "a"){
        direction.vertical = 0;
        direction.horizontal = -1;
    }

    //move snake
    let mut prev_head = Point{ x: snake.head.x, y: snake.head.y};
    if direction.vertical != 0{
        snake.head.y = snake.head.y+direction.vertical;
    }
    if direction.horizontal != 0{        
        snake.head.x = snake.head.x+direction.horizontal;
    }
    
    for idx in 0..snake.body.len() {
            let tmp = Point{ x: snake.body[idx].x, y: snake.body[idx].y };
            snake.body[idx] = Point{x: prev_head.x, y: prev_head.y};
            prev_head = tmp;
    }

    //check food eaten
    if food.x == snake.head.x && food.y == snake.head.y{
        //grow snake
        snake.body.push(Point{x:prev_head.x, y:prev_head.y});

        let mut rng = rand::thread_rng();
        food.x = rng.gen_range(2, screen.width-1);
        food.y = rng.gen_range(2, screen.height-1);
    }

    if snake.head.x < 1 || 
    (snake.head.x > screen.width-2) || 
    snake.head.y < 1 || 
    (snake.head.y > screen.height-2) {
        gameOver(content, snake);
        return 0;
    }

    return 1;

}

fn gameOver(content: &mut Vec<String>, snake: &Snake){

    content.clear();

    let mut s = "\n\n\n".to_string();
    content.push(s);
    content.push("  ___|    \\     \\  | ____|   _\\ \\     / ____|  _ \\  \n".to_string());
    content.push(" |       _ \\   |\\/ | __|    |   |\\ \\   /  __|   |   | \n".to_string());
    content.push(" |   |  ___ \\  |   | |      |   | \\ \\ /   |     __ <  \n".to_string());
    content.push("\\____|_/    _\\_|  _|_____| \\___/   \\_/   _____|_| \\_\\ \n".to_string());
    content.push("\n\n\n".to_string());

    content.push(format!("You scored: {} points!!\n",snake.body.len()));
    content.push("\n\n\n q to quit \n\n\n".to_string());
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
    
    //defaults
    let mut sc = Screen{ height : 40, width : 80};
    let mut food = Point { x: 25, y: 25 };
    let mut body_snake = Vec::new();
    let first_bodypart = Point {x: 20, y: 20};
    body_snake.push(first_bodypart);
    let mut snake = Snake { head: Point { x: 21, y: 20 }, body: body_snake };
    let mut lastKey = "".to_string();
    let mut direction = Move{vertical: 0, horizontal: 1};

    let window = initscr();
    resize_term(sc.height, sc.width);
    window.refresh();
    noecho();
    half_delay(1);
    
    addIntroToScreen(&mut sc, &mut content);
    
    loop {

        drawScreen(&mut content, &window);

        match window.getch() {
            Some(Input::Character(keyin)) => {

                lastKey = keyin.to_string();

                //quit game
                if keyin == 'q' {
                    //have confirmation here
                    break;
                }

                //start game
                if keyin == 'p' && gameRunning == false {
                    gameRunning = true;
                    startGame(&mut sc, &mut snake, &mut food);
                }

            },
            _ => (),
        }
        if gameRunning {
            let ret = iterateScreen(&mut sc, &mut content, &lastKey, &mut snake, &mut food, &mut direction);
            drawScreen(&mut content, &window);
            if(ret == 0){
                gameRunning = false;
            }
        }
    }
    endwin();
}