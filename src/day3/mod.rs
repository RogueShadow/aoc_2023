use crate::colors::*;

pub fn day3_tests() {
    let timer = std::time::Instant::now();
    let result = part1(include_str!("test.txt"));
    let t = timer.elapsed().as_secs_f32() * 1000.0;
    title(3);
    part(1,result);
    time(t);
    assert_eq!(result,4361);

    let timer = std::time::Instant::now();
    let result = part2(include_str!("test.txt"));
    let t = timer.elapsed().as_secs_f32() * 1000.0;
    part(2,result);
    time(t);
    assert_eq!(result,467835);
}
pub fn day3() {
    let timer = std::time::Instant::now();
    let result = part1(include_str!("input.txt"));// 314,085 too low // 527,439 too low 314274
    let t = timer.elapsed().as_secs_f32() * 1000.0;
    title(3);
    part(1,result);
    time(t);
    assert_eq!(result,533784);

    let timer = std::time::Instant::now();
    let result = part2(include_str!("input.txt"));
    let t = timer.elapsed().as_secs_f32() * 1000.0;
    part(2,result);
    time(t);
    assert_eq!(result,78826761);
}

pub fn part1(data: &str) -> u128 {
    let mut number = None;
    let mut number_i = vec![];
    let mut width = 0;
    let mut result = 0;
    let mut numbers = vec![];
    let mut n_coords = vec![];
    let symbals = data.lines().enumerate().map(|(y,line)|{
        width = line.len();
        line.chars().enumerate().filter_map(move |(x,c)|{
            if !c.is_ascii_digit() && !(c == '.') {
                Some((x,y))
            } else {None}
        })
    }).flatten().collect::<Vec<_>>();
    data.lines().enumerate().for_each(|(y,line)| {
        line.as_bytes().iter().enumerate().for_each(|(x,b)| {
           if b.is_ascii_digit() {
               if number.is_some() {
                   number_i.push((x, y));
                   number = Some(number.unwrap() * 10 + (*b - b'0') as usize);
               } else {
                   number_i.push((x, y));
                   number = Some((*b - b'0') as usize);
               }
           } else {
               if number.is_some() {
                   numbers.push(number.unwrap());
                   n_coords.push(number_i.clone());
                   number_i.clear();
                   number = None;
               }
           }
       });
    });

    numbers.iter().enumerate().for_each(|(i,n)| {
        let coords = n_coords.get(i).unwrap();
        'outer: for (x,y) in coords {
            for (px, py) in symbals.iter() {
                let xr = if *x == 0 { *x..=*x + 1 } else { *x - 1..=*x + 1 };
                let yr = if *y == 0 { *y..=*y + 1 } else { *y - 1..=*y + 1 };
                if xr.contains(&px) && yr.contains(&py) {
                    result += *n as u128;
                    break 'outer;
                }
            }
        }
    });

    result
}
pub fn part2(data: &str) -> u128 {
    let mut number = None;
    let mut number_i = vec![];
    let mut width = 0;
    let mut result = 0;
    let mut numbers = vec![];
    let mut n_coords = vec![];
    let symbals = data.lines().enumerate().map(|(y,line)|{
        width = line.len();
        line.chars().enumerate().filter_map(move |(x,c)|{
            if c == '*' {
                Some((x,y))
            } else {None}
        })
    }).flatten().collect::<Vec<_>>();
    data.lines().enumerate().for_each(|(y,line)| {
        line.as_bytes().iter().enumerate().for_each(|(x,b)| {
            if b.is_ascii_digit() {
                if number.is_some() {
                    number_i.push((x, y));
                    number = Some(number.unwrap() * 10 + (*b - b'0') as usize);
                } else {
                    number_i.push((x, y));
                    number = Some((*b - b'0') as usize);
                }
            } else {
                if number.is_some() {
                    numbers.push(number.unwrap());
                    n_coords.push(number_i.clone());
                    number_i.clear();
                    number = None;
                }
            }
        });
    });

    let mut pair = vec![];
    for (px, py) in symbals.iter() {
        numbers.iter().enumerate().for_each(|(i,n)| {
        let coords = n_coords.get(i).unwrap();
        'outer: for (x,y) in coords {
                let xr = if *x == 0 { *x..=*x + 1 } else { *x - 1..=*x + 1 };
                let yr = if *y == 0 { *y..=*y + 1 } else { *y - 1..=*y + 1 };
                if xr.contains(&px) && yr.contains(&py) {
                    pair.push(*n);
                    if pair.len() == 2 {
                        result += pair.iter().product::<usize>() as u128;
                        pair.clear();
                        break 'outer;
                    }
                    break 'outer;
                }
            }
        });
        pair.clear();
    };
    result
}
