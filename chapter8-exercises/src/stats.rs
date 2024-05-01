use std::collections::HashMap;

pub fn median_and_mode(numbers: &Vec<usize>) -> (usize, usize) {
    let size = numbers.len();
    let mut seen_cnt: HashMap<usize, usize> = HashMap::new();
    let mut sum: usize = 0;

    for n in numbers {
        sum += n;
        let cnt = seen_cnt.entry(n.clone()).or_insert(1);
        *cnt += 1;
    }
    let median = sum / size;

    let mut highest_count = 0;
    let mut highest_num = 0;

    for (n, cnt) in &seen_cnt {
        if cnt > &highest_count {
            highest_num = *n;
            highest_count = *cnt;
        };
    }

    println!("seen hashmap: {:?}", seen_cnt);

    return (median, highest_num);
}
