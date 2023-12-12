use std::collections::{HashMap, HashSet};
use std::fs::{ read_to_string};
use crate::{ run_day};


pub fn day10_tests() {
    let test = &*read_to_string("src/day10/test2.txt").unwrap();
    let test2 = &*read_to_string("src/day10/test3.txt").unwrap();
    let (r1,r2) = run_day("10 Tests",vec![test,test2],part1,part2,Some(1.0));
    assert_eq!(r1,8);
    assert_eq!(r2,4);
}
pub fn day10() {
    let data = &*read_to_string("src/day10/input.txt").unwrap();
    let (r1,r2) = run_day("10",vec![data],part1,part2,Some(1.0));
    assert_eq!(r1,6701);
    assert_eq!(r2,303);
}

pub fn part1(input: &str) -> u32 {
    let mut pipes = HashMap::new();
    let mut start = (0,0);
    input.lines().enumerate().for_each(|(y,line)|{
        line.as_bytes().iter().enumerate().for_each(|(x,b)| {
            let pipe = Pipe::from(*b);
            if matches!(pipe,Pipe::Start) {
                start = (x,y);
            }
            pipes.insert((x,y),pipe);
        });
    });
    let start_pipe = get_pipe_from_surroundings(&start,&pipes);

    let (mut pos1,mut pos2) = match start_pipe {
        Pipe::Vertical      => ((start.0,start.1-1),(start.0,start.1+1)),
        Pipe::Horizontal    => ((start.0-1,start.1),(start.0+1,start.1)),
        Pipe::BendNorthWest => ((start.0,start.1-1),(start.0-1,start.1)),
        Pipe::BendNorthEast => ((start.0,start.1-1),(start.0+1,start.1)),
        Pipe::BendSouthWest => ((start.0,start.1+1),(start.0-1,start.1)),
        Pipe::BendSouthEast => ((start.0,start.1+1),(start.0+1,start.1)),
        _ => panic!("Bad starting points")
    };
    let mut last1 = start;
    let mut steps1 = 1u32;
    loop {
        let pipe = pipes.get(&pos1).unwrap();
        let (choice1,choice2) = match pipe {
            Pipe::Vertical      => ((pos1.0,pos1.1-1),(pos1.0,pos1.1+1)),
            Pipe::Horizontal    => ((pos1.0-1,pos1.1),(pos1.0+1,pos1.1)),
            Pipe::BendNorthWest => ((pos1.0,pos1.1-1),(pos1.0-1,pos1.1)),
            Pipe::BendNorthEast => ((pos1.0,pos1.1-1),(pos1.0+1,pos1.1)),
            Pipe::BendSouthWest => ((pos1.0,pos1.1+1),(pos1.0-1,pos1.1)),
            Pipe::BendSouthEast => ((pos1.0,pos1.1+1),(pos1.0+1,pos1.1)),
            Pipe::Start => break,
            _ => panic!("Invalid movement.")
        };

        pos1 = if choice1 == last1 {
            last1 = pos1;
            choice2
        } else {
            last1 = pos1;
            choice1
        };
        steps1 += 1;
    }
    steps1 / 2u32
}

pub fn part2(input: &str) -> u32 {
    let mut pipes = HashMap::new();
    let mut start = (0,0);
    input.lines().enumerate().for_each(|(y,line)|{
        line.as_bytes().iter().enumerate().for_each(|(x,b)| {
            let pipe = Pipe::from(*b);
            if matches!(pipe,Pipe::Start) {
                start = (x,y);
            }
            pipes.insert((x,y),pipe);
        });
    });
    let start_pipe = get_pipe_from_surroundings(&start,&pipes);

    let (mut pos1,mut pos2) = match start_pipe {
        Pipe::Vertical      => ((start.0,start.1-1),(start.0,start.1+1)),
        Pipe::Horizontal    => ((start.0-1,start.1),(start.0+1,start.1)),
        Pipe::BendNorthWest => ((start.0,start.1-1),(start.0-1,start.1)),
        Pipe::BendNorthEast => ((start.0,start.1-1),(start.0+1,start.1)),
        Pipe::BendSouthWest => ((start.0,start.1+1),(start.0-1,start.1)),
        Pipe::BendSouthEast => ((start.0,start.1+1),(start.0+1,start.1)),
        _ => panic!("Bad starting points")
    };
    let mut last1 = start;
    let mut main_loop = HashMap::new();
    let mut max_pos = (0usize,0usize);
    loop {
        let pipe = pipes.get(&pos1).unwrap();
        main_loop.insert(last1,pipes.get(&last1).unwrap().to_owned());
        max_pos.0 = max_pos.0.max(last1.0);
        max_pos.1 = max_pos.1.max(last1.1);
        let (choice1,choice2) = match pipe {
            Pipe::Vertical      => ((pos1.0,pos1.1-1),(pos1.0,pos1.1+1)),
            Pipe::Horizontal    => ((pos1.0-1,pos1.1),(pos1.0+1,pos1.1)),
            Pipe::BendNorthWest => ((pos1.0,pos1.1-1),(pos1.0-1,pos1.1)),
            Pipe::BendNorthEast => ((pos1.0,pos1.1-1),(pos1.0+1,pos1.1)),
            Pipe::BendSouthWest => ((pos1.0,pos1.1+1),(pos1.0-1,pos1.1)),
            Pipe::BendSouthEast => ((pos1.0,pos1.1+1),(pos1.0+1,pos1.1)),
            Pipe::Start => break,
            _ => panic!("Invalid movement.")
        };
        pos1 = if choice1 == last1 {
            last1 = pos1;
            choice2
        } else {
            last1 = pos1;
            choice1
        };
    }

    main_loop.insert(start,start_pipe);
    let mut is_outside = true;
    let mut last_bend = None;
    let mut inside = 0u32;

    for y in 0..=max_pos.1 {
        is_outside = true;
        last_bend = None;
        for x in 0..=max_pos.0 {
            if let Some(pipe) = main_loop.get(&(x,y)) {
                match pipe {
                    Pipe::Vertical => {
                        is_outside = !is_outside;
                        last_bend = None;
                    }
                    Pipe::Horizontal => {}
                    Pipe::BendNorthWest | Pipe::BendNorthEast => {
                        if  matches!(last_bend,Some(Pipe::BendSouthEast) | Some(Pipe::BendSouthWest)) {
                            is_outside = !is_outside;
                        } else {
                            last_bend = Some(*pipe);
                        }
                    }
                    Pipe::BendSouthWest | Pipe::BendSouthEast=> {
                        if  matches!(last_bend,Some(Pipe::BendNorthWest) | Some(Pipe::BendNorthEast)) {
                            is_outside = !is_outside;
                        } else {
                            last_bend = Some(*pipe);
                        }
                    }
                    _ => {panic!("Shouldn't be any here...")}
                }
            } else {
                if !is_outside {
                    inside += 1;
                }
            }
        }
    }

    inside
}

#[derive(Debug,Eq, PartialEq,Copy, Clone)]
pub enum Pipe {
    Ground,
    Vertical,
    Horizontal,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Start,
}

impl From<u8> for Pipe {
    fn from(value: u8) -> Self {
        match value {
            b'|' => Pipe::Vertical,
            b'-' => Pipe::Horizontal,
            b'L' => Pipe::BendNorthEast,
            b'J' => Pipe::BendNorthWest,
            b'7' => Pipe::BendSouthWest,
            b'F' => Pipe::BendSouthEast,
            b'.' => Pipe::Ground,
            b'S' => Pipe::Start,
            _ => panic!("Not a pipe!")
        }
    }
}

pub fn get_neighbor_pipes(pos: &(usize,usize), pipes: &HashMap<(usize,usize),Pipe>) -> (Option<Pipe>,Option<Pipe>,Option<Pipe>,Option<Pipe>) {
    let north = if pos.1 == 0 {
        None
    } else {
        pipes.get(&(pos.0,pos.1-1))
    };

    let south = pipes.get(&(pos.0,pos.1+1));

    let west = if pos.0 == 0 {
        None
    } else {
        pipes.get(&(pos.0-1,pos.1))
    };

    let east = pipes.get(&(pos.0+1,pos.1));

    (north.copied(),south.copied(),west.copied(),east.copied())
}
pub fn get_pipe_from_surroundings(start: &(usize,usize), pipes: &HashMap<(usize,usize),Pipe>) -> Pipe {
    let (north,south,west,east) = get_neighbor_pipes(start,pipes);
    let mut wrong_pipes = vec![Pipe::Start,Pipe::Ground];
    if !matches!(north, Some(Pipe::Vertical) | Some(Pipe::BendSouthEast) | Some(Pipe::BendSouthWest)) {
        wrong_pipes.push(Pipe::Vertical);
        wrong_pipes.push(Pipe::BendNorthEast);
        wrong_pipes.push(Pipe::BendNorthWest);
    }
    if !matches!(south,Some(Pipe::Vertical) | Some(Pipe::BendNorthWest) | Some(Pipe::BendNorthEast)) {
        wrong_pipes.push(Pipe::Vertical);
        wrong_pipes.push(Pipe::BendSouthWest);
        wrong_pipes.push(Pipe::BendSouthEast);
    }
    if !matches!(west,Some(Pipe::Horizontal) | Some(Pipe::BendSouthEast) | Some(Pipe::BendNorthEast)) {
        wrong_pipes.push(Pipe::Horizontal);
        wrong_pipes.push(Pipe::BendSouthWest);
        wrong_pipes.push(Pipe::BendNorthWest);
    }
    if !matches!(east,Some(Pipe::Horizontal) | Some(Pipe::BendNorthWest) | Some(Pipe::BendSouthWest)) {
        wrong_pipes.push(Pipe::Horizontal);
        wrong_pipes.push(Pipe::BendNorthEast);
        wrong_pipes.push(Pipe::BendSouthEast);
    }
    let mut pp = vec![Pipe::Vertical,Pipe::Horizontal,Pipe::BendNorthEast,Pipe::BendNorthWest,Pipe::BendSouthWest,Pipe::BendSouthEast];
    let pipes = pp.iter().filter(|p| !wrong_pipes.contains(p)).collect::<Vec<_>>();
    assert_eq!(pipes.len(), 1, "{start:?} {wrong_pipes:?} {pipes:?}\n {north:?} {south:?} {west:?} {east:?}");
    *pipes.first().unwrap().to_owned()
}