use crate::state::transactions::{Transaction, TransactionType};
use serde::Serialize;

pub type ClientID = u16;

#[derive(Debug, Serialize, Clone)]
pub struct Account {
    pub client: ClientID,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}

#[derive(Clone)]
pub enum AccountPerfomErr {
    UnknownType(TransactionType),
}

impl Account {
    pub fn for_client(client: ClientID) -> Self {
        Account {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
    pub fn perform(&mut self, tx: &Transaction) -> Result<(), AccountPerfomErr> {
        assert_eq!(tx.client, self.client);

        let amount = tx.amount();
        match &tx.tx_type {
            TransactionType::Deposit => {
                self.available += amount;
                self.total = self.calc_total();
                Ok(())
            }
            TransactionType::Withdrawal => {
                self.available -= amount;
                self.total = self.calc_total();
                Ok(())
            }
            tx_type => Err(AccountPerfomErr::UnknownType(tx_type.clone())),
        }
    }
    pub fn dispute(&mut self, tx: &Transaction) -> Result<(), AccountPerfomErr> {
        let disputed = tx.amount();
        self.held += disputed;
        self.available -= disputed;
        self.total = self.calc_total();
        Ok(())
    }
    pub fn resolve(&mut self, tx: &Transaction) -> Result<(), AccountPerfomErr> {
        let disputed = tx.amount();
        self.held -= disputed;
        self.available += disputed;
        self.total = self.calc_total();
        Ok(())
    }
    pub fn chargeback(&mut self, tx: &Transaction) -> Result<(), AccountPerfomErr> {
        let disputed = tx.amount();
        self.held -= disputed;
        self.locked = true;
        self.total = self.calc_total();
        Ok(())
    }
    pub fn calc_total(&self) -> f32 {
        self.available + self.held
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_do_deposit() {
        let mut account = Account {
            client: 1,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        };
        let transaction = Transaction::new(TransactionType::Deposit, 1, 1, Some(1.0));
        let performed_correctly = account.perform(&transaction).is_ok();
        assert!(performed_correctly);
        assert_eq!(account.available, 1.0);
        assert_eq!(account.total, 1.0);
    }

    #[test]
    fn test_do_withdrawal() {
        let mut account = Account {
            client: 1,
            available: 5.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        };
        let transaction = Transaction::new(TransactionType::Withdrawal, 1, 1, Some(1.1234));
        let performed_correctly = account.perform(&transaction).is_ok();
        assert!(performed_correctly);
        assert_eq!(account.available, 3.8766);
        assert_eq!(account.total, 3.8766);
    }

    #[test]
    fn test_do_dispute() {
        let mut account = Account {
            client: 1,
            available: 5.0,
            held: 0.0,
            total: 5.0,
            locked: false,
        };
        let transaction = Transaction::new(TransactionType::Deposit, 1, 1, Some(1.1234));
        let dispute_correctly = account.dispute(&transaction).is_ok();
        assert!(dispute_correctly);
        assert_eq!(account.available, 5.0 - 1.1234);
        assert_eq!(account.held, 1.1234);
        assert_eq!(account.total, 5.0);
    }

    #[test]
    fn test_do_resolve() {
        let mut account = Account {
            client: 1,
            available: 0.0,
            held: 5.0,
            total: 5.0,
            locked: false,
        };
        let transaction = Transaction::new(TransactionType::Deposit, 1, 1, Some(1.1234));
        let resolved_correctly = account.resolve(&transaction).is_ok();
        assert!(resolved_correctly);
        assert_eq!(account.available, 1.1234);
        assert_eq!(account.held, 5.0 - 1.1234);
        assert_eq!(account.total, 5.0);
    }

    #[test]
    fn test_do_chargeback() {
        let mut account = Account {
            client: 1,
            available: 0.0,
            held: 5.0,
            total: 5.0,
            locked: false,
        };
        let transaction = Transaction::new(TransactionType::Deposit, 1, 1, Some(1.1234));
        let resolved_correctly = account.chargeback(&transaction).is_ok();
        assert!(resolved_correctly);
        assert_eq!(account.available, 0.0);
        assert_eq!(account.held, 5.0 - 1.1234);
        assert_eq!(account.total, 5.0 - 1.1234);
        assert_eq!(account.locked, true);
    }
}
