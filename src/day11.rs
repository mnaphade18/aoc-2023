use std::usize;

type Space = Vec<Vec<char>>;

fn expand(space: &mut Space) {
    //expand vert
    let mut index_to_add = Vec::new();
    let mut len = 0;
    for (i, row) in space.iter().enumerate() {
        if len == 0 {
            len = row.len();
        }

        let galaxy_present = row.iter().fold(false, |acc, c| {
            return *c == '#' || acc;
        });

        if !galaxy_present {
            index_to_add.push(i);
        }
    }

    index_to_add.reverse();

    let v = vec!['.'; len];
    for id in index_to_add.iter() {
        space.insert(*id, v.clone());
    }

    //expand horizontal
    index_to_add = Vec::new();
    let vert_len = space.len();
    println!("LENGTHS: VERT: {}, LEN: {}", vert_len, len);

    for i in 0..len {
        let mut galaxy_present = false;
        for j in 0..vert_len {
            galaxy_present |= space[j][i] == '#'
        }
        if !galaxy_present {
            index_to_add.push(i);
        }
    }
    index_to_add.reverse();

    for i in index_to_add.iter() {
        for r in space.iter_mut() {
            r.insert(*i, '.');
        }
    }
}

fn get_galaxies(space: &Space) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    for (i, s) in space.iter().enumerate() {
        for (j, c) in s.iter().enumerate() {
            if *c == '#' {
                result.push((i,j));
            }
        }
    }

    return result;
}

fn find_distances(p: &Vec<(usize,usize)>) -> Vec<usize> {
    let mut result = Vec::new();

    for (i, g1) in p.iter().enumerate() {
        for j in (i + 1)..p.len() {
            let g2 = &p[j];

            let d = g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
            result.push(d);
        }
    }

    return result;
}

pub fn solve() {
    let ip = INPUT;
    let mut space: Space = ip.lines().map(|l| l.chars().collect()).collect();

    expand(&mut space);

    let g = get_galaxies(&space);

    let distances = find_distances(&g);
    let sum: usize = distances.iter().sum();
    println!("DISANCE: {:?}, sum: {}", distances, sum);
}

const INPUTT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

const INPUT: &str = "...........#..........................#..................................................................#......#......#....................
...#....................#..........................................#..............#...........#.............................................
.............................................................................................................................#..............
........#...........#.............#..........#.....#...............................................................#........................
.............................#..............................................................................................................
........................................................................................................#..................................#
.....#..........#....................#.................#............................#.......................................................
.....................................................................#...................#.......#....................................#.....
............................................................................................................................................
........................................#....................................................................#..............#...............
...........#........................................#............#.....................................................#...........#........
......................#.............#..........................................................#.................#..........................
.............................................#..........................................................#...................................
.#.............................#............................#...............#.............#...................................#.............
............................................................................................................................................
...................................................................................#........................................................
.......#.......................................#...............#..........................................#.........#................#......
....................#..............#....................................................#........#........................................#.
...................................................#...................#.....................................................#..............
..........................#....................................................#.............................#..............................
............................................................................................................................................
.........#.....#...........................#...............................................#.......................#........................
..........................................................................................................................#.............#...
.............................................................#..............#.........#...........................................#.........
..................................#....................................................................#....................................
.................................................................................................................#..........................
.............#......................................................#.............................#..................................#......
......................#.....#...........#........#.........#.....................#.........................#...............#................
................................................................#................................................................#..........
.....#.......................................#.......................................................................#......................
.........................................................................#................#..........#......................................
..................#.....#.....................................................#..............................#..............................
.....................................#..................................................................................................#...
.............................................................#..........................................#......................#............
.....................................................#..............#................#............#...................#.....................
............................................................................#........................................................#......
...#.......#................................................................................................................................
.................#............................#..............................................................#............#.................
...........................#.............................................................#......#...........................................
...............................................................#........#...........#.......................................................
......#......................................................................................................................#......#......#
........................................................#................................................#..................................
...............................#................#.............................#.............#...............................................
.........................................#...........................#......................................................................
.#.........................#........#...........................#..................#..................#.....................................
.....................................................#........................................................#...........#.................
.......#..........#......................................................................#..........................#.............#.......#.
............................................................................................................................................
............................................................................................................................................
..................................................................................................#.........................................
..................................#.............................#......#..............#..............................................#......
............................................................................................................................................
.............................................#...................................#........................#.................................
.....#......#............#.....................................................................................#..................#.........
...........................................................#.................................................................#..............
................................#...................................................................................#.......................
................................................#..............................................#........#...................................
..........#..........#.....#........................................................#...........................................#...........
......................................#........................................#..........#..............................................#..
...................................................................#.....#.............................................#....................
.....#..........#.........................................#.................................................................................
.............................................#....................................#...........#.............................................
..........................................................................................................#......................#.........#
.#.................#............#........#..................................................................................................
...........................#..............................................................#..................................#..............
.............#......................#.................................................................#.................................#...
.......................#............................#......#....................................#...........................................
........#......................................#....................#.......................................................................
...............................#...............................................................................#............................
................................................................#.................#........#................................................
.#.....................................................................................................................#....................
................#..........#.......................#....................................................#...................#........#......
..............................................................................................#.............................................
..........#............#.............................................#......................................................................
...............................#.....#....................#.................................................................................
.....................................................................................#.............................................#........
..........................................................................................#........................#........................
...#....................................#..........................#........................................................................
............................#.......................................................................#.......................................
..............................................#.............................................................................................
..................#.....#.......................................#...........#.........................................................#.....
#...........#.......................................................................................................#.......................
..........................................#............#.........................#......................#...................................
.............................................................#............................#.................................................
.........#..........#...............................................#...........................#.......................#..................#
..........................#......................#....................................#......................................#..............
.....................................#.............................................................................................#........
.............#..................#...........................................................................................................
.......#..................................#..............................#.................#.....................#......................#...
.....................#......................................#...............................................................................
.............................#.........................#..........................#..............#..........................................
.......................................#................................................................#...........#.......................
............................................................................................................................................
............................................................................................................................................
..................#............#............#.......#.......................#...............................................#...............
.#........#...........................................................#...............................................#.....................
...............................................................................................................#............................
...................................................................................#.......................................................#
................................................#...........#...............................................................................
...................................................................................................#..............#.........................
.........#............#..............#.................................#...................#...................................#............
................#..............#...................#.............................#..........................................................
.............................................#................#........................................#..................#.................
.....#.........................................................................................................#...................#........
...................................#......................#........#......................................................................#.
........................#.............................................................#........#............................................
..#......................................#....................................#.............................................................
................#...............#.....................#..........................................................#..........................
.....................................................................................................#.......................#..............
...............................................#...........................................................................................#
.........#...........#.....................................#................#........................................#......................
...#..........#.........................#............................#...............................................................#......
............................................................................................................#.............#.................
..........................#...................................#............................#...................................#............
.....................................................................................#......................................................
..........#..............................................................................................................................#..
................#..............#.............#.........................#................................#...........................#.......
....#...............................#.......................#....................#..........................................................
....................................................#...........................................................#............#..............
.................................................................................................#..........................................
............................................................................................................................................
.........#...........#.....................#................................#..........#...........................#........................
...................................#........................................................................................................
..................................................................................#................................................#........
..............#.........................................................#.................................#...................#.........#...
..#...........................................#.............................................................................................
...........................#.........................................................................................#......................
...............................................................#............................................................................
.........#..........#....................................................................#.......#......#........................#..........
...................................#.................#...........................#.............................#............................
.....#.............................................................#........................................................................
.............................#..............................................................................................................
..............#.........................#...........................................................#..........................#.........#..
................................................................#...........................................................................
..#.........................................#.................................#..................................#..........................
.......................#.......#.......................................#....................#..............................#................
....................................................................................#...................#..........................#........
.....#.....#....................................#...................................................................#.......................
..........................................#............#....................................................................................
...............#............................................#..................................#............................................";
