use fifa_world_cup_2026_service::*;
use std::error::Error;
use teaql_core::Entity;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_path = {
        if let Some(mut path) = dirs::home_dir() {
            path.push(".wc2026");
            std::fs::create_dir_all(&path)?;
            path.push("worldcup.db");
            path.to_string_lossy().to_string()
        } else {
            "worldcup.db".to_string()
        }
    };
    
    let config = ServiceRuntimeConfig {
        database_url: format!("sqlite:{}", db_path),
    };
    let ctx = service_runtime(config).await?;
    
    // We will recalculate all standings from matches
    let teams = Q::tournament_teams().purpose("script").execute_for_list(&ctx).await?.data;
    let matches = Q::tournament_matches().purpose("script").execute_for_list(&ctx).await?.data;
    let mut standings = Q::group_standings().purpose("script").execute_for_list(&ctx).await?.data;
    
    let mut fix_count = 0;
    
    for team in teams {
        let team_id = team.id();
        let mut played = 0; let mut won = 0; let mut drawn = 0; let mut lost = 0;
        let mut gf = 0; let mut ga = 0;
        
        for m in &matches {
            let hs = m.home_score();
            let as_sc = m.away_score();
            if m.home_team_id() == team_id {
                played += 1; gf += hs; ga += as_sc;
                if hs > as_sc { won += 1; } else if hs == as_sc { drawn += 1; } else { lost += 1; }
            } else if m.away_team_id() == team_id {
                played += 1; gf += as_sc; ga += hs;
                if as_sc > hs { won += 1; } else if as_sc == hs { drawn += 1; } else { lost += 1; }
            }
        }
        
        let standing = standings.iter_mut().find(|s| s.tournament_team_id() == team_id);
        if let Some(s) = standing {
            let new_points = (won * 3) + drawn;
            let new_gd = gf - ga;
            if s.played() != played || s.won() != won || s.drawn() != drawn || s.lost() != lost || s.goals_for() != gf || s.goals_against() != ga || s.points() != new_points || s.goal_difference() != new_gd {
                println!("Fixing team {}: P:{} W:{} D:{} L:{} Pts:{}", team.team_name(), played, won, drawn, lost, new_points);
                s.update_played(played);
                s.update_won(won);
                s.update_drawn(drawn);
                s.update_lost(lost);
                s.update_goals_for(gf);
                s.update_goals_against(ga);
                s.update_goal_difference(new_gd);
                s.update_points(new_points);
                s.clone().audit_as("Fix dirty data script").save(&ctx).await?;
                fix_count += 1;
            }
        }
    }
    
    println!("Done. Fixed {} standings.", fix_count);
    Ok(())
}
