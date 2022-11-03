use cosmwasm_std::{StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized, owner and sender are not same")]
    Unauthorized {},

    #[error("cycle length is invalid, at least {min_cycle_length} seconds > request {cycle_length_in_seconds} seconds")]
    CycleLengthInvalid {
        min_cycle_length: u64,
        cycle_length_in_seconds: u64,
    },

    #[error("period length is invalid, at least {min_period} cycles > request {period_length_in_cycles} cycles")]
    PeriodLengthInvalid {
        min_period: u64,
        period_length_in_cycles: u64,
    },

    #[error("cycle cannot be zero")]
    CycleNotZero {},

    #[error("timestamp preceeds contract start")]
    TimestampPreceesContractStart {},

    #[error("rewards schedule is null")]
    NoneRewardsSchedule {},

    #[error("already started")]
    AlreadyStarted {},

    #[error("not started, run start()")]
    NotStarted {},

    #[error("disabled")]
    Disabled {},

    #[error("cannot enable, disable state is {disable}")]
    CannotEnable {
        disable: bool,
    },

    #[error("unstaked token cooldown")]
    UnstakedTokenCooldown {},

    #[error("invalid token id")]
    InvalidTokenId {},

    #[error("unstaked token id")]
    UnstakedTokenId {},

    #[error("token steel frozen")]
    TokenSteelFrozen {},

    #[error("invalid nft owner, requester is {requester}, but nft owner is {nft_owner}")]
    InvalidNftOwner {
        requester: String,
        nft_owner: String,
    },

    #[error("invalid claim of requester")]
    InvalidClaim {},

    #[error("next claim is empty")]
    EmptyNextClaim {},

    #[error("have no amout for claim")]
    NoAmountClaim {},

    #[error("have not history")]
    HaveNotHistory {},

    #[error("invalid rewards schedule")]
    InvalidRewardsSchedule {},

    #[error("rewards pool is empty")]
    EmptyRewardsPool {},
}