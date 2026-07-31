#[derive(Clone)]
pub struct ThreadExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Thread>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ThreadExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Thread>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Thread> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Thread> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Thread {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tid(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("tid", |entity| entity.eval_tid());
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

    pub fn get_process_pid(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("process_pid", |entity| entity.eval_process_pid());
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
    pub fn get_process_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("process_id", |entity| entity.eval_process_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_process(self) -> crate::ProcessExpression<'a> {
        let next = self.result.and_then("process", |entity| entity.eval_process());
        crate::ProcessExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct ThreadListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Thread>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ThreadListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Thread>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Thread>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Thread>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Thread> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::ThreadExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ThreadExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::ThreadExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ThreadExpression::new(next, self.root_desc.clone())
    }
}