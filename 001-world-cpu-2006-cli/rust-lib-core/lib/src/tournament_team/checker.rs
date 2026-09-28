
use teaql_runtime::{CheckObjectStatus, CheckResults, ObjectLocation, TypedChecker, UserContext};

pub trait TournamentTeamCheckerLogic: Send + Sync {
    fn check_and_fix_tournament_team(
        &self,
        _context: &UserContext,
        _entity: &mut crate::TournamentTeam,
        _status: CheckObjectStatus,
        _location: &ObjectLocation,
        _results: &mut CheckResults,
    ) {
    }

    fn required(
        &self,
        value: bool,
        field: &str,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if !value {
            results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
        }
    }

    fn required_option<V>(
        &self,
        value: Option<&V>,
        field: &str,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if value.is_none() {
            results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
        }
    }

    fn required_text(
        &self,
        value: &str,
        field: &str,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if value.trim().is_empty() {
            results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
        }
    }

    fn min_string_length(
        &self,
        value: &str,
        field: &str,
        min_len: usize,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if value.chars().count() < min_len {
            results.push(teaql_runtime::CheckResult::min_str(
                location.clone().member(field),
                min_len as u64,
                value.to_owned(),
            ));
        }
    }

    fn max_string_length(
        &self,
        value: &str,
        field: &str,
        max_len: usize,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if value.chars().count() > max_len {
            results.push(teaql_runtime::CheckResult::max_str(
                location.clone().member(field),
                max_len as u64,
                value.to_owned(),
            ));
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NoopTournamentTeamChecker;

impl TournamentTeamCheckerLogic for NoopTournamentTeamChecker {}

#[derive(Clone, Debug)]
pub struct TournamentTeamChecker<L = NoopTournamentTeamChecker> {
    logic: L,
}

impl Default for TournamentTeamChecker<NoopTournamentTeamChecker> {
    fn default() -> Self {
        Self {
            logic: NoopTournamentTeamChecker,
        }
    }
}

impl<L> TournamentTeamChecker<L>
where
    L: TournamentTeamCheckerLogic,
{
    pub fn new(logic: L) -> Self {
        Self { logic }
    }
}

impl<L> TypedChecker<crate::TournamentTeam> for TournamentTeamChecker<L>
where
    L: TournamentTeamCheckerLogic,
{
    fn check_and_fix_typed(
        &self,
        context: &UserContext,
        entity: &mut crate::TournamentTeam,
        status: CheckObjectStatus,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if status.is_create() {
            entity.update_create_time(context.fix_time());
            context.record_fix_evidence(teaql_runtime::FixEvidence::new("TournamentTeam", "create_time", teaql_runtime::FixEvidenceSource::Clock, "graphClock"));
        }

        if status.is_create() {
            entity.update_update_time(context.fix_time());
            context.record_fix_evidence(teaql_runtime::FixEvidence::new("TournamentTeam", "update_time", teaql_runtime::FixEvidenceSource::Clock, "graphClock"));
        }
        if status.is_create() || status.is_update() {
            entity.update_update_time(context.fix_time());
            context.record_fix_evidence(teaql_runtime::FixEvidence::new("TournamentTeam", "update_time", teaql_runtime::FixEvidenceSource::Clock, "graphClock"));
        }


        if status.is_update() && !entity.is_loaded("id") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("id"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("team_name") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("team_name"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("team_code") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("team_code"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("emoji_flag") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("emoji_flag"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("fifa_ranking") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("fifa_ranking"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("manager_name") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("manager_name"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("confederation_id") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("confederation"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("group_letter") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("group_letter"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("tournament_id") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("tournament"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("create_time") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("create_time"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("update_time") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("update_time"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("version") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("version"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        self.logic
            .check_and_fix_tournament_team(context, entity, status, location, results);
    }
}