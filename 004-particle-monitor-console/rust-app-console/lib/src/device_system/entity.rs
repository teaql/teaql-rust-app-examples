
// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/device_system
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
#[teaql(reverse_relation(name = "system_status_list", target = "SystemStatus", local_key = "id", foreign_key = "device_system_id", many))]
#[teaql(reverse_relation(name = "device_setting_list", target = "DeviceSetting", local_key = "id", foreign_key = "device_system_id", many))]
#[teaql(reverse_relation(name = "sample_record_list", target = "SampleRecord", local_key = "id", foreign_key = "device_system_id", many))]
#[teaql(entity = "DeviceSystem", table = "device_system_data", data_service = "sqlite")]
pub struct DeviceSystem {
#[teaql(id)]
    id: u64,

// @source model.xml:13
#[teaql(max_length = 100)]
    name: String,

// @source model.xml:13
#[teaql(max_length = 100)]
    serial_number: String,

// @source model.xml:13
    create_time: teaql_core::time::Timestamp,

// @source model.xml:13
    update_time: teaql_core::time::Timestamp,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl DeviceSystem {
    pub const ENTITY_NAME: &'static str = "Device System";

    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRuntimeState) -> Self {
        Self {
            id: 0_u64,
            name: String::new(),
            serial_number: String::new(),
            create_time: teaql_core::time::Timestamp::now(),
            update_time: teaql_core::time::Timestamp::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            __teaql_runtime_state: root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn attach_runtime_state_recursive(&mut self, root: teaql_runtime::EntityRuntimeState) {
        root.adopt_mutations_from(self.__teaql_runtime_state());
        self.__teaql_replace_runtime_state(root.clone());
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


    pub fn name(&self) -> String {
        self.changed_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.name.clone())
    }

    pub fn update_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.name.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "name", value);
        self
    }

    pub fn changed_name(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "name")
    }

    pub fn eval_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "name".to_string(), attempted_path: "name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.name())
                }}


    pub fn serial_number(&self) -> String {
        self.changed_serial_number().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.serial_number.clone())
    }

    pub fn update_serial_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.serial_number = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.serial_number.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "serial_number", value);
        self
    }

    pub fn changed_serial_number(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "serial_number")
    }

    pub fn eval_serial_number(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("serial_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "serial_number".to_string(), attempted_path: "serial_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.serial_number())
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

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn system_status_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::SystemStatus>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "system_status_list",
        )
    }

    pub fn eval_system_status_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::SystemStatus>> {
        let relation = self.system_status_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "system_status_list".to_string(), attempted_path: "system_status_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn device_setting_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::DeviceSetting>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "device_setting_list",
        )
    }

    pub fn eval_device_setting_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::DeviceSetting>> {
        let relation = self.device_setting_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "device_setting_list".to_string(), attempted_path: "device_setting_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn sample_record_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::SampleRecord>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "sample_record_list",
        )
    }

    pub fn eval_sample_record_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::SampleRecord>> {
        let relation = self.sample_record_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "sample_record_list".to_string(), attempted_path: "sample_record_list".to_string() },
        }
    }

}

