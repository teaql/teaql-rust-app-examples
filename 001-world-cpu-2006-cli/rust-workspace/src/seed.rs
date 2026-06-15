use teaql_core::Entity;
use teaql_runtime::UserContext;
use fifa_world_cup_2026_service::*;

pub async fn seed_data(ctx: &UserContext) -> Result<(), Box<dyn std::error::Error>> {
    let standings = Q::group_standings()
        .comment("Check if initialized")
        .purpose("Seed data")
        .execute_for_list(ctx)
        .await?;

    if !standings.data.is_empty() {
        return Ok(());
    }

    let tournaments = Q::tournaments().comment("Check tournament").purpose("Seed data").execute_for_list(ctx).await?;
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

    let groups = Q::match_groups().comment("Check match groups").purpose("Seed data").execute_for_list(ctx).await?;
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

        TeamDef("Mexico",        "MEX", "🇲🇽", 15, "CONCACAF", "A"),
        TeamDef("South Africa",  "RSA", "🇿🇦", 58, "CAF",      "A"),
        TeamDef("South Korea",   "KOR", "🇰🇷", 22, "AFC",      "A"),
        TeamDef("Denmark",       "DEN", "🇩🇰", 21, "UEFA",     "A"),

        TeamDef("Canada",        "CAN", "🇨🇦", 41, "CONCACAF", "B"),
        TeamDef("Qatar",         "QAT", "🇶🇦", 35, "AFC",      "B"),
        TeamDef("Switzerland",   "SUI", "🇨🇭", 17, "UEFA",     "B"),
        TeamDef("Italy",         "ITA", "🇮🇹",  4, "UEFA",     "B"),

        TeamDef("Brazil",        "BRA", "🇧🇷",  3, "CONMEBOL", "C"),
        TeamDef("Morocco",       "MAR", "🇲🇦", 14, "CAF",      "C"),
        TeamDef("Haiti",         "HAI", "🇭🇹", 89, "CONCACAF", "C"),
        TeamDef("Scotland",      "SCO", "🇬🇧", 55, "UEFA",     "C"),

        TeamDef("United States", "USA", "🇺🇸", 11, "CONCACAF", "D"),
        TeamDef("Paraguay",      "PAR", "🇵🇾", 57, "CONMEBOL", "D"),
        TeamDef("Australia",     "AUS", "🇦🇺", 24, "AFC",      "D"),
        TeamDef("Türkiye",       "TUR", "🇹🇷", 42, "UEFA",     "D"),

        TeamDef("Germany",       "GER", "🇩🇪",  2, "UEFA",     "E"),
        TeamDef("Curaçao",       "CUW", "🇨🇼", 85, "CONCACAF", "E"),
        TeamDef("Ivory Coast",   "CIV", "🇨🇮", 49, "CAF",      "E"),
        TeamDef("Ecuador",       "ECU", "🇪🇨", 31, "CONMEBOL", "E"),

        TeamDef("Netherlands",   "NED", "🇳🇱",  6, "UEFA",     "F"),
        TeamDef("Japan",         "JPN", "🇯🇵", 13, "AFC",      "F"),
        TeamDef("Tunisia",       "TUN", "🇹🇳", 39, "CAF",      "F"),
        TeamDef("Ukraine",       "UKR", "🇺🇦", 23, "UEFA",     "F"),

        TeamDef("Belgium",       "BEL", "🇧🇪",  5, "UEFA",     "G"),
        TeamDef("Egypt",         "EGY", "🇪🇬", 33, "CAF",      "G"),
        TeamDef("Iran",          "IRN", "🇮🇷", 20, "AFC",      "G"),
        TeamDef("New Zealand",   "NZL", "🇳🇿", 93, "OFC",      "G"),

        TeamDef("Spain",         "ESP", "🇪🇸",  1, "UEFA",     "H"),
        TeamDef("Cape Verde",    "CPV", "🇨🇻", 74, "CAF",      "H"),
        TeamDef("Saudi Arabia",  "KSA", "🇸🇦", 56, "AFC",      "H"),
        TeamDef("Uruguay",       "URU", "🇺🇾",  9, "CONMEBOL", "H"),

        TeamDef("France",        "FRA", "🇫🇷",  7, "UEFA",     "I"),
        TeamDef("Senegal",       "SEN", "🇸🇳", 18, "CAF",      "I"),
        TeamDef("Norway",        "NOR", "🇳🇴", 46, "UEFA",     "I"),
        TeamDef("UAE",           "UAE", "🇦🇪", 69, "AFC",      "I"),

        TeamDef("Argentina",     "ARG", "🇦🇷",  8, "CONMEBOL", "J"),
        TeamDef("Algeria",       "ALG", "🇩🇿", 36, "CAF",      "J"),
        TeamDef("Austria",       "AUT", "🇦🇹", 27, "UEFA",     "J"),
        TeamDef("Jordan",        "JOR", "🇯🇴", 68, "AFC",      "J"),

        TeamDef("Portugal",      "POR", "🇵🇹", 10, "UEFA",     "K"),
        TeamDef("Uzbekistan",    "UZB", "🇺🇿", 62, "AFC",      "K"),
        TeamDef("Colombia",      "COL", "🇨🇴", 12, "CONMEBOL", "K"),
        TeamDef("Chile",         "CHI", "🇨🇱", 40, "CONMEBOL", "K"),

        TeamDef("England",       "ENG", "🇬🇧",  4, "UEFA",     "L"),
        TeamDef("Croatia",       "CRO", "🇭🇷", 16, "UEFA",     "L"),
        TeamDef("Ghana",         "GHA", "🇬🇭", 65, "CAF",      "L"),
        TeamDef("Panama",        "PAN", "🇵🇦", 47, "CONCACAF", "L")
    
    ];

    for def in all_teams {
        let g = Q::match_groups().with_group_letter_is(def.5).purpose("seed").execute_for_list(ctx).await?.data.pop().unwrap();
        
        let mut team = Q::tournament_teams().purpose("seed").new_entity(ctx);
        team.update_team_name(def.0.to_string());
        team.update_team_code(def.1.to_string());
        team.update_emoji_flag(def.2.to_string());
        team.update_fifa_ranking(def.3);
        team.update_group_letter(def.5.to_string());
        team.update_tournament_id(t_id as i64);
        
        match def.4 {
            "AFC" => { team.update_confederation_to_afc(); },
            "CAF" => { team.update_confederation_to_caf(); },
            "CONCACAF" => { team.update_confederation_to_concacaf(); },
            "CONMEBOL" => { team.update_confederation_to_conmebol(); },
            "OFC" => { team.update_confederation_to_ofc(); },
            "UEFA" => { team.update_confederation_to_uefa(); },
            _ => {}
        }
        team.audit_as("Seed tournament team").save(ctx).await?;
        let saved_team = Q::tournament_teams().with_team_name_is(def.0).purpose("seed").execute_for_list(ctx).await?.data.pop().unwrap();
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
    Ok(())
}
