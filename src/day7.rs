use anyhow::Result;
use std::collections::HashMap;

fn parse() -> HashMap<String, Vec<(usize, String)>> {
    include_str!("../input/day7.txt")
        .lines()
        .map(|line| {
            line.trim_end_matches('.')
                .replace(" bags", "")
                .replace(" bag", "")
        })
        .filter_map(|line| {
            split_once(&line, " contain ").map(|(k, v)| (k.to_string(), v.to_string()))
        })
        .map(|(key, rest)| match rest.as_ref() {
            "no other" => (key, Vec::new()),
            rest => (
                key,
                rest.split(", ")
                    .filter_map(|input| {
                        split_once(input, " ")
                            .map(|(num, color)| (num.parse::<usize>().unwrap(), color.to_string()))
                    })
                    .collect::<Vec<_>>(),
            ),
        })
        .collect()
}

fn split_once<'a, 'b>(input: &'a str, pat: &'b str) -> Option<(&'a str, &'a str)> {
    let mut s = input.splitn(2, pat);
    s.next().and_then(|n| s.next().map(|v| (n, v)))
}

fn contains_bag(bag: &str, map: &HashMap<String, Vec<(usize, String)>>) -> usize {
    map.keys().filter(|k| contains_gold(k, &map)).count() - 1
}

fn contains_gold(k: &str, map: &HashMap<String, Vec<(usize, String)>>) -> bool {
    k == "shiny gold" || map[k].iter().any(|(_, bag)| contains_gold(bag, map))
}

fn count_bags(bag: &str, map: &HashMap<String, Vec<(usize, String)>>) -> usize {
    1 + map[bag]
        .iter()
        .map(|(num, other_bag)| num * count_bags(other_bag, map))
        .sum::<usize>()
}

pub fn compute() {
    let map = parse();

    let one = contains_bag("shiny gold", &map);
    let two = count_bags("shiny gold", &map) - 1;
    println!("p1 {}", one);
    println!("p2 {}", two);
}
