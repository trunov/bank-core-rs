use std::collections::HashMap;

type Name = String;
type Balance = i64;

struct Storage {
    accounts: HashMap<Name, Balance>,
}

impl Storage {
    fn new() -> Self {
        Storage {
            accounts: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, name: Name) -> Option<Balance> {
        if self.accounts.contains_key(&name) {
            None
        } else {
            self.accounts.insert(name, 0);
            Some(0)
        }
    }

    pub fn remove_user(&mut self, name: &Name) -> Option<Balance> {
        self.accounts.remove(name)
    }

    pub fn get_balance(&self, name: &Name) -> Option<Balance> {
        self.accounts.get(name).copied()
    }

    pub fn deposit(&mut self, name: &Name, amount: Balance) -> Result<(), String> {
        if let Some(balance) = self.accounts.get_mut(name) {
            *balance += amount;
            Ok(())
        } else {
            Err("User is not found".into())
        }
    }

    pub fn withdraw(&mut self, name: &Name, amount:Balance) -> Result<(), String> {
        if let Some(balance) = self.accounts.get_mut(name) {
            if *balance >= amount {
                *balance -= amount;
                Ok(())
            } else {
                Err("Not enough money on balance".into())
            }
        } else {
            Err("User is not found".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // подключаем все из родительского модуля
    #[test]
    fn test_new_storage_is_empty() {
        let bank = Storage::new();
        assert_eq!(bank.accounts.len(), 0);
    }

    #[test]
    fn test_add_user() {
        let mut storage = Storage::new();
        let option1 = storage.add_user("Kirill".to_string());
        assert_eq!(option1, Some(0));

        let option2 = storage.add_user("Kirill".to_string());
        assert_eq!(option2, None);
    }

    #[test]
    fn test_remove_user() {
        let mut storage = Storage::new();
        storage.add_user("Kirill".to_string());
        storage.deposit(&"Kirill".to_string(), 100).unwrap();

        assert_eq!(storage.remove_user(&"Kirill".to_string()), Some(100)); // удаляем и получаем баланс
        assert_eq!(storage.remove_user(&"Kirill".to_string()), None); // второй раз — не найден
    }

    #[test]
    fn test_deposit_and_withdraw() {
        let mut storage = Storage::new();
        storage.add_user("Kirill".to_string());
        let result = storage.deposit(&"Kirill".to_string(), 100);
        // Успешное пополнение
        assert_eq!(result, Ok(()));
        assert_eq!(storage.get_balance(&"Kirill".to_string()), Some(100));

        // Успешное снятие
        assert!(storage.withdraw(&"Kirill".to_string(), 50).is_ok());
        assert_eq!(storage.get_balance(&"Kirill".to_string()), Some(50));

        // Ошибка: недостаточно средств
        assert!(storage.withdraw(&"Kirill".to_string(), 100).is_err());
        assert_eq!(storage.get_balance(&"Kirill".to_string()), Some(50));
    }

    #[test]
    fn test_nonexistent_user() {
        let mut storage = Storage::new();

        assert!(storage.deposit(&"Diana".to_string(), 100).is_err());
        assert!(storage.withdraw(&"Diana".to_string(), 50).is_err());

        assert_eq!(storage.get_balance(&"Diana".to_string()), None);      
    }
}
