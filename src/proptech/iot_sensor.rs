use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorTelemetry {
    pub device_id: String,
    pub building_id: String,
    pub structural_fault_detected: bool,
    pub metric_reading: f64,
}

pub struct PropertyIotBridge;

impl PropertyIotBridge {
    pub fn process_telemetry(&self, data: SensorTelemetry) -> Option<serde_json::Value> {
        if data.structural_fault_detected {
            // Generates an automated work order dispatch trigger for the operations wing
            return Some(serde_json::json!({
                "action": "DISPATCH_MAINTENANCE",
                "target_building": data.building_id,
                "urgency": "HIGH"
            }));
        }
        None
    }
}
