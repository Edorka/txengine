use crate::accounts::{Account, AccountPerfomErr, ClientID};
use crate::transactions::{Transaction, TransactionID, TransactionType};
use std::collections::HashMap;

pub struct State {
    accounts: HashMap<ClientID, Account>,
    transactions: HashMap<TransactionID, Transaction>,
}

impl Default for State {
    fn default() -> Self {
        State {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }
}

impl State {
    fn perform(&mut self, tx: &Transaction) -> Result<(), AccountPerfomErr> {
        let client_id: ClientID = tx.client;
        let account = self
            .accounts
            .entry(tx.client)
            .or_insert(Account::for_client(client_id));
        self.transactions.insert(tx.tx, tx.clone());
        return account.perform(&tx);
    }
    fn get_account(&self, id: ClientID) -> &Account {
        self.accounts.get(&id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_accounts_receive_deposit() {
        let mut state = State::default();
        let transaction = Transaction::new(TransactionType::Deposit, 1, 1, Some(1.1234));
        let performed_correctly = state.perform(&transaction).is_ok();
        let account = state.get_account(1);
        assert!(performed_correctly);
        assert_eq!(account.available, 1.1234);
    }

    #[test]
    fn test_accounts_receive_withdrawal() {
        let mut state = State::default();
        let transaction = Transaction::new(TransactionType::Withdrawal, 1, 1, Some(1.1234));
        let performed_correctly = state.perform(&transaction).is_ok();
        let account = state.get_account(1);
        assert!(performed_correctly);
        assert_eq!(account.available, -1.1234);
    }
}
