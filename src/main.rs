use clap::Parser;
use oreos::cli::{Command, Transaction, CLI};
use oreos::{causal_send, create_account, decrypt_tx};

fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();
    let result = match cli.command {
        Command::Account(acc) => create_account(acc),
        Command::Transaction(tx) => match tx {
            Transaction::Decrypt {
                hash,
                incoming_viewkey,
                outgoing_viewkey,
                endpoint,
            } => decrypt_tx(hash, incoming_viewkey, outgoing_viewkey, endpoint),
            Transaction::Send {
                hash,
                incoming_viewkey,
                outgoing_viewkey,
                spending_key,
                receiver,
                amount,
                fee,
                expiration,
                memo,
                endpoint,
            } => causal_send(
                hash,
                incoming_viewkey,
                outgoing_viewkey,
                spending_key,
                receiver,
                amount,
                fee,
                expiration,
                memo,
                endpoint,
            ),
        },
    };
    match result {
        Ok(output) => println!("{}\n", output),
        Err(error) => println!("⚠️  {error}\n"),
    };
    Ok(())
}
