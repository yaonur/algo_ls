fn two_number_sum(array: &Vec<i32>, target_sum: i32) -> Option<(i32, i32)> {
    for i in 0..array.len() {
        for j in i+1..array.len() {
            if array[i] + array[j] == target_sum {
                return Some((array[i], array[j]));
            }
        }
    }
    None
}

fn two_number_sum_hash(array: &Vec<i32>, target_sum: i32) -> Option<(i32, i32)> {
    let mut nums = std::collections::HashMap::new();
    for &num in array {
        let possible_match = target_sum - num;
        if nums.contains_key(&possible_match) {
            return Some((num, possible_match));
        } else {
            nums.insert(num, true);
        }
    }
    None
}

fn two_number_sum_marker(mut array: Vec<i32>, target_sum: i32) -> Option<(i32, i32)> {
    array.sort();
    let (mut left, mut right) = (0, array.len() - 1);
    while left < right {
        let sum = array[left] + array[right];
        if sum == target_sum {
            return Some((array[left], array[right]));
        } else if sum < target_sum {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
}

fn main() {
    let array = vec![3, 5, -4, 8, 11, 1, -1, 6];
    let target_sum = 10;

    let solution = two_number_sum(&array, target_sum);
    println!("{:?}", solution);

    let solution_hash = two_number_sum_hash(&array, target_sum);
    println!("{:?}", solution_hash);

    let solution_marker = two_number_sum_marker(array, target_sum);
    println!("{:?}", solution_marker);
}