use rand::{distributions::Alphanumeric, seq::SliceRandom, thread_rng, Rng};
use reqwest::blocking::Client;
use serde::Serialize;
use chrono::prelude::*;
use std::{thread, time::Duration};

const BACKEND_URL: &str = "http://127.0.0.1:8000/submit"; // Update if needed

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

//  Tiered scoring logic
fn calculate_score_and_tier(mu_level: u8, event_type: &str) -> (u32, &'static str) {
    if event_type == "witness" {
        return (5, "μOracle");
    }

    match mu_level {
        15 => (50, "μScout"),
        16..=17 => (75, "μForged"),
        18 => (100, "μLegend"),
        19..=20 => (250, "μMythic"),
        21..=u8::MAX => (500, "μHonorius"),
        _ => (0, "Unknown"),
    }
}

fn simulate_submission(
    client: &Client,
    wallet: &str,
    event_type: &str,
    mu_level: u8,
    block_height: u64,
    date_mined: &str,
) {
    // Use new tiered scoring
    let (score, tier) = calculate_score_and_tier(mu_level, event_type);

    let payload = Submission {
        wallet: wallet.to_string(),
        score,
        mu_level,
        block_height,
        date_mined: date_mined.to_string(),
        event_type: event_type.to_string(),
    };

    let res = client.post(BACKEND_URL).json(&payload).send();

    match res {
        Ok(_) => println!(
            "✅ Submitted [{}] for {} → μ = {}, Tier = {}, Score = {}",
            event_type, wallet, mu_level, tier, score
        ),
        Err(e) => eprintln!("❌ Failed for {}: {:?}", wallet, e),
    }
}

fn main() {
    let client = Client::new();
    let t: f64 = 2_f64.powi(256);
    let mut round = 0;

    loop {
        round += 1;
        println!("⏳ Simulating block round #{}...", round);

        // Generate 1 to 6 wallets
        let wallet_count = thread_rng().gen_range(1..=6);
        let mut wallets = generate_wallets(wallet_count);
        wallets.shuffle(&mut thread_rng());

        //  Simulate μ-level
        let d_actual = thread_rng().gen_range(1.0..(t / 850.0)); // Adjust difficulty spread
        let mu_level = (t / d_actual).log2().floor() as u8;

        if mu_level < 15 {
            println!("⏭ Skipped (μ = {}) — Not eligible", mu_level);
        } else {
            let block_height = thread_rng().gen_range(500_000..1_000_000);
            let date_mined = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

            // 👷 Miner
            let miner = &wallets[0];
            simulate_submission(
                &client,
                miner,
                "mined",
                mu_level,
                block_height,
                &date_mined,
            );

            // 🧾 Witnesses
            let witnesses = &wallets[1..];
            if !witnesses.is_empty() {
                let witness_count = thread_rng().gen_range(1..=witnesses.len());
                let selected_witnesses = &witnesses[..witness_count];

                for witness in selected_witnesses {
                    simulate_submission(
                        &client,
                        witness,
                        "witness",
                        mu_level,
                        block_height,
                        &date_mined,
                    );
                }
            }
        }

      //simulated Kaspa block time
        thread::sleep(Duration::from_millis(1000));
    }
}
