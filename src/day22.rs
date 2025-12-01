struct CoOrd {
    x: usize,
    y: usize,
    z: usize,
}
struct Brick {
    start: CoOrd,
    end: CoOrd,
}
impl Brick {
    pub fn from_str(ip: &str) -> Vec<Brick> {
        ip.lines().map(|line| {
            let mut parts = line.split("~");
            let mut start_points = parts.next().unwrap().split(",");
            let start_coord = CoOrd {
                x: start_points.next().unwrap().parse::<usize>().unwrap(),
                y: start_points.next().unwrap().parse::<usize>().unwrap(),
                z: start_points.next().unwrap().parse::<usize>().unwrap(),
            };

            let mut end_points = parts.next().unwrap().split(",");
            let end_coord = CoOrd {
                x: end_points.next().unwrap().parse::<usize>().unwrap(),
                y: end_points.next().unwrap().parse::<usize>().unwrap(),
                z: end_points.next().unwrap().parse::<usize>().unwrap(),
            };


            return Brick {
                start: start_coord,
                end: end_coord,
            }
        }).collect()
    }

    pub fn get(&self, direction: char) -> std::ops::Range<usize> {
        match direction {
            'x' => self.start.x..self.end.x,
            'y' => self.start.y..self.end.y,
            'z' => self.start.z..self.end.z,
            _ => unimplemented!("Invalid dir"),
        }
    }

    pub fn touching(&self, other: &Brick) -> bool {
        let self_x = self.get('x');
        let other_x = other.get('x');
        let x_match =  self_x.end > other_x.start && self_x.start < other_x.end;

        let self_y = self.get('y');
        let other_y = other.get('y');
        let y_match =  self_y.end > other_y.start && self_y.start < other_y.end;

        let self_z = self.get('z');
        let other_z = other.get('z');
        let z_match =  self_z.end > other_z.start && self_z.start < other_z.end;

        return (x_match && y_match) || (y_match && z_match) || (z_match && x_match);
    }
}

pub fn solve() {
    let input = TEST_INPUT;
    let tower = Brick::from_str(input);
}

const TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
