#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::fmt::{LowerHex, format};
use std::process::Child;
use std::{fs,env};
use std::error::Error;
//use reqwest;
use soup::prelude::*;
use std::time::Instant;
use regex::Regex;
use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::cmp::Reverse;
use std::fs::File;
use std::io::Write;

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
    let text = get_text(5,true,1).unwrap();
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
fn day6() {
    let text = get_text(6,true,1).unwrap();
    let mut it = text.split('\n');
    let time:Vec<_> = it.next().unwrap().split(':').nth(1).unwrap().split_whitespace().filter_map(|x| x.parse::<f64>().ok()).collect();
    let distance:Vec<_> = it.next().unwrap().split(':').nth(1).unwrap().split_whitespace().filter_map(|x| x.parse::<f64>().ok()).collect();
    fn num_posible(t:f64,d:f64) -> i64 {
        let mut left = ((t - (t*t - 4.0 * d).sqrt())/2.0).ceil() as i64;
        for (i,j) in (-3..=3).map(|x| left + x).collect::<Vec<_>>().windows(2).map(|v| (v[0],v[1])) {
            if i * (t as i64 - i) <= d as i64 && j * (t as i64 - j) > d as i64 {
                left = j as _;
                break
            }
        }
        let mut right = ((t + (t*t - 4.0 * d).sqrt())/2.0).floor() as i64;
        for (i,j) in (-3..=3).map(|x| right + x).collect::<Vec<_>>().windows(2).map(|v| (v[0],v[1])) {
            if i * (t as i64 - i) > d as i64 && j * (t as i64 - j) <= d as i64 {
                right = i as _;
                break
            }
        }
        1 + right - left
    }
    println!("part 1: {:?}",time.into_iter().zip(distance.into_iter()).map(|(t,d)| num_posible(t,d)).fold(1,|acc,x| acc * x));
    let mut it = text.split('\n');
    let time:f64 = it.next().unwrap().split(':').nth(1).unwrap().bytes().fold(0.0,|tot,x| match x {b'0'..=b'9' => tot * 10.0 + (x-b'0') as f64,_=>tot});
    let distance:f64 = it.next().unwrap().split(':').nth(1).unwrap().bytes().fold(0.0,|tot,x| match x {b'0'..=b'9' => tot * 10.0 + (x-b'0') as f64,_=>tot});
    println!("part 2: {:?}", num_posible(time,distance));
}
fn day7() {
    let text = get_text(7,false,1).unwrap();
    let mut num_rank = HashMap::new();
    for (i,b) in [b'A',b'K',b'Q',b'J',b'T',b'9',b'8',b'7',b'6',b'5',b'4',b'3',b'2',b'1'].into_iter().enumerate() {
        num_rank.insert(b,i as i32);
    }
    fn num_ranks(hand: &[u8],num_rank: &HashMap<u8,i32>) -> (i32,i32,i32,i32,i32) {
        (num_rank[&hand[0]],num_rank[&hand[1]],num_rank[&hand[2]],num_rank[&hand[3]],num_rank[&hand[4]])
    }
    fn count_hand(hand: &[u8], num_rank: &HashMap<u8,i32>) -> Vec<i32> {
        let mut ans = vec![0;14];
        for b in hand {
            ans[num_rank[b] as usize] += 1;
        }
        ans.sort_unstable();
        ans.into_iter().filter(|&x| x > 0).collect()
    }
    fn rank_map(hand: &str, num_rank: &HashMap<u8,i32>) -> [i32;6] {
        match (&count_hand(hand.as_bytes(),num_rank)[..], num_ranks(hand.as_bytes(),num_rank)) {
            ([5,],(a,b,c,d,e)) => [1,a,b,c,d,e],
            ([1,4],(a,b,c,d,e)) => [2,a,b,c,d,e],
            ([2,3],(a,b,c,d,e)) => [3,a,b,c,d,e],
            ([1,1,3],(a,b,c,d,e)) => [4,a,b,c,d,e],
            ([1,2,2],(a,b,c,d,e)) => [5,a,b,c,d,e],
            ([1,1,1,2],(a,b,c,d,e)) => [6,a,b,c,d,e],
            (_,(a,b,c,d,e)) => [7,a,b,c,d,e],
        }
    }
    let mut hands = text.split('\n')
    .map(|row| (rank_map(row.split_whitespace().nth(0).unwrap(),&num_rank),
                    row.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap())
                ).collect::<Vec<_>>();
    hands.sort_unstable_by_key(|(a,_)| a.clone());
    //println!("{hands:?}");
    let n = hands.len() as i32;
    println!("part 1:{:?}",hands.into_iter().enumerate().map(|(i,(_,val))| val * (n - i as i32)).sum::<i32>());
    num_rank.insert(b'J',14);
    fn count_hand_wild(hand: &[u8],num_rank: &HashMap<u8,i32>) -> Vec<i32> {
        let mut ans = vec![0;15];
        for b in hand {
            ans[num_rank[b] as usize] += 1;
        }
        let wild = ans[14];
        ans[14] = 0;
        ans.sort_unstable();
        let mut ans:Vec<i32> = ans.into_iter().filter(|&x| x > 0).collect();
        if ans.is_empty() {return vec![5]}
        *ans.last_mut().unwrap() += wild;
        ans
    }
    fn rank_map_wild(hand: &str, num_rank: &HashMap<u8,i32>) -> [i32;6] {
        match (&count_hand_wild(hand.as_bytes(),num_rank)[..], num_ranks(hand.as_bytes(),num_rank)) {
            ([5,],(a,b,c,d,e)) => [1,a,b,c,d,e],
            ([1,4],(a,b,c,d,e)) => [2,a,b,c,d,e],
            ([2,3],(a,b,c,d,e)) => [3,a,b,c,d,e],
            ([1,1,3],(a,b,c,d,e)) => [4,a,b,c,d,e],
            ([1,2,2],(a,b,c,d,e)) => [5,a,b,c,d,e],
            ([1,1,1,2],(a,b,c,d,e)) => [6,a,b,c,d,e],
            (_,(a,b,c,d,e)) => [7,a,b,c,d,e],
        }
    }
    let mut hands = text.split('\n')
    .map(|row| (rank_map_wild(row.split_whitespace().nth(0).unwrap(),&num_rank),
                    row.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap())
                ).collect::<Vec<_>>();
    hands.sort_unstable_by_key(|(a,_)| a.clone());
    println!("part 2:{:?}",hands.into_iter().enumerate().map(|(i,(_,val))| val * (n - i as i32)).sum::<i32>());
}
fn day8() {
    let text = get_text(8,false,3).unwrap();
    let mut it = text.split('\n');
    let dirs = it.next().unwrap().as_bytes();
    it.next();
    let mut map = HashMap::new();
    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    for row in it {
        let Some((_,[key,left,right])) = re.captures(row).map(|cap| cap.extract()) else {unreachable!("bad parse")};
        map.insert(key.to_string(),(left.to_string(),right.to_string()));
        //AAA = (BBB, CCC)
        //map.insert(row.split(" = ").nth(0).unwrap().to_string(),(row.split('(').nth(1).unwrap()[..3].to_string(),row.split(", ").nth(1).unwrap()[..3].to_string()));
    }
    let mut step = 0;
    let mut cur = "AAA".to_string();
    while cur != "ZZZ".to_string() {
        for d in dirs.iter() {
            step += 1;
            cur = if d == &b'L' {map.get(&cur).unwrap().0.clone()} else {map.get(&cur).unwrap().1.clone()};
        }
    }
    println!("part 1: {:?}",step); 
    let mut cur_list = VecDeque::new();
    for key in map.keys() {
        if key.ends_with('A') {cur_list.push_back(key.to_string());}
    }
    let mut step_times = HashMap::new();
    let mut times = HashSet::new();
    for i in 0..cur_list.len() {
        let mut cur = cur_list[i].clone();
        step = 0;
        loop {
            while !cur.ends_with('Z') {
                for d in dirs.iter() {
                    step += 1;
                    if *d == b'L' {
                        cur = map.get(&cur).unwrap().0.clone();
                    } else {
                        cur = map.get(&cur).unwrap().1.clone();
                    }
                }
            }
            if !times.insert(step) {
                step_times.insert(cur_list[i].clone(),times);
                times = HashSet::new();
                break
            }
        }
    }
    fn gcd(mut x:i64,mut y:i64) -> i64 {
        if x > y {(x,y) = (y,x);}
        if x == 0 {return y}
        gcd(y%x,x)
    }
    fn lcm(x:i64,y:i64) -> i64 {
        x * y / gcd(x,y)
    }
    println!("part 2: {:?}",step_times.values().map(|set| *set.into_iter().next().unwrap()).fold(1,|lmult,cur| lcm(lmult,cur)));
}
fn day9() {
    let text = get_text(9,false,1).unwrap();
    let it = text.split('\n').map(|row| get_last_val(&row.split_whitespace().filter_map(|x| x.parse::<i64>().ok()).collect::<Vec<i64>>()));
    fn get_last_val(v: &[i64]) -> i64 {
        if v.iter().all(|x| *x == 0) {return 0}
        let nxt:Vec<i64> = v.windows(2).map(|pair| pair[1] - pair[0]).collect();
        v.last().unwrap() + get_last_val(&nxt)
    }
    println!("part 1: {:?}",it.sum::<i64>());
    fn get_first_val(v: &[i64]) -> i64 {
        if v.iter().all(|x| *x == 0) {return 0}
        let nxt:Vec<i64> = v.windows(2).map(|pair| pair[1] - pair[0]).collect();
        v[0] - get_first_val(&nxt)
    }
    let it_fisrt = text.split('\n').map(|row| get_first_val(&row.split_whitespace().filter_map(|x| x.parse::<i64>().ok()).collect::<Vec<i64>>()));
    println!("part 2: {:?}",it_fisrt.sum::<i64>());
}
fn day10() {
    let text = get_text(10,false,15).unwrap();
    // north east south west
    let map = HashMap::from([
        ('|',[true,false,true,false]),
        ('-',[false,true,false,true]),
        ('L',[true,true,false,false]),
        ('J',[true,false,false,true]),
        ('7',[false,false,true,true]),
        ('F',[false,true,true,false]),
        ('.',[false,false,false,false]),
        ('S',[true,true,true,true]),
        ('I',[false,false,false,false]),
        ('O',[false,false,false,false]),
        ('#',[false,false,false,false]),
        ]);
    let mut cur = [0,0];
    let grid = text.split('\n').map(|row| row.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    'outer: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                cur = [i,j];
                break 'outer;
            }
        }
    }
    let mut pos = VecDeque::from([cur.clone()]);
    let mut seen = HashSet::from([cur.clone()]);
    let mut step = 0;
    while !pos.is_empty() {
        //println!("{pos:?}");
        step += 1;
        for _ in 0..pos.len() {
            if let Some([i,j]) = pos.pop_front() {
                for (p,(x,y)) in [(if i > 0 {i-1} else {i},j),(i,j+1),(i+1,j),(i,if j >0 {j-1} else {j})].iter().enumerate() {
                    //println!("{x} {y} {} {:?}",grid[i][j],map[&grid[i][j]]);
                    if *x < grid.len() && *y < grid[0].len() && map[&grid[i][j]][p] && map[&grid[*x][*y]][(p+2)%4] && seen.insert([*x,*y]) {
                        pos.push_back([*x,*y]);
                    }
                }
            }
        }
        if pos.len() == 1 {
            println!("part 1: {step}");
            break
        }
    }
    let mut big_grid = vec![vec!['.';grid[0].len() * 2];grid.len() * 2];
    for &[x,y] in &seen {
        big_grid[2*x][2*y] = grid[x][y];
    }
    //println!("init big_grid");
    //for row in &big_grid {  println!("{row:?}"); }
    let mut start = [cur[0] * 2, cur[1] * 2];
    for _ in 0..=seen.len() {
        let [i,j] = start;
        //println!("{start:?}");
        for (p,(x,y)) in [(i.checked_sub(2).unwrap_or(usize::MAX),j),(i,j+2),(i+2,j),(i,j.checked_sub(2).unwrap_or(usize::MAX))].into_iter().enumerate() {
            //println!("{x} {y} {} {:?}",grid[i][j],map[&grid[i][j]]);
            if x < big_grid.len() && y < big_grid[0].len() && map[&big_grid[i][j]][p] && map[&big_grid[x][y]][(p+2)%4] && big_grid[(i+x)/2][(j+y)/2] == '.' {
                big_grid[(i+x)/2][(j+y)/2] = '#';
                start = [x,y];
                break
            }
        }
    }
    //for row in &grid { println!("{row:?}"); }
    //for row in &big_grid {  println!("{row:?}"); }
    fn flood(i:usize,j:usize,big_grid: &mut [Vec<char>]) {
        big_grid[i][j] = 'O';
        if i > 0 && big_grid[i-1][j] == '.' {flood(i-1,j,big_grid)}
        if j > 0 && big_grid[i][j-1] == '.' {flood(i,j-1,big_grid)}
        if i + 1 < big_grid.len() && big_grid[i+1][j] == '.' {flood(i+1,j,big_grid)}
        if j + 1 < big_grid[0].len() && big_grid[i][j+1] == '.' {flood(i,j+1,big_grid)}
    }
    //flood from boundary
    for col in (0..big_grid[0].len()).rev() {
        if big_grid[0][col] == '.' {flood(0,col,&mut big_grid)}
        //last column is all '.'
    }
    for r in 0..big_grid.len() {
        if big_grid[r][0] == '.' {flood(r,0,&mut big_grid)}
        //last row is all '.'
    }
    //for row in &big_grid {  println!("{row:?}"); }
    let mut part2 = 0;
    for i in (0..big_grid.len()).step_by(2) {
        for j in (0..big_grid[0].len()).step_by(2) {
            if big_grid[i][j] == '.' {part2 += 1;}
        }
    }
    println!("part 2: {part2}");
}
fn day11() {
    let text = get_text(11,false,1).unwrap();
    let mut extra_cols: Vec<usize> = (0..text.split('\n').next().unwrap().len()).collect();
    let mut extra_rows = Vec::new();
    for (r,row) in text.split('\n').enumerate() {
        for (c,x) in row.chars().enumerate() {
            if x == '#' && extra_cols.contains(&c) {extra_cols.retain(|y| *y != c);}
        }
        if row.chars().all(|x| x == '.') {extra_rows.push(r);}
    }
    let mut points = Vec::new();
    let mut points2 = Vec::new();
    for (r,row) in text.split('\n').enumerate() {
        for (c,x) in row.chars().enumerate() {
            if x == '#' {
                points.push((
                    r + match extra_rows.binary_search(&r) {Ok(j)=> j,Err(j) => j},
                    c + match extra_cols.binary_search(&c) {Ok(j)=> j,Err(j) => j}
                ));
                points2.push((
                    r + match extra_rows.binary_search(&r) {Ok(j)=> j,Err(j) => j} * 999_999,
                    c + match extra_cols.binary_search(&c) {Ok(j)=> j,Err(j) => j} * 999_999
                ));
            }
        }
    }
    //println!("{points:?}");
    let mut p1 = 0;
    let mut p2 = 0;
    for i in 1..points.len() {
        for j in 0..i {
            p1 += (points[i].0 as i64 - points[j].0 as i64).abs() + (points[i].1 as i64 - points[j].1 as i64).abs();
            p2 += (points2[i].0 as i64 - points2[j].0 as i64).abs() + (points2[i].1 as i64 - points2[j].1 as i64).abs();
        }
    }
    println!("part 1: {}",p1);
    println!("part 2: {}",p2);
}
fn day12() {
    let text = get_text(12,false,2).unwrap();
    fn dfs(h: &[u8], nums: &[i32], mem: &mut HashMap<(Vec<u8>,Vec<i32>),i64>) -> i64 {
        if h.is_empty() {return if nums.is_empty() {1} else {0}}
        if nums.is_empty() {return if h.contains(&b'#') {0} else {1}}
        if let Some(ans) = mem.get(&(h.to_vec(),nums.to_vec())) {return *ans}
        let n = nums[0] as usize;
        if (h.len() as i32) < nums.iter().sum::<i32>() {return 0}
        let mut ans = 0;
        for i in 0..=h.len() - n {
            if i > 0 && h[i-1] == b'#' {break}
            if h[i..i+n].iter().all(|b| *b != b'.') {
                if h.len() == i + n {
                    ans += if nums.len() == 1 {1} else {0};
                } else if h[n+i] == b'#' {
                    ()
                } else {
                    ans += dfs(&h[i+n + 1..],&nums[1..],mem);
                }
            }
        }
        mem.insert((h.to_vec(),nums.to_vec()),ans);
        ans
    }
    let mut mem:HashMap<(Vec<u8>,Vec<i32>),i64> = HashMap::new();
    fn count_pos(row: &str, mem: &mut HashMap<(Vec<u8>,Vec<i32>),i64>) -> i64 {
        let (h,nums) = (row.split(' ').next().unwrap().as_bytes(),
            row.split(' ').nth(1).unwrap().split(',').filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>());
        dfs(h,&nums, mem)
    }
    fn count_pos_5(row: &str, mem: &mut HashMap<(Vec<u8>,Vec<i32>),i64>) -> i64 {
        let (h,nums) = (row.split(' ').next().unwrap().as_bytes(),
            row.split(' ').nth(1).unwrap().split(',').filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>());
        let len = nums.len() * 5;
        let mut h5 = Vec::new();
        for _ in 0..5 {
            for x in h {
                h5.push(*x);
            }
            h5.push(b'?');
        }
        h5.pop();
        dfs(&h5, &nums.into_iter().cycle().take(len).collect::<Vec<i32>>(), mem)
    }
    println!("part 1: {:?}",text.split('\n').map(|row| count_pos(row, &mut mem)).sum::<i64>());
    println!("part 2: {:?}",text.split('\n').map(|row| count_pos_5(row, &mut mem)).sum::<i64>());
}
fn day13() {
    let text = get_text(13, false, 1).unwrap();
    let mut boards = Vec::new();
    let mut cur = Vec::new();
    for row in text.split('\n').into_iter().chain(std::iter::once("")) {
        if row.is_empty() {
            boards.push(cur);
            cur = Vec::new();
        } else {
            cur.push(row);
        }
    }
    fn count_rows(board:&[&str]) -> i64 {
        //println!("{board:?}");
        for i in 1..board.len() {
            if board[..i].iter().rev().zip(board[i..].iter()).all(|(a,b)| a == b) {
                return (100 * i) as _
            }
        }
        let mut trans = vec![vec![0;board.len()];board[0].len()];
        for (i,row) in board.into_iter().enumerate() {
            for (j,b) in row.bytes().enumerate() {
                trans[j][i] = b;
            }
        }
        for i in 1..trans.len() {
            if trans[..i].iter().rev().zip(trans[i..].iter()).all(|(a,b)| a == b) {
                return i as _
            }
        }
        //unreachable!("no reflection found {trans:?}")
        0
    }
    println!("part 1: {:?}",boards.iter().map(|board| count_rows(&board)).sum::<i64>());
    fn count_rows2(board:&[&str]) -> i64 {
        //println!("{board:?}");
        for i in 1..board.len() {
            if board[..i].iter().rev().zip(board[i..].iter())
                .map(|(&a,&b)| a.bytes().zip(b.bytes()).map(|(x,y)| if x == y {0} else {1}).sum::<i64>() ).sum::<i64>() == 1 {
                return (100 * i) as _
            }
        }
        let mut trans = vec![vec![0;board.len()];board[0].len()];
        for (i,row) in board.into_iter().enumerate() {
            for (j,b) in row.bytes().enumerate() {
                trans[j][i] = b;
            }
        }
        for i in 1..trans.len() {
            if trans[..i].iter().rev().zip(trans[i..].iter())
                .map(|(a,b)| a.iter().zip(b.iter()).map(|(x,y)| if x == y {0} else {1}).sum::<i64>() ).sum::<i64>() == 1 {
                return i as _
            }
        }
        //unreachable!("no reflection found {trans:?}")
        0
    }
    println!("part 2: {:?}",boards.iter().map(|board| count_rows2(&board)).sum::<i64>());
}
fn day14() {
    let text = get_text(14, false, 1).unwrap();
    let mut grid:Vec<Vec<char>> = Vec::new();
    for row in text.split('\n') {
        grid.push(row.chars().collect());
    }
    let n = grid.len();
    let mut ans = 0_i64;
    for c in 0..n {
        let mut support = 0;
        for r in 0..n {
            match grid[r][c] {
                '#' => support = r as i64 + 1,
                'O' => {ans += n as i64 - support; support += 1;},
                _ => (),
            }
        }
    }
    println!("part 1: {:?}",ans);
    fn slide_up(grid: &mut Vec<Vec<char>>,n:usize) {
        for c in 0..n {
            let mut support = 0;
            for r in 0..n {
                match grid[r][c] {
                    '#' => support = r + 1,
                    'O' if r > support => {grid[support][c] = 'O'; support += 1; grid[r][c] = '.';},
                    'O' => support += 1,
                    _ => (),
                }
            }
        }
    }
    fn score(grid:&Vec<Vec<char>>,n:usize) -> i64 {
        let mut ans = 0;
        for c in 0..n {
            for r in 0..n {
                if grid[r][c] == 'O' {ans += n - r;}
            }
        }
        ans as _
    }
    fn rotate(grid:&mut Vec<Vec<char>>,n:usize) {
        //flip
        let (mut t, mut b) = (0,n-1);
        while t < b {
            let temp = grid[t].clone();
            grid[t] = grid[b].clone();
            grid[b] = temp;
            t +=1; b -= 1;
        }
        //transpose
        for r in 1..n {
            for c in 0..r {
                let temp = grid[r][c];
                grid[r][c] = grid[c][r];
                grid[c][r] = temp;
            }
        }
    }
    fn cycle(grid: &mut Vec<Vec<char>>,n:usize) {
        for _ in 0..4 {
            slide_up(grid,n);
            rotate(grid,n);
        }
    }
    let mut mem = HashMap::new();
    let mut i = 0;
    while i < 1_000_000_000 {
        cycle(&mut grid,n);
        i += 1;
        if let Some(s) = mem.get(&grid) {
            let jump = i - *s ;
            while i + jump < 1_000_000_000 {
                i += jump;
            }
            break
        } else {
            mem.insert(grid.clone(),i);
        }
    }
    while i < 1_000_000_000 {
        cycle(&mut grid,n);
        i += 1;
    }
    //slide_up(&mut grid, n);
    //slide_up(&mut grid, n);
    //for r in 0..n {println!("{:?}",grid[r].iter().collect::<String>());}
    println!("part 2: {:?}",score(&grid,n));
}
fn day15() {
    let text = get_text(15, false, 1).unwrap();
    fn proc(s:&str) -> i64 {
        s.bytes().fold(0_i64,|cur,x| ((cur + x as i64) * 17) % 256)
    }
    println!("part 1: {:?}",text.split(',').map(proc).sum::<i64>());
    let mut map:Vec<Vec<(Vec<u8>,i64)>> = vec![vec![];256];
    for s in text.split(',') {
        let mut key: Vec<u8> = s.bytes().collect();
        if let Some(d) = key.pop() {
            if d == b'-' {
                let i = proc(&s[..s.len() -1]) as usize;
                if let Some(j) = map[i].iter().position(|x| x.0 == key) {
                    map[i].remove(j);
                }
            } else {
                key.pop();
                let i = proc(&s[..s.len() -2]) as usize;
                if let Some(j) = map[i].iter().position(|x| x.0 == key) {
                    map[i][j] = (key.clone(),(d - b'0') as i64);
                } else {
                    map[i].push((key.clone(),(d - b'0') as i64));
                }
            }
        }
    }
    println!("part 2: {:?}",map.into_iter().enumerate().map(|(i,v)| (i+1) as i64 *v.iter().enumerate().map(|(j,x)| (j+1) as i64 *x.1).sum::<i64>()).sum::<i64>());
}
fn day16() {
    let text = get_text(16, false, 1).unwrap();
    let mut grid = Vec::new();
    for row in text.split('\n') {
        grid.push(row.chars().collect::<Vec<_>>());
    }
    #[derive(PartialEq, Eq, Hash, Clone)]
    enum Directions {
        Up,Down,Left,Right,
    }
    
    fn nxt(t:(usize,usize,Directions),tile:char,size:usize) -> [Option<(usize, usize, Directions)>;2] {
        match (t.2.clone(),tile) {
            (Directions::Up,'.') => if let Some(x) = t.0.checked_sub(1) {[Some((x,t.1,t.2)),None]} else {[None,None]},
            (Directions::Down,'.') => if t.0 + 1 < size { [Some((t.0 + 1,t.1,t.2)),None]} else {[None,None]},
            (Directions::Left,'.') => if let Some(y) = t.1.checked_sub(1) {[Some((t.0,y,t.2)),None]} else {[None,None]},
            (Directions::Right,'.') => if t.1 + 1 < size {[Some((t.0,t.1 + 1, t.2)),None]} else {[None,None]},
            (Directions::Up,'/') => nxt((t.0,t.1,Directions::Right),'.',size),
            (Directions::Down,'/') => nxt((t.0,t.1,Directions::Left),'.',size),
            (Directions::Left,'/') => nxt((t.0,t.1,Directions::Down),'.',size),
            (Directions::Right,'/') => nxt((t.0,t.1,Directions::Up),'.',size),
            (Directions::Up,'\\') => nxt((t.0,t.1,Directions::Left),'.',size),
            (Directions::Down,'\\') => nxt((t.0,t.1,Directions::Right),'.',size),
            (Directions::Left,'\\') => nxt((t.0,t.1,Directions::Up),'.',size),
            (Directions::Right,'\\') => nxt((t.0,t.1,Directions::Down),'.',size),
            (Directions::Up,'-') => [nxt((t.0,t.1,Directions::Left),'.',size)[0].clone(),nxt((t.0,t.1,Directions::Right),'.',size)[0].clone()],
            (Directions::Down,'-') => [ nxt((t.0,t.1,Directions::Left),'.',size)[0].clone(),(nxt((t.0,t.1,Directions::Right),'.',size))[0].clone()],
            (Directions::Left,'-') => nxt((t.0,t.1,Directions::Left),'.',size),
            (Directions::Right,'-') => nxt((t.0,t.1,Directions::Right),'.',size),
            (Directions::Up,'|') => nxt((t.0,t.1,Directions::Up),'.',size),
            (Directions::Down,'|') => nxt((t.0,t.1,Directions::Down),'.',size),
            (Directions::Left,'|') => [ nxt((t.0,t.1,Directions::Up),'.',size)[0].clone(),nxt((t.0,t.1,Directions::Down),'.',size)[0].clone()],
            (Directions::Right,'|') => [ nxt((t.0,t.1,Directions::Up),'.',size)[0].clone(),nxt((t.0,t.1,Directions::Down),'.',size)[0].clone()],
            _ => unreachable!("invalid tile")
        }
    }
    fn how_much_energized(x:usize,y:usize,d:Directions,grid:&[Vec<char>]) -> usize {
        let mut seen = HashSet::new();
        let mut energized = HashSet::new();
        let mut level = VecDeque::new();
        level.push_back((x,y,d.clone()));
        energized.insert((x,y));
        seen.insert((x,y,d));
        while let Some(v) = level.pop_front() {
            for w in nxt(v.clone(),grid[v.0][v.1],grid.len()) {
                if let Some(ww) = w {
                    if seen.insert(ww.clone()) {
                        energized.insert((ww.0,ww.1));
                        level.push_back(ww);
                    }
                }
            }
        }
        energized.len()
    }
    println!("part 1: {:?}",how_much_energized(0,0,Directions::Right,&grid));
    let (sender, receiver) = channel();
    (0..grid.len()).into_par_iter().for_each_with(sender,|s,x| 
        s.send(
            how_much_energized(x,0,Directions::Right,&grid).max(
            how_much_energized(x,grid.len() -1,Directions::Left,&grid)).max(
            how_much_energized(grid.len() - 1,x,Directions::Up,&grid)).max(
            how_much_energized(0,x,Directions::Down,&grid))
        ).unwrap()
    );
    let best: Vec<_> = receiver.iter().collect();
    println!("part 2: {:?}",best.into_iter().max().unwrap());
}
fn day17() {
    let text = get_text(17, true, 1).unwrap();
    let mut grid:Vec<Vec<i64>> = Vec::new();
    for row in text.split('\n') {
        grid.push(row.bytes().map(|b| (b - b'0') as i64).collect::<Vec<i64>>());
    }
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0,0,0,4,0)));
    let mut seen = HashSet::new();
    let bottom_right = (grid.len()-1, grid[0].len() -1);
    while let Some(Reverse((tot,x,y,d,straight_steps))) = heap.pop() {
        if (x,y) == bottom_right  {
            println!("part 1: {:?}",tot); 
            break}
        if seen.insert((x,y,d,straight_steps)) {
            for (d_next,(i,j)) in [(x+1,y),(x.checked_sub(1).unwrap_or(usize::MAX),y),(x,y+1),(x,y.checked_sub(1).unwrap_or(usize::MAX))].into_iter().enumerate() {
                if i < grid.len() && j < grid[0].len() && (d_next != d || straight_steps < 3) && ![(0,1),(1,0),(2,3),(3,2)].contains(&(d,d_next)) {
                    heap.push(Reverse((tot + grid[i][j],i,j,d_next,if d == d_next {straight_steps + 1} else {1})));
                }
            }
        }
    }
    heap.clear();
    heap.push(Reverse((grid[0][1],0,1,2,1)));
    heap.push(Reverse((grid[1][0],1,0,0,1)));
    seen.clear();
    while let Some(Reverse((tot,x,y,d,straight_steps))) = heap.pop() {
        if (x,y) == bottom_right && straight_steps >= 4 {
            println!("part 2: {:?}",tot); 
            break}
        if seen.insert((x,y,d,straight_steps)) {
            for (d_next,(i,j)) in [(x+1,y),(x.checked_sub(1).unwrap_or(usize::MAX),y),(x,y+1),(x,y.checked_sub(1).unwrap_or(usize::MAX))].into_iter().enumerate() {
                if i < grid.len() && j < grid[0].len() && 
                ((d_next != d && straight_steps >= 4) || (d_next == d && straight_steps < 10)) && 
                ![(0,1),(1,0),(2,3),(3,2)].contains(&(d,d_next)) {
                    heap.push(Reverse((tot + grid[i][j],i,j,d_next,if d == d_next {straight_steps + 1} else {1})));
                }
            }
        }
    }
}
fn day18() {
    let text = get_text(18, false, 1).unwrap();
    let mut cur = [0,0];
    let mut tot:i64 = 0;
    let mut prev = text.split('\n').last().unwrap().chars().rev().next().unwrap();
    for row in text.split('\n') {
        let mut it =  row.split_whitespace();
        let d = it.next().unwrap().chars().next().unwrap(); 
        let num = it.next().unwrap().parse::<i64>().unwrap();
        match d {
            'R' => {cur[0] += num; tot += num*cur[1]; if prev == 'D' {tot -= 2};},
            'D' => {cur[1] -= num; tot += num*cur[0]; tot += 2 * num; if prev == 'R' {tot += 2};},
            'L' => {cur[0] -= num; tot -= num*cur[1]; tot += 2 * num;},
            'U' => {cur[1] += num; tot -= num*cur[0]},
            _ => unreachable!("{d} is not a valid direction")
        }
        prev = d;
    }
    println!("part 1: {:?}",tot / 2);
    cur = [0,0];
    tot = 0;
    let mut prev = text.split('\n').last().unwrap().chars().rev().nth(1).unwrap();
    for row in text.split('\n') {
        let mut s =  row.split("(#").nth(1).unwrap().to_string();
        s.pop();
        let d = s.pop().unwrap(); 
        let num = s.bytes().rev().enumerate().map(|(i,x)| match x {b'0'..=b'9' => (x-b'0') as i64,_ => (x-b'a') as i64 + 10}* 16_i64.pow(i as _)).sum::<i64>();
        match d {
            '0'|'R' => {cur[0] += num; tot += num*cur[1]; if prev == 'D' || prev == '1' {tot -= 2};},
            '1'|'D' => {cur[1] -= num; tot += num*cur[0]; tot += 2 * num; if prev == 'R' || prev == '0' {tot += 2};},
            '2'|'L' => {cur[0] -= num; tot -= num*cur[1]; tot += 2 * num;},
            '3'|'U' => {cur[1] += num; tot -= num*cur[0]},
            _ => unreachable!("{d} is not a valid direction")
        }
        prev = d;
    }
    println!("part 2: {:?}",tot / 2);
}
fn day19() {
    let text = get_text(19, false, 1).unwrap();
    let mut workflows = HashMap::new();
    let mut it = text.split('\n');
    let re = Regex::new(r"(.+)\{(.+)\}").unwrap();
    while let Some(row) = it.next() {
        if row.is_empty() {break}
        let Some((_,[k,v])) = re.captures(row).map(|caps| caps.extract()) else { unreachable!("invalid row: {row}")};
        workflows.insert(k.to_string(),v.split(',').map(|s| s.to_string()).collect::<Vec<String>>());
    }
    let mut parts = Vec::new();
    for row in it {
        let part = row.split(&['{','}']).nth(1).unwrap().split(',').map(|p| (p.split('=').next().unwrap().chars().next().unwrap(),p.split('=').nth(1).unwrap().parse::<i32>().unwrap())).collect::<HashMap<char,i32>>();
        parts.push(part);
    }
    fn run_flow(part:HashMap<char,i32>, flow: &[String]) -> String {
        for s in flow {
            if s.find(':').is_none() {return s.to_string()}
            let mut it = s.chars();
            let part_val = part[&it.next().unwrap()];
            let gtlt = it.next().unwrap();
            let mut flow_val = 0;
            while let Some(d) = it.next() {
                match d {
                    '0'..='9' => flow_val = flow_val * 10 + ((d as u8) - b'0') as i32,
                    _ => break,
                }
            }
            match gtlt {
                '>' if part_val > flow_val => {return it.collect()},
                '<' if part_val < flow_val => {return it.collect()},
                _ => (),
            }
        }
        String::new()
    }
    fn check_part(part:&HashMap<char,i32>, workflows: &HashMap<String,Vec<String>>) -> i32 {
        let mut cur = "in".to_string();
        loop {
            //println!("key is {cur}");
            let out = run_flow(part.clone(),&workflows[&cur]);
            match out.as_str() {
                "A" => return part.values().sum::<i32>(),
                "R" => return 0,
                _ => cur = out,
            }
        }
    }
    println!("part 1: {}",parts.into_iter().map(|part| check_part(&part, &workflows)).sum::<i32>());
    let mut cons = VecDeque::new();
    let mut out = Vec::new();
    cons.push_back(("in".to_string(), HashMap::from([('x',[1,4000]),('m',[1,4000]),('a',[1,4000]),('s',[1,4000])])));
    while let Some((s,mut m)) = cons.pop_front() {
        for rule in &workflows[&s] {
            if rule.find(':').is_none() {
                if rule == "R" {
                    ();
                } else if rule == "A" {
                    out.push(m.clone()); 
                } else {
                    cons.push_back((rule.to_string(),m.clone()));
                }
                break
            }
            let mut it = rule.chars();
            let part_char = &it.next().unwrap();
            let gtlt = it.next().unwrap();
            let mut flow_val = 0;
            while let Some(d) = it.next() {
                match d {
                    '0'..='9' => flow_val = flow_val * 10 + ((d as u8) - b'0') as i32,
                    _ => break,
                }
            }
            let nxt:String = it.clone().collect();
            match gtlt {
                '>' => {//pass 
                    if nxt == "A".to_string() {
                        let mut mc = m.clone(); mc.get_mut(part_char).unwrap()[0] = flow_val + 1;
                        out.push(mc); 
                    } else if nxt != "R".to_string() {
                        let mut mc = m.clone(); mc.get_mut(part_char).unwrap()[0] = flow_val + 1;
                        cons.push_back((nxt.to_string(),mc));
                    }
                    //fail
                    m.get_mut(part_char).unwrap()[1] = flow_val;
                },
                '<' => {//pass 
                    if nxt == "A".to_string() {
                        let mut mc = m.clone(); mc.get_mut(part_char).unwrap()[1] = flow_val - 1;
                        out.push(mc); 
                    } else if nxt != "R".to_string() {
                        let mut mc = m.clone(); mc.get_mut(part_char).unwrap()[1] = flow_val - 1;
                        cons.push_back((nxt.to_string(),mc));
                    }
                    //fail
                    m.get_mut(part_char).unwrap()[0] = flow_val;
                },
                _ => (),
            }
        }
    }
    let mut tot:i128 = 0;
    fn prod(w: &HashMap<char,[i32;2]>) -> i128 {
        w.values().map(|[a,b]| if a > b {0} else {(b - a + 1) as i128}).product()
    }
    fn intersect2(w:&HashMap<char,[i32;2]>,m:&HashMap<char,[i32;2]>) -> HashMap<char,[i32;2]> {
        let mut w2 = w.clone();
        for (k,[a,b]) in w2.iter_mut() {
            *a = (*a).max(m.get(k).unwrap()[0]);
            *b = (*b).min(m.get(k).unwrap()[1]);
        }
        w2.clone()
    }
    for (i,w) in out.iter().enumerate() {
        tot += prod(w);
        for m in &out[..i] {
            tot -= prod(&intersect2(w,m));
        }
    }
    println!("part 2: {tot}");

}
fn day20() {
    let text = get_text(20, false, 3).unwrap();
    enum Module {
        FlipFlop(bool,Vec<String>), // prefix %
        Conjunction(HashMap<String,bool>,Vec<String>), // prefix &
        Broadcast(Vec<String>),
    }
    impl Module {
        fn new() -> Self {
            Module::Broadcast(Vec::new())
        }
        fn is_conjunction(&self) -> bool {
            if let Module::Conjunction(_,_) = self {true}
            else {false}
        }
        fn get_children(&self) -> &Vec<String> {
            match self {
                Module::FlipFlop(_,children) => children,
                Module::Conjunction(_,children ) => children,
                Module::Broadcast(children) => children,
            }
        }
        fn pulse(&mut self,is_high:bool,from:&String) -> (i64,i64,bool,Vec<String>) {
            match self {
                Module::FlipFlop(is_on,children) => {
                    if is_high {
                        (0,0,false,Vec::new())
                    } else if *is_on {
                        *is_on = false;
                        (children.len() as _, 0,false, children.clone())
                    } else {
                        *is_on = true;
                        (0, children.len() as _, true, children.clone())
                    }
                },
                Module::Conjunction(mem,children ) => {
                    mem.entry(from.to_string()).and_modify(|state| *state = is_high);
                    if mem.values().all(|is_high_pulse| *is_high_pulse) {
                        (children.len() as _, 0, false, children.clone())
                    } else {
                        (0, children.len() as _, true, children.clone())
                    }
                },
                Module::Broadcast(children) => {
                    if is_high {
                        (0,children.len() as _, is_high, children.clone())
                    } else {
                        (children.len() as _, 0, is_high, children.clone())
                    }
                },
            }
        }
    }
    struct CableSystem {
        m: HashMap<String,Module>,
        low_pules: i64,
        high_pules: i64,
    }
    impl CableSystem {
        fn new(text: &String) -> Self {
            let mut m = HashMap::new();
            let mut map_from = HashMap::new();
            for row in text.split('\n') {
                let out:Vec<String> = row.split("->").nth(1).unwrap().split(',').map(|x| x.trim().to_string()).collect();
                let mut cur_name = row.split("->").next().unwrap()[1..].trim().to_string();
                if cur_name.ends_with("caster") {cur_name = "broadcaster".to_string();}
                if row.starts_with("broadcaster") {
                    m.insert(cur_name.clone(),Module::Broadcast(out.clone()));
                } else if row.starts_with('%') {
                    m.insert(cur_name.clone(),Module::FlipFlop(false,out.clone()));
                } else if row.starts_with('&') {
                    m.insert(cur_name.clone(),Module::Conjunction(HashMap::new(),out.clone()));
                }
                for name in out {
                    map_from.entry(name).or_insert(Vec::new()).push(cur_name.clone());
                }
            }
            for (k,module) in m.iter_mut() {
                if module.is_conjunction() {
                    let mut state = HashMap::new();
                    for name in &map_from[k] {
                        state.insert(name.to_string(),false);
                    }
                    *module = Module::Conjunction(state, module.get_children().to_owned());
                }
            }
            Self{m,low_pules:0,high_pules:0}
        }
        fn print_prod(&self) {
            println!("low {} * high {} = {}",self.low_pules, self.high_pules,self.low_pules * self.high_pules);
        }
        fn pulse(&mut self) {
            let mut q = VecDeque::new();
            self.low_pules += 1; // for pushing the button
            q.push_back((String::new(),"broadcaster".to_string(),false));
            while let Some((prev, name,signal)) = q.pop_front() {
                let (low,high,next_signal,children) = self.m.get_mut(&name).unwrap_or(&mut Module::new()).pulse(signal,&prev);
                self.low_pules += low;
                self.high_pules += high;
                for child in children {
                    q.push_back((name.clone(),child,next_signal));
                }
            }
        }
        fn pulse_and_check(&mut self, machine: String, send_high: bool) -> bool {
            let mut q = VecDeque::new();
            q.push_back((String::new(),"broadcaster".to_string(),false));
            while let Some((prev, name,signal)) = q.pop_front() {
                if name == machine && signal == send_high {return true} 
                let (_,_,next_signal,children) = self.m.get_mut(&name).unwrap_or(&mut Module::new()).pulse(signal,&prev);
                for child in children {
                    q.push_back((name.clone(),child,next_signal));
                }
            }
            false
        }
    }
    let mut cables = CableSystem::new(&text);
    let machine = "mp".to_string();
    for i in 1..1_000_000 {
        if cables.pulse_and_check(machine.clone(),false) {
            println!("{machine} is on: {i}");
            break
        }
    }
    //mp 3917
    //qt 4007
    //qb 4027
    //ng 3919
    //so the product of the parts is 247702167614647
    //cables.print_prod();
}
fn day21() {
    let text = get_text(21, false, 1).unwrap();
    let mut grid = Vec::new();
    for row in text.split('\n') {
        grid.push(row.chars().collect::<Vec<_>>());
    }
    let (n,m) = (grid.len(), grid[0].len());
    let mut start: (usize, usize) = (0,0);
    'outer: for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'S' {
                start = (i,j);
                break 'outer
            }
        }
    }
    let mut level = HashSet::new();
    level.insert(start.clone());
    for _ in 0..64 {
        let mut nxt_level = HashSet::new(); 
        for (i,j) in level {
            for (x,y) in [(i + 1,j),(i.wrapping_add_signed(-1),j),(i,j+1),(i,j.wrapping_add_signed(-1))] {
                if x < n && y < m && grid[x][y] != '#' {
                    nxt_level.insert((x,y));
                }
            }
        }
        level = nxt_level;
    }
    println!("part 1: {}", level.len());
    fn mult_grid(grid:&Vec<Vec<char>>,mult: usize) -> Vec<Vec<char>> {
        let mut ans = Vec::new();
        for _ in 0..mult {
            for row in grid {
                ans.push(row.iter().map(|x| if *x=='S' {'.'} else {*x}).cycle().take(grid[0].len() * mult).collect::<Vec<_>>());
            }
        }
        let n = ans.len();
        ans[n/2][n/2] = 'S';
        ans
    }
    let (n,m) = (n as i64, m as i64);
    let mut seen = HashSet::new();
    let mut frontier = VecDeque::new();
    let mut cnt_odd = 0;
    let mut pat = Vec::new();
    //let mut map = grid.iter().map(|row| row.iter().map(|x| x.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>();
    frontier.push_back((start.0 as i64, start.1 as i64));
    for step in 1..2000 {
        for _ in 0..frontier.len() {
            if let Some((i,j)) = frontier.pop_front() {
                for (x,y) in [(i+1,j),(i-1,j),(i,j+1),(i,j-1)] {
                    if grid[x.rem_euclid(n) as usize][y.rem_euclid(m) as usize] != '#' 
                        && seen.insert((x,y)) {
                        frontier.push_back((x,y));
                        if step % 2 == 1 {cnt_odd += 1;}
                        //if &map[x.rem_euclid(n) as usize][y.rem_euclid(m) as usize] == "." {
                        //    map[x.rem_euclid(n) as usize][y.rem_euclid(m) as usize] = step.to_string();
                        //}
                    }
                }
            }
        }
        if step % 262 == 65 {pat.push(cnt_odd);}
    }
    println!("part 2: {}", cnt_odd);
    //let mut buffer = File::create("test_out_21.txt").unwrap();
    //for row in map {buffer.write_all(row.iter().map(|x| format!("{x:>4}")).chain(std::iter::once("\n".to_string())).collect::<String>().as_bytes()).unwrap();}
    //buffer.flush().unwrap();
    //for row in mult_grid(&grid, 3) {println!("{row:?}");}
    println!("{start:?} {n} {m}");
    println!("{pat:?}");
    fn pattern(n:i128) -> i128 {
        //pattern(n) = garden plots he can reach after 262 * (n - 1) + 65 steps
        //2 * (3831 - 15_287 * n + 15_286 * n * n)
        4 * (8560 - 22875 *n + 15286 * n *n)
    } 
    println!("{:?}",(1..20).map(pattern).collect::<Vec<_>>());
    println!("{}",pattern((26_501_365 / 262) + 1));
    //1251174150198660 is too high
    //1251161780767462 is too high
    //625587097150084
}
fn day22() {
    let text = get_text(22, false, 1).unwrap();
    let mut bricks = Vec::new();
    for row in text.split('\n') {
        bricks.push(row.split('~').map(|e| e.split(',').filter_map(|n| n.parse::<i32>().ok()).collect::<Vec<_>>()).collect::<Vec<_>>());
    }
    bricks.sort_by_key(|b| b[0][2]);
    fn overlaps(i:usize,j:usize,bricks: &[Vec<Vec<i32>>]) -> bool {
        bricks[j][0][0] <= bricks[i][1][0] &&
        bricks[i][0][0] <= bricks[j][1][0] &&
        bricks[j][0][1] <= bricks[i][1][1] &&
        bricks[i][0][1] <= bricks[j][1][1] 
    }
    for i in 0..bricks.len() {
        let mut bottom = 1;
        for j in 0..i {
            if overlaps(i,j,&bricks) {
                bottom = bottom.max(bricks[j][1][2] + 1);
            }
        }
        let fall_dist = bricks[i][0][2] - bottom;
        if fall_dist > 0 {
            bricks[i][0][2] -= fall_dist;
            bricks[i][1][2] -= fall_dist;
        }
    }
    //if i is in supports[j] then brick i is held up by brick j
    let mut supports = vec![vec![];bricks.len()];
    for i in 0..bricks.len() {
        if bricks[i][0][2] == 1 {
        } else {
            for j in 0..i {
                if (bricks[j][1][2] + 1 == bricks[i][0][2]) && overlaps(i, j, &bricks) {
                    supports[j].push(i);
                }
            }
        }
    }
    
    let mut supported_by = vec![vec![];bricks.len()];
    for i in 0..bricks.len() {
        for &j in &supports[i] {
            supported_by[j].push(i);
        }
    }
    //how many bricks can be safely removed?
    let mut p1 = 0;
    for i in 0..bricks.len() {
        if supports[i].iter().all(|&j| supported_by[j].len() > 1) {
            p1 += 1;
        }
    }

    //println!("{bricks:?}");
    //println!("{supports:?}");
    //println!("{supported_by:?}");
    println!("part 1: {}",p1);
    let mut falls = vec![0; bricks.len()];
    let mut now_falling = HashSet::new();
    for i in (0..bricks.len()).rev() {
        now_falling.insert(i);
        for j in i + 1 .. bricks.len() {
            if bricks[j][0][2] > 1 && supported_by[j].iter().all(|k| now_falling.contains(k)) {
                now_falling.insert(j);
            }
        }
        falls[i] = now_falling.len() - 1;
        now_falling.clear();
    }
    //println!("{falls:?}");
    println!("part 2: {:?}",falls.into_iter().sum::<usize>());
    //111728 is too high
    //131270 is too high
}
fn main() {
    let now = Instant::now();
    let _ = day22();
    println!("Elapsed: {:.2?}", now.elapsed());
}
