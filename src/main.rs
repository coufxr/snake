use crate::food::Food;
use crate::game::{get_key_code, Direction};
use crate::map::Map;
use crate::snake::Snake;
use std::time::Duration;
use tokio::time::sleep;
mod constants;
mod food;
mod game;
mod map;
mod snake;

#[tokio::main]
async fn main() {
    let mut map = Map::new();
    let mut snake = Snake::new();
    let mut food = Food::new();

    // 方向
    let mut current_direction = Direction::RIGHT;
    //
    loop {
        game::clear_screen();
        // 将蛇加入地图
        map.add_snake(&snake);
        // 将食物加入地图
        map.add_food(&food);
        // 随机生成食物
        food.random_new();
        map.add_food(&food);
        // 打印地图、蛇、食物
        map.print_map();

        let event = get_key_code().await;
        // 获取新的方向
        current_direction = game::input_control(current_direction, event);
        // 运动
        snake.move_snake(&mut food, &mut map, current_direction);

        sleep(Duration::from_millis(snake.speed as u64)).await;
    }
    // Command::new("cmd.exe")
    //     .arg("/c")
    //     .arg("pause")
    //     .status()
    //     .expect("clear error!");
}
