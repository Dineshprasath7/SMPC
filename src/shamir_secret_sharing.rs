pub const GF256_EXP: [u8; 512] = generate_exp_table();
pub const GF256_LOG: [u8; 256] = generate_log_table();

const fn generate_exp_table() -> [u8; 512] {
    let mut table = [0u8; 512];
    let mut x = 1u16;

    let mut i = 0;
    while i < 255 {
        table[i] = x as u8;
        table[i + 255] = x as u8;

        let original = x;
        x <<= 1;
        if x & 0x100 != 0 {
            //For AES, that polynomial is 0x11b ($x^8 + x^4 + x^3 + x + 1$)
            x ^= 0x11b; // XOR with AES irreducible polynomial
        }

        x ^= original; //3 is a primitve element
        i += 1
    }
    table
}

const fn generate_log_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut x = 1u16;
    let mut i = 0;
    while i < 255 {
        table[x as usize] = i as u8;

        let original = x;

        x <<= 1;
        if x & 0x100 != 0 {
            //For AES, that polynomial is 0x11b ($x^8 + x^4 + x^3 + x + 1$). => 0x11b (283)10
            x ^= 0x11b;
        }

        x ^= original;
        i += 1;
    }
    table[0] = 0;
    table
}

pub fn gf_mul(a: u8, b: u8) -> u8 {
    if a == 0 || b == 0 {
        return 0;
    }

    let log_a = GF256_LOG[a as usize] as usize;
    let log_b = GF256_LOG[b as usize] as usize;

    GF256_EXP[log_a + log_b]
}

pub fn gf_div(a: u8, b: u8) -> u8 {
    if a == 0 {
        return 0;
    }

    assert!(b != 0, "Division by zero");

    let log_a = GF256_LOG[a as usize] as usize;
    let log_b = GF256_LOG[b as usize] as usize;

    GF256_EXP[log_a + 255 - log_b]
}

#[derive(Debug, Clone)]
pub struct Share {
    pub x: u8,
    pub data: Vec<u8>,
}
use rand::RngExt;

pub fn split_secret(secret: &[u8], n: u8, k: u8) -> Vec<Share> {
    assert!(k >= 2);
    assert!(k <= n);
    let mut rng = rand::rng();

    let mut shares = (1..=n)
        .map(|x| Share {
            x,
            data: Vec::with_capacity(secret.len()),
        })
        .collect::<Vec<_>>();

    println!("{:?}", shares);
    for &byte in secret {
        let coeffs: Vec<u8> = (0..k - 1).map(|_| rng.random::<u8>()).collect();

        for share in &mut shares {
            let mut y = *coeffs.last().unwrap();

            for c in coeffs[..coeffs.len() - 1].iter().rev() {
                y = gf_mul(y, share.x) ^ *c;
            }

            y = gf_mul(y, share.x) ^ byte;

            share.data.push(y);
        }
    }
    shares
}







































pub fn reconstruct_secret(shares: &[Share]) -> Vec<u8> {
    assert!(!shares.is_empty());

    let len = shares[0].data.len();
    let mut secret = Vec::with_capacity(len);

    for byte in 0..len {
        let mut secret_byte = 0;

        for i in 0..shares.len() {
            let xi = shares[i].x;
            let yi = shares[i].data[byte];
            let mut numerator = 1;
            let mut denominator = 1;

            for j in 0..shares.len() {
                if i == j {
                    continue;
                }
                let xj = shares[j].x;

                numerator = gf_mul(numerator, xj);
                denominator = gf_mul(denominator, xi ^ xj);
            }
            let lambda = gf_div(numerator, denominator);

            secret_byte ^= gf_mul(yi, lambda);
        }
        secret.push(secret_byte);
    }

    secret
}
