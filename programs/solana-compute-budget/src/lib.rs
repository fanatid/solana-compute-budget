use anchor_lang::prelude::*;

uint::construct_uint! {
    pub struct U128(2);
}
uint::construct_uint! {
    pub struct U256(4);
}

#[program]
pub mod budget {
    use super::*;

    pub fn empty(_ctx: Context<ZeroAccounts>) -> ProgramResult {
        Ok(())
    }

    pub fn u128_native(
        _ctx: Context<ZeroAccounts>,
        count: u32,
        rate: u128,
        last_update_timestamp: u128,
        current_timestamp: u128,
    ) -> ProgramResult {
        for _ in 0..count {
            calc_u128_native(rate, last_update_timestamp, current_timestamp);
        }

        Ok(())
    }

    pub fn u128_uint(
        _ctx: Context<ZeroAccounts>,
        count: u32,
        rate: u128,
        last_update_timestamp: u128,
        current_timestamp: u128,
    ) -> ProgramResult {
        let rate: U128 = rate.into();
        let last_update_timestamp: U128 = last_update_timestamp.into();
        let current_timestamp: U128 = current_timestamp.into();

        for _ in 0..count {
            calc_u128_uint(rate, last_update_timestamp, current_timestamp);
        }

        Ok(())
    }

    pub fn u256_uint(
        _ctx: Context<ZeroAccounts>,
        count: u32,
        rate: u128,
        last_update_timestamp: u128,
        current_timestamp: u128,
    ) -> ProgramResult {
        let rate: U256 = rate.into();
        let last_update_timestamp: U256 = last_update_timestamp.into();
        let current_timestamp: U256 = current_timestamp.into();

        for _ in 0..count {
            calc_u256_uint(rate, last_update_timestamp, current_timestamp);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ZeroAccounts {}

const RAY_U128_NATIVE: u128 = 1000000000000000000000000000; // 10u128.pow(27);
const HALF_RAY_U128_NATIVE: u128 = 500000000000000000000000000;

const RAY_U128_UINT: U128 = U128([11515845246265065472, 54210108]);
const HALF_RAY_U128_UINT: U128 = U128([5757922623132532736, 27105054]);

const RAY_U256_UINT: U256 = U256([11515845246265065472, 54210108, 0, 0]);
const HALF_RAY_U256_UINT: U256 = U256([5757922623132532736, 27105054, 0, 0]);

// (1+x)^n = 1+n*x+[n/2*(n-1)]*x^2+[n/6*(n-1)*(n-2)*x^3...
fn calc_u128_native(rate: u128, last_update_timestamp: u128, current_timestamp: u128) -> u128 {
    let mut result = RAY_U128_NATIVE;

    let exp = match current_timestamp.checked_sub(last_update_timestamp) {
        Some(exp) => exp,
        None => return result,
    };

    let mut el = rate.checked_mul(exp).unwrap();
    result = result.checked_add(el).unwrap();
    for i in 1..5 {
        if exp <= i {
            break;
        }

        // el = raymul_u128(el * (exp - i), rate) / (i + 1)
        el = raymul_u128_native(el.checked_mul(exp - i).unwrap(), rate)
            .unwrap()
            .checked_div(i + 1)
            .unwrap();
        result = result.checked_add(el).unwrap();
    }
    result
}

fn raymul_u128_native(a: u128, b: u128) -> Option<u128> {
    if a == 0 || b == 0 {
        return Some(0);
    }

    // (a * b + halfRAY) / RAY;
    a.checked_mul(b)
        .and_then(|v| v.checked_add(HALF_RAY_U128_NATIVE))
        .and_then(|v| v.checked_div(RAY_U128_NATIVE))
}

// Same as `calc_u128`
fn calc_u128_uint(rate: U128, last_update_timestamp: U128, current_timestamp: U128) -> U128 {
    let mut result = RAY_U128_UINT;

    let exp = match current_timestamp.checked_sub(last_update_timestamp) {
        Some(exp) => exp,
        None => return result,
    };

    let mut el = rate.checked_mul(exp).unwrap();
    result = result.checked_add(el).unwrap();
    for i in 1..5 {
        if exp <= i.into() {
            break;
        }

        // el = raymul_u128(el * (exp - i), rate) / (i + 1)
        el = raymul_u128_uint(el.checked_mul(exp - i).unwrap(), rate)
            .unwrap()
            .checked_div((i + 1).into())
            .unwrap();
        result = result.checked_add(el).unwrap();
    }
    result
}

fn raymul_u128_uint(a: U128, b: U128) -> Option<U128> {
    if a.is_zero() || b.is_zero() {
        return Some(0.into());
    }

    // (a * b + halfRAY) / RAY;
    a.checked_mul(b)
        .and_then(|v| v.checked_add(HALF_RAY_U128_UINT))
        .and_then(|v| v.checked_div(RAY_U128_UINT))
}

// Same as `calc_u128`
fn calc_u256_uint(rate: U256, last_update_timestamp: U256, current_timestamp: U256) -> U256 {
    let mut result = RAY_U256_UINT;

    let exp = match current_timestamp.checked_sub(last_update_timestamp) {
        Some(exp) => exp,
        None => return result,
    };

    let mut el = rate.checked_mul(exp).unwrap();
    result = result.checked_add(el).unwrap();
    for i in 1..5 {
        if exp <= i.into() {
            break;
        }

        // el = raymul_u128(el * (exp - i), rate) / (i + 1)
        el = raymul_u256_uint(el.checked_mul(exp - i).unwrap(), rate)
            .unwrap()
            .checked_div((i + 1).into())
            .unwrap();
        result = result.checked_add(el).unwrap();
    }
    result
}

fn raymul_u256_uint(a: U256, b: U256) -> Option<U256> {
    if a.is_zero() || b.is_zero() {
        return Some(0.into());
    }

    // (a * b + halfRAY) / RAY;
    a.checked_mul(b)
        .and_then(|v| v.checked_add(HALF_RAY_U256_UINT))
        .and_then(|v| v.checked_div(RAY_U256_UINT))
}
