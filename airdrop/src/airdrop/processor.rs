pub type AirdropResult<T> = Result<T, AirdropError>;

use crate::airdrop::errors::AirdropError;
use crate::airdrop::structs::{Address, TokenAmount, Transactions, Recipient};
use crate::airdrop::utils::VirtualBlockchain; 
use csv::Reader;
use std::fs;
use std::fs::File;
use std::path::Path;



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


    pub fn load_recipients_from_csv<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> AirdropResult<Vec<Recipient>> {
        let content =
            fs::read_to_string(path).map_err(|e| AirdropError::IOError(e.to_string()))?;

        let mut recipients = Vec::new();

        // Skip header line if it exists
        for (i, line) in content.lines().enumerate() {
            if i == 0 && line.trim().starts_with("address") {
                continue; // skip header
            }

            let mut parts = line.split(',').map(str::trim);

            let address_str = parts
                .next()
                .ok_or(AirdropError::InvalidCSVFormat)?
                .trim();
            let amount_str = parts
                .next()
                .ok_or(AirdropError::InvalidCSVFormat)?
                .trim();

            // extra columns?
            if parts.next().is_some() {
                return Err(AirdropError::InvalidCSVFormat);
            }

            let address = Address::new(address_str)?;
            let amount: u64 = amount_str
                .parse()
                .map_err(|_| AirdropError::InvalidAmount)?;

            if amount == 0 {
                return Err(AirdropError::ZeroAmount);
            }

            if recipients.iter().any(|r: &Recipient| r.address == address) {
                return Err(AirdropError::DuplicateAddress);
            }

            recipients.push(Recipient::new(address, TokenAmount::new(amount)));
        }

        Ok(recipients)
    }




}

