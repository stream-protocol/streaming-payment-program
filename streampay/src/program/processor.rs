use crate::{
    instruction::StreamPayInstruction,
    state::PaymentStream,
    error::{StreamError, PaymentError},
    constants::{MINIMUM_AMOUNT, OPERATIONAL_FEE_RATE, MAX_WITHDRAWAL_AMOUNT},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = StreamPayInstruction::unpack(instruction_data)?;

        match instruction {
            StreamPayInstruction::InitializeStream {
                start_time,
                interval,
                amount_per_interval,
            } => {
                msg!("Initialize payment stream instruction received");
                Self::initialize_stream(program_id, accounts, start_time, interval, amount_per_interval)
            }
            StreamPayInstruction::UpdateStream {
                interval,
                amount_per_interval,
            } => {
                msg!("Update payment stream instruction received");
                Self::update_stream(accounts, interval, amount_per_interval)
            }
            StreamPayInstruction::TerminateStream => {
                msg!("Terminate payment stream instruction received");
                Self::terminate_stream(program_id, accounts)
            }
            StreamPayInstruction::Withdraw { amount } => {
                msg!("Withdraw instruction received");
                Self::withdraw(program_id, accounts, amount)
            }
            StreamPayInstruction::PauseStream => {
                msg!("Pause payment stream instruction received");
                Self::pause_stream(program_id, accounts)
            }
            StreamPayInstruction::ResumeStream => {
                msg!("Resume payment stream instruction received");
                Self::resume_stream(program_id, accounts)
            }
            StreamPayInstruction::QueryStream => {
                msg!("Query payment stream instruction received");
                Self::query_stream(program_id, accounts)
            }
        }
    }

    fn initialize_stream(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        start_time: u64,
        interval: u64,
        amount_per_interval: u64,
    ) -> ProgramResult {
        // Ensure correct account permissions
        let account_info_iter = &mut accounts.iter();
        let payer_account = next_account_info(account_info_iter)?;
        let payment_stream_account = next_account_info(account_info_iter)?;

        if payment_stream_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        // Create and initialize the payment stream state
        let mut payment_stream_data = payment_stream_account.try_borrow_mut_data()?;
        let mut payment_stream = PaymentStream::unpack(&payment_stream_data)?;

        if payment_stream.is_initialized {
            return Err(PaymentError::StreamAlreadyInitialized.into());
        }

        let current_time = Self::current_timestamp();
        if start_time < current_time {
            return Err(StreamError::InvalidStartTime.into());
        }

        if amount_per_interval < MINIMUM_AMOUNT {
            return Err(PaymentError::InvalidAmount.into());
        }

        // Initialize other state variables and transitions
        payment_stream.is_initialized = true;
        payment_stream.owner = *payer_account.key;
        payment_stream.start_time = start_time;
        payment_stream.interval = interval;
        payment_stream.amount_per_interval = amount_per_interval;

        PaymentStream::pack(payment_stream, &mut payment_stream_data);

        // Deduct the initial amount from the payer's account
        let withdrawal_amount = amount_per_interval + Self::calculate_operational_fee(amount_per_interval);
        Self::transfer_funds(payer_account, payment_stream_account, withdrawal_amount)?;

        Ok(())
    }

    fn update_stream(
        accounts: &[AccountInfo],
        interval: u64,
        amount_per_interval: u64,
    ) -> ProgramResult {
        // Check account permissions
        let account_info_iter = &mut accounts.iter();
        let program_id = next_account_info(account_info_iter)?; // Add this line
        let payment_stream_account = next_account_info(account_info_iter)?;

        // Verify that the payment stream is owned by the program
        if payment_stream_account.owner != program_id.key {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut payment_stream_data = payment_stream_account.try_borrow_mut_data()?;
        let mut payment_stream = PaymentStream::unpack(&payment_stream_data)?;

        if !payment_stream.is_initialized {
            return Err(PaymentError::StreamNotInitialized.into());
        }

        if payment_stream.is_terminated {
            return Err(PaymentError::StreamAlreadyTerminated.into());
        }

        // Update the payment stream properties
        payment_stream.update(interval, amount_per_interval);
        PaymentStream::pack(payment_stream, &mut payment_stream_data);

        Ok(())
    }

    fn terminate_stream(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        // Check account permissions
        let account_info_iter = &mut accounts.iter();
        let payer_account = next_account_info(account_info_iter)?;
        let payment_stream_account = next_account_info(account_info_iter)?;

        // Verify that the payment stream is owned by the program
        if payment_stream_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut payment_stream_data = payment_stream_account.try_borrow_mut_data()?;
        let mut payment_stream = PaymentStream::unpack(&payment_stream_data)?;

        if !payment_stream.is_initialized {
            return Err(PaymentError::StreamNotInitialized.into());
        }

        if payment_stream.is_terminated {
            return Err(PaymentError::StreamAlreadyTerminated.into());
        }

        // Implement logic to terminate the payment stream
        payment_stream.terminate(); // For example, set payment_stream.is_terminated = true;
        PaymentStream::pack(payment_stream, &mut payment_stream_data);

        Ok(())
    }

    fn transfer_funds(
        from_account: &AccountInfo,
        to_account: &AccountInfo,
        amount: u64,
    ) -> ProgramResult {
        // Implement logic to transfer funds between accounts
        // You can use the solana_program::program::invoke function for token transfers
        // For example:
        // invoke(
        //     &transfer(&from_account.key, &to_account.key, amount),
        //     &[from_account.clone(), to_account.clone()],
        // )?;
        // Ensure that you handle any errors that might occur during the transfer.

        // Placeholder implementation
        // You should replace this with the actual logic for token transfer
        Ok(())
    }

    fn pause_stream(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        // Check account permissions and state transitions

        // Verify that the payment stream is owned by the program
        let account_info_iter = &mut accounts.iter();
        let payer_account = next_account_info(account_info_iter)?;
        let payment_stream_account = next_account_info(account_info_iter)?;

        if payment_stream_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut payment_stream_data = payment_stream_account.try_borrow_mut_data()?;
        let mut payment_stream = PaymentStream::unpack(&payment_stream_data)?;

        if !payment_stream.is_initialized {
            return Err(PaymentError::StreamNotInitialized.into());
        }

        if payment_stream.is_terminated {
            return Err(PaymentError::StreamAlreadyTerminated.into());
        }

        if payment_stream.is_paused {
            return Err(PaymentError::StreamAlreadyPaused.into());
        }

        // Implement logic to pause the payment stream
        payment_stream.pause(); // For example, set payment_stream.is_paused = true;
        PaymentStream::pack(payment_stream, &mut payment_stream_data);

        Ok(())
    }

    fn resume_stream(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        // Check account permissions and state transitions

        // Verify that the payment stream is owned by the program
        let account_info_iter = &mut accounts.iter();
        let payer_account = next_account_info(account_info_iter)?;
        let payment_stream_account = next_account_info(account_info_iter)?;

        if payment_stream_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut payment_stream_data = payment_stream_account.try_borrow_mut_data()?;
        let mut payment_stream = PaymentStream::unpack(&payment_stream_data)?;

        if !payment_stream.is_initialized {
            return Err(PaymentError::StreamNotInitialized.into());
        }

        if payment_stream.is_terminated {
            return Err(PaymentError::StreamAlreadyTerminated.into());
        }

        if !payment_stream.is_paused {
            return Err(PaymentError::StreamNotPaused.into());
        }

        // Implement logic to resume the payment stream
        payment_stream.resume(); // For example, set payment_stream.is_paused = false;
        PaymentStream::pack(payment_stream, &mut payment_stream_data);

        Ok(())
    }

    fn current_timestamp() -> u64 {
        // Implement logic to get the current timestamp using Solana's clock
        // For example:
        // solana_program::clock::get()?.unix_timestamp as u64
        // Ensure that you handle any errors that might occur while getting the timestamp
        0 // Change this to the actual implementation
    }

    fn calculate_operational_fee(amount: u64) -> u64 {
        // Implement logic to calculate operational fees based on the amount
        // For example, you can use OPERATIONAL_FEE_RATE constant
        // For example:
        // (amount * OPERATIONAL_FEE_RATE) / 100
        // Ensure that you handle any errors that might occur during the calculation
        0 // Change this to the actual implementation
    }

    fn transfer_funds(
        from_account: &AccountInfo,
        to_account: &AccountInfo,
        amount: u64,
    ) -> ProgramResult {
        // Implement logic to transfer funds between accounts
        // You can use the solana_program::program::invoke function for token transfers
        // For example:
        // invoke(
        //     &transfer(&from_account.key, &to_account.key, amount),
        //     &[from_account.clone(), to_account.clone()],
        // )?;
        Ok(())
    }
}