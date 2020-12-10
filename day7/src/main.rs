extern crate regex;

use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::borrow::Borrow;

fn recurse(current: &String, mapping: &HashMap<String, Box<Vec<(String, i32)>>>, visiting: &mut HashSet<String>) {
    mapping.get(current).map(|nextt| {
        let vec: &Vec<(String, i32)> = nextt.borrow();
        vec.iter().for_each(|tuple| {
           let (next, count) = tuple;
            visiting.insert(next.clone());
            recurse(next, mapping, visiting);
        });
    });
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let re = Regex::new(r"(.+) bags contain (\d.+)\.").unwrap();
    let inner = Regex::new(r"(\d+) (.*) bags?").unwrap();

    let mut fitsIn: HashMap<String, Box<Vec<(String, i32)>>> = HashMap::new();

    input.lines().for_each(|line| {
        re.captures(line).map(|cap| {
            //println!("parsing {}", line);
            cap[2].split(",").for_each(|b| {
                let innercap = inner.captures(b).unwrap();
                fitsIn.entry(innercap[2].to_string()).or_insert(Box::new(Vec::new()))
                    .push((cap[1].to_string(), innercap[1].parse::<i32>().unwrap()));
                //println!("from {}, {} of them would fit in {}", innercap[2].to_string(), innercap[1].parse::<i32>().unwrap(), cap[1].to_string());
            });
        });
    });

    let mut set: HashSet<String> = HashSet::new();
    recurse(&"shiny gold".to_string(), &fitsIn, &mut set);

    println!("{}", set.len());
}
