#[derive(Clone)]
pub struct MatchGroupExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::MatchGroup>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MatchGroupExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::MatchGroup>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::MatchGroup> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::MatchGroup> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::MatchGroup {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_group_letter(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("group_letter", |entity| entity.eval_group_letter());
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
    pub fn get_tournament_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("tournament_id", |entity| entity.eval_tournament_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_tournament(self) -> crate::TournamentExpression<'a> {
        let next = self.result.and_then("tournament", |entity| entity.eval_tournament());
        crate::TournamentExpression::new(next, self.root_desc.clone())
    }
    pub fn get_tournament_match_list(self) -> crate::TournamentMatchListExpression<'a> {
        let next = self.result.and_then("tournament_match_list", |entity| entity.eval_tournament_match_list());
        crate::TournamentMatchListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_group_standing_list(self) -> crate::GroupStandingListExpression<'a> {
        let next = self.result.and_then("group_standing_list", |entity| entity.eval_group_standing_list());
        crate::GroupStandingListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct MatchGroupListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MatchGroup>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MatchGroupListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MatchGroup>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::MatchGroup>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::MatchGroup>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::MatchGroup> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::MatchGroupExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MatchGroupExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::MatchGroupExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MatchGroupExpression::new(next, self.root_desc.clone())
    }
}