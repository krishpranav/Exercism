#![allow(unused)]

// should return 40
pub fn expected_minutes_in_oven() -> i32 {
    return 40;
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    return expected_minutes_in_oven() - actual_minutes_in_oven;
}

/* calculate the prep time */
pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    return number_of_layers * 2;
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    return expected_minutes_in_oven() - remaining_minutes_in_oven(actual_minutes_in_oven) + preparation_time_in_minutes(number_of_layers);
}

// + passed!!