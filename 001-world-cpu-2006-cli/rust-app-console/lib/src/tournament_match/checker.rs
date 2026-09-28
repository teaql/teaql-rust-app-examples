
use teaql_runtime::{CheckObjectStatus, CheckResults, ObjectLocation, TypedChecker, UserContext};

pub trait TournamentMatchCheckerLogic: Send + Sync {
    fn check_and_fix_tournament_match(
        &self,
        _context: &UserContext,
        _entity: &mut crate::TournamentMatch,
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
pub struct NoopTournamentMatchChecker;

impl TournamentMatchCheckerLogic for NoopTournamentMatchChecker {}

#[derive(Clone, Debug)]
pub struct TournamentMatchChecker<L = NoopTournamentMatchChecker> {
    logic: L,
}

impl Default for TournamentMatchChecker<NoopTournamentMatchChecker> {
    fn default() -> Self {
        Self {
            logic: NoopTournamentMatchChecker,
        }
    }
}

impl<L> TournamentMatchChecker<L>
where
    L: TournamentMatchCheckerLogic,
{
    pub fn new(logic: L) -> Self {
        Self { logic }
    }
}

impl<L> TypedChecker<crate::TournamentMatch> for TournamentMatchChecker<L>
where
    L: TournamentMatchCheckerLogic,
{
    fn check_and_fix_typed(
        &self,
        context: &UserContext,
        entity: &mut crate::TournamentMatch,
        status: CheckObjectStatus,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if status.is_create() {
            entity.update_create_time(context.fix_time());
            context.record_fix_evidence(teaql_runtime::FixEvidence::new("TournamentMatch", "create_time", teaql_runtime::FixEvidenceSource::Clock, "graphClock"));
        }

        if status.is_create() {
            entity.update_update_time(context.fix_time());
            context.record_fix_evidence(teaql_runtime::FixEvidence::new("TournamentMatch", "update_time", teaql_runtime::FixEvidenceSource::Clock, "graphClock"));
        }
        if status.is_create() || status.is_update() {
            entity.update_update_time(context.fix_time());
            context.record_fix_evidence(teaql_runtime::FixEvidence::new("TournamentMatch", "update_time", teaql_runtime::FixEvidenceSource::Clock, "graphClock"));
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

        if status.is_update() && !entity.is_loaded("match_number") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("match_number"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("match_date") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("match_date"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("venue_name") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("venue_name"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("venue_city") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("venue_city"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("venue_country") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("venue_country"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("home_score") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("home_score"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("away_score") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("away_score"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("extra_time_home") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("extra_time_home"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("extra_time_away") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("extra_time_away"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("penalty_home") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("penalty_home"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("penalty_away") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("penalty_away"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("home_team_id") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("home_team"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("away_team_id") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("away_team"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("match_stage_id") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("match_stage"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("match_group_id") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("match_group"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("match_status_id") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("match_status"),
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
            .check_and_fix_tournament_match(context, entity, status, location, results);
    }
}