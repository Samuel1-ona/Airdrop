use crate::airdrop::errors::AirdropError;
use serde::{Deserialize,Serialize};
use std::str::FromStr;

#[derive(Clone, Serialize, Deserialize , Debug, PartialEq)]
pub struct Address(String);

impl Address{
    pub fn new(address: &str) ->Result<Self, AirdropError> {
        if address.trim().is_empty(){
            return Err(AirdropError::InvalidAddress);
        }

        Ok(Self(address.to_string()))
    }
}

impl FromStr for Address {
    type Err = AirdropError;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        Address::new(address)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct TokenAmount(u64);

impl TokenAmount{
    pub fn new(amount: u64) -> Self {
        Self(amount)
    }
   
   pub fn get_amount(&self) -> u64 {
        self.0
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Transactions{
    pub transaction_id: Option<String>,
    pub status: TransactionStatus,
    pub recipient: Address,
    pub amount: TokenAmount,
    pub transaction_time: u64,
}

impl Transactions{
    pub fn new(recipient: Address, amount: TokenAmount) -> Self {
        Self {
            transaction_id: None,
            status: TransactionStatus::Pending,
            recipient,
            amount,
            transaction_time: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        }
    }

    pub fn complete(&mut self, transaction_id: String){
        self.transaction_id =  Some(transaction_id);
        self.status = TransactionStatus::Complete;
    }

    pub fn failed(&mut self, error: AirdropError){
        self.status = TransactionStatus::Failed(error);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionStatus{
    Pending,
    Complete,
    Failed(AirdropError),
}
impl TransactionStatus{
    pub fn is_complete(&self) -> bool {
        match self {
            TransactionStatus::Complete => true,
            _ => false,
        }
    }

    pub fn is_failed(&self) -> bool {
        match self {
            TransactionStatus::Failed(_) => true,
            _ => false,
        }
    }
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Recipient{
    pub address: Address,
    pub amount: TokenAmount,
}

impl Recipient{
    pub fn new(address: Address, amount: TokenAmount) -> Self {
        Self {
            address,
            amount,
        }
    }
}
