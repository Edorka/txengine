use std::collections::HashMap;
use crate::accounts::{Account, ClientID, AccountPerfomErr};
use crate::transactions::{Transaction, TransactionID, TransactionType};

pub struct State {
    accounts: HashMap<ClientID, Account>
}

impl Default for State {
    fn default() -> Self {
        State {
            accounts: HashMap::new()
        }
    }
}

impl State {
    fn perform(&mut self, tx: Transaction) -> Result<(), AccountPerfomErr> {
        let client_id: ClientID = tx.client;
        let account = self.accounts.entry(tx.client).or_insert(Account::for_client(client_id));
        account.perform(tx)
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
    fn test_receive_deposit() {
        let mut state = State::default();
        let transaction = Transaction::new(
            TransactionType::Withdrawal, 1,1, Some(1.1234)
        );
        state.perform(transaction);
        let account = state.get_account(1);
        assert_eq!(account.available, -1.1234);

    }
}
