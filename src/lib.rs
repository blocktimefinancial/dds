#![no_std]

use soroban_sdk::{
    contracterror, contractimpl, contracttype, symbol, vec, log, AccountId, Address, BytesN, Env, Map,
    Symbol, Vec,
};

mod token {
    soroban_sdk::contractimport!(file = "../soroban-examples/soroban_token_spec.wasm");
}

use token::{Identifier, Signature};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Holder {
    pub addr: Identifier,
    pub amount: i128,
}
pub struct Dds;
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
enum DdsError {
    BeforeExDate = 0,
    InsufficientFunds = 1,
    InvalidAccount = 2,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum DdsDataKys {
    Init,
    Divdata,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Divdata {
    pub token: BytesN<32>,
    pub div: i128,
    pub exdate: u64,
    pub holders: Vec<Holder>,
}

#[contractimpl]
impl Dds {
    pub fn deposit(e: Env, token: BytesN<32>, amount: i128, holders: Vec<Holder>, exdate: u64) {
        log!(&e, "deposit {} {} {} {}", &token, &amount, &holders, &exdate);
        
        // Check to see if the contract has been initialized
        if is_initialized(&e) {
            panic!("Already initialized");
        }
        // Only allow 10 holders at this point
        if holders.len() > 10 {
            panic!("Too many holders");
        }
        // Make sure we have at least one holder
        if holders.len() == 0 {
            panic!("No holders");
        }
        // Make sure the amount is positive
        if amount <= 0 {
            panic!("Negative or zero amount");
        }
        // Make sure the exdate is in the future
        // TODO: This should probably be at least 1 day in the future?
        let now: u64 = e.ledger().timestamp();
        if now > exdate {
            panic!("ExDate is in the past");
        }
 
        // Transfer the tokens to the contract
        transfer_from_account_to_contract(&e, &token, &e.invoker().into(), &amount);

        e.storage().set(
            &(DdsDataKys::Divdata),
            &Divdata {
                token: token,
                div: amount,
                exdate: exdate,
                holders: holders,
            },
        );
        e.storage().set(&(DdsDataKys::Init), &());
    }

    pub fn withdraw(e: Env, token: BytesN<32>, amount: i128) {
        log!(&e, "withdraw {} {}", &token, &amount);
        
        let divdata: Divdata = e.storage().get(&(DdsDataKys::Divdata)).unwrap().unwrap();
        let now: u64 = e.ledger().timestamp();
        if now < divdata.exdate {
            panic!("ExDate is in the future");
        }
        if divdata.token != token {
            panic!("Wrong token");
        }
        if amount > divdata.div {
            panic!("Insufficient funds");
        }
    }
}

fn is_initialized(env: &Env) -> bool {
    env.storage().has(&DdsDataKys::Init)
}

fn get_contract_id(e: &Env) -> Identifier {
    Identifier::Contract(e.get_current_contract())
}

fn transfer_from_account_to_contract(
    e: &Env,
    token_id: &BytesN<32>,
    from: &Identifier,
    amount: &i128,
) {
    let client = token::Client::new(e, token_id);
    client.xfer_from(&Signature::Invoker, &0, from, &get_contract_id(e), amount);
}
