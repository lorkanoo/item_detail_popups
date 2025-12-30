use keyring::Entry;

pub fn store_password(key: &str, value: &str) {
    let entry = create_entry(key);
    entry.set_password(value).expect("Failed to set password");
}

pub fn get_password(key: &str) -> Option<String> {
    let entry = create_entry(key);
    entry.get_password().ok()
}

pub fn delete_password(key: &str) {
    let entry = create_entry(key);
    entry
        .delete_credential()
        .expect("Failed to delete password");
}

fn create_entry(key: &str) -> Entry {
    Entry::new(key, "default").expect("Failed to create keyring entry")
}
