extern crate regex;

use std::env;
use std::fs;
use std::collections::HashMap;

use regex::Regex;
use std::borrow::Borrow;

trait Rule {
    fn to_regex(&self, rule_map: &HashMap<i32, Box<dyn Rule>>) -> String;
}

struct BaseRule { c: char }

impl Rule for BaseRule {
    fn to_regex(&self, _rule_map: &HashMap<i32, Box<dyn Rule>>) -> String {
        self.c.to_string()
    }
}

struct AndRule { rule_ids: Vec<i32>}

fn rule_ids_to_regex(ids: &Vec<i32>, rule_map: &HashMap<i32, Box<dyn Rule>>) -> String {
    ids.iter().map(|id| {
        let rule: &dyn Rule = rule_map[id].borrow();
        rule.to_regex(rule_map)
    }).collect()
}
impl Rule for AndRule {
    fn to_regex(&self, rule_map: &HashMap<i32, Box<dyn Rule>>) -> String {
        rule_ids_to_regex(&self.rule_ids, rule_map)
    }
}

struct OrRule { left_rule_ids: Vec<i32>, right_rule_ids: Vec<i32>}

impl Rule for OrRule {
    fn to_regex(&self, rule_map: &HashMap<i32, Box<dyn Rule>>) -> String {
        format!("(({})|({}))", rule_ids_to_regex(&self.left_rule_ids, rule_map), rule_ids_to_regex(&self.right_rule_ids, rule_map))
    }
}

fn gen_base(line: &str) -> Option<(i32, Box<dyn Rule>)> {
    let base_re: Regex = Regex::new(r#"(\d+): "(.)""#).unwrap();
    base_re.captures(line).map(|cap| {
        //println!("Rule {} is base", &cap[1]);
        let b: Box<dyn Rule> = Box::new(BaseRule { c: cap[2].chars().next().unwrap()});
        (cap[1].parse::<i32>().unwrap(), b)
    })
}

fn gen_and_from_line(line: &str) -> Vec<i32> {
    let ids: Vec<i32> = line.trim().split(' ').map(|c| c.parse().unwrap()).collect();
    //println!("{:?}", &ids);
    ids
}

fn gen_and(line: &str) -> Option<(i32, Box<dyn Rule>)> {
    let and_re = Regex::new(r"(\d+):(( \d+)+)$").unwrap();
    and_re.captures(line).map(|cap| {
        //println!("Rule {} is and", &cap[1]);
        let b: Box<dyn Rule> = Box::new(AndRule { rule_ids: gen_and_from_line(&cap[2])});
        (cap[1].parse::<i32>().unwrap(), b)
    })
}

fn gen_or(line: &str) -> Option<(i32, Box<dyn Rule>)> {
    let or_re = Regex::new(r"(\d+):(.+)\|(.+)").unwrap();
    or_re.captures(line).map(|cap| {
        //println!("Rule {} is or", &cap[1]);
        let b: Box<dyn Rule> = Box::new(OrRule { left_rule_ids: gen_and_from_line(&cap[2]), right_rule_ids: gen_and_from_line(&cap[3])});
        (cap[1].parse::<i32>().unwrap(), b)
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut rule_map: HashMap<i32, Box<dyn Rule>> = HashMap::new();

    let mut i = 0;
    while lines[i].len() > 0 {
        gen_base(lines[i]).map(|b| {
            rule_map.insert(b.0, b.1);
        });
        gen_and(lines[i]).map(|b| {
            rule_map.insert(b.0, b.1);
        });
        gen_or(lines[i]).map(|b| {
            rule_map.insert(b.0, b.1);
        });
        i += 1;
    }
    let built = rule_map[&0].to_regex(&rule_map);
    println!("{}", built);
    let re = Regex::new(format!("^{}$", built.as_str()).as_str()).unwrap();
    i += 1;

    let mut match_count = 0;
    let mut miss_count = 0;
    while i < lines.len() {
        if re.is_match(lines[i]) {
            match_count += 1;
        } else {
            miss_count += 1;
        }
        i += 1;
    }
    println!("{} {}", match_count, miss_count);
}
