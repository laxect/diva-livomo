use once_cell::sync::{Lazy, OnceCell};

static DB: Lazy<sled::Db> = Lazy::new(|| {
    let data_dir = dirs::data_dir()
        .expect("XDG config not setting")
        .join("diva-līvõmō")
        .join("sled");
    sled::open(data_dir).unwrap()
});

static DIFF_ON: OnceCell<bool> = OnceCell::new();

pub(crate) fn is_new<K: AsRef<[u8]>>(key: K) -> bool {
    if *DIFF_ON.get_or_init(|| true) {
        !DB.contains_key(key).unwrap()
    } else {
        true
    }
}

pub(crate) fn add_key<K: AsRef<[u8]>>(key: K) -> anyhow::Result<()> {
    if *DIFF_ON.get_or_init(|| true) {
        DB.insert(key, &[])?;
    }
    Ok(())
}

pub fn save() -> anyhow::Result<()> {
    if *DIFF_ON.get_or_init(|| true) {
        DB.flush()?;
    }
    Ok(())
}

pub fn set_diff_flag(diff_on: bool) {
    DIFF_ON.set(diff_on).unwrap()
}
