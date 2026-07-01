#[derive(Clone)]
pub struct ProcessExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Process>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ProcessExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Process>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Process> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Process> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Process {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_pid(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("pid", |entity| entity.eval_pid());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_state(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("state", |entity| entity.eval_state());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_ppid(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("ppid", |entity| entity.eval_ppid());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_cmdline(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("cmdline", |entity| entity.eval_cmdline());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_thread_count(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("thread_count", |entity| entity.eval_thread_count());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_memory_rss_kb(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("memory_rss_kb", |entity| entity.eval_memory_rss_kb());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_memory_vms_kb(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("memory_vms_kb", |entity| entity.eval_memory_vms_kb());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_cpu_user_ticks(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("cpu_user_ticks", |entity| entity.eval_cpu_user_ticks());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_cpu_system_ticks(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("cpu_system_ticks", |entity| entity.eval_cpu_system_ticks());
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
    pub fn get_system_info_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("system_info_id", |entity| entity.eval_system_info_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_system_info(self) -> crate::SystemInfoExpression<'a> {
        let next = self.result.and_then("system_info", |entity| entity.eval_system_info());
        crate::SystemInfoExpression::new(next, self.root_desc.clone())
    }
    pub fn get_thread_list(self) -> crate::ThreadListExpression<'a> {
        let next = self.result.and_then("thread_list", |entity| entity.eval_thread_list());
        crate::ThreadListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct ProcessListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Process>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ProcessListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Process>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Process>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Process>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Process> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::ProcessExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ProcessExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::ProcessExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ProcessExpression::new(next, self.root_desc.clone())
    }
}