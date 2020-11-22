#![feature(exclusive_range_pattern)]
#![deny(overlapping_patterns)]

macro_rules! m {
    ($s:expr, $t1:pat, $t2:pat) => {
        match $s {
            $t1 => {}
            $t2 => {}
            _ => {}
        }
    }
}

fn main() {
    m!(0u8, 20..=30, 30..=40); //~ ERROR multiple patterns covering the same range
    m!(0u8, 30..=40, 20..=30); //~ ERROR multiple patterns covering the same range
    m!(0u8, 20..=30, 31..=40);
    m!(0u8, 20..=30, 29..=40);
    m!(0u8, 20.. 30, 29..=40); //~ ERROR multiple patterns covering the same range
    m!(0u8, 20.. 30, 28..=40);
    m!(0u8, 20.. 30, 30..=40);
    m!(0u8, 20..=30, 30..=30);
    m!(0u8, 20..=30, 30..=31); //~ ERROR multiple patterns covering the same range
    m!(0u8, 20..=30, 29..=30);
    m!(0u8, 20..=30, 20..=20);
    m!(0u8, 20..=30, 20..=21);
    m!(0u8, 20..=30, 19..=20); //~ ERROR multiple patterns covering the same range
    m!(0u8, 20..=30, 20);
    m!(0u8, 20..=30, 25);
    m!(0u8, 20..=30, 30);
    m!(0u8, 20.. 30, 29);
    m!(0u8, 20, 20..=30); //~ ERROR multiple patterns covering the same range
    m!(0u8, 25, 20..=30);
    m!(0u8, 30, 20..=30); //~ ERROR multiple patterns covering the same range

    match 0u8 {
        0..=10 => {}
        20..=30 => {}
        10..=20 => {} //~ ERROR multiple patterns covering the same range
        _ => {}
    }
    match (0u8, true) {
        (0..=10, true) => {}
        (10..20, true) => {} // not detected
        (10..20, false) => {}
        _ => {}
    }
    match (true, 0u8) {
        (true, 0..=10) => {}
        (true, 10..20) => {} //~ ERROR multiple patterns covering the same range
        (false, 10..20) => {}
        _ => {}
    }
    match Some(0u8) {
        Some(0..=10) => {}
        Some(10..20) => {} //~ ERROR multiple patterns covering the same range
        _ => {}
    }
}
