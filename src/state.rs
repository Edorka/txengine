use crate::accounts::{Account, AccountPerfomErr, ClientID};
use crate::transactions::{Transaction, TransactionID, TransactionType};
use std::collections::{HashMap, HashSet};

pub struct State {
    accounts: HashMap<ClientID, Account>,
    transactions: HashMap<TransactionID, Transaction>,
    disputed_ids: HashSet<TransactionID>,
}

impl Default for State {
    fn default() -> Self {
        State {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
            disputed_ids: HashSet::new(),
        }
    }
}

impl State {
    fn perform(&mut self, tx: &Transaction) -> Result<(), AccountPerfomErr> {
        let client_id: ClientID = tx.client;
        let transaction_id: TransactionID = tx.tx;
        let account = self
            .accounts
            .entry(client_id)
            .or_insert(Account::for_client(client_id));
        let reference: &Transaction = self
            .transactions
            .entry(transaction_id)
            .or_insert(tx.clone());
        match tx.tx_type {
            TransactionType::Deposit | TransactionType::Withdrawal => account.perform(&tx),
            TransactionType::Dispute => {
                self.disputed_ids.insert(transaction_id);
                account.dispute(&reference)
            }
            TransactionType::Resolve => match self.disputed_ids.remove(&transaction_id) {
                true => account.resolve(&reference),
                _ => Ok(()),
            },
            TransactionType::Chargeback => Ok(()),
        }
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

    #[test]
    fn test_accounts_receive_dispute() {
        let mut state = State::default();
        let transaction = Transaction::new(TransactionType::Deposit, 1, 1, Some(1.1234));
        let performed_correctly = state.perform(&transaction).is_ok();
        let dispute = Transaction::new(TransactionType::Dispute, 1, 1, None);
        let dispute_accepted = state.perform(&dispute).is_ok();
        let account = state.get_account(1);
        assert!(performed_correctly);
        assert!(dispute_accepted);
        assert_eq!(account.available, 0.0000);
        assert_eq!(account.held, 1.1234);
        assert_eq!(account.total(), 1.1234);
    }

    #[test]
    fn test_accounts_receive_resolve() {
        let mut state = State::default();
        let transaction = Transaction::new(TransactionType::Deposit, 1, 1, Some(1.1234));
        let performed_correctly = state.perform(&transaction).is_ok();
        let dispute = Transaction::new(TransactionType::Dispute, 1, 1, None);
        let dispute_accepted = state.perform(&dispute).is_ok();
        let resolution = Transaction::new(TransactionType::Resolve, 1, 1, None);
        let resolution_accepted = state.perform(&resolution).is_ok();
        let account = state.get_account(1);
        assert!(performed_correctly);
        assert!(dispute_accepted);
        assert!(resolution_accepted);
        assert_eq!(account.available, 1.1234);
        assert_eq!(account.held, 0.0000);
        assert_eq!(account.total(), 1.1234);
    }

    #[test]
    fn test_accounts_receive_resolve_with_no_dispute() {
        let mut state = State::default();
        let transaction = Transaction::new(TransactionType::Deposit, 1, 1, Some(1.1234));
        let performed_correctly = state.perform(&transaction).is_ok();
        let resolution = Transaction::new(TransactionType::Resolve, 1, 1, None);
        let resolution_accepted = state.perform(&resolution).is_ok();
        let account = state.get_account(1);
        assert!(performed_correctly);
        assert!(resolution_accepted);
        assert_eq!(account.available, 1.1234);
        assert_eq!(account.held, 0.0000);
        assert_eq!(account.total(), 1.1234);
    }
}
