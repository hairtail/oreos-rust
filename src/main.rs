use clap::Parser;
use ironfish_rust::keys::SaplingKey;
use oreos::{decrypt_encrypted_note_print, decrypt_tx_print, recover_key, Cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let result = match cli {
        Cli::Create => {
            let key = SaplingKey::generate_key();
            Ok(key.to_string())
        }
        Cli::Recover(key) => recover_key(key),
        Cli::Decrypt(data) => decrypt_encrypted_note_print(data),
        Cli::Watch(tx) => decrypt_tx_print(tx),
    };
    match result {
        Ok(output) => println!("{}\n", output),
        Err(error) => println!("⚠️  {error}\n"),
    };
    Ok(())
}
