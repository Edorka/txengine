use crate::transactions::{Transaction, TransactionType};

pub type ClientID = u16;

pub struct Account {
    client: ClientID,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

enum AccountPerfomErr {
    UnknownType(TransactionType)

}

impl Account {
    fn perform(&mut self, tx: Transaction) -> Result<(), AccountPerfomErr> {
        assert_eq!(tx.client, self.client);

        match tx.tx_type {
            TransactionType::Deposit => {
                self.available += tx.amount();
                Ok(())
            },
            TransactionType::Withdrawal => {
                self.available -= tx.amount();
                Ok(())
            },
            tx_type => Err(AccountPerfomErr::UnknownType(tx_type))
        }
    } 
    pub fn total(&self) -> f32 {
        self.available + self.held
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_do_deposit() {
        let mut account = Account{
            client: 1,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        };
        let transaction = Transaction::new(
            TransactionType::Deposit, 1,1, Some(1.0)
        );
        account.perform(transaction);
        assert_eq!(account.available, 1.0);
        assert_eq!(account.total(), 1.0);
    }

    #[test]
    fn test_do_withdrawal() {
        let mut account = Account{
            client: 1,
            available: 5.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        };
        let transaction = Transaction::new(
            TransactionType::Withdrawal, 1,1, Some(1.1234)
        );
        account.perform(transaction);
        assert_eq!(account.available, 3.8766);
        assert_eq!(account.total(), 3.8766);
    }
}
