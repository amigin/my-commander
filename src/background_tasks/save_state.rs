use crate::PersistenceState;

pub async fn save_state(state: PersistenceState) {
    let result = state.save().await;

    if let Err(err) = result {
        println!("Failed to save state. Err:{}", err);
    }
}
