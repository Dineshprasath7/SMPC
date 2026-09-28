use std::io::{self, Write};

use smpc::{
    save,
    shamir_secret_sharing::{Share, reconstruct_secret, split_secret},
};

pub fn split_cli() -> Result<(), Box<dyn std::error::Error>> {
    println!("========== Shamir Secret Sharing ==========");
    println!("Split a secret into shares.");
    println!();

    let secret_hex = prompt("Secret (hex): ")?;

    let secret = hex::decode(secret_hex.trim())?;

    if secret.is_empty() {
        return Err("Secret cannot be empty".into());
    }

    let n = prompt_u8("Number of shares (n): ")?;

    let k = prompt_u8("Required shares (k): ")?;

    if n == 0 {
        return Err("Number of shares must be greater than zero".into());
    }

    if k == 0 {
        return Err("Required shares must be greater than zero".into());
    }

    if k > n {
        return Err("k cannot be greater than n".into());
    }

    println!();
    println!("Splitting secret...");
    println!("Secret size : {} bytes", secret.len());
    println!("Shares      : {}", n);
    println!("Threshold   : {}", k);
    println!();

    let shares = split_secret(&secret, n, k);

    save::save_json(&shares, "1.json");
    save::save_yaml(&shares, "1.yaml");
    println!("Generated {} shares:", shares.len());
    println!();

    for (i, share) in shares.iter().enumerate() {
        println!("Share {}:", i + 1);

        println!("  x    : {}", share.x);

        println!("  data : {}", hex::encode(&share.data));

        println!();
    }

    Ok(())
}

pub fn reconstruct_cli() -> Result<(), Box<dyn std::error::Error>> {
    println!("========== Shamir Secret Reconstruction ==========");
    println!();

    let count = prompt_u8("Number of shares to reconstruct with: ")?;

    if count == 0 {
        return Err("At least one share is required".into());
    }

    let mut shares = Vec::with_capacity(count as usize);

    for i in 0..count {
        println!();
        println!("Share {}:", i + 1);

        let x = prompt_u8("  x: ")?;

        let data_hex = prompt("  data (hex): ")?;

        let data = hex::decode(data_hex.trim())?;

        if data.is_empty() {
            return Err(format!("Share {} has empty data", i + 1).into());
        }

        shares.push(Share { x, data });
    }

    println!();
    println!("Reconstructing secret...");

    let secret = reconstruct_secret(&shares);

    println!();
    println!("========== Result ==========");
    println!("Secret length : {} bytes", secret.len());
    println!("Secret (hex)  : {}", hex::encode(&secret));

    match String::from_utf8(secret.clone()) {
        Ok(text) => {
            println!("Secret (text) : {}", text);
        }
        Err(_) => {
            println!("Secret (text) : <binary data>");
        }
    }

    Ok(())
}



fn prompt(message: &str) -> io::Result<String> {
    print!("{message}");
    io::stdout().flush()?;

    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn prompt_u8(message: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let value = prompt(message)?;

    Ok(value.parse::<u8>()?)
}
