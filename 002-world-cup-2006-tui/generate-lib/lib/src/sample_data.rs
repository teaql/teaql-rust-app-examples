use std::collections::BTreeMap;
use crate::TeaqlRuntime;
use crate::Q;
use teaql_core::Entity;
use crate::request_support::TeaqlUserContextExt;
use crate::request_support::AuditedSave;

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
    ctx: &C,
    plan: SampleDataPlan,
) -> Result<SampleDataReport, String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    log::info!("Starting sample data generation. Scale: {:?}, Seed: {}", plan.scale, plan.seed);
    let mut state = SampleDataState::new(plan);

    load_root_tournaments(ctx, &mut state).await?; //depth: 0

    load_constant_card_categories(ctx, &mut state).await?;
    load_constant_confederations(ctx, &mut state).await?;
    load_constant_goal_categories(ctx, &mut state).await?;
    load_constant_match_stages(ctx, &mut state).await?;
    load_constant_match_statuses(ctx, &mut state).await?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_match_groups(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_tournament_teams(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_group_standings(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_tournament_matches(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_match_cards(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_match_goals(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;


    let report = state.into_report();
    log::info!("Sample data generation completed successfully. Generated: {} tables, Skipped: {} tables.", report.generated.len(), report.skipped.len());
    Ok(report)
}

async fn load_root_tournaments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::tournaments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Tournament", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_card_categories<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::card_categories().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Card Category", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_confederations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::confederations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Confederation", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_goal_categories<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::goal_categories().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Goal Category", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_match_stages<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::match_stages().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Match Stage", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_match_statuses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::match_statuses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Match Status", item.id().into_u64());
    }
    Ok(())
}

async fn generate_match_groups<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Tournament").is_empty() {
            state.record_skipped("Match Group", "Required dependency Tournament is missing in reference pool".to_string());
            log::info!("Skipped generating Match Group: Required dependency Tournament is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Match Group (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::match_groups().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Tournament", i as usize, &used_refs) {
                    entity.update_tournament_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_group_letter(format!("{} {}", "A", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Match Group");

        if i % 20 == 0 {
            log::info!("Generating Match Group: {}/{}", i, fanout);
        }

        state.add_reference("Match Group", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Match Group.");
    Ok(())
}


async fn generate_tournament_teams<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Confederation").is_empty() {
            state.record_skipped("Tournament Team", "Required dependency Confederation is missing in reference pool".to_string());
            log::info!("Skipped generating Tournament Team: Required dependency Confederation is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Tournament").is_empty() {
            state.record_skipped("Tournament Team", "Required dependency Tournament is missing in reference pool".to_string());
            log::info!("Skipped generating Tournament Team: Required dependency Tournament is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Tournament Team (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::tournament_teams().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Confederation", i as usize, &used_refs) {
                    entity.update_confederation_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Tournament", i as usize, &used_refs) {
                    entity.update_tournament_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_team_name(format!("{} {}", "Brazil", i + 1));

                entity.update_team_code(format!("{} {}", "BRA", i + 1));

                entity.update_emoji_flag(format!("{} {}", "🇧🇷", i + 1));

                {
                    let max_val: u64 = "3".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_fifa_ranking(rand_val as i64);
                }

                entity.update_manager_name(format!("{} {}", "Dorival Junior", i + 1));

                entity.update_group_letter(format!("{} {}", "C", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Tournament Team");

        if i % 20 == 0 {
            log::info!("Generating Tournament Team: {}/{}", i, fanout);
        }

        state.add_reference("Tournament Team", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Tournament Team.");
    Ok(())
}


async fn generate_group_standings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Tournament Team").is_empty() {
            state.record_skipped("Group Standing", "Required dependency Tournament Team is missing in reference pool".to_string());
            log::info!("Skipped generating Group Standing: Required dependency Tournament Team is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Match Group").is_empty() {
            state.record_skipped("Group Standing", "Required dependency Match Group is missing in reference pool".to_string());
            log::info!("Skipped generating Group Standing: Required dependency Match Group is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Tournament").is_empty() {
            state.record_skipped("Group Standing", "Required dependency Tournament is missing in reference pool".to_string());
            log::info!("Skipped generating Group Standing: Required dependency Tournament is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Group Standing (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::group_standings().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Tournament Team", i as usize, &used_refs) {
                    entity.update_tournament_team_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Match Group", i as usize, &used_refs) {
                    entity.update_match_group_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Tournament", i as usize, &used_refs) {
                    entity.update_tournament_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_played(rand_val as i64);
                }

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_won(rand_val as i64);
                }

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_drawn(rand_val as i64);
                }

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_lost(rand_val as i64);
                }

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_goals_for(rand_val as i64);
                }

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_goals_against(rand_val as i64);
                }

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_goal_difference(rand_val as i64);
                }

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_points(rand_val as i64);
                }

                {
                    let max_val: u64 = "1".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_standing_rank(rand_val as i64);
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Group Standing");

        if i % 20 == 0 {
            log::info!("Generating Group Standing: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Group Standing.");
    Ok(())
}


async fn generate_tournament_matches<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Tournament Team").is_empty() {
            state.record_skipped("Tournament Match", "Required dependency Tournament Team is missing in reference pool".to_string());
            log::info!("Skipped generating Tournament Match: Required dependency Tournament Team is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Tournament Team").is_empty() {
            state.record_skipped("Tournament Match", "Required dependency Tournament Team is missing in reference pool".to_string());
            log::info!("Skipped generating Tournament Match: Required dependency Tournament Team is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Match Stage").is_empty() {
            state.record_skipped("Tournament Match", "Required dependency Match Stage is missing in reference pool".to_string());
            log::info!("Skipped generating Tournament Match: Required dependency Match Stage is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Match Group").is_empty() {
            state.record_skipped("Tournament Match", "Required dependency Match Group is missing in reference pool".to_string());
            log::info!("Skipped generating Tournament Match: Required dependency Match Group is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Match Status").is_empty() {
            state.record_skipped("Tournament Match", "Required dependency Match Status is missing in reference pool".to_string());
            log::info!("Skipped generating Tournament Match: Required dependency Match Status is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Tournament").is_empty() {
            state.record_skipped("Tournament Match", "Required dependency Tournament is missing in reference pool".to_string());
            log::info!("Skipped generating Tournament Match: Required dependency Tournament is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Tournament Match (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::tournament_matches().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Tournament Team", i as usize, &used_refs) {
                    entity.update_home_team_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Tournament Team", i as usize, &used_refs) {
                    entity.update_away_team_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Match Stage", i as usize, &used_refs) {
                    entity.update_match_stage_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Match Group", i as usize, &used_refs) {
                    entity.update_match_group_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Match Status", i as usize, &used_refs) {
                    entity.update_match_status_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Tournament", i as usize, &used_refs) {
                    entity.update_tournament_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                {
                    let max_val: u64 = "1".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_match_number(rand_val as i64);
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_match_date(past.format("%Y-%m-%d").to_string());
                }

                entity.update_venue_name(format!("{} {}", "Azteca Stadium", i + 1));

                entity.update_venue_city(format!("{} {}", "Mexico City", i + 1));

                entity.update_venue_country(format!("{} {}", "Mexico", i + 1));

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_home_score(rand_val as i64);
                }

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_away_score(rand_val as i64);
                }

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_extra_time_home(rand_val as i64);
                }

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_extra_time_away(rand_val as i64);
                }

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_penalty_home(rand_val as i64);
                }

                {
                    let max_val: u64 = "0".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_penalty_away(rand_val as i64);
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Tournament Match");

        if i % 20 == 0 {
            log::info!("Generating Tournament Match: {}/{}", i, fanout);
        }

        state.add_reference("Tournament Match", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Tournament Match.");
    Ok(())
}


async fn generate_match_cards<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Tournament Match").is_empty() {
            state.record_skipped("Match Card", "Required dependency Tournament Match is missing in reference pool".to_string());
            log::info!("Skipped generating Match Card: Required dependency Tournament Match is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Tournament Team").is_empty() {
            state.record_skipped("Match Card", "Required dependency Tournament Team is missing in reference pool".to_string());
            log::info!("Skipped generating Match Card: Required dependency Tournament Team is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Card Category").is_empty() {
            state.record_skipped("Match Card", "Required dependency Card Category is missing in reference pool".to_string());
            log::info!("Skipped generating Match Card: Required dependency Card Category is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Tournament").is_empty() {
            state.record_skipped("Match Card", "Required dependency Tournament is missing in reference pool".to_string());
            log::info!("Skipped generating Match Card: Required dependency Tournament is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Match Card (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::match_cards().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Tournament Match", i as usize, &used_refs) {
                    entity.update_tournament_match_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Tournament Team", i as usize, &used_refs) {
                    entity.update_tournament_team_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Card Category", i as usize, &used_refs) {
                    entity.update_card_category_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Tournament", i as usize, &used_refs) {
                    entity.update_tournament_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_player_name(format!("{} {}", "Casemiro", i + 1));

                {
                    let max_val: u64 = "34".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_minute_issued(rand_val as i64);
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Match Card");

        if i % 20 == 0 {
            log::info!("Generating Match Card: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Match Card.");
    Ok(())
}


async fn generate_match_goals<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Tournament Match").is_empty() {
            state.record_skipped("Match Goal", "Required dependency Tournament Match is missing in reference pool".to_string());
            log::info!("Skipped generating Match Goal: Required dependency Tournament Match is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Tournament Team").is_empty() {
            state.record_skipped("Match Goal", "Required dependency Tournament Team is missing in reference pool".to_string());
            log::info!("Skipped generating Match Goal: Required dependency Tournament Team is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Goal Category").is_empty() {
            state.record_skipped("Match Goal", "Required dependency Goal Category is missing in reference pool".to_string());
            log::info!("Skipped generating Match Goal: Required dependency Goal Category is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Tournament").is_empty() {
            state.record_skipped("Match Goal", "Required dependency Tournament is missing in reference pool".to_string());
            log::info!("Skipped generating Match Goal: Required dependency Tournament is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Match Goal (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::match_goals().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Tournament Match", i as usize, &used_refs) {
                    entity.update_tournament_match_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Tournament Team", i as usize, &used_refs) {
                    entity.update_tournament_team_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Goal Category", i as usize, &used_refs) {
                    entity.update_goal_category_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Tournament", i as usize, &used_refs) {
                    entity.update_tournament_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_player_name(format!("{} {}", "Vinicius Jr", i + 1));

                {
                    let max_val: u64 = "23".parse().unwrap_or(1000);
                    let rand_val = (i as u64 + state.plan.seed) % max_val.max(1) + 1;
                    entity.update_minute_scored(rand_val as i64);
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Match Goal");

        if i % 20 == 0 {
            log::info!("Generating Match Goal: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Match Goal.");
    Ok(())
}
