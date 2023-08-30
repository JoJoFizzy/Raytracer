
pub const THRESHOLD_F32: f32 = 0.00001;

pub fn equals_f32(num1: &f32, num2: &f32) -> bool {
    return (num1 - num2).abs() < THRESHOLD_F32;
}

pub fn clamp_f32(num: f32, low: f32, high: f32) -> f32 {
    if num < low {
        return low;
    }
    if num > high {
        return high;
    }
    return num;
}

pub fn max_f32(arr: &Vec<f32>) -> Option<f32> {
    if arr.len() <= 0 {
        return None;
    }

    let mut max = arr[0];
    for i in 1..arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
    }

    return Some(max);
}

pub fn min_f32(arr: &Vec<f32>) -> Option<f32> {
    if arr.len() <= 0 {
        return None;
    }

    let mut min = arr[0];
    for i in 1..arr.len() {
        if arr[i] < min {
            min = arr[i];
        }
    }

    return Some(min);
}