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
fn main() {
    day1();
}
