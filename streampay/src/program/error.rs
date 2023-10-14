use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum PaymentError {
    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Insufficient funds for withdrawal")]
    InsufficientFunds,

    #[error("Payment stream is not initialized")]
    StreamNotInitialized,

    #[error("Payment stream is already terminated")]
    StreamAlreadyTerminated,

    #[error("Payment stream is already paused")]
    StreamAlreadyPaused,

    #[error("Invalid start time")]
    InvalidStartTime,

    // Add more custom error variants as needed
}

#[derive(Error, Debug, Copy, Clone)]
pub enum StreamError {
    #[error("Invalid instruction")]
    InvalidInstruction,

    #[error("Sender is not the signer")]
    SenderNotSigner,

    #[error("Recipient is not the signer")]
    RecipientNotSigner,

    #[error("Invalid time frame")]
    InvalidTimeFrame,

    // Add more custom error variants as needed
}

impl From<PaymentError> for ProgramError {
    fn from(e: PaymentError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl From<StreamError> for ProgramError {
    fn from(e: StreamError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
