use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Empty, Uint128};
use cw_competition::core::CompetitionCoreJailMsg;
use dao_interface::ModuleInstantiateInfo;
use dao_pre_propose_base::{
    msg::{ExecuteMsg as ExecuteBase, InstantiateMsg as InstantiateBase, QueryMsg as QueryBase},
    state::PreProposeContract,
};

use crate::{query::DumpStateResponse, state::Ruleset};

#[cw_serde]
pub struct InstantiateExt {
    pub competition_modules_instantiate_info: Vec<ModuleInstantiateInfo>,
    pub rulesets: Vec<Ruleset>,
    pub tax: Decimal,
}

#[cw_serde]
pub enum ExecuteExt {
    UpdateCompetitionModules {
        to_add: Vec<ModuleInstantiateInfo>,
        to_disable: Vec<String>,
    },
    Jail(CompetitionCoreJailMsg),
    UpdateTax {
        tax: Decimal,
    },
    UpdateRulesets {
        to_add: Vec<Ruleset>,
        to_disable: Vec<Uint128>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryExt {
    #[returns(Vec<Addr>)]
    CompetitionModules {
        start_after: Option<String>,
        limit: Option<u32>,
        include_disabled: Option<bool>,
    },
    #[returns(Vec<Ruleset>)]
    Rulesets {
        skip: Option<u128>,
        limit: Option<u32>,
        include_disabled: Option<bool>,
    },
    #[returns(Decimal)]
    Tax { height: Option<u64> },
    #[returns(Addr)]
    CompetitionModule { key: String },
    #[returns(DumpStateResponse)]
    DumpState {},
}

pub type InstantiateMsg = InstantiateBase<InstantiateExt>;
pub type ExecuteMsg = ExecuteBase<Empty, ExecuteExt>;
pub type QueryMsg = QueryBase<QueryExt>;
pub type PrePropose = PreProposeContract<InstantiateExt, ExecuteExt, QueryExt, Empty>;