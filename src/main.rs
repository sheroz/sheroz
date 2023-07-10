fn main() {
        println!("Output of basic usage of tick counter:");
        tick_counter_basic_usage(); 
    
        println!("Output of sample usage of tick counter:");
        tick_counter_sample_usage();

        println!("Output of Magma sample of block encryption:");
        magma_encrypt_block();

        println!("Output of Magma sample of buffer encryption in ECB mode:");
        magma_encrypt_text_ecb(); 
}

fn tick_counter_basic_usage() {
    use std::{thread, time};
    use tick_counter::*;
    let duration = time::Duration::from_millis(20); 
    let start = tick_counter_start();
    thread::sleep(duration);
    let elapsed_ticks = tick_counter_stop() - start;
    println!("Number of elapsed ticks in {:?}: {}", duration, elapsed_ticks);
}

fn tick_counter_sample_usage() {
    use std::{thread, time, env::consts};
    use tick_counter::*;

    println!("Environment: {}/{} {}", consts::OS, consts::FAMILY, consts::ARCH);

    let (counter_frequency, accuracy) = tick_counter_frequency();
    println!("Tick frequency, MHZ: {}", counter_frequency as f64 / 1e6_f64);
    let estimation_source = match accuracy {
        TickCounterFrequencyBase::Hardware => "hardware".to_string(),
        TickCounterFrequencyBase::Measured(duration) => format!("software, estimated in {:?}", duration)
    };
    println!("Tick frequency is provided by: {}", estimation_source);

    let counter_accuracy = tick_counter_precision_nanoseconds(counter_frequency);
    println!("Tick accuracy, nanoseconds: {}", counter_accuracy);

    let counter_start = tick_counter_start();
    thread::sleep(time::Duration::from_secs(1));
    let counter_stop = tick_counter_stop();

    println!("Tick counter start: {}", counter_start);
    println!("Tick counter stop: {}", counter_stop);
    
    let elapsed_ticks = counter_stop - counter_start;
    println!("Elapsed ticks count in ~1 seconds thread::sleep(): {}", elapsed_ticks);

    let elapsed_nanoseconds = (elapsed_ticks as f64) * counter_accuracy;
    println!("Elapsed nanoseconds according to elapsed ticks: {}", elapsed_nanoseconds);
}

/// Magma sample of block encryption
fn magma_encrypt_block() {
    use cipher_magma::Magma;

    let mut magma = Magma::new();

    let cipher_key: [u32;8] = [
        0xffeeddcc, 0xbbaa9988, 0x77665544, 0x33221100, 0xf0f1f2f3, 0xf4f5f6f7, 0xf8f9fafb, 0xfcfdfeff
    ];
    magma.set_key(&cipher_key);

    let source = 0xfedcba9876543210_u64;
    println!("Source block: {:x}", source);

    let encrypted = magma.encrypt(source);
    println!("Encrypted ciphertext: {:x}", encrypted);

    let decrypted = magma.decrypt(encrypted);
    println!("Decrypted block: {:x}", decrypted);
}

/// Magma sample of buffer encryption in ECB mode
fn magma_encrypt_text_ecb() {
    use cipher_magma::{Magma, CipherMode};

    let source_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
        Aenean ac sem leo. Morbi pretium neque eget felis finibus convallis. \
        Praesent tristique rutrum odio at rhoncus. Duis non ligula ut diam tristique commodo. \
        Phasellus vel ex nec leo pretium efficitur. Aliquam malesuada vestibulum magna. \
        Quisque iaculis est et est volutpat posuere.";

    println!("Source text:\n{}\n", source_text);

    let source_bytes = source_text.as_bytes();

    let cipher_key: [u32;8] = [
        0xffeeddcc, 0xbbaa9988, 0x77665544, 0x33221100, 0xf0f1f2f3, 0xf4f5f6f7, 0xf8f9fafb, 0xfcfdfeff
    ];
    let mut magma = Magma::new_with_key(&cipher_key);
    let encrypted = magma.encrypt_buffer(source_bytes, CipherMode::ECB);
    println!("Encrypted ciphertext:\n{:x?}\n", encrypted);

    let mut decrypted = magma.decrypt_buffer(&encrypted, CipherMode::ECB);

    // remove padding bytes
    decrypted.truncate(source_bytes.len());

    let decrypted_text = String::from_utf8(decrypted).unwrap();
    println!("Decrypted text:\n{}\n", decrypted_text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test() {
        main();
    }

    #[test]
    fn tick_counter_basic_usage_test() {
        tick_counter_basic_usage(); 
    }

    #[test]
    fn tick_counter_sample_usage_test() {
        tick_counter_sample_usage();
    }

    #[test]
    fn sample_encrypt_block_test() {
        magma_encrypt_block();
    }

    #[test]
    fn sample_encrypt_text_ecb_test() {
        magma_encrypt_text_ecb();
    }
}