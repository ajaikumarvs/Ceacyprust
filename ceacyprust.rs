use std::io;

fn main() {
    println!("CEACYPRUST");
    println!("----------");

    loop {
        println!("Select an option:");
        println!("1. Encrypt");
        println!("2. Decrypt");
        println!("3. Exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read input");
        let option: u8 = option.trim().parse().expect("Invalid option");

        match option {
            1 => encrypt_message(),
            2 => decrypt_message(),
            3 => {
                println!("Exiting the program.");
                break;
            },
            _ => println!("Invalid option"),
        }
    }
}

fn encrypt_message() {
    println!("Enter the message to encrypt:");
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read input");

    println!("Enter the shift value (0-25):");
    let mut shift = String::new();
    io::stdin().read_line(&mut shift).expect("Failed to read input");
    let shift: u8 = shift.trim().parse().expect("Invalid shift value");

    println!("Encrypting...");

    let mut encrypted_message = String::new();
    for c in message.chars() {
        if c.is_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8) - base;
            let encrypted_offset = (offset + shift) % 26;
            let encrypted_char = (base + encrypted_offset) as char;
            println!("Input: {} -> Encrypted: {}", c, encrypted_char);
            encrypted_message.push(encrypted_char);
        } else {
            encrypted_message.push(c);
        }
    }

    println!("Encrypted message: {}", encrypted_message);
}

fn decrypt_message() {
    println!("Enter the message to decrypt:");
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read input");

    println!("Enter the shift value (0-25):");
    let mut shift = String::new();
    io::stdin().read_line(&mut shift).expect("Failed to read input");
    let shift: u8 = shift.trim().parse().expect("Invalid shift value");

    println!("Decrypting...");

    let mut decrypted_message = String::new();
    for c in message.chars() {
        if c.is_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8) - base;
            let decrypted_offset = (offset + 26 - shift) % 26;
            let decrypted_char = (base + decrypted_offset) as char;
            println!("Input: {} -> Decrypted: {}", c, decrypted_char);
            decrypted_message.push(decrypted_char);
        } else {
            decrypted_message.push(c);
        }
    }

    println!("Decrypted message: {}", decrypted_message);
}

fn encrypt(message: &str, shift: u8) -> String {
    let mut result = String::new();

    for c in message.chars() {
        if c.is_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8) - base;
            let encrypted_offset = (offset + shift) % 26;
            let encrypted_char = (base + encrypted_offset) as char;
            result.push(encrypted_char);
        } else {
            result.push(c);
        }
    }

    result
}

fn decrypt(encrypted_message: &str, shift: u8) -> String {
    let mut result = String::new();

    for c in encrypted_message.chars() {
        if c.is_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8) - base;
            let decrypted_offset = (offset + 26 - shift) % 26;
            let decrypted_char = (base + decrypted_offset) as char;
            result.push(decrypted_char);
        } else {
            result.push(c);
        }
    }

    result
}
