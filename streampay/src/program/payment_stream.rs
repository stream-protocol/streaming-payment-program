use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    clock::UnixTimestamp,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct PaymentStream {
    pub payer: Pubkey,
    pub recipient: Pubkey,
    pub start_time: UnixTimestamp,
    pub interval: u64,
    pub amount_per_interval: u64,
    pub total_amount: u64,
    pub withdrawn_amount: u64,
    pub last_withdraw_time: UnixTimestamp,
    pub is_initialized: bool,
    pub is_terminated: bool,
    pub is_paused: bool,
    pub custom_field: u64, // Add custom fields as needed
    // Add more custom fields based on program requirements
}

impl PaymentStream {
    pub fn new(payer: Pubkey, recipient: Pubkey) -> Self {
        PaymentStream {
            payer,
            recipient,
            start_time: 0,
            interval: 0,
            amount_per_interval: 0,
            total_amount: 0,
            withdrawn_amount: 0,
            last_withdraw_time: 0,
            is_initialized: false,
            is_terminated: false,
            is_paused: false,
            custom_field: 0, // Initialize custom fields
            // Initialize more custom fields here
        }
    }

    pub fn pack(&self, dst: &mut [u8]) -> Result<(), ProgramError> {
        self.try_to_vec()
            .map(|encoded| dst[..encoded.len()].copy_from_slice(&encoded))
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        BorshDeserialize::try_from_slice(input).map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn calculate_max_withdrawable(&self, current_time: UnixTimestamp) -> u64 {
        if !self.is_initialized || self.is_terminated || self.is_paused {
            return 0;
        }

        // Calculate the number of intervals elapsed since the last withdrawal
        let elapsed_intervals = (current_time - self.last_withdraw_time) / self.interval;

        // Calculate the maximum withdrawable amount
        let max_withdrawable = self.amount_per_interval * elapsed_intervals;

        // Ensure the calculated amount doesn't exceed the total amount
        max_withdrawable.min(self.total_amount - self.withdrawn_amount)
    }

    pub fn update(&mut self, current_time: UnixTimestamp) {
        if !self.is_initialized || self.is_terminated || self.is_paused {
            return;
        }

        // Update the last withdrawal time based on the current time
        self.last_withdraw_time = current_time;

        // Add your custom logic for updating the stream based on specific requirements
        // For example, updating total amount, vested amount, etc.
        // Your custom logic here
    }

    pub fn terminate(&mut self) {
        if self.is_initialized && !self.is_terminated {
            // Add your custom logic for terminating the stream based on specific requirements
            // For example, handle early termination penalties, transfer remaining funds, etc.
            // Your custom logic here

            self.is_terminated = true;
        }
    }

    pub fn pause(&mut self) {
        if self.is_initialized && !self.is_terminated && !self.is_paused {
            // Add your custom logic for pausing the stream based on specific requirements
            // For example, implement a pause mechanism, suspend interval updates, etc.
            // Your custom logic here

            self.is_paused = true;
        }
    }

    pub fn resume(&mut self) {
        if self.is_initialized && !self.is_terminated && self.is_paused {
            // Add your custom logic for resuming the stream based on specific requirements
            // For example, resume interval updates, restart payments, etc.
            // Your custom logic here

            self.is_paused = false;
        }
    }

    // Add more custom methods as needed for program-specific functionality
}
