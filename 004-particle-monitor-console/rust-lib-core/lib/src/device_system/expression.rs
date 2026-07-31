#[derive(Clone)]
pub struct DeviceSystemExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::DeviceSystem>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DeviceSystemExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::DeviceSystem>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::DeviceSystem> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::DeviceSystem> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::DeviceSystem {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_serial_number(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("serial_number", |entity| entity.eval_serial_number());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_create_time(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("create_time", |entity| entity.eval_create_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_update_time(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("update_time", |entity| entity.eval_update_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_system_status_list(self) -> crate::SystemStatusListExpression<'a> {
        let next = self.result.and_then("system_status_list", |entity| entity.eval_system_status_list());
        crate::SystemStatusListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_device_setting_list(self) -> crate::DeviceSettingListExpression<'a> {
        let next = self.result.and_then("device_setting_list", |entity| entity.eval_device_setting_list());
        crate::DeviceSettingListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_sample_record_list(self) -> crate::SampleRecordListExpression<'a> {
        let next = self.result.and_then("sample_record_list", |entity| entity.eval_sample_record_list());
        crate::SampleRecordListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct DeviceSystemListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DeviceSystem>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DeviceSystemListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DeviceSystem>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::DeviceSystem>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::DeviceSystem>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::DeviceSystem> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::DeviceSystemExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DeviceSystemExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::DeviceSystemExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DeviceSystemExpression::new(next, self.root_desc.clone())
    }
}