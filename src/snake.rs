use crate::constants::SPEED;

#[derive(Debug, Clone, Copy, PartialEq)] // 需要最底层实现外层的所以特征
pub(crate) struct Coord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Coord {
    pub(crate) fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Snake {
    // ● 蛇头
    pub(crate) head: Coord,
    // 速度
    pub(crate) speed: usize,
    // 蛇身
    pub(crate) body: Vec<Coord>,
}

impl Snake {
    /// TODO 需要修改为随机生产
    pub(crate) fn new() -> Snake {
        let head = Coord::new(5, 6);
        let speed = SPEED;
        let body = vec![Coord::new(5, 5), Coord::new(5, 4)];
        return Snake { head, speed, body };
    }
}

#[test]
fn test_map() {
    let snake = Snake::new();
    println!("{snake:?}")
}
