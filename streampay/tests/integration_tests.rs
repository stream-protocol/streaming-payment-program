use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use crate::instruction::StreamPayInstruction;
use crate::error::PaymentError;

// Constants for testing
const TEST_START_TIME: u64 = 100;
const TEST_INTERVAL: u64 = 10;
const TEST_AMOUNT_PER_INTERVAL: u64 = 5;

// Utility Functions
// -----------------

/// Create a new payment stream for testing purposes.
async fn create_payment_stream(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &solana_sdk::hash::Hash,
) -> Result<(), ProgramError> {
    let instruction = StreamPayInstruction::Create {
        start_time: TEST_START_TIME,
        interval: TEST_INTERVAL,
        amount_per_interval: TEST_AMOUNT_PER_INTERVAL,
    };

    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer], *recent_blockhash);
    banks_client.process_transaction(transaction).await?;

    Ok(())
}

/// Withdraw from an existing payment stream.
async fn withdraw_from_stream(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &solana_sdk::hash::Hash,
    amount: u64,
) -> Result<(), ProgramError> {
    let instruction = StreamPayInstruction::Withdraw { amount };

    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer], *recent_blockhash);
    banks_client.process_transaction(transaction).await?;

    Ok(())
}

// Test Cases
// ----------

#[tokio::test]
async fn test_create_payment_stream() {
    let program_test = ProgramTest::new(
        "streampay",
        crate::id(),
        processor!(crate::streampay::process_instruction),
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let result = create_payment_stream(&mut banks_client, &payer, &recent_blockhash).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_withdraw_from_stream() {
    let program_test = ProgramTest::new(
        "streampay",
        crate::id(),
        processor!(crate::streampay::process_instruction),
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // First, create a payment stream
    create_payment_stream(&mut banks_client, &payer, &recent_blockhash).await.unwrap();

    // Now, test withdrawal
    let amount_to_withdraw = TEST_AMOUNT_PER_INTERVAL - 1; // Just an example
    let result = withdraw_from_stream(&mut banks_client, &payer, &recent_blockhash, amount_to_withdraw).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_invalid_withdrawal_amount() {
    let program_test = ProgramTest::new(
        "streampay",
        crate::id(),
        processor!(crate::streampay::process_instruction),
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // First, create a payment stream
    create_payment_stream(&mut banks_client, &payer, &recent_blockhash).await.unwrap();

    // Try to withdraw an invalid amount
    let invalid_amount = TEST_AMOUNT_PER_INTERVAL + 10; // Just an example
    let result = withdraw_from_stream(&mut banks_client, &payer, &recent_blockhash, invalid_amount).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PaymentError::InsufficientFunds.into());
}

// Add more test cases as needed...
