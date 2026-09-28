
// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/device_setting
use std::collections::BTreeMap;

use teaql_macros::{teaql_entity, TeaqlEntity};


/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[teaql_entity]
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "DeviceSetting", table = "device_setting_data", data_service = "sqlite", audit_mask_fields = "password_hash,password_enabled,super_password_hash")]
pub struct DeviceSetting {
#[teaql(id)]
    id: u64,

// @source model.xml:39
    calibration_point: i64,

// @source model.xml:39
    data_keep_days: i64,

// @source model.xml:39
    sampling_frequency: i64,

// @source model.xml:39
    password_enabled: i64,

// @source model.xml:39
#[teaql(max_length = 100)]
    password_hash: String,

// @source model.xml:39
#[teaql(max_length = 100)]
    super_password_hash: String,

// @source model.xml:39
    create_time: teaql_core::time::Timestamp,

// @source model.xml:39
    update_time: teaql_core::time::Timestamp,
#[teaql(version)]
    version: i64,
// @source model.xml:39
#[teaql(column = "device_system")]
    device_system_id: u64,
// @source model.xml:39
#[teaql(relation(target = "DeviceSystem", local_key = "device_system_id", foreign_key = "id"))]
    device_system: Option<Box<crate::DeviceSystem>>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl DeviceSetting {
    pub const ENTITY_NAME: &'static str = "Device Setting";

    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRuntimeState) -> Self {
        Self {
            id: 0_u64,
            calibration_point: 0_i64,
            data_keep_days: 0_i64,
            sampling_frequency: 0_i64,
            password_enabled: 0_i64,
            password_hash: String::new(),
            super_password_hash: String::new(),
            create_time: teaql_core::time::Timestamp::now(),
            update_time: teaql_core::time::Timestamp::now(),
            version: 0_i64,
            device_system_id: 0_u64,
            device_system: None,
            dynamic: BTreeMap::new(),
            __teaql_runtime_state: root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn attach_runtime_state_recursive(&mut self, root: teaql_runtime::EntityRuntimeState) {
        root.adopt_mutations_from(self.__teaql_runtime_state());
        self.__teaql_replace_runtime_state(root.clone());
        if let Some(entity) = &mut self.device_system {
            entity.attach_runtime_state_recursive(root.clone());
        }
    }

    pub fn is_loaded(&self, field_or_relation: &str) -> bool {
        self.__load_state.is_loaded(field_or_relation)
    }

    pub fn set_load_state(&mut self, state: teaql_core::eval::LoadState) {
        self.__load_state = state;
    }

    pub fn id(&self) -> u64 {
        self.changed_id().and_then(|value| value.try_u64()).unwrap_or(self.id)
    }

    pub fn update_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.id = value.try_u64().unwrap_or(self.id.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "id", value);
        self
    }

    pub fn changed_id(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "id")
    }

    pub fn eval_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "id".to_string(), attempted_path: "id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.id())
                }}


    pub fn calibration_point(&self) -> i64 {
        self.changed_calibration_point().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.calibration_point)
    }

    pub fn update_calibration_point(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.calibration_point = value.try_i64().map(|value| value as i64).unwrap_or(self.calibration_point.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "calibration_point", value);
        self
    }

    pub fn changed_calibration_point(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "calibration_point")
    }

    pub fn eval_calibration_point(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("calibration_point") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "calibration_point".to_string(), attempted_path: "calibration_point".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.calibration_point())
                }}


    pub fn data_keep_days(&self) -> i64 {
        self.changed_data_keep_days().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.data_keep_days)
    }

    pub fn update_data_keep_days(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.data_keep_days = value.try_i64().map(|value| value as i64).unwrap_or(self.data_keep_days.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "data_keep_days", value);
        self
    }

    pub fn changed_data_keep_days(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "data_keep_days")
    }

    pub fn eval_data_keep_days(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("data_keep_days") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "data_keep_days".to_string(), attempted_path: "data_keep_days".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.data_keep_days())
                }}


    pub fn sampling_frequency(&self) -> i64 {
        self.changed_sampling_frequency().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.sampling_frequency)
    }

    pub fn update_sampling_frequency(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.sampling_frequency = value.try_i64().map(|value| value as i64).unwrap_or(self.sampling_frequency.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "sampling_frequency", value);
        self
    }

    pub fn changed_sampling_frequency(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "sampling_frequency")
    }

    pub fn eval_sampling_frequency(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("sampling_frequency") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "sampling_frequency".to_string(), attempted_path: "sampling_frequency".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.sampling_frequency())
                }}


    pub fn password_enabled(&self) -> i64 {
        self.changed_password_enabled().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.password_enabled)
    }

    pub fn update_password_enabled(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.password_enabled = value.try_i64().map(|value| value as i64).unwrap_or(self.password_enabled.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "password_enabled", value);
        self
    }

    pub fn changed_password_enabled(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "password_enabled")
    }

    pub fn eval_password_enabled(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("password_enabled") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "password_enabled".to_string(), attempted_path: "password_enabled".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.password_enabled())
                }}


    pub fn password_hash(&self) -> String {
        self.changed_password_hash().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.password_hash.clone())
    }

    pub fn update_password_hash(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.password_hash = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.password_hash.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "password_hash", value);
        self
    }

    pub fn changed_password_hash(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "password_hash")
    }

    pub fn eval_password_hash(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("password_hash") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "password_hash".to_string(), attempted_path: "password_hash".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.password_hash())
                }}


    pub fn super_password_hash(&self) -> String {
        self.changed_super_password_hash().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.super_password_hash.clone())
    }

    pub fn update_super_password_hash(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.super_password_hash = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.super_password_hash.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "super_password_hash", value);
        self
    }

    pub fn changed_super_password_hash(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "super_password_hash")
    }

    pub fn eval_super_password_hash(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("super_password_hash") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "super_password_hash".to_string(), attempted_path: "super_password_hash".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.super_password_hash())
                }}


    pub fn create_time(&self) -> teaql_core::time::Timestamp {
        self.changed_create_time().and_then(|value| value.try_timestamp()).unwrap_or(self.create_time)
    }

    pub fn update_create_time(&mut self, value: teaql_core::time::Timestamp) -> &mut Self {
        self.create_time = value;
        let value = teaql_core::Value::from(value);
        self.__teaql_runtime_state().set(self.entity_key(), "create_time", value);
        self
    }
    pub fn changed_create_time(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "create_time")
    }

    pub fn eval_create_time(&self) -> teaql_core::eval::EvalResult<teaql_core::time::Timestamp> {
        if !self.is_loaded("create_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "create_time".to_string(), attempted_path: "create_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.create_time())
                }}


    pub fn update_time(&self) -> teaql_core::time::Timestamp {
        self.changed_update_time().and_then(|value| value.try_timestamp()).unwrap_or(self.update_time)
    }

    pub fn update_update_time(&mut self, value: teaql_core::time::Timestamp) -> &mut Self {
        self.update_time = value;
        let value = teaql_core::Value::from(value);
        self.__teaql_runtime_state().set(self.entity_key(), "update_time", value);
        self
    }
    pub fn changed_update_time(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "update_time")
    }

    pub fn eval_update_time(&self) -> teaql_core::eval::EvalResult<teaql_core::time::Timestamp> {
        if !self.is_loaded("update_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "update_time".to_string(), attempted_path: "update_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.update_time())
                }}


    pub fn version(&self) -> i64 {
        self.changed_version().and_then(|value| value.try_i64()).unwrap_or(self.version)
    }

    pub fn update_version(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version = value.try_i64().unwrap_or(self.version.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "version", value);
        self
    }

    pub fn changed_version(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "version")
    }

    pub fn eval_version(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("version") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version".to_string(), attempted_path: "version".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version())
                }}

    pub fn device_system_id(&self) -> u64 {
        self.changed_device_system_id().and_then(|value| value.try_u64()).unwrap_or(self.device_system_id)
    }

    pub fn update_device_system_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.device_system_id = value.try_u64().unwrap_or(self.device_system_id.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "device_system_id", value);
        self
    }

    pub fn changed_device_system_id(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "device_system_id")
    }

    pub fn eval_device_system_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("device_system_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "device_system_id".to_string(), attempted_path: "device_system_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.device_system_id())
                }}
    pub fn device_system(&self) -> Option<&crate::DeviceSystem> {
        self.device_system.as_deref().or_else(|| {
            self.__teaql_runtime_state().resolve_entity(self.device_system_id())})
    }

    pub fn eval_device_system(&self) -> teaql_core::eval::EvalResult<&crate::DeviceSystem> {
        match self.device_system() {
            Some(v) => teaql_core::eval::EvalResult::Value(v),
            None if self.is_loaded("device_system") => teaql_core::eval::EvalResult::Null,
            None => teaql_core::eval::EvalResult::NotLoaded { failed_node: "device_system".to_string(), attempted_path: "device_system".to_string() },
        }
    }

}

