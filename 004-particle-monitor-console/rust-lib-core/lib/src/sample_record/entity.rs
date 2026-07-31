// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/sample_record
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "SampleRecord", table = "sample_record_data", data_service = "sqlite")]
pub struct SampleRecord {
#[teaql(id)]
    id: u64,

// @source model.xml:57
    sample_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:57
    gas: rust_decimal::Decimal,

// @source model.xml:57
    lref: rust_decimal::Decimal,

// @source model.xml:57
    impurity1: i64,

// @source model.xml:57
    impurity2: i64,

// @source model.xml:57
    impurity3: i64,

// @source model.xml:57
    impurity4: i64,

// @source model.xml:57
    impurity5: i64,

// @source model.xml:57
    impurity6: i64,

// @source model.xml:57
    impurity7: i64,

// @source model.xml:57
    impurity8: i64,

// @source model.xml:57
    create_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:57
#[teaql(column = "device_system")]
    device_system_id: u64,

// @source model.xml:57
#[teaql(column = "system_status")]
    system_status_id: u64,
// @source model.xml:57
#[teaql(relation(target = "DeviceSystem", local_key = "device_system_id", foreign_key = "id"))]
    device_system: Option<crate::DeviceSystem>,

// @source model.xml:57
#[teaql(relation(target = "SystemStatus", local_key = "system_status_id", foreign_key = "id"))]
    system_status: Option<crate::SystemStatus>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl SampleRecord {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            sample_time: chrono::Utc::now(),
            gas: rust_decimal::Decimal::ZERO,
            lref: rust_decimal::Decimal::ZERO,
            impurity1: 0_i64,
            impurity2: 0_i64,
            impurity3: 0_i64,
            impurity4: 0_i64,
            impurity5: 0_i64,
            impurity6: 0_i64,
            impurity7: 0_i64,
            impurity8: 0_i64,
            create_time: chrono::Utc::now(),
            version: 0_i64,
            device_system_id: 0_u64,
            system_status_id: 0_u64,
            device_system: None,
            system_status: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("SampleRecord", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.device_system {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.system_status {
            entity.attach_root_recursive(root.clone());
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
        self.root.set(self.entity_key(), "id", value);
        self
    }

    pub fn changed_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "id")
    }

    pub fn eval_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "id".to_string(), attempted_path: "id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.id())
                }}

    pub fn sample_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_sample_time().and_then(|value| value.try_timestamp()).unwrap_or(self.sample_time)
    }

    pub fn update_sample_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.sample_time = value.try_timestamp().unwrap_or(self.sample_time.clone());
        self.root.set(self.entity_key(), "sample_time", value);
        self
    }

    pub fn changed_sample_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "sample_time")
    }

    pub fn eval_sample_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("sample_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "sample_time".to_string(), attempted_path: "sample_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.sample_time())
                }}

    pub fn gas(&self) -> rust_decimal::Decimal {
        self.changed_gas().and_then(|value| value.try_decimal()).unwrap_or(self.gas)
    }

    pub fn update_gas(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.gas = value.try_decimal().unwrap_or(self.gas.clone());
        self.root.set(self.entity_key(), "gas", value);
        self
    }

    pub fn changed_gas(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "gas")
    }

    pub fn eval_gas(&self) -> teaql_core::eval::EvalResult<rust_decimal::Decimal> {
        if !self.is_loaded("gas") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "gas".to_string(), attempted_path: "gas".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.gas())
                }}

    pub fn lref(&self) -> rust_decimal::Decimal {
        self.changed_lref().and_then(|value| value.try_decimal()).unwrap_or(self.lref)
    }

    pub fn update_lref(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.lref = value.try_decimal().unwrap_or(self.lref.clone());
        self.root.set(self.entity_key(), "lref", value);
        self
    }

    pub fn changed_lref(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "lref")
    }

    pub fn eval_lref(&self) -> teaql_core::eval::EvalResult<rust_decimal::Decimal> {
        if !self.is_loaded("lref") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "lref".to_string(), attempted_path: "lref".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.lref())
                }}

    pub fn impurity1(&self) -> i64 {
        self.changed_impurity1().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.impurity1)
    }

    pub fn update_impurity1(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.impurity1 = value.try_i64().map(|value| value as i64).unwrap_or(self.impurity1.clone());
        self.root.set(self.entity_key(), "impurity1", value);
        self
    }

    pub fn changed_impurity1(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "impurity1")
    }

    pub fn eval_impurity1(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("impurity1") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "impurity1".to_string(), attempted_path: "impurity1".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.impurity1())
                }}

    pub fn impurity2(&self) -> i64 {
        self.changed_impurity2().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.impurity2)
    }

    pub fn update_impurity2(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.impurity2 = value.try_i64().map(|value| value as i64).unwrap_or(self.impurity2.clone());
        self.root.set(self.entity_key(), "impurity2", value);
        self
    }

    pub fn changed_impurity2(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "impurity2")
    }

    pub fn eval_impurity2(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("impurity2") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "impurity2".to_string(), attempted_path: "impurity2".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.impurity2())
                }}

    pub fn impurity3(&self) -> i64 {
        self.changed_impurity3().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.impurity3)
    }

    pub fn update_impurity3(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.impurity3 = value.try_i64().map(|value| value as i64).unwrap_or(self.impurity3.clone());
        self.root.set(self.entity_key(), "impurity3", value);
        self
    }

    pub fn changed_impurity3(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "impurity3")
    }

    pub fn eval_impurity3(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("impurity3") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "impurity3".to_string(), attempted_path: "impurity3".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.impurity3())
                }}

    pub fn impurity4(&self) -> i64 {
        self.changed_impurity4().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.impurity4)
    }

    pub fn update_impurity4(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.impurity4 = value.try_i64().map(|value| value as i64).unwrap_or(self.impurity4.clone());
        self.root.set(self.entity_key(), "impurity4", value);
        self
    }

    pub fn changed_impurity4(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "impurity4")
    }

    pub fn eval_impurity4(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("impurity4") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "impurity4".to_string(), attempted_path: "impurity4".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.impurity4())
                }}

    pub fn impurity5(&self) -> i64 {
        self.changed_impurity5().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.impurity5)
    }

    pub fn update_impurity5(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.impurity5 = value.try_i64().map(|value| value as i64).unwrap_or(self.impurity5.clone());
        self.root.set(self.entity_key(), "impurity5", value);
        self
    }

    pub fn changed_impurity5(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "impurity5")
    }

    pub fn eval_impurity5(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("impurity5") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "impurity5".to_string(), attempted_path: "impurity5".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.impurity5())
                }}

    pub fn impurity6(&self) -> i64 {
        self.changed_impurity6().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.impurity6)
    }

    pub fn update_impurity6(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.impurity6 = value.try_i64().map(|value| value as i64).unwrap_or(self.impurity6.clone());
        self.root.set(self.entity_key(), "impurity6", value);
        self
    }

    pub fn changed_impurity6(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "impurity6")
    }

    pub fn eval_impurity6(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("impurity6") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "impurity6".to_string(), attempted_path: "impurity6".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.impurity6())
                }}

    pub fn impurity7(&self) -> i64 {
        self.changed_impurity7().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.impurity7)
    }

    pub fn update_impurity7(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.impurity7 = value.try_i64().map(|value| value as i64).unwrap_or(self.impurity7.clone());
        self.root.set(self.entity_key(), "impurity7", value);
        self
    }

    pub fn changed_impurity7(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "impurity7")
    }

    pub fn eval_impurity7(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("impurity7") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "impurity7".to_string(), attempted_path: "impurity7".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.impurity7())
                }}

    pub fn impurity8(&self) -> i64 {
        self.changed_impurity8().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.impurity8)
    }

    pub fn update_impurity8(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.impurity8 = value.try_i64().map(|value| value as i64).unwrap_or(self.impurity8.clone());
        self.root.set(self.entity_key(), "impurity8", value);
        self
    }

    pub fn changed_impurity8(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "impurity8")
    }

    pub fn eval_impurity8(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("impurity8") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "impurity8".to_string(), attempted_path: "impurity8".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.impurity8())
                }}

    pub fn create_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_create_time().and_then(|value| value.try_timestamp()).unwrap_or(self.create_time)
    }

    pub fn update_create_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.create_time = value.try_timestamp().unwrap_or(self.create_time.clone());
        self.root.set(self.entity_key(), "create_time", value);
        self
    }

    pub fn changed_create_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "create_time")
    }

    pub fn eval_create_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("create_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "create_time".to_string(), attempted_path: "create_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.create_time())
                }}

    pub fn version(&self) -> i64 {
        self.changed_version().and_then(|value| value.try_i64()).unwrap_or(self.version)
    }

    pub fn update_version(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version = value.try_i64().unwrap_or(self.version.clone());
        self.root.set(self.entity_key(), "version", value);
        self
    }

    pub fn changed_version(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "version")
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
        self.root.set(self.entity_key(), "device_system_id", value);
        self
    }

    pub fn changed_device_system_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "device_system_id")
    }

    pub fn eval_device_system_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("device_system_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "device_system_id".to_string(), attempted_path: "device_system_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.device_system_id())
                }}

    pub fn system_status_id(&self) -> u64 {
        self.changed_system_status_id().and_then(|value| value.try_u64()).unwrap_or(self.system_status_id)
    }

    pub(crate) fn update_system_status_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.system_status_id = value.try_u64().unwrap_or(self.system_status_id.clone());
        self.root.set(self.entity_key(), "system_status_id", value);
        self
    }

    pub fn changed_system_status_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "system_status_id")
    }

    pub fn eval_system_status_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("system_status_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "system_status_id".to_string(), attempted_path: "system_status_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.system_status_id())
                }}
    pub fn update_system_status_to_online(&mut self) -> &mut Self {
        self.update_system_status_id(1001_u64)
    }

    pub fn system_status_is_online(&self) -> bool {
        self.system_status_id() == 1001_u64
    }
    pub fn update_system_status_to_offline(&mut self) -> &mut Self {
        self.update_system_status_id(1002_u64)
    }

    pub fn system_status_is_offline(&self) -> bool {
        self.system_status_id() == 1002_u64
    }
    pub fn update_system_status_to_sampling(&mut self) -> &mut Self {
        self.update_system_status_id(1003_u64)
    }

    pub fn system_status_is_sampling(&self) -> bool {
        self.system_status_id() == 1003_u64
    }
    pub fn update_system_status_to_calibrating(&mut self) -> &mut Self {
        self.update_system_status_id(1004_u64)
    }

    pub fn system_status_is_calibrating(&self) -> bool {
        self.system_status_id() == 1004_u64
    }
    pub fn device_system(&self) -> Option<&crate::DeviceSystem> {
        self.device_system.as_ref()
    }

    pub fn eval_device_system(&self) -> teaql_core::eval::EvalResult<&crate::DeviceSystem> {
        if !self.is_loaded("device_system") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "device_system".to_string(), attempted_path: "device_system".to_string() }
        } else {
            match &self.device_system {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn system_status(&self) -> Option<&crate::SystemStatus> {
        self.system_status.as_ref()
    }

    pub fn eval_system_status(&self) -> teaql_core::eval::EvalResult<&crate::SystemStatus> {
        if !self.is_loaded("system_status") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "system_status".to_string(), attempted_path: "system_status".to_string() }
        } else {
            match &self.system_status {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn mark_as_delete(&mut self) -> &mut Self {
        self.root.mark_as_delete(self.entity_key());
        self
    }

    pub fn set_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.root.set_comment(comment);
        self
    }
}

