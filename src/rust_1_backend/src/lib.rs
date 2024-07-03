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
    static MOISTURE_THRESHOLD: RefCell<Option<f64>> = RefCell::new(None);
}

#[update]
fn update_moisture_level(moisture_level: f64) {
    MOISTURE_LEVEL.with(|level| {
        *level.borrow_mut() = moisture_level;
    });
    let timestamp = ic_cdk::api::time();
    MOISTURE_LOG.with(|log| {
        log.borrow_mut().push(MoistureData {
            moisture_level,
            timestamp,
        });
    });
    check_and_control_pump(moisture_level);
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

#[update]
fn set_moisture_threshold(threshold: f64) {
    MOISTURE_THRESHOLD.with(|t| {
        *t.borrow_mut() = Some(threshold);
    });
}

#[query]
fn get_moisture_threshold() -> Option<f64> {
    MOISTURE_THRESHOLD.with(|t| *t.borrow())
}

fn check_and_control_pump(moisture_level: f64) {
    MOISTURE_THRESHOLD.with(|t| {
        if let Some(threshold) = *t.borrow() {
            if moisture_level < threshold {
                control_pump(true); // Turn on the pump
            } else {
                control_pump(false); // Turn off the pump
            }
        }
    });
}

#[update]
fn remove_old_log_entries(cutoff_timestamp: u64) {
    MOISTURE_LOG.with(|log| {
        log.borrow_mut().retain(|entry| entry.timestamp >= cutoff_timestamp);
    });
}

#[query]
fn get_average_moisture(start_timestamp: u64, end_timestamp: u64) -> f64 {
    MOISTURE_LOG.with(|log| {
        let log = log.borrow();
        let filtered: Vec<&MoistureData> = log.iter().filter(|entry| entry.timestamp >= start_timestamp && entry.timestamp <= end_timestamp).collect();
        if filtered.is_empty() {
            0.0
        } else {
            let sum: f64 = filtered.iter().map(|entry| entry.moisture_level).sum();
            sum / filtered.len() as f64
        }
    })
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

    #[test]
    fn test_update_moisture_level() {
        update_moisture_level(25.0);
        let level = get_moisture_level();
        assert_eq!(level, 25.0);
    }

    #[test]
    fn test_control_pump() {
        control_pump(true);
        assert!(get_pump_state());
        control_pump(false);
        assert!(!get_pump_state());
    }

    #[test]
    fn test_set_and_get_moisture_threshold() {
        set_moisture_threshold(30.0);
        assert_eq!(get_moisture_threshold(), Some(30.0));
    }

    #[test]
    fn test_check_and_control_pump() {
        set_moisture_threshold(20.0);
        update_moisture_level(15.0);
        assert!(get_pump_state());
        update_moisture_level(25.0);
        assert!(!get_pump_state());
    }

    #[test]
    fn test_remove_old_log_entries() {
        update_moisture_level(10.0);
        let timestamp = ic_cdk::api::time();
        ic_cdk::api::time::advance(1); // Simulate time passing
        update_moisture_level(20.0);
        remove_old_log_entries(timestamp);
        let log = get_moisture_log();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].moisture_level, 20.0);
    }

    #[test]
    fn test_get_average_moisture() {
        update_moisture_level(10.0);
        let start_timestamp = ic_cdk::api::time();
        ic_cdk::api::time::advance(1); // Simulate time passing
        update_moisture_level(20.0);
        let end_timestamp = ic_cdk::api::time();
        let average = get_average_moisture(start_timestamp, end_timestamp);
        assert_eq!(average, 15.0);
    }
}
