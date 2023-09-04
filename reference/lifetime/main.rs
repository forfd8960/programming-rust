fn main() {
    let nums: Vec<i32> = vec![1, 3, 6, 5, 2, 0];
    let small = smallest(nums.as_slice());
    println!("small: {}", small);
}

fn smallest<'a>(nums: &'a [i32]) -> i32 {
    let mut smallest = nums[0];
    for n in &nums[1..] {
        if *n < smallest {
            smallest = *n;
        }
    }

    return smallest;
}
