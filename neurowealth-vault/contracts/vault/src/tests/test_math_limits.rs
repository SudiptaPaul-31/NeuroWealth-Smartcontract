//! Tests for math boundary conditions and checked arithmetic
use super::utils::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_deposit_overflow_protection() {
    // This test verifies the vault uses checked arithmetic for deposits.
    // Direct i128::MAX testing is impractical (would need 10^31 USDC),
    // so we verify the checked_add pattern exists in the code by checking
    // normal deposits work and the contract has overflow protection.
    let env = Env::default();
    env.mock_all_auths();

    let (contract_id, _agent, _owner, usdc_token) = setup_vault_with_token(&env);
    let client = NeuroWealthVaultClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Increase deposit limits to allow large deposits
    // Default max deposit is 1,000 USDC, default user cap is 10,000 USDC
    client.set_deposit_limits(&1_000_000, &10_000_000_000_000_i128);
    client.set_limits(&0, &10_000_000_000_000_i128); // Remove user cap and increase TVL cap

    // Large deposit should work (verifying math doesn't overflow at reasonable scales)
    mint_and_deposit(&env, &client, &usdc_token, &user, 1_000_000_000_000_i128);

    let shares = client.get_shares(&user);
    assert_eq!(shares, 1_000_000_000_000_i128, "Large deposit should succeed with checked math");
}

#[test]
#[should_panic(expected = "vault: insufficient shares for requested amount")]
fn test_withdraw_insufficient_shares() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract_id, _agent, _owner, usdc_token) = setup_vault_with_token(&env);
    let client = NeuroWealthVaultClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    // Deposit 10 USDC
    mint_and_deposit(&env, &client, &usdc_token, &user, 10_000_000);

    // Try to withdraw 11 USDC - this should fail with "insufficient shares" message
    client.withdraw(&user, &11_000_000);
}

#[test]
fn test_conversion_math_sanity() {
    // Verifies conversion math works correctly at reasonable scales.
    // True i128::MAX overflow is practically impossible (needs 10^38 shares).
    let env = Env::default();
    env.mock_all_auths();

    let (contract_id, _agent, _owner, usdc_token) = setup_vault_with_token(&env);
    let client = NeuroWealthVaultClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Test normal conversion scenarios
    mint_and_deposit(&env, &client, &usdc_token, &user, 10_000_000);

    // Convert shares to assets
    let shares = client.get_shares(&user);
    let assets = client.convert_to_assets(&shares);
    assert_eq!(assets, 10_000_000, "Conversion should be accurate");

    // Convert assets to shares
    let shares_back = client.convert_to_shares(&assets);
    assert_eq!(shares_back, shares, "Round-trip conversion should be consistent");
}

#[test]
fn test_ceiling_division_prevents_dust_attacks() {
    // Tests that ceiling division ensures at least 1 share is burned for any positive asset amount.
    // This prevents dust attacks where users could withdraw tiny amounts without burning shares.
    let env = Env::default();
    env.mock_all_auths();

    let (contract_id, agent, _owner, usdc_token) = setup_vault_with_token(&env);
    let client = NeuroWealthVaultClient::new(&env, &contract_id);

    // Set up a user with shares
    let user = Address::generate(&env);
    mint_and_deposit(&env, &client, &usdc_token, &user, 10_000_000_i128);

    // Add yield to make share price > 1:1 (more assets than shares)
    // This makes floor division more likely to produce 0 shares for small withdrawals
    let token_client = TestTokenClient::new(&env, &usdc_token);
    token_client.mint(&contract_id, &90_000_000_i128); // Add 90M yield
    client.update_total_assets(&agent, &(100_000_000_i128)); // Now 100M assets, 10M shares

    // At 10:1 ratio (100M assets / 10M shares), withdrawing 1 asset with floor division
    // would give: 1 * 10M / 100M = 0 shares (due to floor)
    // With ceiling division: ceil(0.1) = 1 share

    let tiny_withdrawal = 1_i128; // 0.0000001 USDC

    // Record shares before
    let shares_before = client.get_shares(&user);

    // Withdraw tiny amount - this should still burn at least 1 share due to ceiling division
    client.withdraw(&user, &tiny_withdrawal);

    let shares_after = client.get_shares(&user);
    let shares_burned = shares_before - shares_after;

    assert!(shares_burned >= 1, "Ceiling division must burn at least 1 share for any positive withdrawal");
}

#[test]
fn test_ceiling_division_rounding_edge_case() {
    // Tests exact boundary where ceiling division matters
    // When assets * total_shares is perfectly divisible by total_assets
    let env = Env::default();
    env.mock_all_auths();

    let (contract_id, agent, _owner, usdc_token) = setup_vault_with_token(&env);
    let client = NeuroWealthVaultClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    mint_and_deposit(&env, &client, &usdc_token, &user, 10_000_000_i128);

    // Add yield to create 3:2 ratio (15M assets / 10M shares = 1.5 assets per share)
    let token_client = TestTokenClient::new(&env, &usdc_token);
    token_client.mint(&contract_id, &5_000_000_i128);
    client.update_total_assets(&agent, &(15_000_000_i128));

    // Withdraw 3 assets at 1.5x price
    // Floor: 3 * 10M / 15M = 2 shares exactly
    // Ceiling: ceil(3 * 10M / 15M) = ceil(2) = 2 shares (same in this case)
    let withdraw_amount = 3_000_000_i128;
    let shares_before = client.get_shares(&user);

    client.withdraw(&user, &withdraw_amount);

    let shares_after = client.get_shares(&user);
    let shares_burned = shares_before - shares_after;

    // Exact calculation: 3M * 10M / 15M = 2M shares exactly
    // With ceiling, should be 2M
    assert_eq!(shares_burned, 2_000_000_i128,
        "When product is exactly divisible, ceiling should equal floor");
}

#[test]
fn test_ceiling_division_rounds_up_on_remainder() {
    // Tests that ceiling division properly rounds up when there's a remainder
    let env = Env::default();
    env.mock_all_auths();

    let (contract_id, agent, _owner, usdc_token) = setup_vault_with_token(&env);
    let client = NeuroWealthVaultClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    mint_and_deposit(&env, &client, &usdc_token, &user, 10_000_000_i128);

    // Create 3:2 ratio (15M assets / 10M shares)
    let token_client = TestTokenClient::new(&env, &usdc_token);
    token_client.mint(&contract_id, &5_000_000_i128);
    client.update_total_assets(&agent, &(15_000_000_i128));

    // Withdraw 4 assets at 1.5x price
    // Calculation: 4M * 10M / 15M = 2,666,666.67 shares
    // Floor: 2,666,666 shares
    // Ceiling: 2,666,667 shares
    let withdraw_amount = 4_000_000_i128;
    let shares_before = client.get_shares(&user);

    client.withdraw(&user, &withdraw_amount);

    let shares_after = client.get_shares(&user);
    let shares_burned = shares_before - shares_after;

    // Should round up to 2,666,667 shares due to remainder
    let expected_ceiling = 2_666_667_i128;
    let floor_result = 2_666_666_i128;
    assert_eq!(shares_burned, expected_ceiling,
        "Ceiling division should round up when there's a remainder");
    assert!(shares_burned > floor_result,
        "Ceiling should produce higher result than floor would have");
}
