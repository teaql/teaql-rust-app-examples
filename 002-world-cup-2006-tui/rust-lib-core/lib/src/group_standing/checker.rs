
use teaql_runtime::{CheckObjectStatus, CheckResults, ObjectLocation, TypedChecker, UserContext};

pub trait GroupStandingCheckerLogic: Send + Sync {
    fn check_and_fix_group_standing(
        &self,
        _context: &UserContext,
        _entity: &mut crate::GroupStanding,
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
pub struct NoopGroupStandingChecker;

impl GroupStandingCheckerLogic for NoopGroupStandingChecker {}

#[derive(Clone, Debug)]
pub struct GroupStandingChecker<L = NoopGroupStandingChecker> {
    logic: L,
}

impl Default for GroupStandingChecker<NoopGroupStandingChecker> {
    fn default() -> Self {
        Self {
            logic: NoopGroupStandingChecker,
        }
    }
}

impl<L> GroupStandingChecker<L>
where
    L: GroupStandingCheckerLogic,
{
    pub fn new(logic: L) -> Self {
        Self { logic }
    }
}

impl<L> TypedChecker<crate::GroupStanding> for GroupStandingChecker<L>
where
    L: GroupStandingCheckerLogic,
{
    fn check_and_fix_typed(
        &self,
        context: &UserContext,
        entity: &mut crate::GroupStanding,
        status: CheckObjectStatus,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if status.is_create() {
            entity.update_create_time(context.fix_time());
            context.record_fix_evidence(teaql_runtime::FixEvidence::new("GroupStanding", "create_time", teaql_runtime::FixEvidenceSource::Clock, "graphClock"));
        }

        if status.is_create() {
            entity.update_update_time(context.fix_time());
            context.record_fix_evidence(teaql_runtime::FixEvidence::new("GroupStanding", "update_time", teaql_runtime::FixEvidenceSource::Clock, "graphClock"));
        }
        if status.is_create() || status.is_update() {
            entity.update_update_time(context.fix_time());
            context.record_fix_evidence(teaql_runtime::FixEvidence::new("GroupStanding", "update_time", teaql_runtime::FixEvidenceSource::Clock, "graphClock"));
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

        if status.is_update() && !entity.is_loaded("played") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("played"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("won") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("won"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("drawn") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("drawn"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("lost") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("lost"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("goals_for") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("goals_for"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("goals_against") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("goals_against"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("goal_difference") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("goal_difference"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("points") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("points"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("standing_rank") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("standing_rank"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("tournament_team_id") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("tournament_team"),
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
            .check_and_fix_group_standing(context, entity, status, location, results);
    }
}