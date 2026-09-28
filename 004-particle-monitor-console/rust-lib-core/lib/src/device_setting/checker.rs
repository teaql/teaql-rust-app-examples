
use teaql_runtime::{CheckObjectStatus, CheckResults, ObjectLocation, TypedChecker, UserContext};

pub trait DeviceSettingCheckerLogic: Send + Sync {
    fn check_and_fix_device_setting(
        &self,
        _context: &UserContext,
        _entity: &mut crate::DeviceSetting,
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
pub struct NoopDeviceSettingChecker;

impl DeviceSettingCheckerLogic for NoopDeviceSettingChecker {}

#[derive(Clone, Debug)]
pub struct DeviceSettingChecker<L = NoopDeviceSettingChecker> {
    logic: L,
}

impl Default for DeviceSettingChecker<NoopDeviceSettingChecker> {
    fn default() -> Self {
        Self {
            logic: NoopDeviceSettingChecker,
        }
    }
}

impl<L> DeviceSettingChecker<L>
where
    L: DeviceSettingCheckerLogic,
{
    pub fn new(logic: L) -> Self {
        Self { logic }
    }
}

impl<L> TypedChecker<crate::DeviceSetting> for DeviceSettingChecker<L>
where
    L: DeviceSettingCheckerLogic,
{
    fn check_and_fix_typed(
        &self,
        context: &UserContext,
        entity: &mut crate::DeviceSetting,
        status: CheckObjectStatus,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if status.is_create() {
            entity.update_create_time(context.fix_time());
            context.record_fix_evidence(teaql_runtime::FixEvidence::new("DeviceSetting", "create_time", teaql_runtime::FixEvidenceSource::Clock, "graphClock"));
        }

        if status.is_create() {
            entity.update_update_time(context.fix_time());
            context.record_fix_evidence(teaql_runtime::FixEvidence::new("DeviceSetting", "update_time", teaql_runtime::FixEvidenceSource::Clock, "graphClock"));
        }
        if status.is_create() || status.is_update() {
            entity.update_update_time(context.fix_time());
            context.record_fix_evidence(teaql_runtime::FixEvidence::new("DeviceSetting", "update_time", teaql_runtime::FixEvidenceSource::Clock, "graphClock"));
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

        if status.is_update() && !entity.is_loaded("device_system_id") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("device_system"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("calibration_point") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("calibration_point"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("data_keep_days") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("data_keep_days"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("sampling_frequency") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("sampling_frequency"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("password_enabled") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("password_enabled"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("password_hash") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("password_hash"),
                )
                .with_message("Mutation requires a fully loaded entity"),
            );
        }

        if status.is_update() && !entity.is_loaded("super_password_hash") {
            results.push(
                teaql_runtime::CheckResult::new(
                    teaql_runtime::CheckRule::InvalidType,
                    location.clone().member("super_password_hash"),
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
            .check_and_fix_device_setting(context, entity, status, location, results);
    }
}