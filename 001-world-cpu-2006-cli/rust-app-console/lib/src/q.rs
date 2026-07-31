use teaql_core::Expr;

use crate::*;

pub struct PurposedQuery<T> {
    pub inner: T,
    pub purpose: String,
}

impl<T> PurposedQuery<T> {
    pub fn new(inner: T, purpose: impl Into<String>) -> Self {
        Self { inner, purpose: purpose.into() }
    }
}

pub struct Q;

impl Q {
    pub fn match_stages() -> MatchStageRequest {
        MatchStageRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn match_stages_minimal() -> MatchStageRequest {
        MatchStageRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn match_stages_with_children() -> MatchStageRequest {
        MatchStageRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn match_statuses() -> MatchStatusRequest {
        MatchStatusRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn match_statuses_minimal() -> MatchStatusRequest {
        MatchStatusRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn match_statuses_with_children() -> MatchStatusRequest {
        MatchStatusRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn goal_categories() -> GoalCategoryRequest {
        GoalCategoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn goal_categories_minimal() -> GoalCategoryRequest {
        GoalCategoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn goal_categories_with_children() -> GoalCategoryRequest {
        GoalCategoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn card_categories() -> CardCategoryRequest {
        CardCategoryRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn card_categories_minimal() -> CardCategoryRequest {
        CardCategoryRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn card_categories_with_children() -> CardCategoryRequest {
        CardCategoryRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn confederations() -> ConfederationRequest {
        ConfederationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn confederations_minimal() -> ConfederationRequest {
        ConfederationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn confederations_with_children() -> ConfederationRequest {
        ConfederationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tournaments() -> TournamentRequest {
        TournamentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tournaments_minimal() -> TournamentRequest {
        TournamentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tournaments_with_children() -> TournamentRequest {
        TournamentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tournament_teams() -> TournamentTeamRequest {
        TournamentTeamRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tournament_teams_minimal() -> TournamentTeamRequest {
        TournamentTeamRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tournament_teams_with_children() -> TournamentTeamRequest {
        TournamentTeamRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn match_groups() -> MatchGroupRequest {
        MatchGroupRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn match_groups_minimal() -> MatchGroupRequest {
        MatchGroupRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn match_groups_with_children() -> MatchGroupRequest {
        MatchGroupRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tournament_matches() -> TournamentMatchRequest {
        TournamentMatchRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tournament_matches_minimal() -> TournamentMatchRequest {
        TournamentMatchRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tournament_matches_with_children() -> TournamentMatchRequest {
        TournamentMatchRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn match_goals() -> MatchGoalRequest {
        MatchGoalRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn match_goals_minimal() -> MatchGoalRequest {
        MatchGoalRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn match_goals_with_children() -> MatchGoalRequest {
        MatchGoalRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn match_cards() -> MatchCardRequest {
        MatchCardRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn match_cards_minimal() -> MatchCardRequest {
        MatchCardRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn match_cards_with_children() -> MatchCardRequest {
        MatchCardRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn group_standings() -> GroupStandingRequest {
        GroupStandingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn group_standings_minimal() -> GroupStandingRequest {
        GroupStandingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn group_standings_with_children() -> GroupStandingRequest {
        GroupStandingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}