#[derive(Clone)]
pub struct TournamentExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Tournament>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TournamentExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Tournament>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Tournament> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Tournament> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Tournament {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("tournament_name", |entity| entity.eval_tournament_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_host_countries(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("host_countries", |entity| entity.eval_host_countries());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_start_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("start_date", |entity| entity.eval_start_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_end_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("end_date", |entity| entity.eval_end_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_total_teams(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("total_teams", |entity| entity.eval_total_teams());
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
    pub fn get_match_stage_list(self) -> crate::MatchStageListExpression<'a> {
        let next = self.result.and_then("match_stage_list", |entity| entity.eval_match_stage_list());
        crate::MatchStageListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_status_list(self) -> crate::MatchStatusListExpression<'a> {
        let next = self.result.and_then("match_status_list", |entity| entity.eval_match_status_list());
        crate::MatchStatusListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_goal_category_list(self) -> crate::GoalCategoryListExpression<'a> {
        let next = self.result.and_then("goal_category_list", |entity| entity.eval_goal_category_list());
        crate::GoalCategoryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_card_category_list(self) -> crate::CardCategoryListExpression<'a> {
        let next = self.result.and_then("card_category_list", |entity| entity.eval_card_category_list());
        crate::CardCategoryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_confederation_list(self) -> crate::ConfederationListExpression<'a> {
        let next = self.result.and_then("confederation_list", |entity| entity.eval_confederation_list());
        crate::ConfederationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament_team_list(self) -> crate::TournamentTeamListExpression<'a> {
        let next = self.result.and_then("tournament_team_list", |entity| entity.eval_tournament_team_list());
        crate::TournamentTeamListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_group_list(self) -> crate::MatchGroupListExpression<'a> {
        let next = self.result.and_then("match_group_list", |entity| entity.eval_match_group_list());
        crate::MatchGroupListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament_match_list(self) -> crate::TournamentMatchListExpression<'a> {
        let next = self.result.and_then("tournament_match_list", |entity| entity.eval_tournament_match_list());
        crate::TournamentMatchListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_goal_list(self) -> crate::MatchGoalListExpression<'a> {
        let next = self.result.and_then("match_goal_list", |entity| entity.eval_match_goal_list());
        crate::MatchGoalListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_card_list(self) -> crate::MatchCardListExpression<'a> {
        let next = self.result.and_then("match_card_list", |entity| entity.eval_match_card_list());
        crate::MatchCardListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_group_standing_list(self) -> crate::GroupStandingListExpression<'a> {
        let next = self.result.and_then("group_standing_list", |entity| entity.eval_group_standing_list());
        crate::GroupStandingListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct TournamentListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Tournament>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TournamentListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Tournament>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Tournament>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Tournament>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Tournament> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::TournamentExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TournamentExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::TournamentExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TournamentExpression::new(next, self.root_desc.clone())
    }
}