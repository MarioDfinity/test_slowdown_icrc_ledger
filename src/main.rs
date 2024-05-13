use candid::{Decode, Encode, Nat, Principal};
use clap::Parser;
use elliptic_curve::SecretKey;
use ic_agent::Agent;
use ic_agent::identity::{BasicIdentity, Secp256k1Identity};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{Memo, TransferArg, TransferError};
use rasciigraph::{plot, Config};
use ring::signature::Ed25519KeyPair;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ic_url: String,

    #[arg(short, long)]
    ledger_id: String,

}

const TEST_PRINCIPAL: &str = "imx2d-dctwe-ircfz-emzus-bihdn-aoyzy-lkkdi-vi5vw-npnik-noxiy-mae";
const TEST_PRIVATE_KEY: &str = "-----BEGIN EC PRIVATE KEY-----
MHQCAQEEIIBzyyJ32Kdjixx+ZJvNeUWsqAzSQZfLsOyXKgxc7aH9oAcGBSuBBAAK
oUQDQgAECWc6ZRn9bBP96RM1G6h8ZAtbryO65dKg6cw0Oij2XbnAlb6zSPhU+4hh
gc2Q0JiGrqKks1AVi+8wzmZ+2PQXXA==
-----END EC PRIVATE KEY-----";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let identity = Secp256k1Identity::from_private_key(SecretKey::from_sec1_pem(TEST_PRIVATE_KEY)?);
    let agent = Agent::builder()
        .with_url(args.ic_url)
        .with_identity(identity)
        .build()?;
    agent.fetch_root_key().await?;
    let ledger_id = Principal::from_text(args.ledger_id)?;

    let start_time = chrono::offset::Local::now();
    let result_file = format!("result_{}.csv", start_time.to_rfc3339());
    let mut out = std::fs::File::create(result_file)?;
    let mut times = vec![];
    for i in 0..100_000u64 {
        let before = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        let mut subaccount = [0u8; 32];
        subaccount[24..].copy_from_slice(i.to_be_bytes().as_slice());
        let to = Account {
            owner: ledger_id,
            subaccount: Some(subaccount),
        };
        let created_at_time = Some(before);
        let memo = Some(Memo::from(vec![1u8;32]));
        let arg = TransferArg {
            from_subaccount: None,
            to,
            fee: None,
            created_at_time,
            memo,
            amount: Nat::from(1_000_000u64),
        };
        let res = agent.update(&ledger_id, "icrc1_transfer")
            .with_arg(Encode!(&arg).unwrap())
            .call_and_wait()
            .await?;
        let res = Decode!(&res, Result<Nat, TransferError>);
        let after = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        let elapsed = after - before;
        println!("{i} nanos elapsed: {elapsed} res: {res:?}");
        out.write_all(format!("{i}, {elapsed}, {res:?}\n").as_bytes())?;
        times.push((i, elapsed as f64, res));
    }

    Ok(())
}
