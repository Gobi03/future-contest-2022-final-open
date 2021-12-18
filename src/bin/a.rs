#[allow(unused_imports)]
use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{fastout, input};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(unused_imports)]
use rand::rngs::ThreadRng;
#[allow(unused_imports)]
use rand::seq::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};
#[allow(unused_imports)]
use std::io::Write;
use std::time::SystemTime;

#[allow(dead_code)]
const MOD: usize = 1e9 as usize + 7;

const N: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: isize,
    y: isize,
}

#[allow(dead_code)]
impl Coord {
    fn new(p: (isize, isize)) -> Self {
        Coord { x: p.0, y: p.1 }
    }
    fn from_usize_pair(p: (usize, usize)) -> Self {
        Coord {
            x: p.0 as isize,
            y: p.1 as isize,
        }
    }

    fn in_field(&self) -> bool {
        (0 <= self.x && self.x < N as isize) && (0 <= self.y && self.y < N as isize)
    }

    // ペアへの変換
    fn to_pair(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    // マンハッタン距離
    fn distance(&self, that: &Self) -> isize {
        (self.x - that.x).abs() + (self.y - that.y).abs()
    }

    fn mk_4dir(&self) -> Vec<Self> {
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        delta
            .iter()
            .map(|&p| self.plus(&Coord::new(p)))
            .filter(|&pos| pos.in_field())
            .collect()
    }

    fn com_to_delta(com: char) -> Self {
        match com {
            'U' => Coord::new((0, -1)),
            'D' => Coord::new((0, 1)),
            'L' => Coord::new((-1, 0)),
            'R' => Coord::new((1, 0)),
            _ => unreachable!(),
        }
    }

    // 四則演算
    fn plus(&self, that: &Self) -> Self {
        Coord::new((self.x + that.x, self.y + that.y))
    }
    fn minus(&self, that: &Self) -> Self {
        Coord::new((self.x - that.x, self.y - that.y))
    }

    fn access_matrix<'a, T>(&'a self, mat: &'a Vec<Vec<T>>) -> &'a T {
        &mat[self.y as usize][self.x as usize]
    }

    fn set_matrix<T>(&self, mat: &mut Vec<Vec<T>>, e: T) {
        mat[self.y as usize][self.x as usize] = e;
    }
}

struct Input {
    start: Coord,
    h: Vec<Vec<bool>>, // x, x+1 の間に壁がある
    v: Vec<Vec<bool>>, // y, y+1 の間に壁がある
}
impl Input {
    fn new(sy: usize, sx: usize, h: Vec<Vec<char>>, v: Vec<Vec<char>>) -> Self {
        let mut hh = vec![vec![false; N - 1]; N];
        let mut vv = vec![vec![false; N]; N - 1];
        for y in 0..N {
            for x in 0..N - 1 {
                if h[y][x] == '1' {
                    hh[y][x] = true;
                }
            }
        }
        for y in 0..N - 1 {
            for x in 0..N {
                if v[y][x] == '1' {
                    vv[y][x] = true;
                }
            }
        }

        Self {
            start: Coord::from_usize_pair((sx, sy)),
            h: hh,
            v: vv,
        }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn to_delta(&self) -> Coord {
        match *self {
            Self::Left => Coord::new((-1, 0)),
            Self::Right => Coord::new((1, 0)),
            Self::Up => Coord::new((0, -1)),
            Self::Down => Coord::new((0, 1)),
        }
    }

    fn rotate_right(&self) -> Self {
        match *self {
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
        }
    }
    fn rotate_left(&self) -> Self {
        match *self {
            Self::Left => Self::Down,
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Down => Self::Right,
        }
    }
}

enum Command {
    TurnR,
    TurnL,
    Turnr,
    Turnl,
    F,
}
impl Command {
    fn to_char(&self) -> char {
        match *self {
            Self::TurnR => 'R',
            Self::TurnL => 'L',
            Self::Turnr => 'r',
            Self::Turnl => 'l',
            Self::F => 'F',
        }
    }
}

struct Robot {
    pos: Coord,
    direction: Direction,
}
impl Robot {
    fn new(input: &Input) -> Self {
        Self {
            pos: input.start.clone(),
            direction: Direction::Up,
        }
    }

    fn can_progress(&self, input: &Input) -> bool {
        let next = self.pos.plus(&self.direction.to_delta());
        if next.in_field() {
            match self.direction {
                Direction::Left => !next.access_matrix(&input.h).clone(),
                Direction::Right => !self.pos.access_matrix(&input.h).clone(),
                Direction::Up => !next.access_matrix(&input.v).clone(),
                Direction::Down => !self.pos.access_matrix(&input.v).clone(),
            }
        } else {
            false
        }
    }

    // valid な命令が来る前提
    fn do_command(&mut self, command: &Command) {
        match command {
            Command::TurnR => self.direction = self.direction.rotate_right(),
            Command::TurnL => self.direction = self.direction.rotate_left(),
            Command::Turnr => self.direction = self.direction.rotate_right(),
            Command::Turnl => self.direction = self.direction.rotate_left(),
            Command::F => self.pos = self.pos.plus(&self.direction.to_delta()),
        }
    }
}

#[fastout]
fn main() {
    let system_time = SystemTime::now();
    let mut rng = thread_rng();

    input! {
        sy: usize,
        sx: usize,
        h: [Chars; N],
        v: [Chars; N-1],
    }

    let input = Input::new(sy, sx, h, v);
    let mut robot = Robot::new(&input);

    let mut ans = vec![];
    for _ in 0..10_000 {
        let command = if robot.can_progress(&input) {
            Command::F
        } else {
            if rng.gen_bool(0.5) {
                Command::Turnl
            } else {
                Command::Turnr
            }
        };

        robot.do_command(&command);
        ans.push(command.to_char());
    }

    println!("{}", ans.iter().map(|c| c.to_string()).collect::<String>());

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}
