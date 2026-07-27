use smpc::aes_gcm::{decrypt, encrypt};
use smpc::ed25519::sign_and_verify_msg;
use smpc::shamir_secret_sharing::{GF256_EXP, GF256_LOG, reconstruct_secret, split_secret};
use smpc::shamir_secret_sharing::{gf_div, gf_mul};
use std::time::Instant;

fn start_test() {
    // println!("Hello, world!");

    println!("==============GF(256) EXPONENTIAL TABLE===========");
    println!("{:?}", GF256_EXP);
    println!("==============GF(256) EXPONENTIAL TABLE===========\n");

    println!("\n==============GF(256) LOGARITHM TABLE===========");
    println!("{:?}", GF256_LOG);
    println!("==============GF(256) LOGARITHM TABLE===========\n");

    assert_eq!(gf_mul(15, 17), 255);
    assert_eq!(gf_div(255, 15), 17);

    for x in 1..=255u8 {
        assert_eq!(gf_div(gf_mul(x, 42), 42), x);
    }

    println!("15 * 17 = {}", gf_mul(15, 17));
    assert_eq!(gf_mul(15, 17), 255);
    println!("Passed!");

    assert_eq!(gf_mul(15, 17), 255);

    for i in 1u8..=255 {
        assert_eq!(GF256_EXP[GF256_LOG[i as usize] as usize], i);
    }

    let start = Instant::now();

    for _ in 0..1_000_000 {
        let _ = gf_mul(157, 93);
    }

    println!("Table: {:?}", start.elapsed());

    let key: [u8; 32] = rand::random();

    let plaintext = b"Hello, World!";

    // Encrypt with the original key
    let (ciphertext, nonce) = encrypt(&key, plaintext).unwrap();

    println!("Ciphertext: {:?}", ciphertext);
    println!("Nonce: {:?}", nonce);
    // Split the key into shares
    let shares = split_secret(&key, 4, 3);

    // // Recover the key from any 3 shares
    // let recovered = reconstruct_secret(&shares[..1]);

    // let recovered_key: [u8; 32] = recovered.try_into().expect("Expected 32-byte key");

    // // Decrypt using the reconstructed key
    // let decrypted = decrypt(&recovered_key, &nonce, &ciphertext).unwrap();

    // println!("{}", String::from_utf8(decrypted).unwrap());

    let check = sign_and_verify_msg();
    println!("{check}")
}

use std::io::{self, Write};

fn main() {
    println!("Welcome to the CLI app! Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        let args: Vec<&str> = input.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "hello" => hello(),
            "add" => {
                if args.len() < 3 {
                    print!("Enter first number: ");
                    io::stdout().flush().unwrap();
                    let mut num1 = String::new();
                    io::stdin().read_line(&mut num1).unwrap();
                    print!("Enter second number: ");
                    io::stdout().flush().unwrap();
                    let mut num2 = String::new();
                    io::stdin().read_line(&mut num2).unwrap();
                    let n1: i32 = num1.trim().parse().unwrap_or(0);
                    let n2: i32 = num2.trim().parse().unwrap_or(999999999);
                    add(n1, n2);
                } else {
                    let n1: i32 = args[1].parse().unwrap_or(0);
                    let n2: i32 = args[2].parse().unwrap_or(0);
                    add(n1, n2);
                }
            }
            "exit" => {
                exit();
                break;
            }
            _ => println!("Unknown command: {}", args[0]),
        }
    }
}

fn hello() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) {
    println!("Result: {}", a + b);
}

fn exit() {
    println!("Goodbye!");
}
