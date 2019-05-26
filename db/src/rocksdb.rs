use rocksdb;

use accounts::account::Account;

pub fn set(account: Account) {
    let db = DB::open_default("path/for/rocksdb/storage").unwrap();
    db.put(account.address.bytes(), serde_json::to_string(&account));
    db.delete(b"my key").unwrap();
}

pub fn get(address: &String) -> Result<Option<Account>> {
    match db.get(address.bytes()) {
        Ok(Some(value)) => {
            let account: Account = serde_json::from_str(value.to_utf8().unwrap());
            return Result<account>;
        },
        Ok(None) => return Result<None>,
        Err(e) => return Result(e),
    }
}
