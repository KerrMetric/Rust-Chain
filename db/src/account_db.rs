use accounts::account::Account;
use rocksdb;

lazy_static! {
    static ref DB: rocksdb::DB =
        { rocksdb::DB::open_default("/Users/takumikaribe/Develop/rust-chain/tmp").unwrap() };
}

pub fn set(account: &Account) -> Result<(), rocksdb::Error> {
    DB.put(
        &(account.address.as_bytes()[..]).as_ref(),
        &(serde_json::to_string(&account).ok().unwrap().as_bytes()[..]).as_ref(),
    )
}

pub fn get(address: &String) -> Option<Account> {
    match DB.get(address) {
        Ok(Some(value)) => {
            let account: Option<Account> = serde_json::from_str(value.to_utf8().unwrap()).ok();
            return account;
        }
        _ => None,
    }
}

pub fn delete(address: &String) -> Result<(), rocksdb::Error> {
    DB.delete(&(address.as_bytes()[..]).as_ref())
}
