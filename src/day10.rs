use std::{collections::HashMap, vec};

#[derive(Debug, PartialEq, Eq)]
enum Side {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct Direction {
    c: char,
    sides: Vec<Side>,
}

type Map = HashMap<String, Direction>;

#[derive(Debug)]
struct Tunnel {
    start: (usize, usize),
    len: usize,
    positions: HashMap<(usize, usize), bool>,
    curr: (usize, usize, Side),
    map: Map,
    size: (usize, usize),
}

impl Tunnel {
    fn move_start(&mut self) {
        // up
        let e = format!("{},{}", self.curr.0 - 1, self.curr.1);
        let x = self.curr.0 - 1;
        let y = self.curr.1;
        let d = self.map.get(&e);

        if let Some(p) = d {
            if p.sides.contains(&Side::Down) {
                self.curr = (self.curr.0 - 1, self.curr.1, Side::Down);
                return;
            }
        }

        // right
        let e = format!("{},{}", self.curr.0 , self.curr.1 + 1);
        let d = self.map.get(&e);

        if let Some(p) = d {
            if p.sides.contains(&Side::Left) {
                self.curr = (self.curr.0, self.curr.1 + 1, Side::Left);
                return;
            }
        }

        // down
        let e = format!("{},{}", self.curr.0 + 1, self.curr.1);
        let d = self.map.get(&e);

        if let Some(p) = d {
            if p.sides.contains(&Side::Up) {
                self.curr = (self.curr.0 + 1, self.curr.1, Side::Up);
                return;
            }
        }

        // left
        let e = format!("{},{}", self.curr.0, self.curr.1 - 1);
        let d = self.map.get(&e);

        if let Some(p) = d {
            if p.sides.contains(&Side::Right) {
                self.curr = (self.curr.0, self.curr.1 - 1, Side::Right);
                return;
            }
        }

        panic!("Cannot find start element movement");
    }

    fn set_position(&mut self, position: (usize, usize)) {
        let key = format!("{},{}", position.0, position.1);
        let entry = self.map.get(&key).unwrap();

        let v = entry.sides.contains(&Side::Right);
        self.positions.insert(position, v);
    }

    fn next(&mut self) {
        if self.curr.0 == self.start.0 && self.curr.1 == self.start.1 {
            self.move_start();

            return;
        }

        let p = self.map.get(&format!("{},{}", self.curr.0, self.curr.1)).unwrap();
        let side = p.sides.iter().find(|s| **s != self.curr.2).unwrap();
        
        match side {
            Side::Up => {
                self.curr = (self.curr.0 - 1, self.curr.1, Side::Down);
            }
            Side::Right => {
                self.curr = (self.curr.0, self.curr.1 + 1, Side::Left);
            }
            Side::Down => {
                self.curr = (self.curr.0 + 1, self.curr.1, Side::Up);
            }
            Side::Left => {
                self.curr = (self.curr.0, self.curr.1 - 1, Side::Right);
            }
        }
    }

    fn traverse(&mut self) -> usize {
        self.set_position(self.start);
        self.next();
        self.set_position((self.curr.0, self.curr.1));
        let mut count = 1;

        println!("Start: {:?} ,Curr: {:?}", self.start, self.curr);
        println!("Cond : {}, {}", self.curr.0 == self.start.0, self.curr.1 != self.start.1);

        while !(self.curr.0 == self.start.0 && self.curr.1 == self.start.1) {
            println!("traversing: {:?}", self.curr);

            self.next();
            self.set_position((self.curr.0, self.curr.1));
            count += 1;
        }

        return count;
    }

    fn find_enclosed_area(&self) -> usize {
        let mut out_c = 0;
        let mut in_c = 0;
        for i in 0..self.size.0 {
            let mut flag = false;
            let mut prev = None;
            for j in 0..self.size.1 {
                let key = format!("{},{}", i, j);
                let entry = self.map.get(&key);
                let is_non_loop = self.positions.get(&(i,j)).is_none();

                if let Some(p) = entry {
                    println!("Visisting for area,{key} {:?}", entry);
                    if flag && is_non_loop {
                        println!("Adding in_c");
                        in_c += 1;
                    } else {
                        println!("Adding out_c");
                        out_c += 1;
                    }

                    if !is_non_loop {
                        flag = !flag;

                        if p.sides.contains(&Side::Right) {
                            prev = p.sides.iter().find(|s| s != &&Side::Right);
                        }
                    }

                }
            }
        }

        println!("Values: Out: {}, IN: {}", out_c, in_c);
        return 0;
    }
}

fn get_sides(c: char) -> Vec<Side> {
    let v = match  c {
        '|' => vec![Side::Up, Side::Down],
        '-' => vec![Side::Left, Side::Right],
        'L' => vec![Side::Up, Side::Right],
        'F' => vec![Side::Right, Side::Down],
        'J' => vec![Side::Up, Side::Left],
        '7' => vec![Side::Down, Side::Left],
        _ => Vec::new(),
    };

    return v;
}

fn create_map(ip: &str) -> Tunnel {
    let mut map = Map::new();
    let mut s = (0,0);
    for (i, line) in ip.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map.insert(format!("{},{}",i,j), Direction { c, sides: get_sides(c) });
            if c == 'S' {
                s = (i,j);
            }
        }
    }

    let mut lines = ip.lines();
    let y_len = lines.next().unwrap().chars().count();
    let x_len = lines.count() + 1;

    return Tunnel {
        start: s,
        len: 0,
        positions: HashMap::new(),
        map,
        curr: (s.0, s.1, Side::Down), // random side
        size: (x_len, y_len),
    };
}

pub fn solve() {
    let ip = INPUTT;
    let mut t = create_map(ip);

    println!("MAP: {:?}", t);

    let c = t.traverse();

    println!("Final: {}", c/2);

    t.find_enclosed_area();
}

const INPUTT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

const INPUTT2: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

const INPUT: &str = "FJ77.F7F.FF.F..7-J.7F|7-7-7-7FJJ-7J-LL-7FL.F-7.F-F77F7F7-77-F--7-FF|-F-L7-|.F7|7.F-77..-FF|--F.J7..|7FFL-JJ7-F-..J77.|FFJ77L---J7F7.J-F77JJ.
F.J-F.7--J-JJ--77J7LF77...|---|JFJ.|.|--JF-J-7FJ.FL-JJJL-JLLJF|.LF-.|L7FF.F-LJ77.--J7FF--|.-JL-77-7.L-J.|JF|FF--7LJ||J-.FLL7-FJF-7|-L-JL7J.F
LF-FJ-77F-7L7|LLJJ..JL77-LF-JJ|J.FFLJ..LJ-|J-LJJ.|7|L|-L|JJ|-7L7LFLF-7LJ.7.|F||F-|F||LJF|L-.FLLFFJ|7FL-JL----7FL-7|L|..FJ.LJ-FJLLL7-L7|..FFJ
.--J.|F7JLJFJJ7L|.FJ.F|J.LLJ.FJ.F|.F|F7-|7L|.F7J.FF-7-7|.|--7L.L7||J.L-L7L77L|7LF|-JL-F.|F.F77FL7F77J|--|.L7||.L7JF-J-7|.F|FF77|.FL7J.LF.-LJ
||LFFJ|JF---J||.-.FLFLJ7FFL.-7|LFJLFJF7LFJLJ7L--JL77|JLJ7||FJL7JF|J.F7JF77L7.LJL|J|.|J.F7FF7-F77L7JF7|.F-77J|JFFJ-77FLLFF-7JLJ--.F.L7-7L-L|.
L-7-.LL-J|F--FJ-7LF.LFLL777.LLJJL7-|.||FJ.F-7L|-7|.-7|FJ|LF|F|JL-777.|7F|L|LJ7F7J-FF7.FF7FJL7||F77FLL-.||.L--7J.-7|FJ77F-LJ.7--7F7.|LLLL7L|J
|FJ-J77FFLJF-|J||JJ.FJJ-LL-|-LJ7|.F|-JF||F-7|FFJF7LJL7LF--F7-|.|.L---F7LJJLL||7JF-7||F7|||F-J|LJL-7F|7LFF7LF||.7L|77--L7F7J-F--J7-7|-.FF.--J
F|J|.L-7F7.--7-7|.77|L77JLF-F7JF7F7LJJ|L-7|-|F|JL-..F7-|J|L--F-FF7J|-|J-J.7.LLJ-L7LJ|||||||F7|F---J77F.LF7F77J-FF-J7L|L|JJ|.J-|FJF|.LJ7LJ-LF
.J7-F-LFJ777.L|FL7LF-F7J7FF7||F||||7|F||LJ7---7JFJ-FFJLF-|7FF|.FJL-7L|7||..F|F77-L-7LJLJ||||LJL77F7J|JFFJLJL7LF7|.L|.|LJLF7L-77|7|J.F7|J.F-7
F.F..|LLJ7LF|77|L-JF7||-F7|||L7|LJL--7L77LL|-JLF-7-LJLFJFFJ-77.L7F-J7.F77L77L7LJ|F-JF-7FJ||L7F-JFJL7FF7L-7F7L-JL-7.|7LF|-L7||L7L|JJ.LLL7FF||
L-JL7|..|L--777J-JFJ||L-J||LJFJ|F7F--J-F7LFF-7F77F.|F7..7.L|F-F-JL-7F7FF|7L7FL-JFL--JF|L-J|FJL7FJF-JFJL--J|L--7F-J-|||LJ7.L--7J|.J|F7.LL-FL7
|.FJ7.F|J-L.LLJ7LFL7LJF7FJL-7L7||LJF7.L||-LJ|F|JFF7-F77-|--F7-L---7||L-7||.7-JL7-F--7FJF--J|F-JL7L-7L----7|F7-LJF7-F--7.F7L|.|.J-LL-L7.|L|J7
|F|F7.FJ|F-L-J||.|LL7FJLJF-7L7LJ|F-JL7.|L7J.F7-7FJ|FJL7.F7.||F-7F7|||F-JF7F7JF.LFL-7|L7L-7FJ|F7FJF-JF7F7FJLJL7-FJL-7LJJ.777FFJ-J7FLL-J-J7J.-
L-LLL-L-JJ-LL7|.FF--J|F7|L7L7|F-JL7F7L7L7|F7||-FJFJ|F-J-F77||L7||LJLJ|F7|LJL7F7F7F7||FJF-JL7|||L7|F-J|||L7F--JFJF-7|..F7.F7|LJ.||J7L..|JL7FL
LJ7J.|.L.L7--F--|L--7LJL7|L7|||F7L||L-JJ|LJ|||FJFJFJ|F7.|L7||FJ||F--7LJ|L7F-J|LJ||LJ|L7L7F7|LJL-J|L-7|||FJ|F--JFJLLJ7F|7F-J-|-F7L-|LJ7|.L-LJ
LL|-L|JJL-FJFLJ-JJ|JL--7L7.||||||FJL77F-JF-J||L7L-JFJ|L7L7|||L7LJL-7|F7|FJL7LL-7|L-7L7|FJ|||F----JJFJ|||L7LJF7FJF7F7-7L7--77|-JL7F7J|FF-|.|J
7-|7.|-FJ7.|JF..LF7JLF7L7L7|||||LJF7L7|F7L7FJL7|F--JFJFJFJ||L7L-7F-JLJLJL7FJF-7||F7L7LJ|FJ||L---7F7L7LJL-JF7|LJL||||L|J.LF|-7J7F|-J-|-L-L7JF
J|L--7-L---FF|7-F||.FJL7L7||LJ|L-7|L-JLJ|FJL-7||L--7L7L7L7||FJF-JL--7F7J7||-L7LJLJL7L-7|L7LJF---J|L7L--7F-JLJF7||||L7J.|JFL-J||JJ..LF7JFJ|-7
.FFL-J7.F-FFF7F7FJL-JF7L-JLJF-JF7|L7F7FFJL--7|LJF--J||FJFJ||L7L7F---J||FFJL7-L--7F-JF7|L-JF-JF--7L7|7F-JL--7L|L-JLJFJ-|J.-|7F7||.F77L.-L-7.F
J7LL-J--7|LFJLJ|L----JL-7F-7L--JLJFJ||FJF---JL-7|F7-FJL7L7||FJFJ|JF7-|L7L7FJF---JL7L||L--7L77|F-JFJL7L7F---JFJF7F-7L7||.L-|-L|-77FJ-FJ.L||-|
|L77FJ|7L--L--7|F7F--7F-J|LL-7F--7|FJLJFJF7F7F7|||L7L-7|FJLJ|.L7L7|L7L7|FJL7L----7L7||F-7L7L7||F7L-7L7|L-7F7|FJLJ.L7L7-7J|L|.LL77|JJ||L.-7-|
L.LJ-FF-7|.|-L|LJ|L7FJL-7|F7FJL7FJ|L7F-J7|||||||||FJF-J||F--JF7|FJ|FJFJ||F-JF7F7FJFJ||L7|-|FJ|LJ|F7L7LJF-J||||F-7F7|FJ7..7|.-7F77.--JJ.7JLJL
FF-J|J|J7FFJ-FJF7L-J|F77LJ||L--JL7L7||JF7|||LJ||LJL7L-7LJL-7-||||FJL7L7LJL7FJ|||L7|-||FJ|FJ|||F-J||FL-7L7FJ||||FJ||LJF77-LL-F|-JJ-L7..|L-J||
|.|FL-J-FLJJ.L-JL--7|||F7FJ|F7|F7L7LJ|FJ|||L7FJL7F7L-7|F---JFJLJ|L-7|7L7F-JL7LJ|FJL7||L7||FJFJ|F7|L7F7|FJL7LJLJL-J|F-JL7JLJ7F-J.||.7.77FJ7FJ
.L-LJF|FFJ.|-|FF7F-JLJ|||L7LJL-J|7L-7|L7LJ|-|L-7LJL-7|||F-77L--7|F-J|F-J|F7FJF7|L-7|||FJ||L-JFJ|||FJ||||F7|F---7F7|L7F-J7FLF-JLFF-777.-JFJL7
F|7JLFLLL.|-FF-J|L---7||L7L-7F--JF--JL-JF7|FJF-JF7F7|LJLJFJF7F7|||F7|L-7||||FJLJF7|LJ|L7|L--7|7|LJL7|LJLJ||L--7LJLJFJL--7JL|7.FFJJLF7FL-J.LJ
F|7-F-J-|.|7.L-7L7F--J||FJF7|L7F7L---7F-JLJL7L7FJ|||L7F--JF|||LJ|||LJF-J||LJL7F7|LJF-JFJ|F-7|L-JF7FJL7F--J|F--JF-7FJF---JF7.77FLL.7||LJ7F7-J
|.|7LL7FF-|LLF-JFJL--7LJL-JLJFJ||F-7-||F--7L|FJ|FJ||FJ|F7F7||L-7||L7FJF-J|F--J||L-7L-7L7|L7||F--JLJ|FJ|FF7|L7F7L7||FJ.F7F7|F|-L|-JLF-JF|J|L|
7-|F||L|L-77-L-7|F7F7L------7|FJ||FJFJ|L-7|FJ|FJ|.||L7LJLJ||L-7|LJFJL7L77||F7.|L-7|F-J7|L-JLJL-7F7F7L7L7|LJFJ|L7|||L-7||||F7-JFFF--JJJ||7F.-
|L7-|7-|7J|7.F-JLJ|||F7F7-F7||L7|||FJFJF7|LJFJL7|FJ|FJF---JL-7|L-7L-7L7|FJ||L7L-7LJL--7L---7F--J|||L7|FJ|F-JJ|FJ|||F-J||||||7.FFJFJ|FF--FJ7|
7-J.|F-L7.FF7|F--7LJLJ||L7|LJL-JLJ|L7||||L-7|F7||L7|L7L---7F7|L-7|F-JFJ|L7||FJF7L-7F--JF---JL-7L||L7|||FJL--7|L-JLJ|F-JLJLJL--7J-F7JF|J|L-F7
L-|FJ--F|7FJ|LJF7L---7|L7||F7F---7|||L7||F-JLJ|||FJL7|F---J|||F7LJL7FJFJFJ||L7|L7F|L-77L---7F-JFJL7|||||F---J|F----JL7F-------JFL|L7-LJF7..|
JLL-JJ77FLL7L--JL7F--JL7||LJLJF7FJL7L7||||F---J||L-7LJL7-F7||LJL--7LJFJ7L7|L7||FJFJF7L-7F7FJL-7|F-J|||||L7F7FJL-7F7JFJL-----777FFJFJFF7.77L.
FF-|F|.FJ-LL----7|L---7LJ|F-7FJ|L-7L-J||LJ|F7F7|L7FL7F-JFJ||L-7F-7|F7L7F7|L7||||FJFJL--J|LJF--J||F7||||L7LJLJF--J|L7L7F--7F-J-F7L7|-FL--LF-L
|J7|7LLL..LF--7J||F--7L-7|L7|L7|F7L7F-JL-7|||||L7|F7||F7L7||F-J|FJLJ|FJ||L7|||||L7|F7.F7L-7|F7-||||||||7|F---JF-7|FJFJ|F7LJL|||L7|L-7L|J.|F|
7L-JLJ.LFFFJF7L-J||F-JF7||FJL7|LJL-JL7F7FJLJ||L7|||||LJ|FJ|||F7||F7FJ|FJL7||||||FJ||L7||F7|||L7|||||LJ|FJL----JFJ||FJFJ||F7FF7L7LJF-J7|.L7|J
JFL7L.FF7FL-JL7F7LJL7FJLJLJF7LJF7F--7|||L--7|L7LJ||||F-J|LLJ||LJ|||L7|L-7||||||||FJ|FJ|||||LJFJ|||||F7LJ|F-----JFJLJFJ||||L7||FJF-J-L7JF|7|J
.JJJ.FFJ|F7F7LLJL7F7LJF7F7FJ|F-JLJF-J|||F7FJL7|F-J|||L7FJF--J|F-J|L7||F7|||LJ||LJ|FJ|FJ|||L-7L7||||LJ|F--JF---7||F--JF7|||FJ||L7L--7-JLLJJLJ
|L7.|FL7LJLJ|F7F7||L--JLJ|L7LJF-77L-7||||||F7||L-7|||F|||L-7FJ|F7|L||||LJ||F-JL-7||FJ|FJ|L7FJFJ|||L7FJL7F-JF-7L7|L7F7||||||FJL-JF--JF7.FL7L|
|FJ-L|FL---7LJ|||LJF----7L-JF7L7L---JLJ|||||LJL-7|||L7LJF--J|FJ|LJFJ||L-7|||F-7FJ||L7|L7|FJ|-L-J||FJL7FLJF7L7L-JL7LJLJLJLJLJF7F7L----7.|LJ--
F7J..F---77L-7|||F7L---7L7F7||LL-7F7F7|LJ|||F7F7|LJ|FJF-JF-7|L7L-7|FJ|F7||||L7||FJ|FJL7||L7L---7LJ|F7|F--JL7L----JF--7F7F7F-JLJ|F-7F-J---JFJ
FJF-FL--7|F7FJLJLJL--7F|FJ|LJL--7LJLJL-77LJ|||||L7FJ|FJF7|FJ|.L7FJ||FJ||||||FJLJL-JL7FJ|L7L7F-7L-7||||L7F-7|F7F7F7L7FJ|||LJF--7LJJLJF7777L|.
LF|7LFF-JLJLJF--7F--7|FJL7L----7L------JF-7||LJ|FJ|FJL7||||FJF-JL7|||FJLJ||||JF-----JL7|FJFJL7|F7|LJLJ7LJFJLJLJLJL-JL-J|L7||F-JF--7FJL7FF7--
.LLF--L7F---7|F-J|F-JLJF7L7.F7FJF7F7F7-FJFJ|L7FJ|FJ|F-J||||L7|F-7||||L-7FJ||L7L--7F-7FJ|L7L7FJ||LJJF7F-7FJF7F7F77F----7L7L7||F7|F-J|F-JFJ|7.
.FJF7J.LJF--J||F-J|F7F7|L7L-JLJFJ||LJL-JFJ|L7|L7|L-JL-7|LJ|FJ|L7||||L7FJL-JL7|F--J|FJ|FJFJ|||-|L7F-JLJFJL-J||LJL-JF---JFJFJ|LJLJL--JL--JFJ-7
-F-JL7FLFJF-7|||F7LJLJ||L|F-7F7|.LJF-7F7|F-7LJFLJF----J|F-JL7|FJ||LJFJL---7LLJL7F-JL7|L7|F-J|FJFJL---7L---7LJF----JF7F7L-J7|F--7F7F7F--7|||7
LL-7FJF7|FJFJ|LJ||F7F7LJFJ|LLJLJF-7L7|||LJFJLF7F7L7F-7FJ|F-7|||FJL77L7F-7FJF--7LJF--J|FJ|L7FJL7|F7F--JF7F7L-7L-7F-7|||L-7F7|L-7LJLJ|L7FJL-7|
|F-JL-JLJL7L-J7FJLJLJL--JFJF---7L7L-JLJL--JF7|LJ|FLJF||J|L7||||L7FJF7LJF|L7|F7L--JF7FJL7|7|L77|||LJF7FJLJ|F7L--J|FJ|||F-J|LJF7L7F-7L7|L---JJ
-|F7F7F-7FJF---JF7F7F7F7FJ7L--7L-JF---7F7F-JLJF7L---7||FJFJ|||L7|L-JL--7L7|LJ|F7F-J||F-JL7|FJFJ|L--JLJ-F7LJL--7L|L7||||F7|F-JL-J|FJ.|L7L|7J|
LLJ||LJFJ|.L--7FJ||LJLJLJF7FF7L--7|F--J|||F---JL7F-7|||L7L7||L7|L7F-7F7L7LJ.FJ||||FJ|L-7FJLJFJFJF------JL----7L7L7LJ|||||||F7-F7|L-7|FJL77.7
|.FLJJ-|FJF7J.||LLJFF7F--JL-J|F7|LJL-7FJLJL---77LJ|||LJFJFJ|L7LJFJL7|||FJF--JFJ||FJFJF-J|F7FJFJ|L7F7F-------7L7L-JF-J|LJLJLJL-J||F-JLJ7..|-J
-J-JLJFLJFJ|7F||F7F-J|L-----7|||F7JF-J|F------JF7F7LJF-JFJ||FJF-JF-JLJ|L7|F7FJFJ||FJFJF7LJ|L7L-7.LJLJF----7FJFJF7FJF-JF---7F7F7||L-7F|7|--J.
||F7J-L7LL7|F7LJ||L-7L------JLJLJL-JF7||JF7F7F7|LJL-7|F7L-7||FJF7L--7FJFJLJ|L7L7||L7L7||F7L7L--JF7F7-L---7LJ7L7|||FJF7L7F7LJLJ|LJF7L7L-|7..7
L-||J7-FF7||||F7|L7JL--------7F--7F-JLJL-JLJLJLJF-7FJLJL7FJLJ|FJ|F-7|L7L-7F|FJFJ|L7|FLJLJ|FJF---JLJL--7F7L-7F7LJ|LJFJL7LJL---7L7FJ|FJ7.FJ-|.
LL-L.FF7|LJLJ||||FJF7F7F----7|L7FJ|F----7F7F7F--JJLJLF--JL--7||FJ||LJ7L7FJFJ|JL7|-||F----JL7L-----7F-7LJL--J||F7L7FJF7L7F----JJLJ7LJF7-7-LJJ
|FL-FFJLJF--7LJ|||FJLJLJF--7LJFJL-JL---7LJ|||L-7-F7F7L-7F7F-J||L7L-7F--JL7L7L-7LJFJ|L---7F7L7F----J|FJF7F---JLJL7|L-JL7|L7F7F7F7F7F-J|.|..LJ
L7|-FL-7FJF-JF7LJLJF7F7FJ|FJF7L7F------JF7LJ|F7L-JLJL7LLJ|L7-LJ|L7FJL7F-7|F|F7L7|L7|F---J|L-JL--7F-JL-JLJF------JL7F7FJL7LJLJLJLJLJF-J777-LF
.LJ.|F7|||L-7|L7F7FJ||||F-JFJL7|L7F7F7F-JL-7LJ|F----7L--7|FJJL|7FJL-7||-|L7LJ|FJ7.LJL7F-7|F-7F7.LJF7F--77L------77LJ|L7FJF-7F-7F--7L-7.-|7J|
F7LF-JLJL7F-J|FJ|LJFJ|LJL-7|7JLJ|LJLJLJF---JF7LJF7F7L---J||J-F-7|F7FJ||JL-J.LLJJ|FLF-JL7LJL7|||F-7|||F7L----7F--JF-7L-J|FJ|||FJ|F7L--JFFLL-F
|7||F7F--JL--JL-JF7L-JFF--JL7F---7F--7FJF7F7|L--JLJL--7F7LJF7|FJLJ|L7|L7J.L||L7F|7.L-7FJF--J|||L7|||LJL----7|L7F-JFJF7FJL-7||L7LJL--7-FJFJ7|
LJ7LJLJ|JF7F7F7F7||F7F7L----JL--7|L-7||FJLJLJF---7F-7FJ||.FJLJ|L|7|FJL7||-J|F-|-F7JFLLJJL7F7LJL-JLJL-------JL7||F7L-J|L7F-JLJJ|F----J7|F|LF|
.|LJJ.LF-J||LJLJLJ|||||F--7-F7F7||FFJ|LJF7F-7L--7|L7LJ||L-JF--J.F-JL-7LJ--FF|.|LJJJF--7F7||L7F------------7F-JLJ||F7FJJ|L----7||F----7J-L7F7
.F7|7-LL-7|L---7F7|||||L-7|FJLJLJL7L7|F7||L7L-7FJL7L---JF-7L--7FJF7F7|JJ.FLJLJ|7|-LL-7LJLJ|JLJF-----------JL----JLJLJ|FJF---7||LJF7F-J-LLL--
F-JL7.LF-JL----J||LJLJL-7||L-7F--7L-J||LJL7L-7|L-7|F-7F-J|L7F-JL7|||LJJ.F-7JLLJF-F-|FL----JF7FJF7F7F7F7F-----------7F7L7L--7LJ|F7|LJ.|FLFL||
J|--F7.L-------7|L7F-7F7LJL-7LJF7L-7FJL7F7L--JL7FJ|L7|L-7F-JL7J-LJLJJ|..|L-77J|LLJ-LFF7F77FJLJFJLJ||LJLJF---------7LJL7|F--JF7LJ|L--7J--|LL7
L|7L|-FF-------J|FJ|FJ|L7F-7L--JL7FJL-7LJL---7JLJ7L7|L7FJL7F-JJ.FLJ.LL7FL-LL7-F-|7.LFJLJL-JF--JF7F||F--7L--------7L-7FJ|L---JL7|L--7|.77J.|J
.L-JL|.L-7F7F7F7|L-J|FJLLJFL---7FJL--7L---7F-JF---7|L7LJ77||7LFJ-LJ7FL|7JF|J|-L777FFJF-7F7FJF--JL7LJL-7L---------JF7LJ|L-7F--7L---7|L7L--FL.
--J7-L7LFJ|LJ||LJF--J||F7F7F--7|L----JF--7LJF-JF--JL-JF7F7LJ7.|||-FF-FJJF.LFJLLLL|-L7||LJLJLL---7L7LF7L7F--------7|L----7LJ-FJF7F7|L-J.|7LJ7
|7.L7JL7L-JF-J|F7L7F7|FJLJLJF7LJF7F---JF-JF7L-7L7F7|F7||||-F7-JF7F7.F|7FLJF7|7FL|J|FJ|F-7F7F7|F7L7L7|L7LJF------7LJF7F-7|F-7L-JLJLJ|FL-|F.FF
77F|-7LF77LL--J||FJ|LJL---7FJL--J||F---JF7||F-JFJ||FJ||LJL-J|7L--J|J-J|-.LF77FF7LFFL-JL7LJLJL7|L-JFJ|FJF7L-----7|F-JLJFJ||FJF--7F--777.|.FLJ
L--7J|||L-7F---J|L7|F-----J|F----J||F7F7|||LJF7L-J|L7LJF---7L-7J|.|J.|F7|.|J-|LJF-F---7L7F7F7|L--7|FJL-JL------JLJF7F7L-J|L-JF7LJF-J7-7|JJ.F
7|.LFF-JF-JL--7FJ|LJL7F-7F7|L7F---JLJLJLJLJF7|L7F7L-JF7L--7|F-JJJLF7.--L7.||-7-JFLL--7|J||LJ|L---JLJF------7JF7F--JLJ|F-7|F--JL--JF-7-JJ7L7J
F-JFFJF7|F7F7-||F7.F-J|7LJLJ.LJF--7F7F---7FJLJ.LJ|F7FJL7F-JLJFJJLF|77||L|FL-7JL-JL|.FJL7LJF-JF-7F7F7|F7|F-7L-JLJF----JL7|||F---7F7|FJJJLL.||
|F77L-J|||LJL-JLJL7L--JF7F-----JF7LJLJ7F7|L----7LLJ|L7FJL-7F77F7--J|.|-F-F-JJ7.|L|F7L-7L--JF-JLLJ|||||L7L7L7F7F7L--7F7FJLJLJF7FJ|LJ|JL|.L-L7
|LF-LF-JLJF-7F7F7FJF7F7||L----7FJL-----J|L7F7F-JF7FL-JL-7FJ||-7L|J.F|LFF.-|J.L|J|FJL--JF--7|F7F-7LJLJL7L-J7LJ|||F-7LJLJF7F7FJLJ||F-JL7.F.FFF
LFJJ|L--7FJ7LJ|||L-JLJ|||F7F7FJ|F---7F-7L7LJLJF-JL---7F7LJF||F77F-7FLFF-JL|.-7L-LL---7FJJFJ|||L7|-F7F7L7F--7||||L7|F---JLJLJF7F-JL--7J||FFJ|
LL||FJ.F||F--7LJL----7|||||||L-JL--7LJLL-JF---JF7F7F7LJL--7||7.FJ7L7JJ||F7JFJL7LLF--7LJF7L-J||FJL-JLJL7LJF7L7LJL-JLJF7F-----J|L7F---JF77FL|7
FJ.-FJ.-||L-7|F7F7F--JLJ||||||F7F7-L---7F7|F---JLJLJL-----J||F77J7|.J7-LF.--JLL7|L-7L-7|L7LFJLJF--7F-7L--JL-JLF----7||L7F---7L-JL----J|--7JJ
F77-L--LLJF-JLJLJLJF---7LJLJL-JLJ|F----J|LJL--7F--------7F7||||F7F|77FJ.J7LL7J-FFF7L7FJL7L-JF-7|F-J||L7F-7F7F7|F---J|L-JL--7L7F-------J.L7-7
JL-7L|FLLLL-7F-7F--JF7JL--7F----7|L-7F-7|F----J|F-------J|||||LJL7--FJ.FJ77|.FF--JL-J|F7L7F7|FJ||F7|F-JL7LJLJ|||F7F7|F7F7F7L7|L----7-|.7.|-7
||7J.LL7|L|FJ|FJ|F--JL---7LJF7F-JL7FJ|FJ|L----7||F7FF7F77||||L7F-JJ-7|-L|L7JFFL7F-7F7LJL7LJLJL-JLJLJ|F--JF---J|LJLJLJ|LJLJ|FJL7F7F-JL|--.-7J
LLL-|7-7JLLL7|L7|L7F7F7F7L-7||L--7|L7||FJF-7F-J|LJL7|LJL-JLJL7|L7|L-F|.F7||F|7LLJFLJL-7FJ7F7F-------J|F--JF7F7|F----7L---7|L7LLJLJJJ-77J7|L7
||L7||7.|.JLLJJ|L7LJ||||L-7|||F--JL-J|LJFJFJL--JF-7LJF7F7F-7FJ|FJ7-FLJF7.FFLF7.||F7F7FJL7FJ|L----7F7FJL---JLJLJL---7L----JL-JF7F7|F7.F777|.7
JFFL7JL7|.LF---JFJ7FJ||L7FJLJLJF7F---JF-J7L7F-7FJLL7FJ|||L7||FJL--7.LF|L7F|LF77F7||||L-7|L7|F-7F7LJLJF-----7F7F7F--JF7|F7F7F7|LJL-JL-JL7-J.|
|F7||7JF7FLL----JF7L-JL-JL--7F-J||JF--JF7F7|L7|L-7LLJFJ||FJLJL7F--JF--JFJ7-F||7|LJLJL-7||FJ|L7||||F7JL----7|||||L---JL-JLJLJLJF--------JJL-J
FJL|L--L7.LF-----JL-7F7F7LF-JL-7||FJF--JLJ|L-JL--JJF7L-J|L-7F-JL--7L7F-JF77FJL7L---7F7|||L7|FJ|||FJ|F7F-7FJLJLJ|F----7F----7F7|LF7F-7JF|.F7J
LJ-77L||F-7L-------7|||||FJF-7FJLJL-JF----JF--7LF7FJ|F-7L-7|L--7F-JFJ|F7|L7|F-JF-7JLJ|LJL-JLJFJ||L7|||L7LJF7F7FJL---7||F7F7|||L-JLJFJF77|LJ-
L-.--7-|JFJ..F7F7F7|LJLJLJFJFJL7F----JF7F--JF-JFJ||FJ|FJF7LJF7FJL7FL7|||L7|||FFJFJF--JF---7F7L-JL-JLJL-JF-JLJ||F7F--JLJ|LJ|LJL7F--7|JJLF-7|.
|LL-JJ7J7FFFFJLJLJ|L7F7F-7|FJF-JL----7||L7F7L7JL7LJL-JL-JL7FJ|L7FJF7|LJL-J|||FJFJ|L-7FJF7FJ||F-7F7F7F7F7|F---JLJ|L-----JF7L-7FJ|F7||J.7--7|7
F7JFL-L77LF7L-7F-7L7||||7LJL-JF7LF7F7LJL7||L-JF7|F7F------JL7|FJL7||L----7LJLJFJ77F7LJL||L7|LJ7||LJLJLJLJL----7FL------7||F7|L7LJ|LJJ.7L7|LJ
LF7|J7J.|FL||FJL7L7LJ|LJF--7.FJL-JLJL7F7LJL-7FJLJ|LJF--7F7-FJ|L7FJ||F---7|F---JF7FJ|-F7||JLJF-7LJJF-----7F----JF------7|||||L7|F7|-J.F-7L|.7
F|-L.L7FL7J|FJF7L7L--JF7L-7L7L-----7FLJL7F-7LJF7FJF7|F-J|L7L7|FJL7||L7F-J|L7F--J|L7L7||||F-7L7|-F7L----7|L-----JF-----JLJLJ|FJ||||||.|-7F---
FL.|F|L.F-F-JFJL7|F---J|F7L7L-7F7F7L---7LJFJF-J|L-J||L-7L7|L||L-7LJ|FJ|F7L7LJF7FJ-L7LJLJ|L7|FJL-J|F7-F-JL--7F7F7L-----7F7F7||FJ|LJ--F.FLFJ.|
|L-J7L7FL.|F7|LLLJL-7F7||L-JF7LJLJL----JF7L7|F7L7F-JL7FJFJL7||F7|F-J|FJ||FJF-JLJ|LFL7F-7L7||L7F-7LJL7L---7FLJLJL------J||||LJL-JF|JL-7JJF-J7
|F|.7F|J.-LJLJ|F7|F7LJ|LJF7FJL7F7F-7F7F-JL7|LJL7||F--JL7L7FJ|LJLJL-7||FJ||FJF7FF7-F7LJFJFJ||FJL7L7F7|.F77L---77F---7F7FJLJL7F-7J-|7J.J.LL|.|
FF|-L-||.J-LF--J|FJL--JF7||L7-LJLJFJ|LJF7FJL---J||L---7L-J|7L-7F---J||L7||L7|L7||FJ|F7L7|FJ|L7FJ7LJ|L-JL----7L-JF--J|LJF7F7LJFJJ.|J.|.F77F-7
-7|L|FL7.LF.L--7LJF--7FJLJL-JF7|F7L-JF-J|L--7FF7LJ-F7.L--7L--7||F-7|||FJ|L7||FJ|||FJ||FJ|L7|JLJLF7FJF7F7F--7L---JF7.|F-JLJL--JF7F777JFJ7JLJJ
.||LFF7F7.-..|7|F7|F-JL7F----JL-JL--7|F7L---JFJL7F7||F--7|F7FJ||L7L7||L7|FJ||L7|||L7|LJFJFJL7F7FJ|L7|LJLJF-J-F--7|L-JL----7F--JL-7LF7||J.F77
FJL-F7-JF.FFFFFJ||||F--JL-------7F-7LJ||LF77FJF7LJ|||L-7|LJ|L7||JL7LJ|F|||FJL7|||L7|L7FJ-L7FJ||L7L7||F---J7F-JF-J|F---7F7FJ|F----J-FF|7-L-J7
|7|L-.LFL-J.LFJFJLJLJF----------J|FJFFJ|FJL7|FJL--J|L7FJ|F-JFJ||F7|F-JFJ||L7FJ|||FJL-JL7F7|L7|L-JFJLJL-----JF-JF-JL7F7LJLJ-||F77|FF|J.F7L|JJ
-FJ-J7F7FJF7LL-JF--7FJF7F7F------JL-7L7|L7FJ||||F77L7|L7||F7L-J||LJL-7L7||FJL7|||L--7F7LJ|L7|L7F7|F7F7F-7F--J7FJF-7LJ|F7F7F|LJL-7J7J|7LL.|L7
LL7L|JLJ7.FF-F7JL-7|L7|LJLJFF--7JF7FJFJL-J|FJL7FJL7FJ|FJ|LJL7F-JL-7F7|FJ|||F7|LJL7F7LJL7FJFJL7||LJ|||||FJL----JFJFJF7LJLJL-JF--7||L---7|.-.-
.-7.|.|FL77F7||FF7||-||F----JF7L-JLJJL7F--JL-7||F-JL7||FJF7FJL7FF7||LJ|FJ||||L-7FJ||F7FJL7|F7LJL-7|||||L7F-7F-7|JL-JL7F-7F-7L-7||JF|.L-77LF7
.L|-LJ.F.FFJLJL-J||L7LJL--7F-JL7F---7FJ|F-7F-J||L7F7|LJ|FJ|L-7L7|||L7FJ|FJ||L-7|L7|||||F-JLJL7F--J||||L7||LLJ||L---7FJL7|L7L--JLJ|LJ-F.|L..J
7|JLF-FJ7LL7F7F-7|L7L7F7F-J|F7-LJF--J|FJL7|L-7||FJ||L-7|L7|F7|FJ||L7|L7|L7||F-J|FJ||||||F77F7||7F7||||7||L---7|F--7|L7FJL-J7F7F--7J|..LJL7-7
LJ-7J|L||J-LJLJFJ||L7LJ|L--J||F--JF7FJ|F7||F-J|||FJL7FJL7|||LJ|FJ|FJL7||FJ||L-7|L7|||LJ||L7|||L7|LJ||L7||F--7|||F7LJJ|L--7F-JLJF7|.-7FLJF.|-
|FF.L77-JL..LF7L7L7|L-7|LF-7||L--7||L7||LJ|L7FJ|||F-JL7F||||F-JL7|L-7||||FJ|.FJ|LLJ|L7FJL7|||L7|L-7||FJ||L-7LJLJ|L-7FJF7FJ|F---J||7|FFL7J7..
.JJ7FJLJ7.LF-JL-JFJF7FJL7L7|||F7L||L-JLJF-JFJL7|LJL--7L7||LJL7F7|L7FJ||LJL7|FJFJF7J|FJ|F7|LJL7|L7FJLJL7||F7L--7FJF7||FJ|L7|L---7LJLFJJ-|F77L
FJ7-7-F-7-|L7F--7L-JLJF7L-JLJLJL-JL--7F-JF7|F7|L---7FJFJLJJF7|||L7LJFJL-7FJ|L7|FJL-JL7LJ|L7F-J|L|L7F--J||||F-7LJFJLJLJFL7LJF7F7|F|.J|--7---L
|.LJ|L--.FF.LJF7|F7F7FJL-----7F--7F7FJ|F7||LJ||F7F-JL7L--7FJLJ|L7L77L7F-J|FJFJ|L--7F7L-7L-JL-7L7L7||F-7|LJLJ||F-JF7F7F7FJF7|||LJ--77L.JJ-|J.
JF|-777.F-JFF7||LJLJLJF------J|F7||LJL|||||F7|||LJ7F7L7F-J|F7FJ-L7L-7|L-7|L7L7|-F-J||F7|F----JFJF||||FJL7F7F7LJF7|LJ|||L-JLJLJ|..|L7-7J7F|F7
LF7.||77|JF7|LJ|JF7.F7L7F-7F-7||LJL--7|||||||||L7F-JL-JL-7|||L7-FJF7|L7FJL7L7||FJF7|||LJL-7F-7L-7LJ||L-7LJ|||F-JLJF7LJL-------7--|-7-|LF|LJ7
FJL--F7FFFJLJF7L-J|FJL-J|J||FJ||F7F7FJ||||||LJL-JL----7F-J||L7L7L-JLJFJL-7L-J||L7||||L7F-7||FJF7|F-JL7FJF7LJLJF7F7|L7F7F7F7F--JJF-JJ.J-||-JJ
-J77L||||L7F7||F-7LJF7F7|FJ|L7|LJ||LJJ||||||F---7F7F--JL-7LJ|L7L----7L7F7L7F-J|FJ|LJL7LJFJ||L7|LJ|F7FJ|FJ|F7F-JLJLJFJ|LJ|||L7|LF7.F.FLF|L7J7
|7-7J|JL|LLJLJ|L7L-7|LJLJL7L7LJF7|L--7LJ||LJL7F7LJLJF7F-7L-7F-JF-7F7L7|||FJL7FJ|FJ.F-JF7L7||||||FJ|||FJL7|||L--7-F7L7|F7LJL7L77J.-F||FLJL|--
-|.F-L.|.|JJF-JFJF-J|F7F-7L-JJFJLJF-7L-7SJFF-J||F7F7||L7L7FJL-7L7LJ|FJLJLJF-J|FJ|F-JF7||FJLJFJL7L7|LJL-7|LJL7F7L-JL7|LJL--7|FJ.7.|L|FF|7J|-J
||-L7FJ--|7.|F-J7L7FJ||L7L----JF-7L7L7FJF-7L-7|||||||L7L7||F--JFJ7FJ|F7F7FJF-JL7|L7FJ|||L--7L7FJFJL-7F-JL--7||L7F-7||F----J||J|--|-FJ7-|FL7|
777.7J-FJJFF||7FF-JL7|L-JF--7F7|-L7L7|L7L7L--J|LJ||LJFJFJLJ|F-7L-7L7LJLJ|L7L-7FJ|FJL7LJ|F--JFJL7L7F7|L7F7F-JLJFJL7LJLJF--7-LJ-.|LF.-7J7LJJ-L
JJF7F7.|7J|7LJF-JF7FJL--7|F-J|LJF-JFJL-JLL7F7FJF-JL7||FJ.F-J|FJF-J||F7F7L7|F7|L7|L-7|F-JL--7|F7L7||||FJ||L---7L-7L7F--JF-J.|-J.|-F-FJ|L7.LFJ
FF.-J7.FF7F|.LL-7|||F---J|L-7|7FJF7L---7FFJ|||FL7F7|FJL-7|F7|L7L--7||LJ|FJ||||FJL7J|||F7F7FJLJL-JLJ||L7|L--7FJF-JFJ|F--JJ7L7.L-JFL-J7FJFJL|7
-JJJ.--FJJLL-FLFJ|LJL7F7FJF-J|FJFJ|F7F7L7L-JLJF-J|LJ|F7FJ||||FJF7FJ|L7JLJFJ|LJL-7L7||LJ|||L--7-F--7||J|L-7.LJ-L-7|J|L7F7.||.7F|.L-FFFJF.F-JL
FL7FF|..L|JL7LL|FJF--J||L7L7FJ|FJFJ|LJL7L-7JF-JF7L7.|||||LJ||L7|LJFJFJF7LL7|F---JFJ|L-7||L7F7|FJF7LJL7|F7L7LF-7FJL-JFJ|L-7F7--FF.F7JLF7-|7F.
F-F7LL-7-F7LF.FJL7L---JL7|FJ|FJ|LL7L7F7|F7|FJF7|L7L7||||F-7LJFJL-7L7L-JL-7LJL-7F7L7L--J|L7||LJL7|L7F7|LJL7L7L7||F7F7L-JF7LJ|.LFJ-LJ7F|.FLLJ.
JJ|..|||JLJ||-L--J-F--7|LJ|FJL7|F7L7LJ|||LJL7|||FJFJLJ|LJFJF-JF7FJ7L7F7F-J|F7FJ||FJF---JFJLJF-7LJLLJ|L7F7L7L7||LJLJL7F7|L--JJFJJF|FLL--J.JJ.
|.|-|J7|FJ7-77|LL|.L-7L-7FJL7||||L-JF-J|L-7FJ|LJL7|F7FJF7L7L-7||L-7FJ||L---J|L7|LJ-L7F7FJ-F7L7L-7F--JFJ|L7L7LJL--7LFJ||L7JFJ-7JJFJ-LJ-FJ-J77
F7.F|.LLL.L.J-|7|LFF7L-7LJF7|FJ||F-7|F7|F7|L-JF--JLJ|L7||FJF-J||F7||FJ|F----JFJL--7FJ||L--J|FJF7LJF-7L-JFJFJF7F7FJFJFJ|FJ7-L7|77L.7|7FJ||.L|
.JJJ-77.|.|..LLFLF-JL--JF7||||FJ||FJLJ|||LJF--JF7F7FJJ|||L7|F7|||LJ||L|L---77|F-7FJL-J|F-7FJ|FJL-7|F|F7FJ.L7|LJLJFL-J-LJ-||.FJF-JFF|JLF-77F-
F7J|7F7J|F-JF7-L.L--7F7FJ||||LJFJ|L7F7||L-7L7F7|||||F-J|L-J|||||L-7||FJF7F7|FJ|FLJF-7FJ|||L7||F7FJ|FJ||L7F-JL-7F7F7-|FJ.|.77L--7.F||..|.J-7J
.|JF|||F|J|.-..FFLL|||||J||LJF-JFJFJ|LJL--J-LJ|||||||F7L--7||||L7FJ|||FJLJ||L7L---JFJL7|FJFJLJ|LJFJ|FJ|FJ|F7F-J||||F77-FF.L|L---FLJ-L-|FLL|7
F-.LL7|F7FL|JL-L7LFFJ|LJ-LJ7LL7FJ|L7L-------7FJ|LJ||LJ|F7FJ||LJFJL7|||L-7FJL7L-7F--JJ-LJL7|JLFJF7L7|L7LJ7LJ||7FJLJLJL-7LLJL|L|7.|-|7.L|7.|JJ
|-J-J-LFL7.J.|.LJ7L|FJJ.FJJF-FJ|F7F|F7F-7F7FJL7|F-JL77||||-|L-7L7FJ|||F-JL7FJF-JL---7.LF-JL-7L7|L7|L7L---7||L-JF7F----J-F7.|.|-7J-|FFF|7..--
77JFLJFFJF7LL77L--JLJJ||7L-7-|FJ|L-J||||LJ||LLLJL7F7L7LJ|L7L--JFJL7LJLJ-LFJL7L-7F-7FJ-F|F-7FJFJ|F|L7L7F-7L7L7F-J|L--7J|LLFFJF-J|LLJJJ-----||
|7F7.LL-7|LFJL|JF7FJJF|J|.LF-LJJL-7FJLJ7|FLJ7JF--J|L7L-7L7L---7|F7|7.|JFJL--JFFJL7LJ-FFLJ-LJLL7L7L-JFJL7L7|FJL7FL7F-J|F7L|J.L..|..JJ.F.|.FFJ
F-|L|J|.|F-L77F7LLJ|LJ7.|--J.LL.J-LJ7LLF|7F|7LL--7|FJF-J||F7F-JLJLJJ--7LFJLJJLL7FJ7|7||LLFFJ7.|FJJJ-L7FJ-|||F7L7FJL---7-FJL7||F7-JJ.FJ.F|J|.
L|..J.-FF-.|LFJ77|.L7||-|L7J77FF7||LJFLLL--7J-LJF||L7|7FLLJLJJ|FJF|7J|-FL--|7J||L-7J-JLJ|||FF-LJJ--7J||F-J|LJ|FJ|F7F--J-|LFJ|7J|LL|-J--7.--|
FF.L7LL7JLFLF-JJ-J.L|7LJ|FL7L|7JLF|7LJJ7.|-F7-|FFLJL|L-77L|LJ|LFJJ.77LFFJJJL7-F|F-J|.FL.FJ.J|LJ|7L||LLJL7FJ-LLJ7LJ||-LJ7--FJ.L7-F-7L||.JJJ.7
7JFLL-LJ.LJJ||JFJF7-|FJF-JJ7|LF7F||F.LLF-JLLL-JFF-|LL--J7--.FL|J|J7.|.LJJ|F|J.LLJLLL7J|7..LLL.LL---7J|..||FFJLLLLLLJJ.L|7-||7LJJJ||.-77FL.F|
|.J7L7JJ..LJ|F7JLL7F-JF77LL-7F|77-JJFFJLL7-L|FFLJ7|7JJ|F|.JFF-7-7F7-|L-||F|7-.JJ|FLF7-LJJ-FF.L7|L7LLF7-7LJ-JF77FJ.|FF7.LL-JLJ.LFFF-FJ|FL-7L|
J-|-|-7-J7F7-LJLFJL|7FJ||7FFF7JLL7J.7J7.F7.|LL7L||.L7LF|J|.FL-|JL-|FLL-|FFL7.JJJLJF-JF.FL.FF-JFJFF7-F.F-LLF-|-77|FL|L-7.FL|F7LF|..F77F7JF77F
L--J|7J-LJ-J7J.FJLLL77.FJ-F-JJF.LLJ-LL|-F7-7LLJJF--J7.LF.-7.LFJ--J.JJ-LJJL|J-LLJLL7JL7-L|-LJ--J-F-|L|-L--JJ-LJL-L-L|J-J-L--J--7LFLJLJ-LFFJ-L";
