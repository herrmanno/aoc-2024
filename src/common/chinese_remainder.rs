/// Chinese Remainder algorithm
/// See https://rosettacode.org/wiki/Chinese_remainder_theorem
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

/// given residues a_1, a_2, ..., a_n and modulii n_1, n_2, ..., n_n, calculates
/// the smallest value x, s.t.
/// x = a_1 + n_1 * x_1
/// x = a_2 + n_2 * x_2
/// ...
/// x = a_n + n_n * x_n
pub(crate) fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
