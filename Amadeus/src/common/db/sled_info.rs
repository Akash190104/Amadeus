use anyhow::Result;

use once_cell::sync::OnceCell;

static SLED: &str = "trees/info.sled";
static SLED_DB: OnceCell<sled::Db> = OnceCell::new();

fn get_db_handle() -> Result<&'static sled::Db> {
  if let Some(existing_handle) = SLED_DB.get() {
    Ok(existing_handle)
  } else {
    let sled = sled::open(SLED)?;
    SLED_DB.set(sled).map_err(|_| anyhow!("Failed to store db handle"))?;
    get_db_handle()
  }
}

pub fn store(key: &str, value: &str) -> Result<()> {
  let sled = get_db_handle()?;
  sled.insert(key, value)?;
  Ok(())
}

pub fn read(key: &str) -> Result<String> {
  let sled = get_db_handle()?;
  match sled.get(key) {
    Ok(Some(value)) => {
      String::from_utf8(value.to_vec())
        .map_err(|error| anyhow!("Failed to parse utf8 {error}"))
    },
    Ok(None) => Err(anyhow!("value not found")),
    Err(e) => Err(anyhow!("operational problem encountered: {e}"))
  }
}

pub fn list() -> Result<String> {
  let sled = get_db_handle()?;
  let mut result = vec![];
  for k in sled.iter().keys().flatten() {
    if let Ok(kk) = String::from_utf8(k.to_vec())
                      .map_err(|error| anyhow!("Failed to parse utf8 {error}")) {
      result.push(kk);
    }
  }
  Ok(result.join("\n"))
}

pub fn delete(key: &str) -> Result<()> {
  let sled = get_db_handle()?;
  sled.remove(key)?;
  Ok(())
}
