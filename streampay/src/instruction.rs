use solana_program::program_error::ProgramError;
use std::convert::TryInto;

/// Enum that defines the instructions supported by the program.
#[derive(Debug, PartialEq)]
pub enum StreamPayInstruction {
    InitializeStream {
        start_time: u64,
        interval: u64,
        amount_per_interval: u64,
    },
    UpdateStream {
        interval: u64,
        amount_per_interval: u64,
    },
    TerminateStream,
    Withdraw {
        amount: u64,
    },
    PauseStream,
    ResumeStream,
    QueryStream,
}

impl StreamPayInstruction {
    /// Unpacks a byte array into a StreamPayInstruction.
    pub fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        let instruction = data[0]; // First byte is the instruction code

        match instruction {
            0 => {
                // InitializeStream instruction
                let start_time = u64::from_le_bytes(data[1..9].try_into().unwrap());
                let interval = u64::from_le_bytes(data[9..17].try_into().unwrap());
                let amount_per_interval = u64::from_le_bytes(data[17..25].try_into().unwrap());
                Ok(StreamPayInstruction::InitializeStream {
                    start_time,
                    interval,
                    amount_per_interval,
                })
            }
            1 => {
                // UpdateStream instruction
                let interval = u64::from_le_bytes(data[1..9].try_into().unwrap());
                let amount_per_interval = u64::from_le_bytes(data[9..17].try_into().unwrap());
                Ok(StreamPayInstruction::UpdateStream {
                    interval,
                    amount_per_interval,
                })
            }
            2 => Ok(StreamPayInstruction::TerminateStream),
            3 => {
                // Withdraw instruction
                let amount = u64::from_le_bytes(data[1..9].try_into().unwrap());
                Ok(StreamPayInstruction::Withdraw { amount })
            }
            4 => Ok(StreamPayInstruction::PauseStream),
            5 => Ok(StreamPayInstruction::ResumeStream),
            6 => Ok(StreamPayInstruction::QueryStream),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }

    /// Packs a StreamPayInstruction into a byte array.
    pub fn pack(&self) -> Vec<u8> {
        let mut data = vec![0; 9]; // Initialize with instruction code (1 byte) and placeholders (8 bytes)

        match self {
            StreamPayInstruction::InitializeStream {
                start_time,
                interval,
                amount_per_interval,
            } => {
                data[0] = 0; // Instruction code for InitializeStream
                data[1..9].copy_from_slice(&start_time.to_le_bytes());
                data.extend_from_slice(&interval.to_le_bytes());
                data.extend_from_slice(&amount_per_interval.to_le_bytes());
            }
            StreamPayInstruction::UpdateStream {
                interval,
                amount_per_interval,
            } => {
                data[0] = 1; // Instruction code for UpdateStream
                data[1..9].copy_from_slice(&interval.to_le_bytes());
                data.extend_from_slice(&amount_per_interval.to_le_bytes());
            }
            StreamPayInstruction::TerminateStream => {
                data[0] = 2; // Instruction code for TerminateStream
            }
            StreamPayInstruction::Withdraw { amount } => {
                data[0] = 3; // Instruction code for Withdraw
                data[1..9].copy_from_slice(&amount.to_le_bytes());
            }
            StreamPayInstruction::PauseStream => {
                data[0] = 4; // Instruction code for PauseStream
            }
            StreamPayInstruction::ResumeStream => {
                data[0] = 5; // Instruction code for ResumeStream
            }
            StreamPayInstruction::QueryStream => {
                data[0] = 6; // Instruction code for QueryStream
            }
        }

        data
    }
}
