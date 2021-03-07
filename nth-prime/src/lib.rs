pub fn nth(n: u32) -> u32 {
    let mut idx = 0;
    let mut res: u32 = 0;
    while idx <= n {
        if is_prime(res) {
            if idx == n {
                return res;
            }
            idx += 1;
        }
        res += 1;
    }

    res
}

fn is_prime(n: u32) -> bool {
    if n == 0 {
        return false;
    }
    if n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }

    for num in 2..=(n / 2) {
        if n % num == 0 {
            return  false;
        }
    }

    true
}
