pub fn is_prime(n: u32) -> bool {
	match n {
		1 => false,
		2 => true,
   		_ => !(2..n - 1).any(|i| n % i == 0),
	}
}

pub fn nth(n: u32) -> u32 {
	(1..).filter(|c| is_prime(*c)).nth(n as usize).unwrap()
}
