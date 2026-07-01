use std::time::{Duration, Instant};
use linux_system_info_core::Q;

use crate::app::{App, View};

pub async fn refresh_data(app: &mut App) {
    if app.last_refresh.elapsed() < Duration::from_secs(2) {
        return;
    }
    app.last_refresh = Instant::now();
    
    let ctx = &app.ctx;
    if let Ok(sys) = Q::system_info().purpose("sys_info").execute_for_one(ctx).await {
        app.sys_info = sys;
    }

    if let View::Dashboard(_) = app.view {
        if let Ok(procs) = Q::processes()
            .with_memory_vms_kb_greater_than(0_i64)
            .order_by_memory_rss_kb_desc()
            .limit(50)
            .purpose("mem")
            .execute_for_list(ctx).await {
            app.mem_procs = procs.into_iter().collect();
        }
        if let Ok(procs) = Q::processes()
            .with_memory_vms_kb_greater_than(0_i64)
            .order_by_create_time_asc()
            .limit(50)
            .purpose("time")
            .execute_for_list(ctx).await {
            app.time_procs = procs.into_iter().collect();
        }
        if let Ok(procs) = Q::processes()
            .with_memory_vms_kb_greater_than(0_i64)
            .order_by_cpu_user_ticks_desc()
            .limit(50)
            .purpose("cpu")
            .execute_for_list(ctx).await {
            app.cpu_procs = procs.into_iter().collect();
        }
        if let Ok(procs) = Q::processes()
            .with_memory_vms_kb_greater_than(0_i64)
            .order_by_thread_count_desc()
            .limit(50)
            .purpose("threads")
            .execute_for_list(ctx).await {
            app.thread_procs = procs.into_iter().collect();
        }
    } else if let View::ProcessDetail(pid) = app.view {
        if let Ok(p) = Q::processes().with_pid_is(pid).purpose("detail").execute_for_one(ctx).await {
            app.detail_proc = p;
        }
        if let Ok(threads) = Q::threads().with_process_pid_is(pid).order_by_tid_asc().purpose("detail_threads").execute_for_list(ctx).await {
            app.detail_threads = threads.into_iter().collect();
        }
    }
}
