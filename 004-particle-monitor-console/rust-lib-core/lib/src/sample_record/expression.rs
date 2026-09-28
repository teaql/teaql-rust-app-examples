#[derive(Clone)]
pub struct SampleRecordExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::SampleRecord>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> SampleRecordExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::SampleRecord>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::SampleRecord> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::SampleRecord> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::SampleRecord {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_sample_time(self) -> crate::ValueExpression<'a, teaql_core::time::Timestamp> {
        let next = self.result.and_then("sample_time", |entity| entity.eval_sample_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_gas(self) -> crate::ValueExpression<'a, rust_decimal::Decimal> {
        let next = self.result.and_then("gas", |entity| entity.eval_gas());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_lref(self) -> crate::ValueExpression<'a, rust_decimal::Decimal> {
        let next = self.result.and_then("lref", |entity| entity.eval_lref());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_impurity1(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("impurity1", |entity| entity.eval_impurity1());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_impurity2(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("impurity2", |entity| entity.eval_impurity2());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_impurity3(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("impurity3", |entity| entity.eval_impurity3());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_impurity4(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("impurity4", |entity| entity.eval_impurity4());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_impurity5(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("impurity5", |entity| entity.eval_impurity5());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_impurity6(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("impurity6", |entity| entity.eval_impurity6());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_impurity7(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("impurity7", |entity| entity.eval_impurity7());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_impurity8(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("impurity8", |entity| entity.eval_impurity8());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_create_time(self) -> crate::ValueExpression<'a, teaql_core::time::Timestamp> {
        let next = self.result.and_then("create_time", |entity| entity.eval_create_time());
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

    pub fn get_system_status_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("system_status_id", |entity| entity.eval_system_status_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_device_system(self) -> crate::DeviceSystemExpression<'a> {
        let next = self.result.and_then("device_system", |entity| entity.eval_device_system());
        crate::DeviceSystemExpression::new(next, self.root_desc.clone())
    }

    pub fn get_system_status(self) -> crate::SystemStatusExpression<'a> {
        let next = self.result.and_then("system_status", |entity| entity.eval_system_status());
        crate::SystemStatusExpression::new(next, self.root_desc.clone())
    }
    pub fn system_status_is_online(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("system_status_id", |entity| {
            if !entity.is_loaded("system_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "system_status_id".to_string(), attempted_path: "system_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.system_status_is_online())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn system_status_is_offline(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("system_status_id", |entity| {
            if !entity.is_loaded("system_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "system_status_id".to_string(), attempted_path: "system_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.system_status_is_offline())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn system_status_is_sampling(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("system_status_id", |entity| {
            if !entity.is_loaded("system_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "system_status_id".to_string(), attempted_path: "system_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.system_status_is_sampling())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn system_status_is_calibrating(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("system_status_id", |entity| {
            if !entity.is_loaded("system_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "system_status_id".to_string(), attempted_path: "system_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.system_status_is_calibrating())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct SampleRecordListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::SampleRecord>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> SampleRecordListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::SampleRecord>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::SampleRecord>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::SampleRecord>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::SampleRecord> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::SampleRecordExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::SampleRecordExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::SampleRecordExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::SampleRecordExpression::new(next, self.root_desc.clone())
    }
}