pub fn quick_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    let mut result = 1.0;
    let mut base = x;
    let mut exp = n as i64;

    if exp < 0 {
        base = 1.0 / base;
        exp = -exp;
    }

    while exp > 0 {
        if exp % 2 == 1 {
            result *= base;
        }
        base *= base;
        exp /= 2;
    }

    result
}

pub fn quick_pow_recursive(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    let mut base = x;
    let mut exp = n as i64;

    if exp < 0 {
        base = 1.0 / base;
        exp = -exp;
    }

    let result = quick_pow_recursive(base, (exp / 2_i64) as i32);
    if n % 2 == 0 {
        result * result
    } else {
        result * result * base
    }
}
