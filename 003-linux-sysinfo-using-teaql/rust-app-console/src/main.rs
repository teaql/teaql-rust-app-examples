mod app;
mod service;
mod ui;

use app::App;
use linux_system_info_core::runtime::module_with_behaviors_and_checkers;
use teaql_provider_linux::LinuxDataServiceExecutor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = module_with_behaviors_and_checkers().into_context();
    ctx.register_executor(LinuxDataServiceExecutor::new());

    let mut app = App::new(ctx).await;
    app.run().await
}
