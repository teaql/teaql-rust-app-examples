
use std::collections::BTreeMap;
use crate::TeaqlRuntime;
use crate::Q;
use teaql_core::Entity as _;
use crate::request_support::AuditedSave as _;

pub trait IntoU64 {
    fn into_u64(self) -> u64;
}

impl IntoU64 for u64 {
    fn into_u64(self) -> u64 {
        self
    }
}

impl IntoU64 for Option<&teaql_core::Value> {
    fn into_u64(self) -> u64 {
        self.and_then(|v| v.try_u64()).unwrap_or_default()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SampleDataScale {
    Tiny,
    Small,
    Medium,
}

pub struct SampleDataPlan {
    pub scale: SampleDataScale,
    pub seed: u64,
}

impl SampleDataPlan {
    pub fn small() -> Self {
        Self {
            scale: SampleDataScale::Small,
            seed: 0,
        }
    }
}

pub struct SampleDataReport {
    pub generated: BTreeMap<&'static str, usize>,
    pub skipped: Vec<SampleDataSkipped>,
}

pub struct SampleDataSkipped {
    pub entity: &'static str,
    pub reason: String,
}

#[derive(Debug)]
pub struct SampleDataError {
    message: String,
}

impl SampleDataError {
    fn from_display(error: impl std::fmt::Display) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

impl std::fmt::Display for SampleDataError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for SampleDataError {}

pub struct SampleDataState {
    pub plan: SampleDataPlan,
    pub references: BTreeMap<&'static str, Vec<u64>>,
    pub generated: BTreeMap<&'static str, usize>,
    pub skipped: Vec<SampleDataSkipped>,
}

impl SampleDataState {
    pub fn new(plan: SampleDataPlan) -> Self {
        Self {
            plan,
            references: BTreeMap::new(),
            generated: BTreeMap::new(),
            skipped: Vec::new(),
        }
    }

    pub fn add_reference(&mut self, entity: &'static str, id: u64) {
        self.references.entry(entity).or_default().push(id);
    }

    pub fn ids(&self, entity: &'static str) -> &[u64] {
        self.references.get(entity).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn pick_id(&self, entity: &'static str, salt: usize) -> Option<u64> {
        let ids = self.ids(entity);
        if ids.is_empty() {
            None
        } else {
            Some(ids[salt % ids.len()])
        }
    }

    pub fn pick_unused_id(&self, entity: &'static str, salt: usize, used: &std::collections::HashSet<u64>) -> Option<u64> {
        let ids = self.ids(entity);
        if ids.is_empty() {
            return None;
        }

        let best_id = ids[salt % ids.len()];
        if !used.contains(&best_id) {
            return Some(best_id);
        }

        for id in ids {
            if !used.contains(id) {
                return Some(*id);
            }
        }

        Some(best_id)
    }

    pub fn record_generated(&mut self, entity: &'static str) {
        *self.generated.entry(entity).or_default() += 1;
    }

    pub fn record_skipped(&mut self, entity: &'static str, reason: String) {
        self.skipped.push(SampleDataSkipped { entity, reason });
    }

    pub fn into_report(self) -> SampleDataReport {
        SampleDataReport {
            generated: self.generated,
            skipped: self.skipped,
        }
    }
}

pub async fn generate_sample_data<C>(
    context: &C,
    plan: SampleDataPlan,
) -> Result<SampleDataReport, SampleDataError>
where
    C: TeaqlRuntime + ?Sized,
{
    log::info!("Starting sample data generation. Scale: {:?}, Seed: {}", plan.scale, plan.seed);
    let mut state = SampleDataState::new(plan);

    load_root_device_systems(context, &mut state).await?; //depth: 0

    load_constant_system_statuses(context, &mut state).await?;

    generate_device_settings(context, &mut state).await?;

    generate_sample_records(context, &mut state).await?;


    let report = state.into_report();
    log::info!("Sample data generation completed successfully. Generated: {} tables, Skipped: {} tables.", report.generated.len(), report.skipped.len());
    Ok(report)
}

async fn load_root_device_systems<C>(
    context: &C,
    state: &mut SampleDataState,
) -> Result<(), SampleDataError>
where
    C: TeaqlRuntime + ?Sized,
{
    let list = Q::device_systems().comment("what: inspect existing entities before sample-data initialization").purpose("why: avoid duplicate sample records").execute_for_list(context).await.unwrap_or_default();
    for item in list {
        state.add_reference(crate::DeviceSystem::ENTITY_NAME, item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_system_statuses<C>(
    context: &C,
    state: &mut SampleDataState,
) -> Result<(), SampleDataError>
where
    C: TeaqlRuntime + ?Sized,
{
    let list = Q::system_statuses().comment("what: inspect existing entities before sample-data initialization").purpose("why: avoid duplicate sample records").execute_for_list(context).await.unwrap_or_default();
    for item in list {
        state.add_reference(crate::SystemStatus::ENTITY_NAME, item.id().into_u64());
    }
    Ok(())
}

async fn generate_device_settings<C>(
    context: &C,
    state: &mut SampleDataState,
) -> Result<(), SampleDataError>
where
    C: TeaqlRuntime + ?Sized,
{
        if state.ids("Device System").is_empty() {
            state.record_skipped(crate::DeviceSetting::ENTITY_NAME, "Required dependency Device System is missing in reference pool".to_string());
            log::info!("Skipped generating Device Setting: Required dependency Device System is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Device Setting (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::device_settings().comment("what: initialize a sample entity").purpose("why: populate the requested sample dataset").new_entity(context);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Device System", i as usize, &used_refs) {
                    entity.update_device_system_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_calibration_point(rand_val as i64);
                }

                {
                    let max_val: u64 = "30".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_data_keep_days(rand_val as i64);
                }

                {
                    let max_val: u64 = "10".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_sampling_frequency(rand_val as i64);
                }

                {
                    let max_val: u64 = "1".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_password_enabled(rand_val as i64);
                }

                entity.update_password_hash(format!("{} {}", "pass_123456", i + 1));

                entity.update_super_password_hash(format!("{} {}", "pass_888888", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(teaql_core::time::Timestamp(past.and_utc().timestamp_millis()));
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(teaql_core::time::Timestamp(past.and_utc().timestamp_millis()));
                }



entity.audit_as("Init Sample Data").save(context).await.map_err(SampleDataError::from_display)?;

        state.record_generated(crate::DeviceSetting::ENTITY_NAME);

        if i % 20 == 0 {
            log::info!("Generating Device Setting: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Device Setting.");
    Ok(())
}


async fn generate_sample_records<C>(
    context: &C,
    state: &mut SampleDataState,
) -> Result<(), SampleDataError>
where
    C: TeaqlRuntime + ?Sized,
{
        if state.ids("Device System").is_empty() {
            state.record_skipped(crate::SampleRecord::ENTITY_NAME, "Required dependency Device System is missing in reference pool".to_string());
            log::info!("Skipped generating Sample Record: Required dependency Device System is missing in reference pool.");
            return Ok(());
        }

        if state.ids("System Status").is_empty() {
            state.record_skipped(crate::SampleRecord::ENTITY_NAME, "Required dependency System Status is missing in reference pool".to_string());
            log::info!("Skipped generating Sample Record: Required dependency System Status is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Sample Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::sample_records().comment("what: initialize a sample entity").purpose("why: populate the requested sample dataset").new_entity(context);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Device System", i as usize, &used_refs) {
                    entity.update_device_system_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("System Status", i as usize, &used_refs) {
                    entity.update_system_status_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_sample_time(teaql_core::time::Timestamp(past.and_utc().timestamp_millis()));
                }

                {
                    let max_val: u64 = "28.3".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_gas(rand_val as i64);
                }

                {
                    let max_val: u64 = "2.45".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_lref(rand_val as i64);
                }

                {
                    let max_val: u64 = "1250".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_impurity1(rand_val as i64);
                }

                {
                    let max_val: u64 = "850".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_impurity2(rand_val as i64);
                }

                {
                    let max_val: u64 = "420".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_impurity3(rand_val as i64);
                }

                {
                    let max_val: u64 = "180".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_impurity4(rand_val as i64);
                }

                {
                    let max_val: u64 = "45".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_impurity5(rand_val as i64);
                }

                {
                    let max_val: u64 = "12".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_impurity6(rand_val as i64);
                }

                {
                    let max_val: u64 = "3".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_impurity7(rand_val as i64);
                }

                {
                    let max_val: u64 = "1".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_impurity8(rand_val as i64);
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(teaql_core::time::Timestamp(past.and_utc().timestamp_millis()));
                }



entity.audit_as("Init Sample Data").save(context).await.map_err(SampleDataError::from_display)?;

        state.record_generated(crate::SampleRecord::ENTITY_NAME);

        if i % 20 == 0 {
            log::info!("Generating Sample Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Sample Record.");
    Ok(())
}
