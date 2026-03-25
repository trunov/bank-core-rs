use crate::{Balance, Name, storage::Storage};

#[derive(Debug)]
pub enum BalanceManagerError {
    UserNotFound(Name),
    NotEnoughMoney { required: i64, available: i64 },
}

pub trait BalanceManager {
    fn deposit_simple(&mut self, name: &Name, amount: Balance) -> Result<(), BalanceManagerError>;
    fn withdraw_simple(&mut self, name: &Name, amount: Balance) -> Result<(), BalanceManagerError>;
}

impl BalanceManager for Storage {
    fn deposit_simple(&mut self, name: &Name, amount: Balance) -> Result<(), BalanceManagerError> {
        if let Some(balance) = self.accounts.get_mut(name) {
            *balance += amount;
            Ok(())
        } else {
            // "Пользователь не найден".into()
            Err(BalanceManagerError::UserNotFound(name.clone()))
        }
    }

    fn withdraw_simple(&mut self, name: &Name, amount: Balance) -> Result<(), BalanceManagerError> {
        if let Some(balance) = self.accounts.get_mut(name) {
            if *balance >= amount {
                *balance -= amount;
                Ok(())
            } else {
                // "Недостаточно средств".into()
                Err(BalanceManagerError::NotEnoughMoney {
                    required: amount,
                    available: *balance,
                })
            }
        } else {
            // "Пользователь не найден".into()
            Err(BalanceManagerError::UserNotFound(name.clone()))
        }
    }
}

fn process_if_deposit(
    storage: &mut Storage,
    is_deposit_and_sums: &[(bool, Name, Balance)],
) -> Result<(), BalanceManagerError> {
    for (is_deposit, name, sum) in is_deposit_and_sums {
        if *is_deposit {
            storage.deposit_simple(name, *sum)?;
        } else {
            storage.withdraw_simple(name, *sum)?;
        }
    }

    Ok(())
}
