fn main() {
    let fibonacci_nums = fibonacci(19);
    println!("{:?}", fibonacci_nums);  
}

fn fibonacci(secuences: u32) -> Vec<i32> {
    let mut fibonacci_nums = Vec::new();
    fibonacci_nums.push(1);
    fibonacci_nums.push(1);

    for num in 2..secuences {
        let a: usize = (num - 2).try_into().unwrap();
        let b: usize = (num - 1).try_into().unwrap();
        
        fibonacci_nums.push(fibonacci_nums[a] + fibonacci_nums[b]);
    }

    fibonacci_nums
}
