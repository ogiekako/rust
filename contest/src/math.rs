/// math library.

pub fn inv(value: i32, modulo_prime: i32) -> i32 {
    return pow_mod(value, modulo_prime - 2, modulo_prime);
}

pub fn pow_mod(value: i32, exponent: i32, modulo: i32) -> i32 {
    if exponent < 0 {
        panic!("Exponent should >= 0.");
    }
    if modulo == 1 {
        return 0;
    }
    let modulo = modulo as i64;
    let mut value = value as i64;
    if value >= modulo {
        value %= modulo;
    }
    if value < 0 {
        value = value % modulo + value;
    }
    let mut res = 1;
    let mut exponent = exponent;
    while exponent > 0 {
        if (exponent & 1) == 1 {
            res = res * value % modulo;
        }
        value = value * value % modulo;
        exponent >>= 1;
    }
    res as i32
}
