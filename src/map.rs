use crate::constants::{HEIGHT, SNAKE_BODY, SNAKE_HEAD, WIDTH};
use crate::food::Food;
use crate::snake::Snake;

pub(crate) struct Map {
    pub(crate) grid: [[&'static str; WIDTH]; HEIGHT],
}

impl Map {
    /// 创建新的 Map
    pub(crate) fn new() -> Self {
        let block = "■";
        let empty = " ";
        let mut grid = [[empty; WIDTH]; HEIGHT];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                // 第一行和最后一行边界
                if i == 0 || i == grid.len() - 1 {
                    grid[i][j] = block;
                }

                // 第一列和最后一列边界
                if j == 0 || j == grid[i].len() - 1 {
                    grid[i][j] = block;
                }
            }
        }
        return Self { grid };
    }
    /// 将蛇加入地图
    pub(crate) fn add_snake(&mut self, snake: &Snake) {
        let head = snake.head;
        self.grid[head.x][head.y] = SNAKE_HEAD;
        for i in snake.body.iter() {
            self.grid[i.x][i.y] = SNAKE_BODY;
        }
    }

    /// 将食物加入地图
    pub(crate) fn add_food(&mut self, food: &Food) {
        if food.eat {
            // 食物如果被吃了，不要加到地图
            return;
        }
        let position = food.position;
        self.grid[position.x][position.y] = "▣";
    }

    /// 打印地图
    pub(crate) fn print_map(&self) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                print!("{}", self.grid[i][j]);
            }
            println!();
        }
    }
}

#[test]
fn test_map() {
    let mut map = Map::new();
    let snake = Snake::new();
    let food = Food::new();
    map.add_snake(&snake);
    map.add_food(&food);
    map.print_map()
}
