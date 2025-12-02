use std::{
    error::Error,
    fs,
    io::{self, prelude::*},
};

pub fn part1() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("2025/inputs/day01.txt")?;
    let reader = io::BufReader::new(file);

    // dial position
    let mut dial = 50;

    // actual password
    let mut pass = 0;

    let mut l = 0;

    for line in reader.lines() {
        let line = line?;
        let change = line[1..].parse::<i32>()? % 100;

        if line.starts_with('R') {
            dial += change;
            dial = dial % 100;
        } else {
            dial -= change;
        }

        while dial < 0 {
            dial += 100;
        }

        println!("line {l}, dial: {dial}");

        l += 1;

        if dial == 0 {
            pass += 1;
        }
    }

    println!("password: {pass}");

    Ok(())
}

pub fn part2() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("2025/inputs/day01.txt")?;
    let reader = io::BufReader::new(file);

    // dial position
    let mut dial = 50;

    // actual password
    let mut pass = 0;

    let mut l = 0;

    for line in reader.lines() {
        let line = line?;
        let change = line[1..].parse::<i32>()?;

        // only update `pass` if you are crossing 0 or at 0 in this iteration

        // if dial == 0 {
        //     pass -= change / 100;
        // }

        let distance: i32 = line[1..].parse()?;

        let full_loops = distance / 100;
        pass += full_loops;

        let rem = distance % 100;

        if line.starts_with('R') {
            // steps from current dial to 0 when moving right
            let steps_to_zero = (100 - dial) % 100;
            if steps_to_zero != 0 && rem >= steps_to_zero {
                pass += 1;
            }
            dial = (dial + rem) % 100;
        } else {
            // steps from current dial to 0 when moving left
            let steps_to_zero = dial % 100;
            if steps_to_zero != 0 && rem >= steps_to_zero {
                pass += 1;
            }
            dial = (dial - rem).rem_euclid(100);
        }

        // if line.starts_with('R') {
        //     dial += change;
        // } else {
        //     if dial == 0 {
        //         pass -= 1;
        //     }
        //     dial -= change;
        //     if dial == 0 {
        //         pass += 1;
        //     }
        // }

        // while dial > 99 {
        //     dial -= 100;
        //     pass += 1;
        // }

        // while dial < 0 {
        //     dial += 100;
        //     pass += 1;
        // }

        l += 1;
        println!("line {l}, dial: {dial}, passes: {pass}");
    }

    println!("password: {pass}");

    Ok(())
}

// pub fn part2() -> Result<(), Box<dyn Error>> {
//     let file = fs::File::open("2025/inputs/day01.txt")?;
//     let reader = io::BufReader::new(file);

//     // dial position
//     let mut dial = 50;

//     // actual password
//     let mut pass = 0;

//     let mut l = 0;

//     for line in reader.lines() {
//         let line = line?;
//         let change = line[1..].parse::<i32>()?;

//         if line.starts_with('R') {
//             dial += change;
//         } else {
//             if dial == 0 {
//                 pass -= 1;
//             }
//             dial -= change;
//         }

//         if dial < 0 {
//             while dial < 0 {
//                 dial += 100;
//                 pass += 1;
//             }
//         } else if dial > 99 {
//             while dial > 99 {
//                 dial -= 100;
//                 pass += 1;
//             }
//         } else if dial == 0 {
//             pass += 1;
//         }

//         println!("line {l}, dial: {dial}, passes: {pass}");

//         l += 1;
//     }

//     println!("password: {pass}");

//     Ok(())
// }

// pub fn part2() -> Result<(), Box<dyn Error>> {
//     let file = fs::File::open("2025/inputs/day01.txt")?;
//     let reader = io::BufReader::new(file);

//     // dial position
//     let mut dial = 50;

//     // actual password
//     let mut pass = 0;

//     let mut l = 0;

//     for line in reader.lines() {
//         let line = line?;
//         let change = line[1..].parse::<i32>()?;

//         if line.starts_with('R') {
//             dial += change;
//         } else {
//             dial -= change;
//         }

//         if dial < 0 {
//             pass += 1;
//             dial += 100;
//         }

//         pass += dial / 100;
//         dial %= 100;

//         println!("line {l}, dial: {dial}, passes: {pass}");

//         l += 1;

//         // if dial == 0 {
//         //     pass += 1;
//         // }
//     }

//     println!("password: {pass}");

//     Ok(())
// }
