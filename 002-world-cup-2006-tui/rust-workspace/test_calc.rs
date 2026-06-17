use fifa_world_cup_2026_service::*;
use teaql_runtime::{UserContext, ServiceRuntimeConfig};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_path = "test.db";
    let config = ServiceRuntimeConfig {
        database_url: format!("sqlite:{}", db_path),
    };
    let ctx = fifa_world_cup_2026_service::service_runtime(config).await?;
    
    // Create tables and seed data
    let mut app = app::App::new(ctx.clone(), std::sync::Arc::new(std::sync::Mutex::new(vec![])));
    app.fetch_data().await?;
    
    // Let's record a goal to create a match
    app.record_goal("Mexico", "Player1").await?;
    app.record_goal("Mexico", "Player2").await?;
    app.record_goal("South Africa", "Player3").await?; // "South Africa" is in Group A with Mexico? Let's check seed.rs

    let standings = Q::group_standings().execute_for_list(&ctx).await?.data;
    for s in standings {
        if s.played() > 0 {
            let t = Q::tournament_teams().with_id_is(s.tournament_team_id()).execute_for_list(&ctx).await?.data.pop().unwrap();
            println!("{}: P:{} W:{} D:{} L:{} GF:{} GA:{} GD:{} Pts:{}", t.team_name(), s.played(), s.won(), s.drawn(), s.lost(), s.goals_for(), s.goals_against(), s.goal_difference(), s.points());
        }
    }
    Ok(())
}
