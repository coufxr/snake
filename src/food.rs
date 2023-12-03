use rand::Rng;

use crate::constants::{HEIGHT, WIDTH};
use crate::snake::Coord;

/// 食物
#[derive(Debug, Clone, Copy)]
pub(crate) struct Food {
    pub(crate) position: Coord,
    pub(crate) eat: bool,
}

impl Food {
    pub(crate) fn new() -> Food {
        let position = Coord::new(8, 10);
        return Food {
            position,
            eat: false,
        };
    }
    /// TODO 随机生成食物,这里仅是简单实现，生成食物的时候需要避开蛇的身体
    pub(crate) fn random_new(&mut self) -> Option<Food> {
        if !self.eat {
            // 如果当前的食物没有被吃掉，则不生成新的食物
            return None;
        }
        // let HEAD = snake.HEAD;
        // let body = &(snake.body);
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1..HEIGHT - 1);
        let y = rng.gen_range(1..WIDTH - 1);
        self.position = Coord::new(x, y);
        self.eat = false;

        return Some(*self);
    }
}
