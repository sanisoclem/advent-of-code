//! ```cargo
//! [dependencies]
//! ```

fn main() {
    test();

    let contents = std::fs::read_to_string("day1.data")
        .expect("Something went wrong reading the file");
    let nums = string_to_numbers(&contents);

    let sol1 = solution1(&nums);
    let sol2 = solution2(&nums);

    println!("Solution1: {:?}", sol1);
    println!("Solution2: {:?}", sol2);
}

fn string_to_numbers(input: &str) -> Vec<i32> {
    input.split('\n')
        .map(|x| x.parse::<i32>().expect("all entries should be ints"))
        .collect()
}

fn solution1(nums: &[i32]) -> i32 {
    let (total, _) =
        nums
        .iter()
        .fold((0, None), |(total, prev), val| {
            match prev {
                Some(prev) => if val > prev { (total + 1, Some(val)) } else { (total, Some(val)) },
                _ => (total, Some(val))
            }
        });
    total
}

fn solution2(nums: &[i32]) -> i32 {
    let numsBy3: Vec<_> =
        nums
        .iter()
        .scan((None, None), |state: &mut (Option<i32>, Option<i32>), val| {
            let (prev1, prev2) = *state;
            let emit =
                match (prev1, prev2) {
                    (Some(prev1), Some(prev2)) => Some(Some(val + prev1 + prev2)),
                    _ => Some(None)
                };
            *state = (Some(*val), prev1);
            emit

        })
        .flatten() // rust's scan is weird
        .collect();
    solution1(&numsBy3)
}

fn test() {
    assert_eq!(0, solution1(&[1,1,1,1]));
    assert_eq!(3, solution1(&[1,2,3,4]));
    assert_eq!(0, solution1(&[5]));


    assert_eq!(0, solution2(&[1,1,1]));
    assert_eq!(1, solution2(&[1,2,3,4]));
    assert_eq!(3, solution2(&[1,2,3,4,5,6]));
    assert_eq!(0, solution2(&[1,2,3,1,2,3]));
}