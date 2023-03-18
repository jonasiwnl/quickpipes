use termion::color::{self, Color};
use std::time::SystemTime;

// Generate random num and mod by ceil
fn gen_rand(ceil: u16) -> u16 {
    let mut lcg = Lcg::new();
    let num = lcg.next().unwrap() as u16 % ceil;

    num
}

fn color(n: u8) -> Box<dyn Color> {
    match n {
        0 => return Box::new(color::Black),
        1 => return Box::new(color::Red),
        2 => return Box::new(color::Green),
        3 => return Box::new(color::Yellow),

        4 => return Box::new(color::Blue),
        5 => return Box::new(color::Magenta),
        6 => return Box::new(color::Cyan),
        7 => return Box::new(color::White),

        8 => return Box::new(color::LightBlack),
        9 => return Box::new(color::LightRed),
        10 => return Box::new(color::LightGreen),
        11 => return Box::new(color::LightYellow),

        12 => return Box::new(color::LightBlue),
        13 => return Box::new(color::LightMagenta),
        14 => return Box::new(color::LightCyan),
        15 => return Box::new(color::LightWhite),

        _ => panic!()
    }
}

// Pseudorandom number gen
struct Lcg {
    state: u64,
    a: u64,
    c: u64,
}

impl Lcg {
    fn new() -> Self {
        // initialize with "random" state
        let state = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        Lcg {
            state,
            a: 1664525,
            c: 1013904223,
        }
    }
}

impl Iterator for Lcg {
    type Item = u64;

    // Generate new "random" number
    fn next(&mut self) -> Option<Self::Item> {
        self.state = self.a.wrapping_mul(self.state).wrapping_add(self.c);
        Some(self.state)
    }
}


pub enum Direction {
    Left, Right,
    Up, Down,
}

impl Direction {
    // u8 to direction
    pub fn direction(n: u8) -> Self {
        match n {
            0 => Self::Left,
            1 => Self::Up,
            2 => Self::Right,
            3 => Self::Down,
            _ => panic!(),
        }
    }
    // direction to u8
    pub fn int(&self) -> u8 {
        match self {
            Self::Left => 0,
            Self::Up => 1,
            Self::Right => 2,
            Self::Down => 3,
        }
    }
 }

// Simple Point datatype to store a position on the screen
// and direction. 
pub struct Point {
    pub pos: (u16, u16),
    pub color: Box<dyn Color>,
    pub next_direction: Direction,
    pub direction: Direction,
}

impl Point {
    fn new(
        pos: (u16, u16),
        color: Box<dyn Color>,
        direction: Direction,
        next_direction: Direction,
        ) -> Self 
    { Point { pos, color, direction, next_direction} }

    // random point initalization
    pub fn rand_init(c: u8, bounds: &(u16, u16)) -> Vec<Point> {
        let mut pv: Vec<Point> = vec![];

        for _ in 0..c {
            let direction = gen_rand(4) as u8;

            pv.push(Self::new(
                (
                    gen_rand(bounds.0 - 1) + 1,
                    gen_rand(bounds.1 - 1) + 1
                ),
                color(gen_rand(16) as u8),
                Direction::direction(direction),
                Direction::direction(direction),
            ));
        }

        pv
    }

    // step a point, with bounds checking
    pub fn step(&mut self, bounds: &(u16, u16)) -> () {
        self.direction = Direction::direction(self.next_direction.int());

        // move function
        // if point is out of bounds, pass thru
        // this is so unclean
        match self.direction {
            Direction::Left => {
                self.pos.0 -= 1;
                if self.pos.0 < 1 {
                    self.pos.0 = bounds.0;
                }
            },
            Direction::Right => {
                self.pos.0 += 1;
                if self.pos.0 > bounds.0 {
                    self.pos.0 = 1;
                }
            },
            Direction::Up => {
                self.pos.1 -= 1;
                if self.pos.1 < 1 {
                    self.pos.1 = bounds.1;
                }
            },
            Direction::Down => {
                self.pos.1 += 1;
                if self.pos.1 > bounds.1 {
                    self.pos.1 = 1;
                }
            },
        }

        // randomly change direction
        let gr = gen_rand(25);
        if gr != 1 { return; }
        let mv = gen_rand(2);
        if self.next_direction.int() % 2 == 0 {
            match mv {
                0 => self.next_direction = Direction::Up,
                1 => self.next_direction = Direction::Down,
                _ => panic!(),
            }
        } else {
            match mv {
                0 => self.next_direction = Direction::Left,
                1 => self.next_direction = Direction::Right,
                _ => panic!(),
            }
        }
    }
}
