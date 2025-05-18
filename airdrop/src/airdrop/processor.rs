pub type AirdropResult<T> = Result<T, AirdropError>;

use crate::airdrop::errors::AirdropError;
use crate::airdrop::structs::{Address, TokenAmount, Transactions, Recipient};
use crate::airdrop::utils::VirtualBlockchain; 
use csv::Reader;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;



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


   pub fn validate_batch(&self, recipients: &[Recipient]) -> AirdropResult<u64> {
    if recipients.is_empty() {
        return Err(AirdropError::EmptyBatch);
    }

    if recipients.len() > self.max_batch_size {
        return Err(AirdropError::BatchSizeLimitExceeded);
    }

    // check duplicates & zero amounts
    let mut addresses = HashSet::new();
    for recipient in recipients {
        if !addresses.insert(&recipient.address) {
            return Err(AirdropError::DuplicateAddress);
        }
        if recipient.amount == TokenAmount::new(0) {
            return Err(AirdropError::ZeroAmount);
        }
    }

    // sum amounts
  let total_amount: u64 = recipients.iter().map(|r| r.amount.0).sum();

    if total_amount > self.balance.0 {
        return Err(AirdropError::InsufficientFunds {
            requires:  total_amount,
            available: self.balance.0
        });
    }

    Ok(total_amount)
}

   pub fn process_batch(
    &mut self,
    recipients: Vec<Recipient>
   ) -> AirdropResult<Vec<Transactions>> {

    let total_amount = self.validate_batch(&recipients)?;


    // Create transactions
    let mut transactions = Vec::new();
    for recipient in recipients {
        let transaction = Transactions::new(recipient.address.clone(), recipient.amount.clone());
        transactions.push(transaction);
    }

    // Simulate sending transactions to the blockchain
    let mut completed_transactions = Vec::new();
    let mut successful_transaction = true;

    for transaction in transactions.iter_mut() {
        let _transaction_id = self.blockchain.get_nounce();
        let result = self.blockchain.send_transaction(&transaction.recipient, &transaction.amount);
        match result {
            Ok(tx_id) => {
                transaction.complete(tx_id);
                completed_transactions.push(transaction.clone());
            }
            Err(e) => {
                transaction.failed(e);
                successful_transaction = false;
            }
        }
    }

    // if all transactions are successful, update the balance
    if successful_transaction {
        self.balance.0 -= total_amount;
    } else {
        // If any transaction fails, revert the balance
        for transaction in completed_transactions.iter() {
            self.balance.0 += transaction.amount.0;
        }
    }
    // Store the transactions
    self.transaction.extend(completed_transactions.clone());
    
    Ok(transactions)

   }
    
    
    pub fn process_batches(
     &mut self,
    recipients: Vec<Recipient>
   ) -> AirdropResult<Vec<Transactions>> {

    if recipients.is_empty() {
        return Err(AirdropError::EmptyBatch);
    }

     let mut all_transactions = Vec::new();

      for chuck in recipients.chunks(self.max_batch_size) {
        let transactions = self.process_batch(chuck.to_vec())?;
        all_transactions.extend(transactions);
   }

   Ok(all_transactions)
   }

}

