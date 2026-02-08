pub mod helpers;

use self::helpers::read_file;
use crate::{Balance, Name};
use std::collections::HashMap;
use std::fs::{self};
use std::io::{self, BufRead, BufWriter, Cursor};
use std::io::Write;
use std::path::Path;

pub struct Storage {
    accounts: HashMap<Name, Balance>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            accounts: HashMap::new(),
        }
    }

    pub fn load_data(file: &str) -> Storage {
        let mut storage = Storage::new();

        if Path::new(file).exists() {
            let file = match read_file(file) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to open file: {e}");
                    return storage;
                }
            };

            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.trim().split(',').collect();

                    if parts.len() == 2 {
                        let name = parts[0].to_string();
                        let balance: i64 = parts[1].parse().unwrap_or(0);

                        storage.add_user(name.clone());
                        let _ = storage.deposit(&name, balance);
                    }
                }
            }
        } else {
            for u in ["John", "Alice", "Bob", "Vasya"] {
                storage.add_user(u.to_string());
            }
        }
        storage
    }

    pub fn save(&self, file: &str) {
        let mut data = String::new();

        for (name, balance) in self.get_all() {
            data.push_str(&format!("{},{}\n", name, balance));
        }

        fs::write(file, data).expect("Could not write file")
    }

    pub fn get_all(&self) -> Vec<(Name, i64)> {
        self.accounts.iter().map(|(n, b)| (n.clone(), *b)).collect()
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

    pub fn withdraw(&mut self, name: &Name, amount: Balance) -> Result<(), String> {
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

    use std::{fs::{self}, io::BufReader};
    #[test]
    // TODO: make tests for load_data and save
    fn test_load_data_existing_file() {
        let data = b"John,100\nAlice,200\nBob,50\n";
        let mut cursor = Cursor::new(&data[..]);

        let mut storage: Storage = Storage::new();
        let reader = BufReader::new(&mut cursor);
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 2 {
                let name = parts[0].to_string();
                let balance = parts[1].parse().unwrap_or(0);
                storage.add_user(name.clone());
                storage.deposit(&name, balance).unwrap();
            }
        }

        assert_eq!(storage.get_balance(&"John".to_string()), Some(100));
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(200));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(50));
        // Пользователь Vasya не добавлен в файле, поэтому None
        assert_eq!(storage.get_balance(&"Vasya".to_string()), None);
    }

    #[test]
    fn test_save_writes_to_cursor_correctly() {
        // Создаём Storage и добавляем пользователей
        let mut storage = Storage::new();
        storage.add_user("John".to_string());
        storage.add_user("Alice".to_string());
        storage.deposit(&"John".to_string(), 150).unwrap();
        storage.deposit(&"Alice".to_string(), 300).unwrap();

        // Сохраняем в память через BufWriter
        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);
        {
            let mut writer = BufWriter::new(&mut cursor);
            for (name, balance) in storage.get_all() {
                writeln!(writer, "{},{}", name, balance).unwrap();
            }
            writer.flush().unwrap();
        }

        // Читаем обратно из памяти
        cursor.set_position(0);
        let mut lines: Vec<String> = BufReader::new(cursor).lines().map(|l| l.unwrap()).collect();
        lines.sort(); // сортируем для сравнения

        assert_eq!(lines, vec!["Alice,300", "John,150"]);
    } 

    #[test]
    fn test_save_creates_file_with_correct_data() {
        const FILE_NAME: &str = "test.csv";

        fs::write(FILE_NAME, "").expect("Could not write file");

        let mut storage: Storage = Storage::load_data(FILE_NAME);

        for (name, amount) in [("Alice", 300), ("John", 150)] {
            storage.add_user(name.to_string());
            match storage.deposit(&name.to_string(), amount) {
                Ok(()) => {
                    println!("Deposited: {} amount {}", name, amount);
                }
                Err(e) => println!("Error: {}", e),
            }
            storage.save(FILE_NAME);
        }

        let file = match read_file(FILE_NAME) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open file: {e}");
                return;
            }
        };

        let mut lines = vec![];
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                lines.push(line);
            }
        }

        lines.sort();
        assert_eq!(lines, vec!["Alice,300", "John,150"]);

        fs::remove_file(FILE_NAME).ok();
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
