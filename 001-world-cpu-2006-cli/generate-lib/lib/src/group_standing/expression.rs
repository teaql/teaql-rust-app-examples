#[derive(Clone)]
pub struct GroupStandingExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::GroupStanding>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> GroupStandingExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::GroupStanding>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::GroupStanding> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::GroupStanding> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::GroupStanding {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_played(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("played", |entity| entity.eval_played());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_won(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("won", |entity| entity.eval_won());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_drawn(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("drawn", |entity| entity.eval_drawn());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_lost(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("lost", |entity| entity.eval_lost());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_goals_for(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("goals_for", |entity| entity.eval_goals_for());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_goals_against(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("goals_against", |entity| entity.eval_goals_against());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_goal_difference(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("goal_difference", |entity| entity.eval_goal_difference());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_points(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("points", |entity| entity.eval_points());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_standing_rank(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("standing_rank", |entity| entity.eval_standing_rank());
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
    pub fn get_tournament_team_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("tournament_team_id", |entity| entity.eval_tournament_team_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_group_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("match_group_id", |entity| entity.eval_match_group_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("tournament_id", |entity| entity.eval_tournament_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_tournament_team(self) -> crate::TournamentTeamExpression<'a> {
        let next = self.result.and_then("tournament_team", |entity| entity.eval_tournament_team());
        crate::TournamentTeamExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_group(self) -> crate::MatchGroupExpression<'a> {
        let next = self.result.and_then("match_group", |entity| entity.eval_match_group());
        crate::MatchGroupExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament(self) -> crate::TournamentExpression<'a> {
        let next = self.result.and_then("tournament", |entity| entity.eval_tournament());
        crate::TournamentExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct GroupStandingListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::GroupStanding>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> GroupStandingListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::GroupStanding>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::GroupStanding>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::GroupStanding>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::GroupStanding> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::GroupStandingExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::GroupStandingExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::GroupStandingExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::GroupStandingExpression::new(next, self.root_desc.clone())
    }
}