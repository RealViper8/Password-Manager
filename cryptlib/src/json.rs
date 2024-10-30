#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{Error, Write},
    vec::Vec,
};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub user_id: Option<u32>,
    pub email: Option<String>,
    pub username: String,
    pub password: String, // From BcryptResult<String>
}

#[derive(Default, Debug)]
pub struct UserInputAccount {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Default)]
pub struct AccountManager {
    accounts_hashmap: HashMap<u32, Account>,
    pub accounts: Vec<Account>,
}

#[derive(Debug, Error)]
pub enum PasswordManagerError {
    #[error("Hashing algorithm not supported: {0}")]
    HashingAlgorithmNotSupported(String),

    #[error("Failed to hash password: {0}")]
    HashingFailed(String),

    #[error("Failed to save: {0}")]
    NoAccounts(String),

    #[error("Failed to convert json: {0}")]
    SerdeJson(String),

    #[error("Failed to encode username/email: {0}")]
    EncodingFailed(String),

    #[error("Failed to decode value: {0}")]
    DecodingFailed(String),

    #[error("Invalid password: {0}")]
    InvalidPassword(String),

    #[error("Invalid username/email format: {0}")]
    InvalidUsernameEmail(String),

    #[error("Account error: {0}")]
    AccountError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("File I/O error: {0}")]
    FileIoError(#[from] std::io::Error),

    #[error("An unknown error ocurred")]
    Unknown,
}

type AccountManagerError = PasswordManagerError;

impl AccountManager {
    pub fn to_hasmap(mut self) -> HashMap<u32, Account> {
        let mut index: u32 = 0;
        self.accounts_hashmap = self
            .accounts
            .into_iter()
            .map(|account| (account.user_id.unwrap_or(index), account))
            .collect::<HashMap<u32, Account>>();

        self.accounts_hashmap
    }

    pub fn add(&mut self, account: Account) -> Result<(), PasswordManagerError> {
        let mut account = account;
        self.accounts.push(Account {
            user_id: Some(self.accounts.len() as u32),
            email: account.email,
            password: account.password,
            username: account.username,
        });
        Ok(())
    }

    pub fn edit(&mut self, account: Account, id: usize) -> Result<(), Error> {
        let mut account = self.accounts.get(id);

        match account {
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    PasswordManagerError::NoAccounts(String::from("No accounts to save")),
                ))
            }
            Some(e) => (),
        }

        Ok(())
    }

    pub fn display_accounts(&self) {
        for account in &self.accounts {
            println!(
                "ID: {}, Username: {}, Email: {}",
                account.user_id.unwrap_or(0),
                account.username,
                account.email.as_ref().unwrap(),
            );
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<(), PasswordManagerError> {
        println!("Length: {} index: {}", self.accounts.len(), index);
        self.accounts.remove(index);
        Ok(())
    }

    pub fn save(&self, filename: &str) {
        let json_data = serde_json::to_string(&self.accounts).unwrap();

        let mut file = File::create(filename).unwrap();
        file.write_all(json_data.as_bytes()).unwrap();
        file.flush();
    }

    pub fn read(&mut self, filename: &str) -> Result<(), PasswordManagerError> {
        let json_data = match std::fs::read_to_string(filename) {
            Ok(s) => s,
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "I/O failure",
            ))?,
        };

        let accs: Vec<Account> = serde_json::from_str(&json_data).unwrap();
        self.accounts = accs;

        Ok(())
    }

    pub fn close(&self, filename: &str) -> Result<(), Error> {
        if (!self.accounts.is_empty()) {
            self.save(filename);
            Ok(())
        } else {
            Err(Error::new(
                std::io::ErrorKind::NotFound,
                PasswordManagerError::NoAccounts(String::from("No accounts to save")),
            ))
        }
    }
}
