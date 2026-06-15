#[derive(Clone)]
pub struct TournamentMatchExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::TournamentMatch>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TournamentMatchExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::TournamentMatch>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::TournamentMatch> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::TournamentMatch> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::TournamentMatch {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_number(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("match_number", |entity| entity.eval_match_number());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("match_date", |entity| entity.eval_match_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_venue_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("venue_name", |entity| entity.eval_venue_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_venue_city(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("venue_city", |entity| entity.eval_venue_city());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_venue_country(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("venue_country", |entity| entity.eval_venue_country());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_home_score(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("home_score", |entity| entity.eval_home_score());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_away_score(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("away_score", |entity| entity.eval_away_score());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_time_home(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("extra_time_home", |entity| entity.eval_extra_time_home());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_time_away(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("extra_time_away", |entity| entity.eval_extra_time_away());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_penalty_home(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("penalty_home", |entity| entity.eval_penalty_home());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_penalty_away(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("penalty_away", |entity| entity.eval_penalty_away());
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
    pub fn get_home_team_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("home_team_id", |entity| entity.eval_home_team_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_away_team_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("away_team_id", |entity| entity.eval_away_team_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_stage_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("match_stage_id", |entity| entity.eval_match_stage_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_group_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("match_group_id", |entity| entity.eval_match_group_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_status_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("match_status_id", |entity| entity.eval_match_status_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("tournament_id", |entity| entity.eval_tournament_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_home_team(self) -> crate::TournamentTeamExpression<'a> {
        let next = self.result.and_then("home_team", |entity| entity.eval_home_team());
        crate::TournamentTeamExpression::new(next, self.root_desc.clone())
    }

    pub fn get_away_team(self) -> crate::TournamentTeamExpression<'a> {
        let next = self.result.and_then("away_team", |entity| entity.eval_away_team());
        crate::TournamentTeamExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_stage(self) -> crate::MatchStageExpression<'a> {
        let next = self.result.and_then("match_stage", |entity| entity.eval_match_stage());
        crate::MatchStageExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_group(self) -> crate::MatchGroupExpression<'a> {
        let next = self.result.and_then("match_group", |entity| entity.eval_match_group());
        crate::MatchGroupExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_status(self) -> crate::MatchStatusExpression<'a> {
        let next = self.result.and_then("match_status", |entity| entity.eval_match_status());
        crate::MatchStatusExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament(self) -> crate::TournamentExpression<'a> {
        let next = self.result.and_then("tournament", |entity| entity.eval_tournament());
        crate::TournamentExpression::new(next, self.root_desc.clone())
    }
    pub fn match_stage_is_group(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("match_stage_id", |entity| {
            if !entity.is_loaded("match_stage_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_stage_id".to_string(), attempted_path: "match_stage_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.match_stage_is_group())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn match_stage_is_round_of32(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("match_stage_id", |entity| {
            if !entity.is_loaded("match_stage_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_stage_id".to_string(), attempted_path: "match_stage_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.match_stage_is_round_of32())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn match_stage_is_round_of16(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("match_stage_id", |entity| {
            if !entity.is_loaded("match_stage_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_stage_id".to_string(), attempted_path: "match_stage_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.match_stage_is_round_of16())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn match_stage_is_quarter_final(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("match_stage_id", |entity| {
            if !entity.is_loaded("match_stage_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_stage_id".to_string(), attempted_path: "match_stage_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.match_stage_is_quarter_final())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn match_stage_is_semi_final(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("match_stage_id", |entity| {
            if !entity.is_loaded("match_stage_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_stage_id".to_string(), attempted_path: "match_stage_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.match_stage_is_semi_final())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn match_stage_is_third_place(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("match_stage_id", |entity| {
            if !entity.is_loaded("match_stage_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_stage_id".to_string(), attempted_path: "match_stage_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.match_stage_is_third_place())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn match_stage_is_final(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("match_stage_id", |entity| {
            if !entity.is_loaded("match_stage_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_stage_id".to_string(), attempted_path: "match_stage_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.match_stage_is_final())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn match_status_is_scheduled(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("match_status_id", |entity| {
            if !entity.is_loaded("match_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_status_id".to_string(), attempted_path: "match_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.match_status_is_scheduled())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn match_status_is_live(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("match_status_id", |entity| {
            if !entity.is_loaded("match_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_status_id".to_string(), attempted_path: "match_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.match_status_is_live())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn match_status_is_finished(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("match_status_id", |entity| {
            if !entity.is_loaded("match_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_status_id".to_string(), attempted_path: "match_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.match_status_is_finished())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn match_status_is_postponed(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("match_status_id", |entity| {
            if !entity.is_loaded("match_status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_status_id".to_string(), attempted_path: "match_status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.match_status_is_postponed())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_match_goal_list(self) -> crate::MatchGoalListExpression<'a> {
        let next = self.result.and_then("match_goal_list", |entity| entity.eval_match_goal_list());
        crate::MatchGoalListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_match_card_list(self) -> crate::MatchCardListExpression<'a> {
        let next = self.result.and_then("match_card_list", |entity| entity.eval_match_card_list());
        crate::MatchCardListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct TournamentMatchListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TournamentMatch>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TournamentMatchListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TournamentMatch>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::TournamentMatch>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::TournamentMatch>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::TournamentMatch> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::TournamentMatchExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TournamentMatchExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::TournamentMatchExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TournamentMatchExpression::new(next, self.root_desc.clone())
    }
}