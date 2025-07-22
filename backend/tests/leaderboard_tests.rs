use backend::handlers::leaderboard::{generate_leaderboard, LeaderboardEntry};
use backend::handlers::submission::Submission;


fn date(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}

fn make_submission(wallet: &str, score: u32, mu_level: u8, block_height: u64, date_str: &str, event_type: &str) -> Submission {
    Submission {
        wallet: wallet.to_string(),
        score,
        mu_level,
        block_height,
        date_mined: date(date_str),
        event_type: event_type.to_string(),
    }
}

#[test]
fn test_wallet_mines_twice_score_accumulates() {
    // Edge Case 1: Wallet mines twice
    let submissions = vec![
        make_submission("alice", 10, 15, 100, "2025-07-01", "mined"),
        make_submission("alice", 20, 16, 101, "2025-07-02", "mined"),
    ];

    let leaderboard = generate_leaderboard(&submissions);
    let alice = leaderboard.iter().find(|e| e.wallet == "alice").unwrap();
    assert_eq!(alice.score, 30);
}

#[test]
fn test_wallet_witnesses_twice_score_accumulates() {
    // Edge Case 2: Wallet witnesses twice
    let submissions = vec![
        make_submission("bob", 5, 15, 110, "2025-07-01", "witness"),
        make_submission("bob", 7, 16, 111, "2025-07-03", "witness"),
    ];

    let leaderboard = generate_leaderboard(&submissions);
    let bob = leaderboard.iter().find(|e| e.wallet == "bob").unwrap();
    assert_eq!(bob.score, 12);
}

#[test]
fn test_wallet_mines_and_witnesses_score_accumulates() {
    // Edge Case 3: Wallet mines and witnesses
    let submissions = vec![
        make_submission("carol", 8, 15, 120, "2025-07-01", "mined"),
        make_submission("carol", 4, 15, 121, "2025-07-02", "witness"),
    ];

    let leaderboard = generate_leaderboard(&submissions);
    let carol = leaderboard.iter().find(|e| e.wallet == "carol").unwrap();
    assert_eq!(carol.score, 12);
}

#[test]
fn test_multiple_wallets_correct_ranking_by_total_score() {
    // Edge Case 4: Multiple wallets ranked by total score
    let submissions = vec![
        make_submission("dave", 15, 15, 130, "2025-07-01", "mined"),
        make_submission("erin", 20, 15, 131, "2025-07-01", "mined"),
    ];

    let leaderboard = generate_leaderboard(&submissions);
    assert_eq!(leaderboard[0].wallet, "erin");
    assert_eq!(leaderboard[1].wallet, "dave");
}

#[test]
fn test_tie_breaker_by_latest_date_mined() {
    // Edge Case 5: Tie-breaker using date_mined
    let submissions = vec![
        make_submission("frank", 20, 15, 140, "2025-07-01", "mined"),
        make_submission("gina", 20, 15, 141, "2025-07-03", "mined"),
    ];

    let leaderboard = generate_leaderboard(&submissions);
    assert_eq!(leaderboard[0].wallet, "gina");
    assert_eq!(leaderboard[1].wallet, "frank");
}

#[test]
fn test_tier_based_on_highest_mu_level() {
    // Edge Case 6: Tier assigned using highest mu_level
    let submissions = vec![
        make_submission("hannah", 10, 15, 150, "2025-07-01", "mined"),
        make_submission("hannah", 5, 18, 151, "2025-07-02", "mined"),
    ];

    let leaderboard = generate_leaderboard(&submissions);
    let hannah = leaderboard.iter().find(|e| e.wallet == "hannah").unwrap();
    assert_eq!(hannah.tier, "🦁 μLegend");
}

#[test]
fn test_miner_ranks_above_witness_on_identical_score_and_date() {
    // Edge Case 7: Miner outranks witness with same score, μ, and date
    let submissions = vec![
        make_submission("isaac", 10, 15, 160, "2025-07-02", "witness"),
        make_submission("judy", 10, 15, 161, "2025-07-02", "mined"),
    ];

    let leaderboard = generate_leaderboard(&submissions);
    assert_eq!(leaderboard[0].wallet, "judy");
    assert_eq!(leaderboard[1].wallet, "isaac");
}
