pub fn factors(n: u64) -> Vec<u64> {
    let mut n_dup = n.clone();
    let mut res: Vec<u64> = vec![];

    for num in 2..=n {
        while n_dup % num == 0 {
            res.push(num);
            n_dup /= num;
        }
        if n_dup == 1 {
            break;
        }
    }

    res
}
