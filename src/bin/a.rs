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
const MAX_COMMAND_NUM: usize = 5_000;

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

struct State {
    robot: Robot,
    gone: Vec<Vec<bool>>,
    rest_grid_num: usize,
    command_cnt: usize,
}
impl State {
    fn new(input: &Input) -> Self {
        let mut gone = vec![vec![false; N]; N];
        input.start.set_matrix(&mut gone, true);
        Self {
            robot: Robot::new(&input),
            gone,
            rest_grid_num: N * N - 1,
            command_cnt: 0,
        }
    }

    fn do_command(&mut self, command: &Command, input: &Input) {
        if self.command_cnt < MAX_COMMAND_NUM {
            match command {
                Command::F => {
                    self.robot.do_command(command, input);
                    self.command_cnt += 1;
                    if !self.robot.pos.access_matrix(&self.gone) {
                        self.rest_grid_num -= 1;
                        self.robot.pos.set_matrix(&mut self.gone, true);
                    }
                }
                Command::Iter(n, coms) => {
                    for _ in 0..*n {
                        for com in coms {
                            self.do_command(com, input)
                        }
                    }
                }
                _ => {
                    self.command_cnt += 1;
                    self.robot.do_command(command, input);
                }
            }
        }
    }

    fn get_not_gone_grids(&self) -> Vec<Coord> {
        let mut res = vec![];
        for y in 0..N {
            for x in 0..N {
                if !self.gone[y][x] {
                    res.push(Coord::from_usize_pair((x, y)));
                }
            }
        }
        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    fn to_num(&self) -> usize {
        match *self {
            Self::Up => 0,
            Self::Left => 1,
            Self::Down => 2,
            Self::Right => 3,
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    TurnR,
    TurnL,
    Turnr,
    Turnl,
    F,
    Iter(usize, Vec<Command>),
}
impl Command {
    fn to_string(&self) -> String {
        match self {
            Self::TurnR => "R".to_string(),
            Self::TurnL => "L".to_string(),
            Self::Turnr => "r".to_string(),
            Self::Turnl => "l".to_string(),
            Self::F => "F".to_string(),
            Self::Iter(n, v) => {
                if *n <= 1 || v.len() == 0 {
                    panic!()
                } else if v.len() == 1 {
                    format!("{}{}", n, v[0].to_string())
                } else {
                    let str = v.iter().map(|com| com.to_string()).collect::<String>();
                    format!("{}({})", n, str)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    fn do_command(&mut self, command: &Command, input: &Input) {
        match command {
            Command::TurnR => self.direction = self.direction.rotate_right(),
            Command::TurnL => self.direction = self.direction.rotate_left(),
            Command::Turnr => {
                if !self.can_progress(&input) {
                    self.direction = self.direction.rotate_right()
                }
            }
            Command::Turnl => {
                if !self.can_progress(&input) {
                    self.direction = self.direction.rotate_left()
                }
            }
            Command::F => {
                if !self.can_progress(&input) {
                    panic!("Command F toward wall.");
                } else {
                    self.pos = self.pos.plus(&self.direction.to_delta())
                }
            }
            Command::Iter(_, _) => unreachable!(),
        }
    }
}

#[fastout]
fn main() {
    let system_time = SystemTime::now();
    let mut _rng = thread_rng();

    input! {
        sy: usize,
        sx: usize,
        h: [Chars; N],
        v: [Chars; N-1],
    }

    let input = Input::new(sy, sx, h, v);
    let mut st = State::new(&input);

    let com = {
        use Command::*;
        Iter(
            150,
            vec![
                Iter(4, vec![TurnL, Turnr, Turnr, F]),
                Iter(3, vec![TurnR, Turnl, Turnl, F]),
            ],
        )
    };
    st.do_command(&com, &input);

    eprintln!("rest_num: {}", st.rest_grid_num);
    eprintln!("{:?}", st.robot.pos);

    // TSPフェーズ
    let mut commands = vec![com];
    // 残グリッドを割り出して
    let not_gone_grids = st.get_not_gone_grids();

    let mut now_robot = st.robot.clone();
    for goal in not_gone_grids {
        // エッジを引いて
        let mut deque = VecDeque::new(); // (座標, 向き, コマンド履歴)
        deque.push_front((now_robot.clone(), vec![]));
        let mut dp = vec![vec![vec![false; 4]; N]; N]; // [y][x][dir] := 行ったかどうか
        dp[now_robot.pos.y as usize][now_robot.pos.x as usize][now_robot.direction.to_num()] = true;
        while !deque.is_empty() {
            use Command::*;

            let (robot, history) = deque.pop_front().unwrap();
            if robot.pos == goal {
                eprintln!("{:?}", now_robot.pos);
                eprintln!("{:?}", history);
                commands.extend(history.into_iter());
                now_robot = robot.clone();
                break;
            } else {
                {
                    let mut robot = robot.clone();
                    let command = Command::TurnL;
                    robot.do_command(&command, &input);
                    if !robot.pos.access_matrix(&dp)[robot.direction.to_num()] {
                        dp[robot.pos.y as usize][robot.pos.x as usize][robot.direction.to_num()] =
                            true;
                        let mut next_history = history.clone();
                        next_history.push(command);
                        deque.push_back((robot, next_history))
                    }
                }
                {
                    let mut robot = robot.clone();
                    let command = Command::TurnR;
                    robot.do_command(&command, &input);
                    if !robot.pos.access_matrix(&dp)[robot.direction.to_num()] {
                        dp[robot.pos.y as usize][robot.pos.x as usize][robot.direction.to_num()] =
                            true;
                        let mut next_history = history.clone();
                        next_history.push(command);
                        deque.push_back((robot, next_history))
                    }
                }

                // 前進
                if robot.can_progress(&input) {
                    let mut robot = robot.clone();
                    let mut next_history = history.clone();
                    robot.do_command(&F, &input);
                    if history.len() == 0 {
                        next_history.push(Command::F);
                        deque.push_back((robot, next_history));
                    } else {
                        match &history[history.len() - 1] {
                            Command::F => {
                                next_history[history.len() - 1] = Command::Iter(2, vec![F]);
                                deque.push_back((robot, next_history));
                            }
                            Command::Iter(n, v) if *v == vec![F] => {
                                next_history[history.len() - 1] = Command::Iter(n + 1, vec![F]);
                                deque.push_front((robot, next_history));
                            }
                            _ => {
                                next_history.push(Command::F);
                                deque.push_back((robot, next_history));
                            }
                        }
                    }
                }
            }
        }
    }

    // TODO: TSP (01BFS)

    println!(
        "{}",
        commands
            .iter()
            .map(|com| com.to_string())
            .collect::<String>()
    );

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}
