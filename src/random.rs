/// This file contains my random functions for:
/// - u32
/// - i32
/// - f32
///
/// fast_random_type - function that if called many fast and many times will be almost equal
/// random_type      - function, that a little bit slower, but dont repeat it self when called fast

use std::time::{SystemTime, UNIX_EPOCH};



pub fn fast_random_u32 (min: u32, max: u32) -> u32 {
    (_fast_random_u32()) % (max-min+1) + min
}

pub fn fast_random_i32 (min: i32, max: i32) -> i32 {
    (_fast_random_u32() as i32) % (max-min+1) + min
}

pub fn fast_random_f32 (min: f32, max: f32) -> f32 {
    (_fast_random_f32()) * (max-min) + min
}



pub fn random_u32 (min: u32, max: u32) -> u32 {
    (_good_random_u32()) % (max-min+1) + min
}

pub fn random_i32 (min: i32, max: i32) -> i32 {
    (_good_random_u32() as i32) % (max-min+1) + min
}

pub fn random_f32 (min: f32, max: f32) -> f32 {
    (_good_random_f32()) * (max-min) + min
}



fn _good_random_u32 () -> u32 {
    // 22_695_477 and +1 is from WIKI for random numbers
    return ( (22_695_477_u64*(_fast_random_u32() as u64) + 1_u64) % (u32::max_value() as u64) ) as u32;
}

fn _good_random_f32 () -> f32 {
    // 22_695_477 and +1 is from WIKI for random numbers
    return ( (22_695_477_f64*(_fast_random_f32() as f64) + 1_f64) % (1.0_f64) ) as f32;
}

fn _fast_random_u32 () -> u32 {
    let time_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    return time_now;
}

fn _fast_random_f32 () -> f32 {
    let time_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    // 999_999_999 is max time_now, because it is nanoseconds = 10^(-9)
    return time_now as f32 / 999_999_999 as f32;
}





pub fn print_statistics () {

    let mut count_00: u64 = 0;
    let mut count_01: u64 = 0;
    let mut count_02: u64 = 0;
    let mut count_03: u64 = 0;
    let mut count_04: u64 = 0;
    let mut count_05: u64 = 0;
    let mut count_06: u64 = 0;
    let mut count_07: u64 = 0;
    let mut count_08: u64 = 0;
    let mut count_09: u64 = 0;

    let mut random_number_f32: f32;
    let mut iteration: u64 = 0;
    loop {
        random_number_f32 = _good_random_f32();
        // random_number_f32 = _fast_random_f32();

        if iteration % 1_000_000 == 0 {
            println!("iteration = {}_000_000", iteration/1_000_000);
            println!("count_00 = {}", count_00);
            println!("count_01 = {}", count_01);
            println!("count_02 = {}", count_02);
            println!("count_03 = {}", count_03);
            println!("count_04 = {}", count_04);
            println!("count_05 = {}", count_05);
            println!("count_06 = {}", count_06);
            println!("count_07 = {}", count_07);
            println!("count_08 = {}", count_08);
            println!("count_09 = {}", count_09);
            println!();
        }

        match random_number_f32 {
            x if x < 0.0 => {
                println!("ELSE!!!: random_number_f32 = {}", random_number_f32);
                break;
            },
            x if x <  0.1 => { count_00+=1; },
            x if x <  0.2 => { count_01+=1; },
            x if x <  0.3 => { count_02+=1; },
            x if x <  0.4 => { count_03+=1; },
            x if x <  0.5 => { count_04+=1; },
            x if x <  0.6 => { count_05+=1; },
            x if x <  0.7 => { count_06+=1; },
            x if x <  0.8 => { count_07+=1; },
            x if x <  0.9 => { count_08+=1; },
            x if x <= 1.0 => { count_09+=1; },
            _ => {
                println!("ELSE!!!: random_number_f32 = {}", random_number_f32);
                break;
            }
        }

        // if      0.0 <= random_number_f32 && random_number_f32 <= 0.1 { count_00 += 1; }
        // else if 0.1 <= random_number_f32 && random_number_f32 <= 0.2 { count_01 += 1; }
        // else if 0.2 <= random_number_f32 && random_number_f32 <= 0.3 { count_02 += 1; }
        // else if 0.3 <= random_number_f32 && random_number_f32 <= 0.4 { count_03 += 1; }
        // else if 0.4 <= random_number_f32 && random_number_f32 <= 0.5 { count_04 += 1; }
        // else if 0.5 <= random_number_f32 && random_number_f32 <= 0.6 { count_05 += 1; }
        // else if 0.6 <= random_number_f32 && random_number_f32 <= 0.7 { count_06 += 1; }
        // else if 0.7 <= random_number_f32 && random_number_f32 <= 0.8 { count_07 += 1; }
        // else if 0.8 <= random_number_f32 && random_number_f32 <= 0.9 { count_08 += 1; }
        // else if 0.9 <= random_number_f32 && random_number_f32 <= 1.0 { count_09 += 1; }
        // else {
        //     println!("ELSE!!!: random_number_f32 = {}", random_number_f32);
        //     break;
        // }

        iteration += 1;
    }

}
