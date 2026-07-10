use fifa_world_cup_2026_service::*;
use teaql_core::Entity;
use teaql_runtime::UserContext;

pub async fn seed_data(ctx: &UserContext) -> Result<(), Box<dyn std::error::Error>> {
    let standings = Q::group_standings()
        .comment("Check if initialized")
        .purpose("Seed data")
        .execute_for_list(ctx)
        .await?;

    if !standings.data.is_empty() {
        return Ok(());
    }

    let tournaments = Q::tournaments()
        .comment("Check tournament")
        .purpose("Seed data")
        .execute_for_list(ctx)
        .await?;
    let t_id = if tournaments.data.is_empty() {
        let mut t = Q::tournaments().purpose("seed").new_entity(ctx);
        t.update_tournament_name("FIFA World Cup 2026".to_string());
        t.update_host_countries("United States, Mexico, Canada".to_string());
        t.update_start_date(chrono::NaiveDate::from_ymd_opt(2026, 6, 11).unwrap());
        t.update_end_date(chrono::NaiveDate::from_ymd_opt(2026, 7, 19).unwrap());
        t.update_total_teams(48);
        let val = t.id();
        t.audit_as("Seed tournament").save(ctx).await?;
        val
    } else {
        tournaments.data[0].id()
    };

    let groups = Q::match_groups()
        .comment("Check match groups")
        .purpose("Seed data")
        .execute_for_list(ctx)
        .await?;
    if groups.data.is_empty() {
        for letter in 'A'..='L' {
            let mut g = Q::match_groups().purpose("seed").new_entity(ctx);
            g.update_group_letter(letter.to_string());
            g.update_tournament_id(t_id);
            g.audit_as("Seed match group").save(ctx).await?;
        }
    }

    struct TeamDef<'a>(&'a str, &'a str, &'a str, i32, &'a str, &'a str);
    let all_teams = vec![
        TeamDef("Mexico", "MEX", "🇲🇽", 15, "CONCACAF", "A"),
        TeamDef("South Africa", "RSA", "🇿🇦", 58, "CAF", "A"),
        TeamDef("South Korea", "KOR", "🇰🇷", 22, "AFC", "A"),
        TeamDef("Czech Republic", "CZE", "🇨🇿", 38, "UEFA", "A"),
        TeamDef("Canada", "CAN", "🇨🇦", 41, "CONCACAF", "B"),
        TeamDef("Bosnia and Herzegovina", "BIH", "🇧🇦", 63, "UEFA", "B"),
        TeamDef("Qatar", "QAT", "🇶🇦", 35, "AFC", "B"),
        TeamDef("Switzerland", "SUI", "🇨🇭", 17, "UEFA", "B"),
        TeamDef("Brazil", "BRA", "🇧🇷", 3, "CONMEBOL", "C"),
        TeamDef("Morocco", "MAR", "🇲🇦", 14, "CAF", "C"),
        TeamDef("Haiti", "HAI", "🇭🇹", 89, "CONCACAF", "C"),
        TeamDef("Scotland", "SCO", "🏴󠁧󠁢󠁳󠁣󠁴󠁿", 55, "UEFA", "C"),
        TeamDef("United States", "USA", "🇺🇸", 11, "CONCACAF", "D"),
        TeamDef("Paraguay", "PAR", "🇵🇾", 57, "CONMEBOL", "D"),
        TeamDef("Australia", "AUS", "🇦🇺", 24, "AFC", "D"),
        TeamDef("Türkiye", "TUR", "🇹🇷", 42, "UEFA", "D"),
        TeamDef("Germany", "GER", "🇩🇪", 2, "UEFA", "E"),
        TeamDef("Curaçao", "CUW", "🇨🇼", 85, "CONCACAF", "E"),
        TeamDef("Ivory Coast", "CIV", "🇨🇮", 49, "CAF", "E"),
        TeamDef("Ecuador", "ECU", "🇪🇨", 31, "CONMEBOL", "E"),
        TeamDef("Netherlands", "NED", "🇳🇱", 6, "UEFA", "F"),
        TeamDef("Japan", "JPN", "🇯🇵", 13, "AFC", "F"),
        TeamDef("Sweden", "SWE", "🇸🇪", 37, "UEFA", "F"),
        TeamDef("Tunisia", "TUN", "🇹🇳", 39, "CAF", "F"),
        TeamDef("Belgium", "BEL", "🇧🇪", 5, "UEFA", "G"),
        TeamDef("Egypt", "EGY", "🇪🇬", 33, "CAF", "G"),
        TeamDef("Iran", "IRN", "🇮🇷", 20, "AFC", "G"),
        TeamDef("New Zealand", "NZL", "🇳🇿", 93, "OFC", "G"),
        TeamDef("Spain", "ESP", "🇪🇸", 1, "UEFA", "H"),
        TeamDef("Cape Verde", "CPV", "🇨🇻", 74, "CAF", "H"),
        TeamDef("Saudi Arabia", "KSA", "🇸🇦", 56, "AFC", "H"),
        TeamDef("Uruguay", "URU", "🇺🇾", 9, "CONMEBOL", "H"),
        TeamDef("France", "FRA", "🇫🇷", 7, "UEFA", "I"),
        TeamDef("Senegal", "SEN", "🇸🇳", 18, "CAF", "I"),
        TeamDef("Iraq", "IRQ", "🇮🇶", 55, "AFC", "I"),
        TeamDef("Norway", "NOR", "🇳🇴", 46, "UEFA", "I"),
        TeamDef("Argentina", "ARG", "🇦🇷", 8, "CONMEBOL", "J"),
        TeamDef("Algeria", "ALG", "🇩🇿", 36, "CAF", "J"),
        TeamDef("Austria", "AUT", "🇦🇹", 27, "UEFA", "J"),
        TeamDef("Jordan", "JOR", "🇯🇴", 68, "AFC", "J"),
        TeamDef("Portugal", "POR", "🇵🇹", 10, "UEFA", "K"),
        TeamDef("DR Congo", "COD", "🇨🇩", 55, "CAF", "K"),
        TeamDef("Uzbekistan", "UZB", "🇺🇿", 62, "AFC", "K"),
        TeamDef("Colombia", "COL", "🇨🇴", 12, "CONMEBOL", "K"),
        TeamDef("England", "ENG", "🏴󠁧󠁢󠁥󠁮󠁧󠁿", 4, "UEFA", "L"),
        TeamDef("Croatia", "CRO", "🇭🇷", 16, "UEFA", "L"),
        TeamDef("Ghana", "GHA", "🇬🇭", 65, "CAF", "L"),
        TeamDef("Panama", "PAN", "🇵🇦", 47, "CONCACAF", "L"),
    ];

    for def in all_teams {
        let g = Q::match_groups()
            .with_group_letter_is(def.5)
            .purpose("seed")
            .execute_for_list(ctx)
            .await?
            .data
            .pop()
            .unwrap();

        let mut team = Q::tournament_teams().purpose("seed").new_entity(ctx);
        team.update_team_name(def.0.to_string());
        team.update_team_code(def.1.to_string());
        team.update_emoji_flag(def.2.to_string());
        team.update_fifa_ranking(def.3);
        team.update_group_letter(def.5.to_string());
        team.update_tournament_id(t_id as i64);

        match def.4 {
            "AFC" => {
                team.update_confederation_to_afc();
            }
            "CAF" => {
                team.update_confederation_to_caf();
            }
            "CONCACAF" => {
                team.update_confederation_to_concacaf();
            }
            "CONMEBOL" => {
                team.update_confederation_to_conmebol();
            }
            "OFC" => {
                team.update_confederation_to_ofc();
            }
            "UEFA" => {
                team.update_confederation_to_uefa();
            }
            _ => {}
        }
        team.audit_as("Seed tournament team").save(ctx).await?;
        let saved_team = Q::tournament_teams()
            .with_team_name_is(def.0)
            .purpose("seed")
            .execute_for_list(ctx)
            .await?
            .data
            .pop()
            .unwrap();
        let team_id = saved_team.id();

        let mut standing = Q::group_standings().purpose("seed").new_entity(ctx);
        standing.update_tournament_team_id(team_id);
        standing.update_match_group_id(g.id());
        standing.update_tournament_id(t_id);
        standing.update_played(0);
        standing.update_won(0);
        standing.update_drawn(0);
        standing.update_lost(0);
        standing.update_goals_for(0);
        standing.update_goals_against(0);
        standing.update_goal_difference(0);
        standing.update_points(0);
        standing.audit_as("Seed group standing").save(ctx).await?;
    }

    // --- Generate knockout bracket ---
    // Fetch all teams sorted by FIFA ranking (ascending = best first)
    let mut ranked_teams = Q::tournament_teams()
        .order_by_fifa_ranking_asc()
        .purpose("seed knockout")
        .execute_for_list(ctx)
        .await?
        .data;

    // Take top 32 for the knockout bracket
    ranked_teams.truncate(32);

    // Build a lookup from group_letter -> group id
    let all_groups = Q::match_groups()
        .purpose("seed knockout")
        .execute_for_list(ctx)
        .await?
        .data;
    let group_id_by_letter: std::collections::HashMap<String, u64> = all_groups
        .iter()
        .map(|g| (g.group_letter(), g.id()))
        .collect();

    // Create Round of 32 matches: seed 1 vs seed 32, seed 2 vs seed 31, ...
    let num_r32 = 16;
    for i in 0..num_r32 {
        let home = &ranked_teams[i];
        let away = &ranked_teams[31 - i];

        let home_group_id = group_id_by_letter
            .get(&home.group_letter())
            .copied()
            .unwrap_or(0);

        let match_number = 73 + i as i32;

        let mut m = Q::tournament_matches().purpose("seed").new_entity(ctx);
        m.update_match_number(match_number);
        m.update_match_date("TBD");
        m.update_venue_name("TBD");
        m.update_venue_city("TBD");
        m.update_venue_country("TBD");
        m.update_home_team_id(home.id());
        m.update_away_team_id(away.id());
        m.update_home_score(0);
        m.update_away_score(0);
        m.update_extra_time_home(0);
        m.update_extra_time_away(0);
        m.update_penalty_home(0);
        m.update_penalty_away(0);
        m.update_match_group_id(home_group_id);
        m.update_tournament_id(t_id);
        m.update_match_stage_to_round_of32();
        m.update_match_status_to_scheduled();
        m.audit_as("Seed R32 knockout match").save(ctx).await?;
    }

    // Helper: create placeholder shell matches for later rounds
    // (home_team_id=0, away_team_id=0, scores=0)
    struct ShellRound {
        count: i32,
        match_number_start: i32,
        stage: &'static str,
    }

    let shell_rounds = vec![
        ShellRound { count: 8, match_number_start: 89, stage: "R16" },
        ShellRound { count: 4, match_number_start: 97, stage: "QF" },
        ShellRound { count: 2, match_number_start: 101, stage: "SF" },
        ShellRound { count: 1, match_number_start: 103, stage: "3RD" },
        ShellRound { count: 1, match_number_start: 104, stage: "FINAL" },
    ];

    for round in &shell_rounds {
        for j in 0..round.count {
            let match_number = round.match_number_start + j;

            let mut m = Q::tournament_matches().purpose("seed").new_entity(ctx);
            m.update_match_number(match_number);
            m.update_match_date("TBD");
            m.update_venue_name("TBD");
            m.update_venue_city("TBD");
            m.update_venue_country("TBD");
            m.update_home_team_id(0_u64);
            m.update_away_team_id(0_u64);
            m.update_home_score(0);
            m.update_away_score(0);
            m.update_extra_time_home(0);
            m.update_extra_time_away(0);
            m.update_penalty_home(0);
            m.update_penalty_away(0);
            m.update_match_group_id(0_u64);
            m.update_tournament_id(t_id);

            match round.stage {
                "R16" => { m.update_match_stage_to_round_of16(); }
                "QF" => { m.update_match_stage_to_quarter_final(); }
                "SF" => { m.update_match_stage_to_semi_final(); }
                "3RD" => { m.update_match_stage_to_third_place(); }
                "FINAL" => { m.update_match_stage_to_final(); }
                _ => {}
            }
            m.update_match_status_to_scheduled();
            m.audit_as("Seed knockout shell match").save(ctx).await?;
        }
    }

    Ok(())
}
