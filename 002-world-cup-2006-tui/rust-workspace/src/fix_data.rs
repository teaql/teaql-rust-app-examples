use fifa_world_cup_2026_service::*;
use std::error::Error;
use teaql_core::{Entity, UpdateCommand};
use teaql_runtime::UserContext;

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
    let teams = Q::tournament_teams()
        .purpose("script")
        .execute_for_list(&ctx)
        .await?
        .data;
    let matches = Q::tournament_matches()
        .purpose("script")
        .execute_for_list(&ctx)
        .await?
        .data;
    let match_groups = Q::match_groups()
        .purpose("script")
        .execute_for_list(&ctx)
        .await?
        .data;
    let goals = Q::match_goals()
        .purpose("script")
        .execute_for_list(&ctx)
        .await?
        .data;
    let mut standings = Q::group_standings()
        .purpose("script")
        .execute_for_list(&ctx)
        .await?
        .data;

    ctx.entity_root().clear_current_change_set();

    let mut goal_counts: std::collections::HashMap<(u64, u64), i32> =
        std::collections::HashMap::new();
    let mut match_ids_with_goals = std::collections::HashSet::new();
    for goal in &goals {
        *goal_counts
            .entry((goal.tournament_match_id(), goal.tournament_team_id()))
            .or_insert(0) += 1;
        match_ids_with_goals.insert(goal.tournament_match_id());
    }
    let team_groups: std::collections::HashMap<u64, String> = teams
        .iter()
        .map(|team| (team.id(), team.group_letter()))
        .collect();
    let group_ids: std::collections::HashMap<String, u64> = match_groups
        .iter()
        .map(|group| (group.group_letter(), group.id()))
        .collect();

    let match_data_service = ctx.entity_data_service::<DataServiceExecutor>("TournamentMatch")?;
    for m in &matches {
        if !is_group_stage_match(m, &team_groups, &group_ids) {
            continue;
        }
        let home_score = goal_counts
            .get(&(m.id(), m.home_team_id()))
            .copied()
            .unwrap_or(0);
        let away_score = goal_counts
            .get(&(m.id(), m.away_team_id()))
            .copied()
            .unwrap_or(0);
        if m.home_score() != home_score || m.away_score() != away_score {
            println!(
                "Fixing match {}: {}-{} -> {}-{}",
                m.id(),
                m.home_score(),
                m.away_score(),
                home_score,
                away_score
            );
            let cmd = UpdateCommand::new("TournamentMatch", m.id())
                .value("home_score", home_score)
                .value("away_score", away_score)
                .value("version", m.version() + 1);
            match_data_service.update(&cmd).await?;
        }
    }
    let fixture_fix_count = fix_knockout_fixtures(&ctx, &teams, &matches).await?;
    println!("Fixed {} knockout fixture records.", fixture_fix_count);

    let mut fix_count = 0;

    for team in teams {
        let team_id = team.id();
        let mut played = 0;
        let mut won = 0;
        let mut drawn = 0;
        let mut lost = 0;
        let mut gf = 0;
        let mut ga = 0;

        for m in &matches {
            if !match_ids_with_goals.contains(&m.id()) {
                continue;
            }
            if !is_group_stage_match(m, &team_groups, &group_ids) {
                continue;
            }
            let hs = goal_counts
                .get(&(m.id(), m.home_team_id()))
                .copied()
                .unwrap_or(0);
            let as_sc = goal_counts
                .get(&(m.id(), m.away_team_id()))
                .copied()
                .unwrap_or(0);
            if m.home_team_id() == team_id {
                played += 1;
                gf += hs;
                ga += as_sc;
                if hs > as_sc {
                    won += 1;
                } else if hs == as_sc {
                    drawn += 1;
                } else {
                    lost += 1;
                }
            } else if m.away_team_id() == team_id {
                played += 1;
                gf += as_sc;
                ga += hs;
                if as_sc > hs {
                    won += 1;
                } else if as_sc == hs {
                    drawn += 1;
                } else {
                    lost += 1;
                }
            }
        }

        let standing = standings
            .iter_mut()
            .find(|s| s.tournament_team_id() == team_id);
        if let Some(s) = standing {
            let new_points = (won * 3) + drawn;
            let new_gd = gf - ga;
            if s.played() != played
                || s.won() != won
                || s.drawn() != drawn
                || s.lost() != lost
                || s.goals_for() != gf
                || s.goals_against() != ga
                || s.points() != new_points
                || s.goal_difference() != new_gd
            {
                println!(
                    "Fixing team {}: P:{} W:{} D:{} L:{} Pts:{}",
                    team.team_name(),
                    played,
                    won,
                    drawn,
                    lost,
                    new_points
                );
                s.update_played(played);
                s.update_won(won);
                s.update_drawn(drawn);
                s.update_lost(lost);
                s.update_goals_for(gf);
                s.update_goals_against(ga);
                s.update_goal_difference(new_gd);
                s.update_points(new_points);
                let data_service =
                    ctx.entity_data_service::<DataServiceExecutor>("GroupStanding")?;
                let cmd = UpdateCommand::new("GroupStanding", s.id())
                    .value("played", s.played())
                    .value("won", s.won())
                    .value("drawn", s.drawn())
                    .value("lost", s.lost())
                    .value("goals_for", s.goals_for())
                    .value("goals_against", s.goals_against())
                    .value("goal_difference", s.goal_difference())
                    .value("points", s.points())
                    .value("version", s.version() + 1);
                data_service.update(&cmd).await?;
                fix_count += 1;
            }
        }
    }

    println!("Done. Fixed {} standings.", fix_count);
    Ok(())
}

#[derive(Clone, Copy)]
struct KnockoutFixture {
    match_number: i32,
    stage_id: u64,
    home: &'static str,
    away: &'static str,
    home_score: Option<i32>,
    away_score: Option<i32>,
    penalty_home: i32,
    penalty_away: i32,
}

async fn fix_knockout_fixtures(
    ctx: &UserContext,
    teams: &[TournamentTeam],
    matches: &[TournamentMatch],
) -> Result<usize, Box<dyn Error>> {
    let team_ids: std::collections::HashMap<String, u64> = teams
        .iter()
        .map(|team| (team.team_name(), team.id()))
        .collect();
    let teams_by_id: std::collections::HashMap<u64, &TournamentTeam> =
        teams.iter().map(|team| (team.id(), team)).collect();
    let data_service = ctx.entity_data_service::<DataServiceExecutor>("TournamentMatch")?;
    let mut fix_count = 0;

    for fixture in reported_knockout_fixtures() {
        let Some(home_id) = team_ids.get(fixture.home).copied() else {
            println!("Skipping fixture: missing team {}", fixture.home);
            continue;
        };
        let Some(away_id) = team_ids.get(fixture.away).copied() else {
            println!("Skipping fixture: missing team {}", fixture.away);
            continue;
        };

        let existing_match = matches.iter().find(|m| {
            (m.home_team_id() == home_id && m.away_team_id() == away_id)
                || (m.home_team_id() == away_id && m.away_team_id() == home_id)
        });

        let Some(existing_match) = existing_match else {
            let Some(home_team) = teams_by_id.get(&home_id).copied() else {
                continue;
            };
            println!("Creating fixture: {} vs {}", fixture.home, fixture.away);
            create_knockout_match(ctx, fixture, home_id, away_id, home_team.tournament_id())
                .await?;
            fix_count += 1;
            continue;
        };

        let fixture_matches_existing_order = existing_match.home_team_id() == home_id;
        let (home_score, away_score, penalty_home, penalty_away) = if fixture_matches_existing_order
        {
            (
                fixture.home_score.unwrap_or(existing_match.home_score()),
                fixture.away_score.unwrap_or(existing_match.away_score()),
                fixture.penalty_home,
                fixture.penalty_away,
            )
        } else {
            (
                fixture.away_score.unwrap_or(existing_match.home_score()),
                fixture.home_score.unwrap_or(existing_match.away_score()),
                fixture.penalty_away,
                fixture.penalty_home,
            )
        };
        let match_status = if fixture.home_score.is_some() && fixture.away_score.is_some() {
            1003_u64
        } else {
            1001_u64
        };

        if existing_match.match_number() != fixture.match_number
            || existing_match.match_stage_id() != fixture.stage_id
            || existing_match.match_status_id() != match_status
            || existing_match.home_score() != home_score
            || existing_match.away_score() != away_score
            || existing_match.penalty_home() != penalty_home
            || existing_match.penalty_away() != penalty_away
        {
            println!(
                "Fixing fixture match {}: {} vs {} -> stage {} score {}-{} pens {}-{}",
                existing_match.id(),
                existing_match.home_team_id(),
                existing_match.away_team_id(),
                fixture.stage_id,
                home_score,
                away_score,
                penalty_home,
                penalty_away
            );
            let cmd = UpdateCommand::new("TournamentMatch", existing_match.id())
                .value("match_number", fixture.match_number)
                .value("match_stage_id", fixture.stage_id)
                .value("match_status_id", match_status)
                .value("home_score", home_score)
                .value("away_score", away_score)
                .value("penalty_home", penalty_home)
                .value("penalty_away", penalty_away)
                .value("version", existing_match.version() + 1);
            data_service.update(&cmd).await?;
            fix_count += 1;
        }
    }

    Ok(fix_count)
}

async fn create_knockout_match(
    ctx: &UserContext,
    fixture: KnockoutFixture,
    home_id: u64,
    away_id: u64,
    tournament_id: u64,
) -> Result<(), Box<dyn Error>> {
    let mut m = Q::tournament_matches().purpose("fix_data").new_entity(ctx);
    m.update_match_number(fixture.match_number);
    m.update_match_date("2026-07-07");
    m.update_venue_name("TBD");
    m.update_venue_city("TBD");
    m.update_venue_country("TBD");
    m.update_home_team_id(home_id);
    m.update_away_team_id(away_id);
    m.update_match_group_id(0_u64);
    m.update_tournament_id(tournament_id);
    m.update_home_score(fixture.home_score.unwrap_or(0));
    m.update_away_score(fixture.away_score.unwrap_or(0));
    m.update_extra_time_home(0);
    m.update_extra_time_away(0);
    m.update_penalty_home(fixture.penalty_home);
    m.update_penalty_away(fixture.penalty_away);
    match fixture.stage_id {
        1002 => {
            m.update_match_stage_to_round_of32();
        }
        1003 => {
            m.update_match_stage_to_round_of16();
        }
        _ => {
            m.update_match_stage_to_group();
        }
    };
    if fixture.home_score.is_some() && fixture.away_score.is_some() {
        m.update_match_status_to_finished();
    } else {
        m.update_match_status_to_scheduled();
    }
    m.audit_as("Create reported knockout fixture")
        .save(ctx)
        .await?;
    ctx.entity_root().clear_current_change_set();
    Ok(())
}

fn reported_knockout_fixtures() -> Vec<KnockoutFixture> {
    vec![
        KnockoutFixture {
            match_number: 73,
            stage_id: 1002,
            home: "Canada",
            away: "South Africa",
            home_score: Some(1),
            away_score: Some(0),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 74,
            stage_id: 1002,
            home: "Brazil",
            away: "Japan",
            home_score: Some(2),
            away_score: Some(1),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 75,
            stage_id: 1002,
            home: "Germany",
            away: "Paraguay",
            home_score: Some(1),
            away_score: Some(1),
            penalty_home: 3,
            penalty_away: 4,
        },
        KnockoutFixture {
            match_number: 76,
            stage_id: 1002,
            home: "Netherlands",
            away: "Morocco",
            home_score: Some(1),
            away_score: Some(1),
            penalty_home: 2,
            penalty_away: 3,
        },
        KnockoutFixture {
            match_number: 77,
            stage_id: 1002,
            home: "Norway",
            away: "Ivory Coast",
            home_score: Some(2),
            away_score: Some(1),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 78,
            stage_id: 1002,
            home: "France",
            away: "Sweden",
            home_score: Some(3),
            away_score: Some(0),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 79,
            stage_id: 1002,
            home: "Mexico",
            away: "Ecuador",
            home_score: Some(2),
            away_score: Some(0),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 80,
            stage_id: 1002,
            home: "England",
            away: "DR Congo",
            home_score: Some(2),
            away_score: Some(1),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 81,
            stage_id: 1002,
            home: "Belgium",
            away: "Senegal",
            home_score: Some(3),
            away_score: Some(2),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 82,
            stage_id: 1002,
            home: "United States",
            away: "Bosnia and Herzegovina",
            home_score: Some(2),
            away_score: Some(0),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 83,
            stage_id: 1002,
            home: "Spain",
            away: "Austria",
            home_score: Some(3),
            away_score: Some(0),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 84,
            stage_id: 1002,
            home: "Portugal",
            away: "Croatia",
            home_score: Some(2),
            away_score: Some(1),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 85,
            stage_id: 1002,
            home: "Switzerland",
            away: "Algeria",
            home_score: Some(2),
            away_score: Some(0),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 86,
            stage_id: 1002,
            home: "Egypt",
            away: "Australia",
            home_score: Some(1),
            away_score: Some(1),
            penalty_home: 4,
            penalty_away: 2,
        },
        KnockoutFixture {
            match_number: 87,
            stage_id: 1002,
            home: "Argentina",
            away: "Cape Verde",
            home_score: Some(3),
            away_score: Some(2),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 88,
            stage_id: 1002,
            home: "Colombia",
            away: "Ghana",
            home_score: Some(1),
            away_score: Some(0),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 89,
            stage_id: 1003,
            home: "France",
            away: "Paraguay",
            home_score: Some(1),
            away_score: Some(0),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 90,
            stage_id: 1003,
            home: "Canada",
            away: "Morocco",
            home_score: Some(0),
            away_score: Some(3),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 91,
            stage_id: 1003,
            home: "Portugal",
            away: "Spain",
            home_score: Some(0),
            away_score: Some(1),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 92,
            stage_id: 1003,
            home: "United States",
            away: "Belgium",
            home_score: Some(1),
            away_score: Some(4),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 93,
            stage_id: 1003,
            home: "Brazil",
            away: "Norway",
            home_score: Some(1),
            away_score: Some(2),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 94,
            stage_id: 1003,
            home: "Mexico",
            away: "England",
            home_score: Some(2),
            away_score: Some(3),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 95,
            stage_id: 1003,
            home: "Argentina",
            away: "Egypt",
            home_score: Some(3),
            away_score: Some(2),
            penalty_home: 0,
            penalty_away: 0,
        },
        KnockoutFixture {
            match_number: 96,
            stage_id: 1003,
            home: "Switzerland",
            away: "Colombia",
            home_score: None,
            away_score: None,
            penalty_home: 0,
            penalty_away: 0,
        },
    ]
}

fn is_group_stage_match(
    m: &TournamentMatch,
    team_groups: &std::collections::HashMap<u64, String>,
    group_ids: &std::collections::HashMap<String, u64>,
) -> bool {
    let Some(home_group) = team_groups.get(&m.home_team_id()) else {
        return false;
    };
    let Some(away_group) = team_groups.get(&m.away_team_id()) else {
        return false;
    };
    if home_group != away_group {
        return false;
    }

    group_ids
        .get(home_group)
        .map(|group_id| *group_id == m.match_group_id())
        .unwrap_or(false)
}
