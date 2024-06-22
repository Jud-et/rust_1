use candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Clone)]
struct MoistureData {
    moisture_level: f64,
    timestamp: u64,
}

thread_local! {
    static MOISTURE_LEVEL: RefCell<f64> = RefCell::new(0.0);
    static MOISTURE_LOG: RefCell<Vec<MoistureData>> = RefCell::new(Vec::new());
    static PUMP_STATE: RefCell<bool> = RefCell::new(false); // false = off, true = on
}

#[update]
fn update_moisture_level(moisture_level: f64) {
    MOISTURE_LEVEL.with(|level| {
        *level.borrow_mut() = moisture_level;
    });
    MOISTURE_LOG.with(|log| {
        log.borrow_mut().push(MoistureData {
            moisture_level,
            timestamp: ic_cdk::api::time(),
        });
    });
}

#[query]
fn get_moisture_level() -> f64 {
    MOISTURE_LEVEL.with(|level| *level.borrow())
}

#[query]
fn get_moisture_log() -> Vec<MoistureData> {
    MOISTURE_LOG.with(|log| log.borrow().clone())
}

#[update]
fn control_pump(state: bool) {
    PUMP_STATE.with(|pump| {
        *pump.borrow_mut() = state;
    });
    // Add hardware control logic here
}

#[query]
fn get_pump_state() -> bool {
    PUMP_STATE.with(|pump| *pump.borrow())
}

candid::export_service!();

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn generate_did_file() {
        let did = __export_service();
        let mut file = File::create("smart_watering_system.did").expect("Could not create file");
        file.write_all(did.as_bytes()).expect("Could not write to file");
    }
}