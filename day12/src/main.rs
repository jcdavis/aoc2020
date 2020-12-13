extern crate regex;

use std::env;
use std::fs;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let re = Regex::new(r"(\w)(\d+)").unwrap();

    let actions: Vec<(String, i32)> = input.lines().map(|line| {
        let cap = re.captures(line).unwrap();
        (cap[1].to_string(), cap[2].parse().unwrap())
    }).collect();

    let mut x = 0;
    let mut y = 0;
    let mut dx = 10;
    let mut dy = 1;

    for action in actions {
        let (m ,n) = action;
        //println!("{}{}", m, n);
        match m.as_str()  {
            "N" => {
                dy += n;
            }
            "S" => {
                dy -= n;
            }
            "E" => {
                dx += n;
            }
            "W" => {
                dx -= n;
            }
            "F" => {
                x += dx*n;
                y += dy*n;
            }
            "L" | "R" => {
                if n == 180 {
                    dx *= -1;
                    dy *= -1;
                } else if (m == "L" && n == 90) || (m == "R" && n == 270) {
                    /*
                       (dx, dy)
                       (1,0) aka east becomes (0, 1)
                       (0,1) aka north becomes (-1, 0)
                       (-1,0) aka west becomes (0,-1)
                       (0,-1) aka south becomes (1,0)
                     */
                    if dx == 0 {
                        dx = -dy;
                        dy = 0; //dx
                    } else if dy == 0 {
                        dy = dx;
                        dx = 0; //dy
                    } else {
                        let signx = dx/dx.abs();
                        let signy = dy/dy.abs();
                        if signx == signy {
                            // upper right or lower left quadrants
                            let tmp = dy;
                            dy = dx;
                            dx = -tmp;
                        } else {
                            // upper left or lower right;
                            let tmp = dx;
                            dx = -dy;
                            dy = tmp;
                        }
                    }
                } else if (m == "L" && n == 270) || (m == "R" && n == 90) {
                    if dx == 0 {
                        dx = dy;
                        dy = 0; // dx
                    } else if dy == 0 {
                        dy = -dx;
                        dx = 0; //dy
                    } else {
                        let signx = dx/dx.abs();
                        let signy = dy/dy.abs();
                        if signx == signy {
                            // upper right or lower left quadrants
                            let tmp = dy;
                            dy = -dx;
                            dx = tmp;
                        } else {
                            // upper left or lower right;
                            let tmp = dx;
                            dx = dy;
                            dy = -tmp;
                        }
                    }
                } else {
                    panic!("wtf {} {}", m, n);
                }
            }
            fail => {
                panic!("wtf {} ?", fail);
            }
        }
    }
    println!("{}", x.abs()+y.abs());
}
