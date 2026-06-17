#[derive(Clone)]
pub struct MatchGoalExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::MatchGoal>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MatchGoalExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::MatchGoal>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::MatchGoal> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::MatchGoal> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::MatchGoal {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_player_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("player_name", |entity| entity.eval_player_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_minute_scored(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("minute_scored", |entity| entity.eval_minute_scored());
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
    pub fn get_tournament_match_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("tournament_match_id", |entity| entity.eval_tournament_match_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament_team_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("tournament_team_id", |entity| entity.eval_tournament_team_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_goal_category_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("goal_category_id", |entity| entity.eval_goal_category_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("tournament_id", |entity| entity.eval_tournament_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_tournament_match(self) -> crate::TournamentMatchExpression<'a> {
        let next = self.result.and_then("tournament_match", |entity| entity.eval_tournament_match());
        crate::TournamentMatchExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament_team(self) -> crate::TournamentTeamExpression<'a> {
        let next = self.result.and_then("tournament_team", |entity| entity.eval_tournament_team());
        crate::TournamentTeamExpression::new(next, self.root_desc.clone())
    }

    pub fn get_goal_category(self) -> crate::GoalCategoryExpression<'a> {
        let next = self.result.and_then("goal_category", |entity| entity.eval_goal_category());
        crate::GoalCategoryExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament(self) -> crate::TournamentExpression<'a> {
        let next = self.result.and_then("tournament", |entity| entity.eval_tournament());
        crate::TournamentExpression::new(next, self.root_desc.clone())
    }
    pub fn goal_category_is_normal(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("goal_category_id", |entity| {
            if !entity.is_loaded("goal_category_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "goal_category_id".to_string(), attempted_path: "goal_category_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.goal_category_is_normal())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn goal_category_is_penalty(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("goal_category_id", |entity| {
            if !entity.is_loaded("goal_category_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "goal_category_id".to_string(), attempted_path: "goal_category_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.goal_category_is_penalty())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn goal_category_is_own_goal(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("goal_category_id", |entity| {
            if !entity.is_loaded("goal_category_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "goal_category_id".to_string(), attempted_path: "goal_category_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.goal_category_is_own_goal())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn goal_category_is_free_kick(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("goal_category_id", |entity| {
            if !entity.is_loaded("goal_category_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "goal_category_id".to_string(), attempted_path: "goal_category_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.goal_category_is_free_kick())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct MatchGoalListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MatchGoal>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MatchGoalListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MatchGoal>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::MatchGoal>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::MatchGoal>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::MatchGoal> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::MatchGoalExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MatchGoalExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::MatchGoalExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MatchGoalExpression::new(next, self.root_desc.clone())
    }
}