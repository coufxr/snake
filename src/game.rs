use std::process::{exit, Command};
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};

use crate::constants::{HEIGHT, SNAKE_BODY, SNAKE_HEAD, WIDTH};
use crate::food::Food;
use crate::map::Map;
use crate::snake::{Coord, Snake};

#[derive(Clone, Copy)]
pub(crate) enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Snake {
    /// 游戏是否结束
    pub(crate) fn is_game_over(&self) {
        let mut flag: bool = false;
        let head = self.head;
        // 如果蛇头触及到边界 触发游戏结束
        if head.x == 0 || head.x == HEIGHT - 1 || head.y == 0 || head.y == WIDTH - 1 {
            flag = true;
        }
        let body_list = &self.body;
        if body_list.is_empty() {
            flag = false;
        }
        // 蛇头触及到蛇的身体
        for body in body_list.iter() {
            if body.x == head.x && body.y == head.y {
                flag = true;
            }
        }

        if flag {
            println!("游戏结束！！！！");
            exit(0);
        }
    }

    /// 移动蛇
    pub(crate) fn move_snake(&mut self, food: &mut Food, map: &mut Map, direction: Direction) {
        let before_head = self.head;
        // 是否吃到了食物
        let eat: bool;
        // 蛇头往前移动一格，以前的蛇头变成蛇身的位置
        match direction {
            Direction::UP => {
                self.head.x -= 1;
                self.is_game_over();
                eat = self.new_grid(food, before_head, map);
            }
            Direction::DOWN => {
                self.head.x += 1;
                self.is_game_over();
                eat = self.new_grid(food, before_head, map);
            }
            Direction::LEFT => {
                self.head.y -= 1;
                self.is_game_over();
                eat = self.new_grid(food, before_head, map);
            }
            Direction::RIGHT => {
                self.head.y += 1;
                // 判断游戏是否结束
                self.is_game_over();
                eat = self.new_grid(food, before_head, map);
            }
        }
        // 蛇身去掉最后一个元素
        if !self.body.is_empty() && !eat {
            let snake_foot = self.body.remove(self.body.len() - 1);
            map.grid[snake_foot.x][snake_foot.y] = " ";
        }
        if eat {
            food.eat = true;
        }
        // 插入蛇身第一个元素
        self.body.insert(0, before_head);
    }

    fn new_grid(&mut self, food: &Food, before_head: Coord, map: &mut Map) -> bool {
        // 蛇头往前移动一格，以前的蛇头变成蛇身的位置
        // 先判断是否吃到了食物
        let eat = self.head == food.position;
        map.grid[before_head.x][before_head.y] = SNAKE_BODY;
        map.grid[self.head.x][self.head.y] = SNAKE_HEAD;
        return eat;
    }
}

/// 清空屏幕
pub(crate) fn clear_screen() {
    Command::new("cmd.exe")
        .arg("/c")
        .arg("cls")
        .status()
        .expect("clear error!");
}

/// 键盘控制
pub(crate) fn input_control(direction: Direction, keycodes: Option<KeyCode>) -> Direction {
    let mut dir = direction;
    if let Some(keycodes) = keycodes {
        match keycodes {
            KeyCode::Char('a') | KeyCode::Left => {
                dir = Direction::LEFT;
            }
            KeyCode::Char('d') | KeyCode::Right => {
                dir = Direction::RIGHT;
            }
            KeyCode::Char('s') | KeyCode::Down => {
                dir = Direction::DOWN;
            }
            KeyCode::Char('w') | KeyCode::Up => {
                dir = Direction::UP;
            }
            _ => {
                dir = direction;
            }
        }
    }
    return dir;
}

/// 等待键盘输入
pub(crate) async fn get_key_code() -> Option<KeyCode> {
    if poll(Duration::from_millis(100)).unwrap() {
        if let Ok(Event::Key(event)) = read() {
            return Some(event.code);
        }
    }
    None
}
