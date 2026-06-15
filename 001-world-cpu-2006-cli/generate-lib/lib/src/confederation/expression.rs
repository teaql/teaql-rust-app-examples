#[derive(Clone)]
pub struct ConfederationExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Confederation>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ConfederationExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Confederation>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Confederation> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Confederation> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Confederation {
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

    pub fn get_code(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("code", |entity| entity.eval_code());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_tournament_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("tournament_id", |entity| entity.eval_tournament_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_tournament(self) -> crate::TournamentExpression<'a> {
        let next = self.result.and_then("tournament", |entity| entity.eval_tournament());
        crate::TournamentExpression::new(next, self.root_desc.clone())
    }
    pub fn get_tournament_team_list(self) -> crate::TournamentTeamListExpression<'a> {
        let next = self.result.and_then("tournament_team_list", |entity| entity.eval_tournament_team_list());
        crate::TournamentTeamListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct ConfederationListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Confederation>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ConfederationListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Confederation>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Confederation>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Confederation>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Confederation> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::ConfederationExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ConfederationExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::ConfederationExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ConfederationExpression::new(next, self.root_desc.clone())
    }
}