use cw20::Cw20ReceiveMsg;
use cw721::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{Snapshot, TokenInfo, Claim, NextClaim};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub cycle_length_in_seconds: u64,
    pub period_length_in_cycles: u64,
    pub white_listed_nft_contract: String,
    pub rewards_token_contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddRewardsForPeriods {
        rewards_per_cycle: u128,
    },
    Receive(Cw20ReceiveMsg),
    Start {},
    Disable {},
    Enable {},
    WithdrawRewardsPool {
        amount: u128,
    },
    WithdrawAllRewardsPool {},
    ReceiveNft(Cw721ReceiveMsg),
    UnstakeNft {
        token_id: String,
        staker: String,
        claim_recipient_address: Option<String>,
    },
    ClaimRewards {
        max_period: u64,
        token_id: String,
        claim_recipient_address: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetRewardsSchedule {},
    StartTime {},
    Disable {},
    TotalRewardsPool {},
    WithdrawRewardsPoolAmount {},
    StakerHistory {
        staker: String,
        token_id: String,
    },
    TokenInfo {
        token_id: String,
    },
    EstimateRewards {
        max_period: u64,
        staker: String,
        token_id: String,
    },
    NextClaim {
        staker: String,
        token_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub owner: String,
    pub cycle_length_in_seconds: u64,
    pub period_length_in_cycles: u64,
    pub white_listed_nft_contract: String,
    pub rewards_token_contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UpdateHistoriesResponse {
    pub staker: String,
    pub current_cycle: u64,
    pub staker_histories_stake: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardsScheduleResponse {
    pub rewards_per_cycle: u128,
    pub res_msg: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StartTimeResponse {
    pub start: bool,
    pub start_time : u64,
    pub res_msg: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DisableResponse {
    pub disable: bool,
    pub res_msg: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TotalRewardsPoolResponse {
    pub total_rewards_pool: u128,
    pub res_msg: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WithdrawRewardsPoolResponse {
    pub withdraw_rewards_pool_amount: u128,
    pub res_msg: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct NextClaimResponse {
    pub next_claim: NextClaim,
    pub res_msg: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StakerHistoryResponse {
    pub staker_tokenid_key: String,
    pub staker_history: Vec<Snapshot>,
    pub res_msg: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TokenInfosResponse {
    pub token_id: String,
    pub token_info: TokenInfo,
    pub res_msg: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct EstimateRewardsResponse {
    pub req_staker_tokenid_key: String,
    pub claim: Claim,
    pub res_msg: String,
}


