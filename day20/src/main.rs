extern crate regex;

use std::env;
use std::fs;

use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Eq, Clone, Hash, PartialEq)]
struct Tile {
    left: i16,
    top: i16,
    right: i16,
    bottom: i16,
}

impl Tile {
    fn print_debug(&self) {
        for i in 0..10 {
            if (self.top & 1<<i) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
        for i in 1..9 {
            if (self.left & 1<<i) != 0 {
                print!("#")
            } else {
                print!(".")
            }
            print!("????????");
            if (self.right & 1<<i) != 0 {
                println!("#");
            } else {
                println!(".");
            }
        }
        for i in 0..10 {
            if (self.bottom & 1<<i) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    fn rotate_clock(&self) -> Tile {
        Tile {
            left: self.bottom,
            top: flip_edge(self.left),
            right: self.top,
            bottom: flip_edge(self.right)
        }
    }

    fn flip_horiz(&self) -> Tile {
        Tile {
            left: self.right,
            top: flip_edge(self.top),
            right: self.left,
            bottom: flip_edge(self.bottom)
        }
    }

    fn flip_vert(&self) -> Tile {
        Tile {
            left: flip_edge(self.left),
            top: self.bottom,
            right: flip_edge(self.right),
            bottom: self.top
        }
    }

    // Include self
    fn gen_permutations(&self) -> HashSet<Tile> {
        let mut perms: HashSet<Tile> = HashSet::new();
        perms.insert(self.clone());
        perms.insert(self.flip_horiz());
        perms.insert(self.flip_vert());
        let r90 = self.rotate_clock();
        perms.insert(r90.flip_horiz());
        perms.insert(r90.flip_vert());
        let r180 = r90.rotate_clock();
        perms.insert(r180.flip_horiz());
        perms.insert(r180.flip_vert());
        let r270 = r180.rotate_clock();
        perms.insert(r270.flip_horiz());
        perms.insert(r270.flip_vert());
        perms.insert(r90);
        perms.insert(r180);
        perms.insert(r270);

        perms
    }
}

// top/bottom are lsb -> msb left to right
// left/right are lsb -> msb top to bottom
fn parse_edge(lines: &Vec<&str>, rs: usize, cs: usize, dr: usize, dc: usize) -> i16 {
    let mut bits: i16 = 0;
    for i in 0..10 {
        if lines[rs + i*dr].as_bytes()[cs + i*dc] == '#' as u8 {
            bits |= 1 << i;
        }
    }
    bits
}
// left,top,right,bottom
fn parse_edges(lines: &Vec<&str>, start: usize) -> Tile {
    let left = parse_edge(lines, start, 0, 1 , 0);
    let top = parse_edge(lines, start, 0, 0, 1);
    let right = parse_edge(lines, start, 9, 1, 0);
    let bottom = parse_edge(lines, start + 9, 0, 0, 1);
    Tile { left: left, top: top, right: right, bottom: bottom}
}

fn flip_edge(edge: i16) -> i16 {
    let bit0 = edge >> 9 & 1 << 0;
    let bit1 = edge >> 7 & 1 << 1;
    let bit2 = edge >> 5 & 1 << 2;
    let bit3 = edge >> 3 & 1 << 3;
    let bit4 = edge >> 1 & 1 << 4;
    let bit5 = edge << 1 & 1 << 5;
    let bit6 = edge << 3 & 1 << 6;
    let bit7 = edge << 5 & 1 << 7;
    let bit8 = edge << 7 & 1 << 8;
    let bit9 = edge << 9 & 1 << 9;
    bit0 | bit1 | bit2 | bit3 | bit4 | bit5 | bit6 | bit7 | bit8 | bit9
}

// Attempt to place a tile at i,j, returns whether or not it succeeded
fn recurse(
    placed: &mut HashMap<(i32, i32), (i32, Tile)>,
    i: i32,
    j: i32,
    n: i32,
    tiles: &HashMap<i32, Tile>,
    used: &mut HashSet<i32>
) -> bool {
    if i == n {
        // base case - all placed
        return true;
    }
    //println!("lookin at ({}, {})", i, j);
    let left = placed.get(&(i, j-1)).map(|t| t.1.right);
    let top = placed.get(&(i-1, j)).map(|t| t.1.bottom);
    for (id, tile) in tiles {
        if !used.contains(id) {
            for perm in tile.gen_permutations() {
                let matches_left = left.map(|edge| edge == perm.left).unwrap_or(true);
                let matches_top = top.map(|edge| edge == perm.top).unwrap_or(true);
                if matches_left && matches_top {
                    //println!("Placing a perm of {} at ({}, {})", *id, i, j);
                    placed.insert((i,j), (*id, perm));
                    used.insert(*id);
                    let (next_i, next_j) = if j == n-1 {
                        (i+1, 0)
                    } else {
                        (i, j+1)
                    };
                    if recurse(placed, next_i, next_j, n, tiles, used) {
                        return true;
                    }
                    // To unplace, just remove from used
                    used.remove(id);
                } else {
                    //println!("{:?} {:?}", left, top);
                    //println!("vs {} {} for {}", perm.left, perm.top, *id);
                }
            }
        }
    }

    //println!("rip");
    false
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let tile_re = Regex::new(r"Tile (\d+):").unwrap();
    let mut tiles: HashMap<i32, Tile> = HashMap::new();

    let mut i = 0;
    while i < lines.len() {
        let cap = tile_re.captures(lines[i]).unwrap();
        let edges = parse_edges(&lines, i+1);
        tiles.insert(cap[1].parse().unwrap(), edges);
        i += 12;
    }

    let mut placed = HashMap::new();
    let mut used = HashSet::new();
    let n = 12;
    let res = recurse(&mut placed, 0, 0, n, &tiles, &mut used);
    println!("Works? {}", res);
    if res {
        println!("{} {}", placed[&(0, 0)].0, placed[&(0, n-1)].0);
        println!("{} {}", placed[&(n-1, 0)].0, placed[&(n-1, n-1)].0);
    }
}
