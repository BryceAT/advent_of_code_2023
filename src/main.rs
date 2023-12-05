#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::{fs,env};
use std::error::Error;
use reqwest;
use soup::prelude::*;

fn get_text(day: i32,sample:bool,part:usize) -> Result<String, Box<dyn Error>> {
    let path = format!("data/day{day}.txt");
    let sample_path = format!("data/day{day}sample{part}.txt");
    let year = 2023;
    match sample {
        false => {
            if let Ok(text) = fs::read_to_string(path.clone()) { return Ok(text)}
            let url = format!("https://adventofcode.com/{year}/day/{day}/input");
            let text = reqwest::blocking::Client::new().get(url).header("cookie",format!("session={}",env::var("AOC_SESSION").unwrap())).send()?.text()?.trim().to_string();
            fs::write(path, text.clone())?;
            Ok(text)
        },
        true => {
            if let Ok(text) = fs::read_to_string(sample_path.clone()) { return Ok(text) }
            let url = format!("https://adventofcode.com/{year}/day/{day}");
            let html_text = reqwest::blocking::Client::new().get(url).header("cookie",format!("session={}",env::var("AOC_SESSION").unwrap())).send()?.text()?;
            let text = &Soup::new(html_text.as_str()).tag("pre").find_all().map(|tag| {tag.text().trim().to_string()}).nth(part - 1).unwrap();
            fs::write(sample_path, text.clone())?;
            Ok(text.clone())
        }  
    }
}

fn day1() -> Result<(), Box<dyn Error>> {
    let text = get_text(1,true,2).unwrap();
    println!("part 1: {:?}", text.split('\n').map(|line| 
        line.chars().filter(|c| c.is_numeric()).next().unwrap_or('0').to_digit(10_u32).unwrap() as i32 * 10 + 
        line.chars().rev().filter(|c| c.is_numeric()).next().unwrap_or('0').to_digit(10_u32).unwrap() as i32
    ).sum::<i32>());
    println!("part 2: {:?}", text.split('\n').map(|line| {
        let mut lbest = (line.len(),0);
        let mut rbest = (0,0);
        for (i,num) in ["one","two","three","four","five","six","seven","eight","nine"].iter().enumerate() {
            if let Some(p) = line.find(num) {
                if p < lbest.0 {lbest = (p,i + 1);}
            }
            if let Some(p) = line.rfind(num) {
                if p > rbest.0 {rbest = (p,i + 1);}
            }
        }
        for (i,num) in ["0","1","2","3","4","5","6","7","8","9"].iter().enumerate() {
            if let Some(p) = line.find(num) {
                if p < lbest.0 {lbest = (p,i);}
            }
            if let Some(p) = line.rfind(num) {
                if p > rbest.0 { rbest = (p,i);}
            }
        }
        lbest.1 * 10 + rbest.1
    }
    ).sum::<usize>());
    Ok(())
}
fn day2() -> Result<(), Box<dyn Error>> {
    let text = get_text(2,false,1)?;
    fn max_color(mut line: String) -> (i32,i32,i32,i32) {
        let tail = line.split_off(line.find(':').unwrap() + 1);
        let game = line[..line.len() -1].split_whitespace().skip(1).next().unwrap().parse::<i32>().unwrap();
        let (mut red, mut green, mut blue) = (0,0,0);
        for hand in tail.split(';') {
            for pair in hand.split(',') {
                match pair.split_whitespace().collect::<Vec<&str>>()[..] {
                    [x, "red"] => red = red.max(x.parse::<i32>().unwrap()),
                    [x, "green"] => green = green.max(x.parse::<i32>().unwrap()),
                    [x, "blue"] => blue = blue.max(x.parse::<i32>().unwrap()),
                    _ => unreachable!("invald pair: {:?}", pair.split_whitespace().collect::<Vec<&str>>()),
                }
            }
        }
        (game,red,green,blue)
    }
    println!("part 1: {:?}", text.split('\n').filter_map(|line| {
        let (game,red,green,blue) = max_color(line.to_string());
        if red <= 12 && green <= 13 && blue <= 14 {
            Some(game)
        } else {None}
    }).sum::<i32>());
    println!("part 2: {:?}", text.split('\n').map(|line| {
        let (_,red,green,blue) = max_color(line.to_string());
        red * green * blue
    }).sum::<i32>());
    Ok(())
}
fn day3() {
    let text = get_text(3, false, 1).unwrap();
    let mut symbols = HashMap::new();
    for (r,row) in text.split('\n').enumerate() {
        for (c, x) in row.trim().chars().enumerate() {
            match x {
                '0'..='9' => (),
                '.' => (),
                _ => {symbols.insert((r as i32,c as i32),x);},
            }
        }
    }
    //println!("{symbols:?}");
    let mut tot = 0;
    let mut cur = 0;
    let mut has_adj = false;
    for (r,row) in text.split('\n').enumerate() {
        for (c,x) in row.trim().chars().chain(std::iter::once('.')).enumerate() {
            match x {
                '0'..='9' => {
                    cur = cur * 10 + x.to_digit(10).unwrap() as i32; 
                    let (r,c) = (r as i32, c as i32);
                    has_adj = has_adj || [(r-1,c-1),(r,c-1),(r+1,c-1),(r-1,c),(r+1,c),(r-1,c+1),(r,c+1),(r+1,c+1)].iter()
                    .any(|p| symbols.contains_key(p));
                },
                _ if cur > 0 => {if has_adj {tot += cur;}; cur = 0; has_adj = false;},
                _ => (),
            }
        }
    }
    println!("part 1: {}",tot);
    let mut gear_parts:HashMap<(i32,i32),Vec<i32>> = HashMap::new();
    let mut k = None;
    for (r,row) in text.split('\n').enumerate() {
        for (c,x) in row.trim().chars().chain(std::iter::once('.')).enumerate() {
            match x {
                '0'..='9' => {
                    cur = cur * 10 + x.to_digit(10).unwrap() as i32; 
                    let (r,c) = (r as i32, c as i32);
                    for p in [(r-1,c-1),(r,c-1),(r+1,c-1),(r-1,c),(r+1,c),(r-1,c+1),(r,c+1),(r+1,c+1)] {
                        if symbols.get(&p).unwrap_or(&'#') == &'*' {
                            k = Some(p);
                        }
                    }
                },
                _ if cur > 0 => {if let Some(gear) = k {gear_parts.entry(gear).or_default().push(cur)}; cur = 0; k = None;},
                _ => (),
            }
        }
    }
    //println!("{gear_parts:?}");
    println!("part 2: {}", gear_parts.values().filter_map(|v| if v.len() > 1 {Some((v[0] * v[1]) as i64)} else {None}).sum::<i64>());
}
fn day4() {
    let text = get_text(4,false,1).unwrap();
    let mut tot = 0;
    for card in text.split('\n') {
        let [_, win_nums, my_nums] = card.split(&[':','|']).collect::<Vec<&str>>()[..3] else {unreachable!("malformed card")}; 
        let win_nums:HashSet<i32> = win_nums.split_whitespace().filter_map(|n| n.parse::<i32>().ok()).collect();
        let match_num = my_nums.split_whitespace().filter_map(|n| n.parse::<i32>().ok()).filter(|n| win_nums.contains(n)).count();
        if match_num > 0 {
            tot += 2_i32.pow(match_num as u32 -1);
        }
    }
    println!("part 1: {tot}");
    let mut card_cnt = vec![1; text.matches('\n').count() +1];
    for (i,card) in text.split('\n').enumerate() {
        let [_, win_nums, my_nums] = card.split(&[':','|']).collect::<Vec<&str>>()[..3] else {unreachable!("malformed card")}; 
        let win_nums:HashSet<i32> = win_nums.split_whitespace().filter_map(|n| n.parse::<i32>().ok()).collect();
        let match_num = my_nums.split_whitespace().filter_map(|n| n.parse::<i32>().ok()).filter(|n| win_nums.contains(n)).count();
        for j in i+1 ..= i + match_num {
            card_cnt[j] += card_cnt[i];
        }
    }
    println!("part 2: {}", card_cnt.into_iter().sum::<i64>());
}
fn day5() {
    let text = get_text(5,false,1).unwrap();
    let mut it = text.split('\n');
    let mut stuff:VecDeque<i64> = it.next().unwrap().split(":").nth(1).unwrap().split_whitespace().filter_map(|x| x.parse::<i64>().ok()).collect();
    let mut stuff_out = Vec::new();
    for row in it.chain(std::iter::once("")) {
        if row.is_empty() {
            for x in stuff_out {stuff.push_back(x)}
            stuff_out = Vec::new();
        } else if let [dest,source,len] = row.split_whitespace().filter_map(|x| x.parse::<i64>().ok()).collect::<Vec<i64>>()[..] {
            for _ in 0..stuff.len() {
                if let Some(x) = stuff.pop_front() {
                    if x >= source && x < source + len {
                        stuff_out.push(x + dest - source);
                    } else {
                        stuff.push_back(x);
                    }
                }
            }
        }
    }
    println!("part 1: {}", stuff.into_iter().min().unwrap());
    let mut it = text.split('\n');
    let mut ranges:VecDeque<[i64;2]> = it.next().unwrap().split(":").nth(1).unwrap().split_whitespace().filter_map(|x| x.parse::<i64>().ok()).collect::<Vec<_>>().chunks(2).map(|v| [v[0],v[1]]).collect();
    let mut ranges_out = Vec::new();
    for row in it.chain(std::iter::once("")) {
        if row.is_empty() {
            for x in ranges_out {ranges.push_back(x)};
            ranges_out = Vec::new();
        } else if let [target,source,len] = row.split_whitespace().filter_map(|x| x.parse::<i64>().ok()).collect::<Vec<i64>>()[..] {
            for _ in 0..ranges.len() {
                if let Some([a,dist]) = ranges.pop_front() {
                    if a + dist <= source || source + len <= a {
                        ranges.push_back([a,dist]);
                    } else {
                        if a < source { ranges.push_back([a,source - a]); }
                        ranges_out.push([a.max(source) + target - source, (a + dist).min(source + len) - a.max(source)]);
                        if a + dist > source + len { ranges.push_back([source + len, a + dist - source - len])}
                    }
                }
            }
        }
    }
    println!("part 2: {}", ranges.iter().map(|&[a,_]| a).min().unwrap());
}
fn main() {
    let _ = day5();
}
