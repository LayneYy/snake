extern crate piston_window;

use std::collections::LinkedList;
use std::time::{Duration, SystemTime};

use piston_window::*;
use rand::Rng;
use types::Color;

//蛇的颜色
const SNAKE_COLOR: Color = [0.73, 0.23, 0.60, 1.0];
//食物的颜色
const FOOD_COLOR: Color = [1.00, 1.00, 0.00, 1.0];
//块的大小
const BLOCK_SIZE: f64 = 50.0;
//浮点计算允许的最大精度差
const FLOAT: f64 = 0.0001;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("layne's snake", (700, 700))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build Window: {}", e) });

    let mut snake = Snake::new();
    let mut food: Food = Food::produce();

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            if let Button::Keyboard(key) = button {
                if let Some(d) = trans_key_to_direction(key) {
                    snake.ch_direction(d);
                    snake.do_move();
                }
            }
        } else if snake.pass_secs(1) {
            snake.do_move();
        }
        if food.can_be_eaten(&snake) {
            snake.eat(food);
            food = Food::produce();
        }
        window.draw_2d(&e, |c, g, _d| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            snake.draw(c, g);
            food.draw(c, g, FOOD_COLOR)
        });
    }
}

struct Snake {
    body: LinkedList<Block>,
    direction: Direction,
    last_move_time: SystemTime,
}

impl Snake {
    //新建一个蛇
    fn new() -> Self {
        let mut body = LinkedList::new();
        let block1 = Block::new(0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE);
        let block2 = Block::new(BLOCK_SIZE, 0.0, BLOCK_SIZE, BLOCK_SIZE);
        let block3 = Block::new(BLOCK_SIZE * 2.0, 0.0, BLOCK_SIZE, BLOCK_SIZE);
        body.push_front(block1);
        body.push_front(block2);
        body.push_front(block3);
        Snake { body, direction: Direction::Right, last_move_time: SystemTime::now() }
    }
    fn ch_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    //画蛇
    fn draw(&self, c: Context, g: &mut G2d) {
        self.body.iter().for_each(|block| block.draw(c, g, SNAKE_COLOR));
    }
    //移动蛇
    fn do_move(&mut self) {
        let ref mut body = self.body;
        let (x, y) = {
            let Block { x, y, w: _, h: _ } = body.front().unwrap();
            (*x, *y)
        };
        let mut tail = body.pop_back().unwrap();

        match self.direction {
            Direction::Up => {
                tail.x = x;
                tail.y = y - BLOCK_SIZE;
            }
            Direction::Down => {
                tail.x = x;
                tail.y = y + BLOCK_SIZE;
            }
            Direction::Left => {
                tail.y = y;
                tail.x = x - BLOCK_SIZE;
            }
            Direction::Right => {
                tail.y = y;
                tail.x = x + BLOCK_SIZE;
            }
        }
        body.push_front(tail);
        self.last_move_time = SystemTime::now();
    }

    fn pass_secs(&self, secs: u64) -> bool {
        SystemTime::now().duration_since(self.last_move_time).unwrap() >= Duration::from_secs(secs)
    }

    fn eat(&mut self, mut block: Block) {
        let (x, y) = {
            let Block { x, y, w: _, h: _ } = self.body.front().unwrap();
            (*x, *y)
        };
        match self.direction {
            Direction::Up => {
                block.x = x;
                block.y = y - BLOCK_SIZE;
            }
            Direction::Down => {
                block.x = x;
                block.y = y + BLOCK_SIZE;
            }
            Direction::Left => {
                block.y = y;
                block.x = x - BLOCK_SIZE;
            }
            Direction::Right => {
                block.y = y;
                block.x = x + BLOCK_SIZE;
            }
        }
        self.body.push_front(block);
    }
}

//方向
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//将键盘输入转化成方向
fn trans_key_to_direction(key: Key) -> Option<Direction> {
    match key {
        Key::Up => Some(Direction::Up),
        Key::Down => Some(Direction::Down),
        Key::Right => Some(Direction::Right),
        Key::Left => Some(Direction::Left),
        _ => None,
    }
}

//组成身体的块,其实就是一个rectangle
struct Block {
    //很轴
    x: f64,
    //纵轴
    y: f64,
    //宽
    w: f64,
    //高
    h: f64,
}

impl Block {
    //新建一个块
    fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }
    //画一个块
    fn draw(&self, c: Context, g: &mut G2d, color: Color) {
        let Block { x, y, w, h } = *self;
        rectangle(color,
                  [x, y, w, h],
                  c.transform,
                  g);
    }
}

type Food = Block;

impl Food {
    fn produce() -> Self {
        let gen = || {
            let mut rng = rand::thread_rng();
            rng.gen_range(0, 13)
        };
        let (x, y) = (gen(), gen());
        Block::new(BLOCK_SIZE * x as f64, BLOCK_SIZE * y as f64, BLOCK_SIZE, BLOCK_SIZE)
    }

    fn can_be_eaten(&self, snake: &Snake) -> bool {
        let Block { x, y, w: _, h: _ } = *self;
        let Block { x: x1, y: y1, w: _, h: _ } = *snake.body.front().unwrap();
        (x - x1).abs() <= FLOAT && (y - y1).abs() <= FLOAT
    }
}