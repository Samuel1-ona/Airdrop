pub type AirdropResult<T> = Result<T, AirdropError>;

use crate::airdrop::errors::AirdropError;
use crate::airdrop::structs::{Address, TokenAmount, Transactions, Recipient};
use crate::airdrop::utils::VirtualBlockchain; 
use csv::Reader;
use std::fs::File;

#[derive(Debug)]
pub struct AirdropProcessor{
    pub blockchain : VirtualBlockchain,
    pub transaction: Vec<Transactions>,
    pub balance: TokenAmount,
    max_batch_size: usize,
}


impl AirdropProcessor {
    pub fn new(initial_amount: TokenAmount, max_batch_size: usize) -> Self {
        Self {
            blockchain: VirtualBlockchain::new(),
            transaction: Vec::new(),
            balance: initial_amount,
            max_batch_size,
        }
    }

    pub fn get_balance(&self)-> TokenAmount {
        self.balance
    }
    pub fn get_transaction(&self)-> Vec<Transactions> {
        self.transaction.clone()
    }
    pub fn get_max_batch_size(&self)-> usize {
        self.max_batch_size
    }


}

