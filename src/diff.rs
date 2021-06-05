use std::lazy::SyncLazy;

static DB: SyncLazy<sled::Db> = SyncLazy::new(|| {
    let data_dir = dirs::data_dir()
        .expect("XDG config not setting")
        .join("diva-īvõmō")
        .join("sled");
    sled::open(data_dir).unwrap()
});

pub(crate) fn is_new<K: AsRef<[u8]>>(key: K) -> bool {
    !DB.contains_key(key).unwrap()
}

pub(crate) fn add_key<K: AsRef<[u8]>>(key: K) -> anyhow::Result<()> {
    DB.insert(key, &[])?;
    Ok(())
}

pub fn save() -> anyhow::Result<()> {
    DB.flush()?;
    Ok(())
}
