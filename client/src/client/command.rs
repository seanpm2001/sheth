use super::error::Error;
use crate::accounts::AddressedAccount;
use bigint::U256;
use sheth::multiproof::Multiproof;
use sheth::process::process_transactions;
use sheth::state::State;
use sheth::transaction::{Transaction, Transfer};

/// A enum that describes the possible commands a user might send to the client and their required
/// arguments.
pub enum Command {
    Balance(BalanceCmd),
    Transfer(TransferCmd),
    Accounts(AccountsCmd),
    Exit,
}

/// The balance command will return the balance of a specified address.
pub struct BalanceCmd {
    pub(crate) address: U256,
}

/// The transfer command will transfer some amount from one specified account to another.
pub struct TransferCmd {
    pub(crate) from: U256,
    pub(crate) to: U256,
    pub(crate) amount: u64,
}

/// The accounts command will list the accounts managed by the client.
pub struct AccountsCmd();

impl BalanceCmd {
    pub fn execute(&self, db: &Multiproof) -> Result<(), Error> {
        let value = db
            .value(self.address.into())
            .map_err(|_| Error::AddressUnknown("".to_string()))?;

        println!("Balance is: {}", value);

        Ok(())
    }
}

impl TransferCmd {
    pub fn execute(&self, db: &mut Multiproof) -> Result<(), Error> {
        let nonce = db
            .nonce(self.from.into())
            .map_err(|_| Error::AddressUnknown("".to_string()))?;

        let tx = Transaction::Transfer(Transfer {
            to: self.to.into(),
            from: self.from.into(),
            nonce,
            amount: self.amount,
            signature: [0u8; 96],
        });

        process_transactions(db, &vec![tx]).map_err(|_| Error::TransactionFailed("bad".to_string()))
    }
}

impl AccountsCmd {
    pub fn execute(&self, accounts: &Vec<AddressedAccount>) -> Result<(), Error> {
        for account in accounts {
            let mut buf = [0u8; 32];
            account.0.to_big_endian(&mut buf);
            println!("0x{}", hex::encode(buf));
        }

        Ok(())
    }
}
