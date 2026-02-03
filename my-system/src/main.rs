use bank_system::BalanceManager;
use bank_system::Name;
use bank_system::Storage;
use bank_system::UserManager;

fn main() {
    let mut storage = Storage::new();

    let user: Name = "Mark".to_string();

    let amount: i64 = 200;

    storage.add_user(user.clone());
    let _ = storage.deposit(&user, amount);

    println!("{:?}", storage.get_balance(&user));
    println!("{:?}", storage.get_balance(&"Kirill".to_string()));
}