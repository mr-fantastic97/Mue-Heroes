use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::blocking::Client;
use serde::Serialize;
use chrono::prelude::*;

const BACKEND_URL: &str = "http://127.0.0.1:8000/submit"; // Change if needed

#[derive(Serialize)]
struct Submission {
    wallet: String,
    score: u32,
    mu_level: u8,
    block_height: u64,
    date_mined: String,
    event_type: String, // "mined" or "witness"
}

fn generate_wallets(n: usize) -> Vec<String> {
    (0..n)
        .map(|_| {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect()
        })
        .collect()
}

fn simulate_submission(client: &Client, wallet: &str, event_type: &str, t: f64) {
    let d_actual = thread_rng().gen_range(1.0..(t / 100.0));
    let mu_level = (t / d_actual).log2().floor() as u8;
    let score = if event_type == "mined" { mu_level as u32 } else { 5 };

    let block_height = thread_rng().gen_range(500_000..1_000_000);
    let date_mined = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let payload = Submission {
        wallet: wallet.to_string(),
        score,
        mu_level,
        block_height,
        date_mined,
        event_type: event_type.to_string(),
    };

    let res = client.post(BACKEND_URL).json(&payload).send();

    match res {
        Ok(resp) => println!("✅ Submitted for {}: {:?}", wallet, resp.status()),
        Err(e) => eprintln!("❌ Failed for {}: {:?}", wallet, e),
    }
}

fn main() {
    let client = Client::new();
    let wallets = generate_wallets(4);
    let t: f64 = 2_f64.powi(256); // ✅ Safe inside a function

    for wallet in &wallets {
        simulate_submission(&client, wallet, "mined", t);
        simulate_submission(&client, wallet, "witness", t);
    }
}
