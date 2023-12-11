#![allow(unused_imports)]
#![allow(dead_code)]
#![recursion_limit = "2000000"]
use std::collections::*;
use std::{fs,env};
use std::error::Error;
//use reqwest;
use soup::prelude::*;
use std::time::Instant;
use regex::Regex;

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
fn main() {
    let now = Instant::now();
    let _ = day11();
    println!("Elapsed: {:.2?}", now.elapsed());
}
