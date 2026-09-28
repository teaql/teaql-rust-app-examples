#[derive(Clone)]
pub struct DeviceSettingExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::DeviceSetting>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DeviceSettingExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::DeviceSetting>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::DeviceSetting> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::DeviceSetting> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::DeviceSetting {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_calibration_point(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("calibration_point", |entity| entity.eval_calibration_point());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_data_keep_days(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("data_keep_days", |entity| entity.eval_data_keep_days());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_sampling_frequency(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("sampling_frequency", |entity| entity.eval_sampling_frequency());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_password_enabled(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("password_enabled", |entity| entity.eval_password_enabled());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_password_hash(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("password_hash", |entity| entity.eval_password_hash());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_super_password_hash(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("super_password_hash", |entity| entity.eval_super_password_hash());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_create_time(self) -> crate::ValueExpression<'a, teaql_core::time::Timestamp> {
        let next = self.result.and_then("create_time", |entity| entity.eval_create_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_update_time(self) -> crate::ValueExpression<'a, teaql_core::time::Timestamp> {
        let next = self.result.and_then("update_time", |entity| entity.eval_update_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_device_system_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("device_system_id", |entity| entity.eval_device_system_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_device_system(self) -> crate::DeviceSystemExpression<'a> {
        let next = self.result.and_then("device_system", |entity| entity.eval_device_system());
        crate::DeviceSystemExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct DeviceSettingListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DeviceSetting>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DeviceSettingListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DeviceSetting>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::DeviceSetting>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::DeviceSetting>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::DeviceSetting> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::DeviceSettingExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DeviceSettingExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::DeviceSettingExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DeviceSettingExpression::new(next, self.root_desc.clone())
    }
}