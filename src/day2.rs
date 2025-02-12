#[derive(Debug)]
struct Balls {
    red: usize,
    blue: usize,
    green: usize,
}

pub fn solve() {
    let lines = INPUT1.split("\n");
    let mut result = 0;

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let num = parse_line2(line);

        result += num;
    }

    println!("Final: {}", result);
}

fn parse_line(line: &str) -> (usize, bool) {
    println!("Line: {}", line);
    let seperator = line.find(":").unwrap();
    let id = line[5..seperator].parse::<usize>().unwrap();

    let games = line[seperator+2..].split("; ");
    for game in games {
        let balls = parse_game(game);

        if balls.red > 12 || balls.green > 13 || balls.blue > 14 {
            return (id, false);
        }
    }

    println!("Valid game: {}",line);
    return (id, true);
}

fn parse_line2(line: &str) -> usize {
    let seperator = line.find(":").unwrap();
    let mut least_balls = Balls { red: 0, green: 0, blue: 0 };

    let games = line[seperator+2..].split("; ");
    for game in games {
        let balls = parse_game(game);

        println!("Game {:?}, least: {:?}", balls, least_balls);
        if balls.red > least_balls.red {
            least_balls.red = balls.red;
        }
        if balls.green > least_balls.green {
            least_balls.green = balls.green;
        }
        if balls.blue > least_balls.blue {
            least_balls.blue = balls.blue;
        }
    }

    println!("Valid game: {}, {:?}",line, least_balls);
    return least_balls.red * least_balls.green * least_balls.blue;
}

fn parse_game(game: &str) -> Balls {
    let iter = game.split(", ");

    let mut balls = Balls{ red: 0, blue: 0, green: 0 };

    for output in iter {
        let mut draw = output.split(" ");
        let num = draw.next().unwrap().parse::<usize>().unwrap();
        let color = draw.next().unwrap();

        match color {
            "blue" => balls.blue += num,
            "green" => balls.green += num,
            "red" => balls.red += num,
            _ => panic!("invalid color"),
        }
    }

    return balls;
}

const INPUT1T: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

const INPUT1: &str = "Game 1: 1 green, 2 blue; 13 red, 2 blue, 3 green; 4 green, 14 red
Game 2: 2 blue, 11 green; 4 blue, 12 red, 4 green; 7 red, 1 blue, 9 green; 10 green, 12 red, 6 blue
Game 3: 1 blue, 12 green, 2 red; 9 red, 16 green; 1 red, 10 green, 1 blue; 1 red, 14 green
Game 4: 8 green, 18 blue; 4 green, 14 blue, 2 red; 3 blue, 5 green, 11 red
Game 5: 7 red, 15 blue, 1 green; 13 blue; 18 red, 2 green, 9 blue; 19 blue, 5 green, 10 red; 9 green, 2 blue, 7 red
Game 6: 1 red, 8 blue, 2 green; 1 blue, 3 red, 5 green; 2 green, 3 red, 2 blue; 1 blue, 4 green
Game 7: 4 red, 6 green, 1 blue; 3 blue, 9 green, 5 red; 5 blue, 5 green, 4 red; 6 green, 5 blue, 5 red; 13 red; 4 red, 2 blue, 9 green
Game 8: 7 blue, 14 green, 5 red; 1 green, 7 blue, 11 red; 6 blue, 16 red, 4 green; 8 red, 11 green, 7 blue; 6 blue, 18 red, 9 green
Game 9: 3 blue, 1 green, 6 red; 1 red, 1 blue, 1 green; 6 red, 2 blue, 1 green
Game 10: 6 red, 11 blue, 12 green; 1 blue, 2 red, 3 green; 14 green, 6 red, 7 blue; 6 red, 10 blue, 9 green; 6 blue, 2 red
Game 11: 5 blue, 2 red, 18 green; 2 blue, 13 green; 8 blue, 15 green, 2 red
Game 12: 5 red, 4 blue; 1 blue, 8 green, 8 red; 15 green, 5 red, 4 blue
Game 13: 12 blue, 1 red, 6 green; 9 red, 6 blue, 12 green; 3 red, 11 green, 6 blue; 8 red, 4 green, 10 blue; 3 green, 7 blue
Game 14: 9 red, 5 green, 1 blue; 1 red, 3 blue; 10 green, 7 red; 5 blue, 8 green, 10 red
Game 15: 3 blue, 9 red, 14 green; 15 green, 2 blue, 6 red; 7 green, 1 red, 5 blue
Game 16: 7 green, 4 blue, 6 red; 6 green, 4 blue, 8 red; 5 red, 2 blue, 10 green; 4 blue, 4 red, 15 green; 9 red, 15 green, 6 blue; 7 green, 5 blue, 2 red
Game 17: 8 red, 6 green, 2 blue; 4 green, 1 blue, 17 red; 1 green, 11 red, 1 blue
Game 18: 13 blue, 1 green, 3 red; 2 red, 2 green; 11 red, 1 green, 7 blue; 2 green, 7 blue, 2 red
Game 19: 2 green, 13 red, 2 blue; 1 green, 6 blue, 4 red; 1 green, 2 blue, 15 red
Game 20: 10 red, 11 blue, 8 green; 6 red, 13 blue, 2 green; 6 blue, 8 green, 3 red; 10 red, 2 blue, 8 green; 2 red, 3 blue, 7 green; 3 red
Game 21: 2 blue, 10 red; 5 green, 3 red; 5 green, 4 red, 4 blue; 6 blue, 9 red, 10 green
Game 22: 5 red, 1 green; 5 red; 4 red, 1 blue; 1 blue, 10 red
Game 23: 2 red, 11 blue, 4 green; 4 blue; 1 blue, 6 green, 4 red; 3 green, 4 blue, 2 red
Game 24: 4 green; 12 green, 4 blue, 1 red; 11 green, 1 blue, 2 red
Game 25: 10 red, 1 blue, 3 green; 3 green, 6 blue, 6 red; 2 green, 9 red, 2 blue; 1 blue, 3 green, 7 red; 6 blue, 4 green; 1 red
Game 26: 4 red, 11 green, 5 blue; 2 blue, 11 red, 9 green; 11 green, 3 red; 14 green, 13 red, 5 blue; 10 red, 5 blue, 9 green; 12 red, 4 blue, 6 green
Game 27: 8 red, 9 blue, 7 green; 14 red, 15 blue, 2 green; 7 green, 13 blue, 9 red; 8 green, 4 blue, 1 red
Game 28: 4 blue, 4 green, 3 red; 4 green, 9 red, 4 blue; 13 red, 10 blue, 4 green
Game 29: 7 red; 1 blue, 1 green; 3 red, 1 green, 1 blue; 2 blue; 17 red, 2 blue, 1 green; 6 red, 1 green, 2 blue
Game 30: 10 green, 4 red, 4 blue; 5 red, 6 blue, 9 green; 2 red, 3 green, 4 blue; 1 blue, 9 green, 1 red; 1 red
Game 31: 6 red, 3 green; 1 blue, 3 green, 9 red; 2 blue, 11 red; 13 red, 2 blue, 3 green; 5 green, 3 blue
Game 32: 2 red, 1 blue, 1 green; 3 green, 1 blue; 1 red, 6 green; 3 red, 3 green; 1 blue; 12 green, 3 red, 1 blue
Game 33: 17 blue, 14 green, 7 red; 5 red, 9 green, 16 blue; 8 green, 3 blue, 3 red; 10 green, 12 blue, 1 red
Game 34: 2 red, 1 green, 9 blue; 14 blue, 2 green, 9 red; 10 blue, 2 green, 16 red; 2 green, 5 red, 14 blue; 5 blue, 12 red, 2 green; 8 blue, 15 red, 3 green
Game 35: 14 green, 4 red; 16 green, 4 red; 4 red, 3 blue, 3 green; 5 green, 4 red, 4 blue; 3 red, 1 blue, 8 green
Game 36: 1 blue, 8 red, 4 green; 2 green, 15 blue, 4 red; 13 blue, 5 red, 8 green
Game 37: 2 red, 8 green, 7 blue; 9 green, 20 red, 3 blue; 3 blue, 1 green, 2 red; 9 red, 1 blue, 4 green; 6 green, 2 blue, 20 red
Game 38: 8 red, 2 blue, 2 green; 7 green, 4 red; 4 red, 10 green, 2 blue
Game 39: 9 green, 11 red, 6 blue; 6 blue, 2 green, 19 red; 2 red, 3 blue, 2 green; 4 green, 2 blue, 13 red; 19 red, 7 green, 1 blue
Game 40: 7 blue, 2 green, 11 red; 19 red, 4 green, 2 blue; 1 green, 10 red, 5 blue; 2 red, 1 green, 2 blue
Game 41: 1 red, 1 blue, 13 green; 13 green, 11 red, 5 blue; 19 green, 3 blue; 18 green, 1 red
Game 42: 1 blue, 2 green, 7 red; 13 green, 5 red, 7 blue; 6 red, 13 green, 9 blue; 12 red, 17 blue, 13 green; 9 red, 16 blue, 5 green; 11 red, 4 green
Game 43: 3 blue, 4 red; 6 red, 5 blue; 7 blue, 1 green, 5 red; 3 blue, 10 red
Game 44: 10 green, 10 blue; 2 blue, 10 red, 10 green; 2 green, 5 red, 4 blue; 8 red, 2 green, 2 blue; 14 red, 3 blue, 10 green; 14 red, 5 green, 3 blue
Game 45: 9 green, 2 red, 2 blue; 2 blue, 2 red, 7 green; 9 blue, 6 green
Game 46: 2 blue, 1 green; 1 blue, 1 red, 1 green; 1 blue; 1 blue, 1 green
Game 47: 4 green, 10 red, 14 blue; 19 red, 3 blue, 1 green; 4 green, 14 blue, 15 red; 9 blue, 17 red; 3 green, 12 blue, 7 red; 1 red, 11 blue, 4 green
Game 48: 9 red, 2 green, 1 blue; 7 red, 3 blue; 8 red, 1 green; 1 blue, 3 red
Game 49: 2 red, 3 green, 13 blue; 7 red, 15 blue, 3 green; 15 blue, 7 red; 11 blue, 5 red, 3 green
Game 50: 4 red, 4 green, 2 blue; 2 green, 3 red; 2 green, 3 red, 11 blue; 3 green, 2 red
Game 51: 4 blue, 17 green; 3 blue, 17 green, 1 red; 6 green, 8 blue
Game 52: 14 blue, 3 red; 11 green, 6 red, 9 blue; 6 blue, 10 red; 1 red, 1 green, 4 blue; 9 blue, 6 green; 3 red, 2 blue, 8 green
Game 53: 1 blue, 9 red, 9 green; 11 green, 1 red, 7 blue; 11 green, 9 red, 7 blue; 12 green, 15 red, 7 blue; 7 blue, 12 red, 1 green; 6 blue, 2 green, 4 red
Game 54: 7 blue, 2 red, 4 green; 2 red, 16 blue, 8 green; 15 blue, 7 green; 5 blue, 3 red, 2 green; 9 green, 15 blue; 1 green, 14 blue
Game 55: 5 red, 10 blue, 2 green; 5 blue, 5 red, 2 green; 6 red, 1 green; 7 red, 8 blue, 1 green; 1 green, 13 blue
Game 56: 14 blue, 11 green, 3 red; 10 blue, 7 green, 2 red; 2 red, 13 green, 1 blue
Game 57: 10 blue, 6 red; 10 red, 4 blue, 6 green; 2 green, 16 blue; 6 blue, 11 red, 7 green
Game 58: 3 red, 2 blue; 9 green, 12 blue; 1 red, 1 green, 5 blue; 1 blue, 2 green
Game 59: 6 red, 9 green, 2 blue; 6 green, 6 red, 7 blue; 2 green, 7 blue, 9 red; 1 green, 10 blue; 1 green, 3 blue, 1 red; 10 green, 1 red, 3 blue
Game 60: 13 green, 5 red, 9 blue; 3 blue, 12 green, 9 red; 4 blue, 17 green, 9 red; 12 green, 2 red; 6 red, 3 green, 8 blue; 13 green, 12 red
Game 61: 4 green, 15 red; 5 green, 15 red; 1 blue, 7 green, 12 red; 8 green, 3 blue, 4 red; 11 green, 11 red, 3 blue; 7 red, 12 green, 2 blue
Game 62: 3 red; 2 green, 8 red; 10 red, 8 blue; 1 blue, 4 red, 2 green; 2 green, 13 red; 6 red, 9 blue
Game 63: 4 green, 12 blue, 5 red; 5 blue, 5 green, 5 red; 3 blue, 17 green; 1 red, 1 blue, 17 green; 10 green, 4 red, 15 blue; 15 blue, 4 green
Game 64: 12 red, 9 blue, 4 green; 1 green, 1 red, 8 blue; 10 green, 11 red, 1 blue; 2 red, 10 blue
Game 65: 1 red, 3 green, 11 blue; 2 red, 6 green, 3 blue; 1 red, 7 green, 1 blue
Game 66: 2 green, 2 red, 4 blue; 19 red, 11 blue, 4 green; 6 blue, 2 green, 13 red; 16 blue, 4 green
Game 67: 1 blue, 5 red, 1 green; 1 green, 1 red, 1 blue; 2 green, 1 blue, 19 red
Game 68: 3 green, 1 red; 1 red, 3 blue; 3 blue, 2 red, 10 green; 4 green
Game 69: 5 green, 11 blue, 5 red; 13 green, 19 red, 8 blue; 19 red, 2 green, 11 blue
Game 70: 7 blue, 9 green; 1 red, 11 blue, 2 green; 6 blue, 10 green, 1 red
Game 71: 2 red, 2 green, 9 blue; 1 green, 20 blue, 1 red; 17 blue, 3 green, 2 red; 3 green, 13 blue, 2 red; 16 blue, 3 green; 11 blue
Game 72: 6 red, 4 green, 10 blue; 6 red, 7 blue; 1 blue, 8 green; 3 green, 6 red, 1 blue
Game 73: 2 green, 12 blue, 2 red; 6 red, 6 blue, 3 green; 8 red, 3 green, 1 blue; 3 green, 5 red, 10 blue
Game 74: 1 red, 15 green; 3 blue, 5 green, 6 red; 7 red, 5 green; 12 red, 12 green
Game 75: 14 blue, 2 green, 8 red; 13 blue, 6 green, 4 red; 4 green, 6 red, 1 blue
Game 76: 12 red, 5 blue, 1 green; 6 blue, 11 red; 2 red, 7 blue; 6 blue, 16 red
Game 77: 7 red, 2 green, 1 blue; 6 blue; 3 red, 1 green, 10 blue; 5 blue, 2 green, 1 red
Game 78: 9 blue, 3 green, 14 red; 16 red, 15 blue; 16 red, 9 green, 10 blue; 2 red, 8 blue, 3 green
Game 79: 7 green, 5 blue, 2 red; 5 red, 5 blue, 1 green; 2 red, 3 blue, 10 green; 5 green, 1 blue, 3 red; 5 blue, 3 red, 7 green
Game 80: 7 red, 6 blue; 6 red, 6 blue; 7 blue, 2 green; 3 red, 9 blue
Game 81: 8 green, 9 red, 1 blue; 9 red, 7 green; 1 green, 5 red; 2 green, 1 red; 1 blue, 14 red, 6 green; 1 blue, 11 green, 6 red
Game 82: 4 blue, 14 red, 1 green; 2 red, 4 green, 2 blue; 7 green, 4 blue, 13 red; 2 red, 1 green, 6 blue
Game 83: 9 red, 2 blue, 2 green; 6 green, 6 red; 9 red, 3 blue, 6 green
Game 84: 9 green, 1 red, 1 blue; 4 red, 15 green; 7 green, 1 blue, 12 red; 1 blue, 11 red, 14 green; 2 green, 11 red
Game 85: 13 green; 11 green; 15 blue, 10 green; 8 blue, 6 green; 11 green, 1 red, 3 blue; 13 green
Game 86: 13 green, 1 blue, 4 red; 1 green, 5 red; 1 blue, 9 green, 5 red
Game 87: 4 green, 5 red, 2 blue; 4 green; 8 green, 2 blue, 1 red; 4 blue, 8 green, 1 red; 5 red, 9 blue, 8 green; 7 blue
Game 88: 5 green, 1 blue, 3 red; 2 blue, 2 red, 3 green; 3 green; 2 blue, 1 red, 6 green
Game 89: 13 blue, 6 red, 15 green; 5 green, 14 blue, 9 red; 3 green, 15 blue, 5 red; 13 red, 13 green; 18 red, 4 green, 19 blue; 10 green, 10 red, 18 blue
Game 90: 1 red, 1 green; 1 blue, 2 green, 1 red; 5 red, 2 blue
Game 91: 9 blue; 6 green, 12 blue, 2 red; 3 red, 1 green, 8 blue; 7 green, 2 red, 8 blue
Game 92: 15 red, 7 green, 1 blue; 19 green, 12 red, 1 blue; 7 red, 16 green; 8 green, 1 blue, 14 red; 8 red, 1 blue, 9 green
Game 93: 11 red, 4 green; 7 green, 8 red; 4 blue, 6 green; 8 red, 6 green, 1 blue; 6 green; 1 red, 1 green, 4 blue
Game 94: 15 green; 1 red, 4 blue, 4 green; 1 red, 1 blue; 4 red, 1 green, 5 blue; 18 green, 3 red, 3 blue; 1 red
Game 95: 1 blue, 3 green, 10 red; 1 blue, 6 green; 15 green, 2 red, 3 blue; 2 blue, 8 red, 11 green; 13 green, 10 red; 5 red
Game 96: 8 green; 5 blue, 2 red, 4 green; 3 green, 4 red
Game 97: 5 red, 4 blue; 1 blue, 2 red, 9 green; 10 green, 3 red; 4 green, 3 blue, 4 red; 5 red, 4 green, 3 blue
Game 98: 3 green, 3 blue, 2 red; 2 blue, 2 red, 1 green; 3 green, 5 blue
Game 99: 11 green, 4 red, 12 blue; 9 red, 4 blue; 20 green, 6 blue
Game 100: 12 red, 9 green; 12 red; 9 red, 3 green; 8 red, 4 blue, 4 green; 8 blue, 11 red, 2 green
";
