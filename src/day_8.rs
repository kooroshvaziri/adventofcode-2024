use std::collections::HashSet;
use std::collections::HashMap;

#[derive (Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn calculate_antinodes(map: &Point, antennas: &HashMap<char, Vec<Point>>, resonance: &bool)->usize {
    let mut antinodes: HashSet::<Point> = HashSet::new();
    for (_, locations) in antennas {
        for i in 0..locations.len() {
            let center = locations[i].clone();
            let mut other_locs = locations.clone();
            other_locs.remove(i);
            for l in other_locs {
                if *resonance {
                    let mut factor = 1;
                    loop {
                        let antinode = Point {x: center.x + factor*(l.x - center.x), y: center.y + factor*(l.y - center.y)};
                        if antinode.x>=0 && antinode.y>=0 && antinode.x<map.x && antinode.y<map.y {
                            antinodes.insert(antinode.clone());
                        } else {
                            break;
                        }
                        factor += 1;                        
                    }
                } else {         
                    let antinode = Point {x: center.x + 2*(l.x - center.x), y: center.y + 2*(l.y - center.y)};
                    if antinode.x>=0 && antinode.y>=0 && antinode.x<map.x && antinode.y<map.y {
                        antinodes.insert(antinode.clone());
                    }
                }
            }
        } 
    }
    antinodes.iter().count()
}

fn construct_map(source: &str)->Vec<Vec<char>> {
    source.split('\n').collect::<Vec<&str>>().iter()
            .map(|l| 
                
                l.chars().collect::<Vec<char>>()
            ).collect::<Vec<Vec<char>>>()
}  

fn construct_antennas(map: &Vec<Vec<char>>)->HashMap::<char, Vec<Point>> {
    let mut hash_map: HashMap::<char, Vec<Point>> = HashMap::new();
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            let c = map[j][i];
            if c != '.' {
                let mut childs = if let Some(childs) = hash_map.get(&c) {
                    childs.clone()
                } else {
                    Vec::<Point>::new()
                };            
                childs.push(Point{x: i as i32, y: j as i32});
                hash_map.insert(c, childs);
            }
        }
    }
    hash_map
}

fn main() {

    let source =
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    
    
    let map = construct_map(&source);
    let antennas = construct_antennas(&map);
    let bounds = Point {x: map[0].len() as i32, y: map.len() as i32};
    assert_eq!(14, calculate_antinodes(&bounds, &antennas, &false));
    assert_eq!(34, calculate_antinodes(&bounds, &antennas, &true));
    
    let source =
"...............................s..................
..................s..............q.............p..
.....a............................................
........c......Y.......Q..........................
............................................4.....
........Y.........y............m..........4.......
......................Y...s..........S............
.........................................S........
...............N.............y....................
...........a.......y..................1...........
................................................S.
...c........k.............q....t............S.....
.............................qM...................
........a.........................................
..................................................
..................................................
..c..........k...Q..q....P........................
5.................Q...................8...........
......yc..........................................
........................E............4............
.........6........................u..p.....4......
.........5.............P..n......1.........N......
6..............................1.........J.t......
..6..................................3.u..t.....p.
....5...k..........................u..............
.......................E..................u....x..
..................E.................x.............
...k..................P.............3.............
...........0.....9.5...........E.........31e....N.
......0.................................N.........
.................CU.....................t....x....
......7....................e......................
....0..........K......C...........................
.....6....j......M............................J...
......K.................................p.........
.....9........................U...................
............................3....n................
.............K.........2.....C..................x.
....................P........UJ...................
.....0......X...C.........T..............U........
.......M.....8j....7.............2........Q.......
9...............K.................................
....e.....8.........................2.A.m.........
..e......8.........s...n..........................
.....................................T..nm........
...................X............2.........m......A
......................X..j....................T...
.........7..M......j.............T................
....9...7....................................A....
..........................................A.......";
  
    let map = construct_map(&source);
    let antennas = construct_antennas(&map);
    let bounds = Point {x: map[0].len() as i32, y: map.len() as i32};
    assert_eq!(244, calculate_antinodes(&bounds, &antennas, &false));
    assert_eq!(912, calculate_antinodes(&bounds, &antennas, &true));
}
