use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use crate::payment_stream::PaymentStream;
use crate::error::PaymentError;

// Constants for testing
const TEST_START_TIME: u64 = 100;
const TEST_INTERVAL: u64 = 10;
const TEST_AMOUNT_PER_INTERVAL: u64 = 5;

#[tokio::test]
async fn test_payment_stream_creation() {
    let payment_stream = PaymentStream::new(TEST_START_TIME, TEST_INTERVAL, TEST_AMOUNT_PER_INTERVAL);
    
    assert_eq!(payment_stream.start_time, TEST_START_TIME);
    assert_eq!(payment_stream.interval, TEST_INTERVAL);
    assert_eq!(payment_stream.amount_per_interval, TEST_AMOUNT_PER_INTERVAL);
    assert_eq!(payment_stream.withdrawn_amount, 0);
    assert_eq!(payment_stream.is_terminated(), false);
    assert_eq!(payment_stream.is_spl_token, false);
}

#[tokio::test]
async fn test_payment_stream_termination() {
    let mut payment_stream = PaymentStream::new(TEST_START_TIME, TEST_INTERVAL, TEST_AMOUNT_PER_INTERVAL);
    payment_stream.terminate();
    
    assert_eq!(payment_stream.is_terminated(), true);
}

#[tokio::test]
async fn test_payment_stream_fee_calculation() {
    let payment_stream = PaymentStream::new(TEST_START_TIME, TEST_INTERVAL, TEST_AMOUNT_PER_INTERVAL);
    let fee = payment_stream.calculate_fee(TEST_AMOUNT_PER_INTERVAL);
    
    assert_eq!(fee, (TEST_AMOUNT_PER_INTERVAL as f64 * 0.015) as u64);
}

#[tokio::test]
async fn test_payment_stream_pack_unpack() {
    let payment_stream = PaymentStream::new(TEST_START_TIME, TEST_INTERVAL, TEST_AMOUNT_PER_INTERVAL);
    let mut packed_data = vec![0u8; PaymentStream::LEN];
    PaymentStream::pack_into_slice(&payment_stream, &mut packed_data);
    
    let unpacked_stream = PaymentStream::unpack_from_slice(&packed_data).unwrap();
    assert_eq!(unpacked_stream, payment_stream);
}

// Add more test cases as needed...
