use pms_service_core::{
    AuditedSave, Q, ServiceRuntime,
    service_runtime_from_env,
    teaql_core::Entity,
};
use chrono::Utc;
use rand::Rng;
use rust_decimal::Decimal;

pub async fn init_db() -> Result<ServiceRuntime, Box<dyn std::error::Error>> {
    std::env::set_var("PMS_SERVICE_CORE_DATABASE_URL", "sqlite::memory:");
    let ctx = service_runtime_from_env().await?;
    ctx.ensure_schema().await?;

    seed_data(&ctx).await?;
    Ok(ctx)
}

pub async fn seed_data(ctx: &ServiceRuntime) -> Result<(), Box<dyn std::error::Error>> {
    // Check if device_settings exists
    let settings = Q::device_settings()
        .comment("Check device settings")
        .purpose("Check seed state")
        .execute_for_list(ctx)
        .await?;

    if settings.is_empty() {
        // Create default device settings for root device system (id = 1)
        let mut setting = Q::device_settings()
            .purpose("Seed device settings")
            .new_entity(ctx);
        setting.update_device_system_id(1_u64);
        setting.update_calibration_point(0);
        setting.update_data_keep_days(30);
        setting.update_sampling_frequency(10);
        setting.update_password_enabled(1);
        setting.update_password("123456".to_string());
        setting.update_super_password("888888".to_string());
        setting.audit_as("Initialize default device settings").save(ctx).await?;

        // Seed initial historical sample records (last 20 samples)
        let now = Utc::now();
        let mut rng = rand::thread_rng();

        for i in (0..20).rev() {
            let sample_time = now - chrono::Duration::minutes(i * 5);

            let base_c1 = rng.gen_range(1100..1500);
            let base_c2 = rng.gen_range(700..950);
            let base_c3 = rng.gen_range(350..500);
            let base_c4 = rng.gen_range(120..220);
            let base_c5 = rng.gen_range(30..60);
            let base_c6 = rng.gen_range(8..18);
            let base_c7 = rng.gen_range(2..6);
            let base_c8 = rng.gen_range(0..2);

            let mut rec = Q::sample_records()
                .purpose("Seed sample record")
                .new_entity(ctx);
            rec.update_device_system_id(1_u64);
            rec.update_sample_time(sample_time);
            rec.update_gas(Decimal::from_str_exact("28.3")?);
            rec.update_lref(Decimal::from_str_exact("2.45")?);
            rec.update_impurity1(base_c1);
            rec.update_impurity2(base_c2);
            rec.update_impurity3(base_c3);
            rec.update_impurity4(base_c4);
            rec.update_impurity5(base_c5);
            rec.update_impurity6(base_c6);
            rec.update_impurity7(base_c7);
            rec.update_impurity8(base_c8);

            rec.audit_as("Seed initial sample history").save(ctx).await?;
        }
    }

    Ok(())
}

pub async fn trigger_new_sample(ctx: &ServiceRuntime) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let mut rng = rand::thread_rng();

    let gas_val = format!("{:.1}", 28.3 + (rng.gen_range(-5..=5) as f64) * 0.1);
    let lref_val = format!("{:.2}", 2.45 + (rng.gen_range(-2..=2) as f64) * 0.01);

    let base_c1 = rng.gen_range(1150..1450);
    let base_c2 = rng.gen_range(750..920);
    let base_c3 = rng.gen_range(380..480);
    let base_c4 = rng.gen_range(140..200);
    let base_c5 = rng.gen_range(35..55);
    let base_c6 = rng.gen_range(10..16);
    let base_c7 = rng.gen_range(2..5);
    let base_c8 = rng.gen_range(0..2);

    let mut rec = Q::sample_records()
        .purpose("Realtime sampling trigger")
        .new_entity(ctx);
    rec.update_device_system_id(1_u64);
    rec.update_sample_time(now);
    rec.update_gas(Decimal::from_str_exact(&gas_val)?);
    rec.update_lref(Decimal::from_str_exact(&lref_val)?);
    rec.update_impurity1(base_c1);
    rec.update_impurity2(base_c2);
    rec.update_impurity3(base_c3);
    rec.update_impurity4(base_c4);
    rec.update_impurity5(base_c5);
    rec.update_impurity6(base_c6);
    rec.update_impurity7(base_c7);
    rec.update_impurity8(base_c8);

    rec.audit_as("Record real-time particle sample").save(ctx).await?;

    Ok(())
}
