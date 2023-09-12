use crate::state::{CompetitionModule, Ruleset, KEYS, TAX};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Deps, Empty, Env, StdResult, Uint128};
use cw_storage_plus::Bound;
use cw_utils::maybe_addr;

#[cw_serde]
pub struct DumpStateResponse {
    pub tax: Decimal,
    pub competition_modules: Vec<CompetitionModuleResponse>,
    pub rulesets: Vec<Ruleset>,
}

#[cw_serde]
pub struct CompetitionModuleResponse {
    pub key: String,
    pub addr: Addr,
    pub is_enabled: bool,
    pub competition_count: Uint128,
}

impl CompetitionModule {
    pub fn to_response(&self, deps: Deps) -> StdResult<CompetitionModuleResponse> {
        let competition_count: Uint128 = deps.querier.query_wasm_smart(
            self.addr.to_string(),
            &cw_competition::msg::QueryBase::<Empty, Empty>::CompetitionCount {},
        )?;

        Ok(CompetitionModuleResponse {
            key: self.key.clone(),
            addr: self.addr.clone(),
            is_enabled: self.is_enabled,
            competition_count,
        })
    }
}

pub fn competition_modules(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
    include_disabled: Option<bool>,
) -> StdResult<Vec<CompetitionModuleResponse>> {
    let start_after_bound = maybe_addr(deps.api, start_after)?.map(Bound::exclusive);
    let limit = limit.unwrap_or(10).max(30);
    let include_disabled = include_disabled.unwrap_or(false);

    let competition_modules_map = crate::state::competition_modules();

    if include_disabled {
        cw_paginate::paginate_indexed_map(
            &competition_modules_map,
            deps.storage,
            start_after_bound,
            Some(limit),
            |_x, y| y.to_response(deps),
        )
    } else {
        competition_modules_map
            .idx
            .is_enabled
            .prefix(true.to_string())
            .range(
                deps.storage,
                start_after_bound,
                None,
                cosmwasm_std::Order::Ascending,
            )
            .map(|x| x.map(|y| y.1.to_response(deps)))
            .take(limit as usize)
            .try_fold(Vec::new(), |mut acc, res| {
                acc.push(res??);

                Ok(acc)
            })
    }
}

pub fn tax(deps: Deps, env: Env, height: Option<u64>) -> StdResult<Decimal> {
    Ok(TAX
        .may_load_at_height(deps.storage, height.unwrap_or(env.block.height))?
        .unwrap_or(Decimal::zero()))
}

pub fn rulesets(
    deps: Deps,
    start_after: Option<Uint128>,
    limit: Option<u32>,
    include_disabled: Option<bool>,
) -> StdResult<Vec<Ruleset>> {
    let start_after_bound = start_after.map(Bound::exclusive);
    let limit = limit.unwrap_or(10).max(30);
    let include_disabled = include_disabled.unwrap_or(false);

    let rulesets_map = crate::state::rulesets();

    if include_disabled {
        cw_paginate::paginate_indexed_map(
            &rulesets_map,
            deps.storage,
            start_after_bound,
            Some(limit),
            |_x, y| Ok(y),
        )
    } else {
        rulesets_map
            .idx
            .is_enabled
            .prefix(true.to_string())
            .range(
                deps.storage,
                start_after_bound,
                None,
                cosmwasm_std::Order::Ascending,
            )
            .map(|x| x.map(|y| y.1))
            .take(limit as usize)
            .collect::<StdResult<Vec<_>>>()
    }
}

pub fn competition_module(deps: Deps, key: String) -> StdResult<Option<CompetitionModuleResponse>> {
    let maybe_addr = KEYS.may_load(deps.storage, key)?;

    match maybe_addr {
        Some(addr) => crate::state::competition_modules()
            .may_load(deps.storage, addr)?
            .map(|x| x.to_response(deps))
            .transpose(),
        None => Ok(None),
    }
}

pub fn dump_state(deps: Deps, env: Env) -> StdResult<DumpStateResponse> {
    Ok(DumpStateResponse {
        tax: tax(deps, env, None)?,
        competition_modules: competition_modules(deps, None, None, None)?,
        rulesets: rulesets(deps, None, None, None)?,
    })
}
