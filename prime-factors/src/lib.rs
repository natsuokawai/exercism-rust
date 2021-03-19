pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut n_dup = n.clone();

    for num in 2..=n {
        while n_dup % num == 0 {
            result.push(num);
            n_dup /= num;
        }
        if n_dup == 1 {
            break;
        }
    }

    result
}
