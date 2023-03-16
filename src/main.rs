use clap::Parser;
use ironfish_rust::{
    errors::IronfishError,
    keys::{Language, SaplingKey},
    IncomingViewKey, MerkleNote, OutgoingViewKey,
};

#[derive(Debug, Parser, Clone)]
#[clap(name = "oreos", author = "hairtail")]
#[clap(author, version, about, long_about = None)]
pub enum Cli {
    /// Create a new wallet
    Create,
    /// Recover wallet from spendingKey | mnemonic
    Recover(Recover),
    /// Decrypt an encrypted note
    Decrypt(Decrypt),
}

#[derive(Debug, Parser, Clone)]
pub struct Recover {
    /// Mnemonic or spendingKey used to recover wallet from
    #[clap(short, long)]
    pub data: String,
    /// Language if mnemonic is used
    #[clap(short, long, default_value_t=String::from("en"))]
    pub language: String,
}

#[derive(Debug, Parser, Clone)]
pub struct Decrypt {
    /// Raw data of encrypted note
    #[clap(short, long)]
    pub data: String,
    /// Hex encoded account incoming view key
    #[clap(short, long)]
    pub incoming_viewkey: String,
    /// Hex encoded account outgoing view key
    #[clap(short, long)]
    pub outgoing_viewkey: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let result: Result<String, IronfishError> = match cli {
        Cli::Create => {
            let key = SaplingKey::generate_key();
            Ok(key.to_string())
        }
        Cli::Recover(Recover { data, language }) => recover_key(data, language),
        Cli::Decrypt(Decrypt {
            data,
            incoming_viewkey,
            outgoing_viewkey,
        }) => decrypt_encnote(data, incoming_viewkey, outgoing_viewkey),
    };
    match result {
        Ok(output) => println!("{output}\n"),
        Err(error) => println!("⚠️  {error}\n"),
    };
    Ok(())
}

fn recover_key(data: String, language: String) -> Result<String, IronfishError> {
    let key_from_spending = SaplingKey::from_hex(&data);
    match key_from_spending {
        Ok(key) => Ok(key.to_string()),
        Err(_error) => SaplingKey::from_words(
            data,
            Language::from_language_code(&language).unwrap_or(Language::English),
        )
        .map(|key| key.to_string()),
    }
}

fn decrypt_encnote(
    data: String,
    incoming_key: String,
    outgoing_key: String,
) -> Result<String, IronfishError> {
    let data = hex::decode(data).unwrap();
    let note_enc = MerkleNote::read(&data[..])?;

    let incoing_view_key = IncomingViewKey::from_hex(&incoming_key)?;
    let note_as_receiver = note_enc.decrypt_note_for_owner(&incoing_view_key);
    if let Ok(note) = note_as_receiver {
        return Ok(note.to_string());
    }

    let outgoing_view_key = OutgoingViewKey::from_hex(&outgoing_key)?;
    let note_as_spender = note_enc.decrypt_note_for_spender(&outgoing_view_key);
    if let Ok(note) = note_as_spender {
        return Ok(note.to_string());
    }

    Ok("You are not owner/spender of this note".to_string())
}
