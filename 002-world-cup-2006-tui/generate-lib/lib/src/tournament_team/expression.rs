#[derive(Clone)]
pub struct TournamentTeamExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::TournamentTeam>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TournamentTeamExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::TournamentTeam>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::TournamentTeam> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::TournamentTeam> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::TournamentTeam {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_team_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("team_name", |entity| entity.eval_team_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_team_code(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("team_code", |entity| entity.eval_team_code());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_emoji_flag(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("emoji_flag", |entity| entity.eval_emoji_flag());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_fifa_ranking(self) -> crate::ValueExpression<'a, i32> {
        let next = self.result.and_then("fifa_ranking", |entity| entity.eval_fifa_ranking());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_manager_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("manager_name", |entity| entity.eval_manager_name());
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
    pub fn get_confederation_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("confederation_id", |entity| entity.eval_confederation_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("tournament_id", |entity| entity.eval_tournament_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_confederation(self) -> crate::ConfederationExpression<'a> {
        let next = self.result.and_then("confederation", |entity| entity.eval_confederation());
        crate::ConfederationExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament(self) -> crate::TournamentExpression<'a> {
        let next = self.result.and_then("tournament", |entity| entity.eval_tournament());
        crate::TournamentExpression::new(next, self.root_desc.clone())
    }
    pub fn confederation_is_afc(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("confederation_id", |entity| {
            if !entity.is_loaded("confederation_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "confederation_id".to_string(), attempted_path: "confederation_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.confederation_is_afc())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn confederation_is_caf(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("confederation_id", |entity| {
            if !entity.is_loaded("confederation_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "confederation_id".to_string(), attempted_path: "confederation_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.confederation_is_caf())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn confederation_is_concacaf(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("confederation_id", |entity| {
            if !entity.is_loaded("confederation_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "confederation_id".to_string(), attempted_path: "confederation_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.confederation_is_concacaf())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn confederation_is_conmebol(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("confederation_id", |entity| {
            if !entity.is_loaded("confederation_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "confederation_id".to_string(), attempted_path: "confederation_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.confederation_is_conmebol())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn confederation_is_ofc(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("confederation_id", |entity| {
            if !entity.is_loaded("confederation_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "confederation_id".to_string(), attempted_path: "confederation_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.confederation_is_ofc())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn confederation_is_uefa(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("confederation_id", |entity| {
            if !entity.is_loaded("confederation_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "confederation_id".to_string(), attempted_path: "confederation_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.confederation_is_uefa())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_tournament_match_list_as_home_team(self) -> crate::TournamentMatchListExpression<'a> {
        let next = self.result.and_then("tournament_match_list_as_home_team", |entity| entity.eval_tournament_match_list_as_home_team());
        crate::TournamentMatchListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tournament_match_list_as_away_team(self) -> crate::TournamentMatchListExpression<'a> {
        let next = self.result.and_then("tournament_match_list_as_away_team", |entity| entity.eval_tournament_match_list_as_away_team());
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
pub struct TournamentTeamListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TournamentTeam>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TournamentTeamListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TournamentTeam>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::TournamentTeam>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::TournamentTeam>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::TournamentTeam> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::TournamentTeamExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TournamentTeamExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::TournamentTeamExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TournamentTeamExpression::new(next, self.root_desc.clone())
    }
}