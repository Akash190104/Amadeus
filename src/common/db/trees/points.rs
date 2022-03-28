use crate::common::db::trees;

use cannyls::lump::{ LumpData, LumpId };

use tokio::task;

use bincode::config;

#[derive(bincode::Encode, bincode::Decode)]
struct Points {
  count: u64,
  streak: u64
}

pub async fn add_points( guild_id: u64
                       , user_id: u64
                       , new_points: u64) {
  let mut storage = trees::STORAGE.lock().await;
  let u64_2: u128 = (guild_id as u128) << 64 | user_id as u128; // >
  let lump_id: LumpId = LumpId::new(u64_2);
  if let Err(awhy) = task::spawn_blocking(move || {
    match storage.get(&lump_id) {
      Ok(mbdata) => {
        if let Some(mut data) = mbdata {
          let byte_data: &mut [u8] = data.as_bytes_mut();
          if let Ok((mut points, _len)) = bincode::decode_from_slice::<Points,_>(byte_data, config::standard()) {
            points.count += new_points;
            let mut new_bytes = [0u8; 16];
            if let Ok(_encoded_len) = bincode::encode_into_slice(&points, &mut new_bytes, config::standard()) {
              (*byte_data).copy_from_slice(&new_bytes[..]);
              match storage.put(&lump_id, &data) {
                Ok(added) => {
                  if added {
                    error!("error updating points");
                  }
                }, Err(not_added) => {
                  error!("failed to add points {not_added}");
                }
              }
            }
          }
        } else {
          let points = Points { count: 0, streak: 0 };
          let mut encoded = [0u8; 16];
          if let Ok(_encoded_len) = bincode::encode_into_slice(&points, &mut encoded, config::standard()) {
            if let Ok(lump_data) = LumpData::new(encoded.into()) {
              match storage.put(&lump_id, &lump_data) {
                Ok(added) => {
                  if !added {
                    error!("error on points initialization");
                  }
                }, Err(not_added) => {
                  error!("error on points initialization {not_added}");
                }
              }
            }
          }
        }
        if let Err(khm) = storage.journal_sync() {
          error!("failed to sync {khm}");
        }
      }, Err(why) => {
        error!("Failed to get key: {why}");
      }
    }
  }).await {
    error!("failed to spawn tokio task {awhy}")
  }
}

pub async fn give_points( guild_id: u64
                        , user_id: u64
                        , target_user_id: u64
                        , points_count: u64) -> (bool, String) {
  let mut storage = trees::STORAGE.lock().await;
  let u64_2: u128 = (guild_id as u128) << 64 | user_id as u128; // >
  let tu64_2: u128 = (guild_id as u128) << 64 | target_user_id as u128; // >
  let lump_id: LumpId = LumpId::new(u64_2);
  let target_lump_id: LumpId = LumpId::new(tu64_2);
  match storage.get(&lump_id) {
    Ok(mbdata) => {
      if let Some(mut data) = mbdata {
        let byte_data: &mut [u8] = data.as_bytes_mut();
        if let Ok((mut points, _len)) = bincode::decode_from_slice::<Points,_>(byte_data, config::standard()) {
          if points.count >= points_count {
            points.count -= points_count;
            let mut new_bytes = [0u8; 16];
            if let Err(endode_err) = bincode::encode_into_slice(&points, &mut new_bytes, config::standard()) {
              error!("Error encoding ponts: {endode_err}");
            }
            (*byte_data).copy_from_slice(&new_bytes[..]);
            if let Ok(added) = storage.put(&lump_id, &data) {
              if added {
                error!("Some strange error updating giver points");
              }
            }
            match storage.get(&target_lump_id) {
              Ok(tmbdata) => {
                if let Some(mut tdata) = tmbdata {
                  let tbyte_data: &mut [u8] = tdata.as_bytes_mut();
                  if let Ok((mut tpoints, _len)) = bincode::decode_from_slice::<Points,_>(tbyte_data, config::standard()) {
                    tpoints.count += points_count;
                    let mut tnew_bytes = [0u8; 16];
                    if let Err(endode_err) = bincode::encode_into_slice(&tpoints, &mut tnew_bytes, config::standard()) {
                      error!("Error encoding ponts: {endode_err}");
                    }
                    (*tbyte_data).copy_from_slice(&tnew_bytes[..]);
                    if let Ok(tadded) = storage.put(&target_lump_id, &tdata) {
                      if tadded {
                        error!("Some strange error updating receiver points");
                      }
                    }
                  }
                } else {
                  let tpoints = Points { count: points_count, streak: 0 };
                  let mut tencoded = [0u8; 16];
                  if let Err(endode_err) = bincode::encode_into_slice(&tpoints, &mut tencoded, config::standard()) {
                    error!("Error encoding ponts: {endode_err}");
                  }
                  if let Ok(tlump_data) = LumpData::new(tencoded.into()) {
                    if let Ok(tadded) = storage.put(&target_lump_id, &tlump_data) {
                      if !tadded {
                        error!("Some strange error updating receiver points");
                      }
                    }
                  }
                }
                if let Err(khm) = storage.journal_sync() {
                  error!("failed to sync {:?}", khm);
                }
                (true, format!("{points_count} points transfered"))
              }, Err(why) => {
                error!("Failed to get key: {why}");
                (false, String::from("error accessing points"))
              }
            }
          } else {
            (false, String::from("not enough points to give"))
          }
        } else {
          (false, String::from("you have no points to give"))
        }
      } else {
        (false, String::from("error decoding points"))
      }
    }, Err(why) => {
      error!("Failed to get key: {why}");
      (false, String::from("error accessing points"))
    }
  }
}

pub async fn get_points(guild_id: u64, user_id: u64) -> anyhow::Result<u64> {
  let mut storage = trees::STORAGE.lock().await;
  task::spawn_blocking(move || {
    let u64_2: u128 = (guild_id as u128) << 64 | user_id as u128; // >
    let lump_id: LumpId = LumpId::new(u64_2);
    if let Ok(Some(mut data)) = storage.get(&lump_id) {
      let byte_data: &mut [u8] = data.as_bytes_mut();
      match bincode::decode_from_slice::<Points,_>(byte_data, config::standard()) {
        Ok((points, _len)) => return Ok(points.count),
        Err(get_error) => {
          error!("Get error: {get_error}");
          return Ok(0);
        }
      };
    }
    Ok(0)
  }).await?
}

pub async fn clear_points(guild_id: u64, user_id: u64) -> Result<bool, cannyls::Error> {
  let mut storage = trees::STORAGE.lock().await;
  match task::spawn_blocking(move || {
    let u64_2: u128 = (guild_id as u128) << 64 | user_id as u128; // >
    let lump_id: LumpId = LumpId::new(u64_2);
    storage.delete(&lump_id)
  }).await {
    Ok(r) => r,
    Err(why) => {
      error!("error clearing points {why}");
      Ok(false)
    }
  }
}

pub async fn add_win_points( guild_id: u64
                           , user_id: u64 ) -> u64 {
  let mut storage = trees::STORAGE.lock().await;
  let u64_2: u128 = (guild_id as u128) << 64 | user_id as u128; // >
  let lump_id: LumpId = LumpId::new(u64_2);
  match task::spawn_blocking(move || {
    match storage.get(&lump_id) {
      Ok(mbdata) => {
        if let Some(mut data) = mbdata {
          let byte_data: &mut [u8] = data.as_bytes_mut();
          if let Ok((mut points, _len)) = bincode::decode_from_slice::<Points,_>(byte_data, config::standard()) {
            points.count += 10;
            points.streak += 1;
            if points.streak > 3 {
              let points_multiplier = points.streak - 3;
              points.count += 5 * points_multiplier;
            }
            let mut new_bytes = [0u8; 16];
            if let Err(endode_err) = bincode::encode_into_slice(&points, &mut new_bytes, config::standard()) {
              error!("Error encoding ponts: {endode_err}");
              return 0;
            }
            (*byte_data).copy_from_slice(&new_bytes[..]);
            match storage.put(&lump_id, &data) {
              Ok(added) => {
                if added {
                  error!("error updating points");
                }
                if let Err(khm) = storage.journal_sync() {
                  error!("failed to sync {khm}");
                }
                points.streak
              }, Err(ecn) => {
                error!("Something wrong with cannyls: {ecn}");
                0
              }
            }
          } else { 0 }
        } else {
          let points = Points { count: 10, streak: 1 };
          let mut encoded = [0u8; 16];
          if let Err(endode_err) = bincode::encode_into_slice(&points, &mut encoded, config::standard()) {
            error!("Error encoding ponts: {endode_err}");
            return 0;
          }
          if let Ok(lump_data) = LumpData::new(encoded.into()) {
            if let Ok(added) = storage.put(&lump_id, &lump_data) {
              if !added {
                error!("error on points initialization");
              }
              if let Err(khm) = storage.journal_sync() {
                error!("failed to sync {khm}");
              }
              1
            } else {
              error!("Something is wrong with cannyls");
              0
            }
          } else { 0 }
        }
      }, Err(why) => {
        error!("Failed to get key: {why}");
        0
      }
    }
  }).await {
    Ok(r) => r,
    Err(why) => {
      error!("failed to add win {why}"); 0
    }
  }
}

pub async fn break_streak( guild_id: u64
                         , user_id: u64 ) {
  let mut storage = trees::STORAGE.lock().await;
  let u64_2: u128 = (guild_id as u128) << 64 | user_id as u128; // >
  let lump_id: LumpId = LumpId::new(u64_2);
  if let Err(why) = task::spawn_blocking(move || {
    match storage.get(&lump_id) {
      Ok(mbdata) => {
        if let Some(mut data) = mbdata {
          let byte_data: &mut [u8] = data.as_bytes_mut();
          if let Ok((mut points, _len)) = bincode::decode_from_slice::<Points,_>(byte_data, config::standard()) {
            points.streak = 0;
            let mut new_bytes = [0u8; 16];
            if let Err(endode_err) = bincode::encode_into_slice(&points, &mut new_bytes, config::standard()) {
              error!("Error encoding ponts: {endode_err}");
            }
            (*byte_data).copy_from_slice(&new_bytes[..]);
            if let Ok(added) = storage.put(&lump_id, &data) {
              if added {
                error!("error updating points");
              }
              if let Err(khm) = storage.journal_sync() {
                error!("failed to sync {khm}");
              }
            }
          }
        }
      }, Err(why) => {
        error!("Failed to get key: {why}");
      }
    }
  }).await {
    error!("{why}");
  }
}
