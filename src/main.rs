#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::{fs,env};
use std::error::Error;

fn day1() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("data/day1.txt")?;
 
    println!("part 1: {:?}", text.split('\n').map(|line| 
        line.chars().filter(|c| c.is_numeric()).next().unwrap().to_digit(10_u32).unwrap() as i32 * 10 + 
        line.chars().rev().filter(|c| c.is_numeric()).next().unwrap().to_digit(10_u32).unwrap() as i32
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
    let text = fs::read_to_string("data/day2.txt")?;
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
fn main() {
    let _ = day2();
}
