use teaql_core::Expr;
use teaql_provider_linux::LinuxDataServiceExecutor;
use linux_system_info_core::{
    q::Q,
    e::E,
    runtime::module_with_behaviors_and_checkers,
};

#[tokio::main]
async fn main() {
    let mut ctx = module_with_behaviors_and_checkers().into_context();
    let executor = LinuxDataServiceExecutor::new();
    ctx.register_executor(executor);

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║   TeaQL Linux Provider — System Information Query Demo      ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // ─── 1. System Information ─────────────────────────────────────────
    println!("━━━ 1. SystemInfo ━━━");
    let result = Q::system_info()
        .purpose("Query system info")
        .execute_for_list(&ctx)
        .await
        .unwrap();

    for row in &result {
        println!("  Hostname:    {}", E::system_info(row).get_hostname().or_default());
        println!("  CPU Cores:   {}", E::system_info(row).get_cpu_count().or_default());
        println!("  Total Mem:   {:.1} GB", E::system_info(row).get_memory_total_bytes().or_default() as f64 / 1024.0 / 1024.0 / 1024.0);
        println!("  Avail Mem:   {:.1} GB", E::system_info(row).get_memory_available_bytes().or_default() as f64 / 1024.0 / 1024.0 / 1024.0);
        println!("  Uptime:      {} s", E::system_info(row).get_uptime_seconds().or_default());
        println!("  Load Avg:    {} / {} / {}",
            E::system_info(row).get_load_avg_1().or_default(),
            E::system_info(row).get_load_avg_5().or_default(),
            E::system_info(row).get_load_avg_15().or_default(),
        );
    }
    println!();

    // ─── 2. Process Query: Top 10 by Memory ────────────────────────────
    println!("━━━ 2. Top 10 Processes by Memory ━━━");
    let result = Q::processes()
        .order_by_memory_rss_kb_desc()
        .limit(10)
        .purpose("Top memory processes")
        .execute_for_list(&ctx)
        .await
        .unwrap();

    println!("  {:>6}  {:>10}  {:>10}  {}", "PID", "RSS(MB)", "VMS(MB)", "NAME");
    println!("  {:->6}  {:->10}  {:->10}  {:->20}", "", "", "", "");
    for row in &result {
        println!("  {:>6}  {:>10.1}  {:>10.1}  {}",
            E::process(row).get_pid().or_default(),
            E::process(row).get_memory_rss_kb().or_default() as f64 / 1024.0,
            E::process(row).get_memory_vms_kb().or_default() as f64 / 1024.0,
            E::process(row).get_name().or_default(),
        );
    }
    println!("  ({} results)", result.len());
    println!();

    // ─── 3. Filter Query: Find Specific Processes ──────────────────────
    println!("━━━ 3. Processes containing 'rust' ━━━");
    let result = Q::processes()
        .and_filter(Expr::contain("name", "rust"))
        .order_by_pid_asc()
        .purpose("Find rust processes")
        .execute_for_list(&ctx)
        .await
        .unwrap();

    if result.is_empty() {
        println!("  (No 'rust' processes found, searching for 'linux' instead)");
        let result = Q::processes()
            .and_filter(Expr::contain("cmdline", "linux"))
            .limit(5)
            .purpose("Find linux processes")
            .execute_for_list(&ctx)
            .await
            .unwrap();
        for row in &result {
            println!("  PID={} name={} rss={}MB",
                E::process(row).get_pid().or_default(), E::process(row).get_name().or_default(),
                E::process(row).get_memory_rss_kb().or_default() / 1024);
        }
        println!("  Total {} processes", result.len());
    } else {
        for row in &result {
            println!("  PID={} name={} rss={}MB",
                E::process(row).get_pid().or_default(), E::process(row).get_name().or_default(),
                E::process(row).get_memory_rss_kb().or_default() / 1024);
        }
        println!("  Total {} processes", result.len());
    }
    println!();

    // ─── 4. Condition Filter: Processes > 100MB Memory ─────────────────
    println!("━━━ 4. Processes > 100MB Memory ━━━");
    let result = Q::processes()
        .and_filter(Expr::gt("memory_rss_kb", 100 * 1024_i64))
        .order_by_memory_rss_kb_desc()
        .purpose("Find large processes")
        .execute_for_list(&ctx)
        .await
        .unwrap();

    for row in &result {
        println!("  PID={:<6} {:>8.1}MB  {}",
            E::process(row).get_pid().or_default(),
            E::process(row).get_memory_rss_kb().or_default() as f64 / 1024.0,
            E::process(row).get_name().or_default());
    }
    println!("  Total {} processes over 100MB", result.len());
    println!();

    // ─── 5. Thread Query: Threads of Current Process ───────────────────
    println!("━━━ 5. Threads for Current Process (PID={}) ━━━", std::process::id());
    let result = Q::threads()
        .and_filter(Expr::eq("process_pid", std::process::id() as i64))
        .purpose("Find current process threads")
        .execute_for_list(&ctx)
        .await
        .unwrap();

    for row in &result {
        println!("  TID={} name={} state={}",
            E::thread(row).get_tid().or_default(), E::thread(row).get_name().or_default(), E::thread(row).get_state().or_default());
    }
    println!("  Total {} threads", result.len());
    println!();

    // ─── 6. Total Processes and Threads ────────────────────────────────
    println!("━━━ 6. System Stats ━━━");
    let result = Q::processes()
        .purpose("System stats")
        .execute_for_list(&ctx)
        .await
        .unwrap();

    let total_processes = result.len();
    let total_threads: i64 = result.iter().map(|r| E::process(r).get_thread_count().or_default() as i64).sum();
    let total_rss: i64 = result.iter().map(|r| E::process(r).get_memory_rss_kb().or_default() as i64).sum();

    println!("  Total Processes:  {}", total_processes);
    println!("  Total Threads:    {}", total_threads);
    println!("  Total RSS Memory: {} MB", total_rss / 1024);
    println!();

    println!("✅ All queries completed! TeaQL successfully queried Linux system info via Q and E APIs.");
}
