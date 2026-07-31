#[derive(Clone)]
pub struct SystemInfoExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::SystemInfo>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> SystemInfoExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::SystemInfo>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::SystemInfo> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::SystemInfo> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::SystemInfo {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_hostname(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("hostname", |entity| entity.eval_hostname());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_cpu_count(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("cpu_count", |entity| entity.eval_cpu_count());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_memory_total_bytes(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("memory_total_bytes", |entity| entity.eval_memory_total_bytes());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_memory_available_bytes(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("memory_available_bytes", |entity| entity.eval_memory_available_bytes());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_load_avg_1(self) -> crate::ValueExpression<'a, rust_decimal::Decimal> {
        let next = self.result.and_then("load_avg_1", |entity| entity.eval_load_avg_1());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_load_avg_5(self) -> crate::ValueExpression<'a, rust_decimal::Decimal> {
        let next = self.result.and_then("load_avg_5", |entity| entity.eval_load_avg_5());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_load_avg_15(self) -> crate::ValueExpression<'a, rust_decimal::Decimal> {
        let next = self.result.and_then("load_avg_15", |entity| entity.eval_load_avg_15());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_uptime_seconds(self) -> crate::ValueExpression<'a, rust_decimal::Decimal> {
        let next = self.result.and_then("uptime_seconds", |entity| entity.eval_uptime_seconds());
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
    pub fn get_process_list(self) -> crate::ProcessListExpression<'a> {
        let next = self.result.and_then("process_list", |entity| entity.eval_process_list());
        crate::ProcessListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct SystemInfoListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::SystemInfo>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> SystemInfoListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::SystemInfo>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::SystemInfo>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::SystemInfo>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::SystemInfo> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::SystemInfoExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::SystemInfoExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::SystemInfoExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::SystemInfoExpression::new(next, self.root_desc.clone())
    }
}