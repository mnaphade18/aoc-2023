#[derive(Debug)]
struct Target {
    time: usize,
    distance: usize,
}
pub fn solve() {
    let mut iter = INPUT.lines();
    let empty_string = "";
    let time_line = iter.next().unwrap().split(' ').filter(|x| *x != empty_string).skip(1);
    let mut distance_line = iter.next().unwrap().split(' ').filter(|x| *x != empty_string).skip(1);

    let mut result = 1;

    /* 
    // Part 1
    for t in time_line {
        let d = distance_line.next().unwrap();

        let t = Target {
            time: t.parse::<usize>().unwrap(),
            distance: d.parse::<usize>().unwrap(),
        };


        let valid_outcomes = get_valid_outcomes(&t);
        println!("{t:?} has outomces: {valid_outcomes}");

        result *= valid_outcomes;
    }
    */

    // Part 2 
    let time = time_line.fold(String::new(), |acc, t| acc + t);
    let distance = distance_line.fold(String::new(), |acc, t| acc + t);

    let t = Target { time: time.parse::<usize>().unwrap(), distance: distance.parse::<usize>().unwrap() };
    let valid_outcomes = get_valid_outcomes(&t);
    println!("{t:?} has outomces: {valid_outcomes}");

    println!("Result: {result}");
}


fn get_valid_outcomes(target: &Target) -> usize {
    let mut count = 0;
    for t in 0..target.time {
        if t * (target.time - t) > target.distance {
            // println!("valid outcome: {t}");
            count += 1;
        }
    }

    return  count;
}

const INPUTT: &str = "Time:      7  15   30
Distance:  9  40  200";

const INPUT: &str = "Time:        58     99     64     69
Distance:   478   2232   1019   1071";
