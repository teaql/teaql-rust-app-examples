use teaql_core::Entity;
use teaql_runtime::UserContext;
use fifa_world_cup_2026_service::*;
use crate::ansi::AnsiColors;

pub async fn groups(ctx: &UserContext) -> Result<(), Box<dyn std::error::Error>> {
    let standings = Q::group_standings()
        .select_tournament_team_with(Q::tournament_teams().select_self())
        .select_match_group_with(Q::match_groups().select_self())
        .order_by_points_desc()
        .order_by_goal_difference_desc()
        .order_by_goals_for_desc()
        .purpose("cli").execute_for_list(ctx)
        .await?;

    if standings.data.is_empty() {
        println!("{}", AnsiColors::yellow("⚠ No group data found."));
        return Ok(());
    }

    println!("\n{}  ⚽ FIFA WORLD CUP 2026 — GROUP OVERVIEW{}\n", AnsiColors::BOLD, AnsiColors::RESET);
    
    use std::collections::BTreeMap;
    let mut groups_map: BTreeMap<String, Vec<&GroupStanding>> = BTreeMap::new();
    for s in &standings.data {
        let g = s.match_group().unwrap();
        let letter = g.group_letter().to_string();
        groups_map.entry(letter).or_default().push(s);
    }

    let letters: Vec<String> = groups_map.keys().cloned().collect();
    let col_width = 38;

    for chunk in letters.chunks(2) {
        let left_letter = &chunk[0];
        let right_letter = chunk.get(1);

        let left_teams = groups_map.get(left_letter).unwrap();
        let right_teams = right_letter.and_then(|l| groups_map.get(l));

        let left_header = format!("{}  GROUP {}{}", AnsiColors::BOLD, left_letter, AnsiColors::RESET);
        let right_header = right_letter.map(|r| format!("{}GROUP {}{}", AnsiColors::BOLD, r, AnsiColors::RESET)).unwrap_or_default();
        
        let header_padding = col_width - 9; // "  GROUP A" visible length is 9
        println!("{}{:>width$}  {}", left_header, "", right_header, width=header_padding);

        let max_rows = left_teams.len().max(right_teams.map(|r| r.len()).unwrap_or(0));
        for r in 0..max_rows {
            let left_line = left_teams.get(r).map(|s| format_team_line(s)).unwrap_or_else(|| String::new());
            let right_line = right_teams.and_then(|teams| teams.get(r)).map(|s| format_team_line(s)).unwrap_or_else(|| String::new());
            
            let padding = if left_line.is_empty() { col_width } else { col_width - 33 };
            println!(" {}{:width$} {}", left_line, "", right_line, width=padding);
        }
        println!();
    }
    
    Ok(())
}

fn format_team_line(s: &GroupStanding) -> String {
    let t = s.tournament_team().unwrap();
    let flag = t.emoji_flag();
    let name = t.team_name();
    let pts = s.points();
    
    let pts_str = format!("{} pts", pts);
    let name_str = format!("{} {}", flag, name);
    let name_visible = 2 + 1 + name.chars().count();
    
    let dots_needed = 30usize.saturating_sub(name_visible + pts_str.len()).max(2);
    let dots = format!("{} {} {}", AnsiColors::DIM, ".".repeat(dots_needed), AnsiColors::RESET);
    
    let colored_pts = if pts >= 6 {
        AnsiColors::green(&pts_str)
    } else if pts >= 3 {
        AnsiColors::yellow(&pts_str)
    } else if s.played() > 0 {
        AnsiColors::red(&pts_str)
    } else {
        format!("{}{}{}", AnsiColors::DIM, pts_str, AnsiColors::RESET)
    };
    
    format!(" {}{}{}", name_str, dots, colored_pts)
}


pub async fn group(ctx: &UserContext, letter: &str) -> Result<(), Box<dyn std::error::Error>> {
    let letter = letter.trim().to_uppercase();
    if letter.len() != 1 || letter.chars().next().unwrap() < 'A' || letter.chars().next().unwrap() > 'L' {
        println!("{}✗ Invalid group letter: {}{}", AnsiColors::RED, letter, AnsiColors::RESET);
        return Ok(());
    }

    let g_opt = Q::match_groups().with_group_letter_is(letter.as_str()).purpose("cli").execute_for_list(ctx).await?.data.pop();
    let Some(g) = g_opt else {
        println!("{}⚠ No standings found for Group {}{}", AnsiColors::YELLOW, letter, AnsiColors::RESET);
        return Ok(());
    };

    println!("\n{}  ════════════ GROUP {} ════════════{}\n", AnsiColors::CYAN, letter, AnsiColors::RESET);
    
    let standings = Q::group_standings()
        .select_tournament_team_with(Q::tournament_teams().select_self())
        .with_match_group_matching(Q::match_groups().with_id_is(g.id()))
        .order_by_points_desc()
        .order_by_goal_difference_desc()
        .order_by_goals_for_desc()
        .purpose("cli").execute_for_list(ctx)
        .await?;

    println!("{}  STANDINGS{}", AnsiColors::BOLD, AnsiColors::RESET);
    println!("{}  #    {:40}   P   W   D   L  GF  GA   GD Pts{}", AnsiColors::DIM, "Team", AnsiColors::RESET);
    
    for (i, s) in standings.data.iter().enumerate() {
        let t = s.tournament_team().unwrap();
        let flag = t.emoji_flag();
        let name = t.team_name();
        
        let name_visible = 2 + 1 + name.chars().count(); // flag(2) + space(1) + name
        let padding = 40usize.saturating_sub(name_visible);
        let padded_name = format!("{} {}{}", flag, AnsiColors::bold(&name), " ".repeat(padding));

        
        let gd = s.goal_difference();
        let gd_val = if gd > 0 { format!("+{}", gd) } else { gd.to_string() };
        let gd_padded = format!("{:>4}", gd_val);
        let gd_str = if gd > 0 { AnsiColors::green(&gd_padded) }
                     else if gd < 0 { AnsiColors::red(&gd_padded) }
                     else { gd_padded };
        
        let pts = s.points();
        let pts_padded = format!("{:>3}", pts);
        let pts_str = if pts >= 6 { AnsiColors::green(&pts_padded) }
                      else if pts >= 3 { AnsiColors::yellow(&pts_padded) }
                      else if s.played() > 0 { AnsiColors::red(&pts_padded) }
                      else { format!("{}{}{}", AnsiColors::DIM, pts_padded, AnsiColors::RESET) };
        
        println!(" {:2}    {} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3} {} {}",
            i + 1,
            padded_name,
            s.played(), s.won(), s.drawn(), s.lost(),
            s.goals_for(), s.goals_against(),
            gd_str, pts_str
        );
    }
    println!();
    
    println!("{}  RESULTS / UPCOMING (Feature omitted for brevity){}", AnsiColors::DIM, AnsiColors::RESET);
    println!();

    Ok(())
}

pub async fn rank(ctx: &UserContext, top: usize, group: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut req = Q::group_standings()
        .select_tournament_team_with(Q::tournament_teams().select_self())
        .select_match_group_with(Q::match_groups().select_self())
        .order_by_points_desc()
        .order_by_goal_difference_desc()
        .order_by_goals_for_desc();

    let mut title = "🏆 WORLD CUP 2026 RANKINGS".to_string();
    if let Some(ref g) = group {
        let grp_letter = g.trim().to_uppercase();
        title = format!("🏆 WORLD CUP 2026 RANKINGS — GROUP {}", grp_letter);
        let g_obj = Q::match_groups().with_group_letter_is(grp_letter.as_str()).purpose("cli").execute_for_list(ctx).await?.data.pop().unwrap();
        req = req.with_match_group_matching(Q::match_groups().with_id_is(g_obj.id()));
    }

    let standings = req.purpose("cli").execute_for_list(ctx).await?;

    println!("\n{}{}{}", AnsiColors::BOLD, AnsiColors::cyan(&title), AnsiColors::RESET);
    println!("{}══════════════════════════════════════════════════════════════════{}", AnsiColors::DIM, AnsiColors::RESET);
    println!("{}  #  Grp  {:37}   P   W   D   L  GF  GA   GD Pts{}", AnsiColors::BOLD, "Team", AnsiColors::RESET);

    for (i, s) in standings.data.into_iter().take(top).enumerate() {
        let t = s.tournament_team().unwrap();
        let grp = s.match_group().unwrap();
        let flag = t.emoji_flag();
        let name = t.team_name();
        let name_visible = 2 + 1 + name.chars().count();
        let padding = 37usize.saturating_sub(name_visible);
        let padded_name = format!("{} {}{}", flag, name, " ".repeat(padding));
        let group_letter = grp.group_letter();

        let gd = s.goal_difference();
        let gd_val = if gd > 0 { format!("+{}", gd) } else { gd.to_string() };
        let gd_padded = format!("{:>4}", gd_val);
        let gd_str = if gd > 0 { AnsiColors::green(&gd_padded) }
                     else if gd < 0 { AnsiColors::red(&gd_padded) }
                     else { gd_padded };

        let pts = s.points();
        let pts_padded = format!("{:>3}", pts);
        let pts_str = if pts >= 6 { AnsiColors::green(&pts_padded) }
                      else if pts >= 3 { AnsiColors::yellow(&pts_padded) }
                      else if s.played() > 0 { AnsiColors::red(&pts_padded) }
                      else { format!("{}{}{}", AnsiColors::DIM, pts_padded, AnsiColors::RESET) };

        println!(" {:2}   {}   {} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3} {} {}",
            i + 1, group_letter, padded_name,
            s.played(), s.won(), s.drawn(), s.lost(),
            s.goals_for(), s.goals_against(),
            gd_str, pts_str
        );
    }
    println!();
    Ok(())
}

pub async fn record(ctx: &UserContext, _match_number: Option<i32>, home: Option<String>, away: Option<String>, score: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = score.split('-').collect();
    if parts.len() != 2 {
        println!("{}❌ Invalid score format. Expected H-A (e.g. 3-1){}", AnsiColors::RED, AnsiColors::RESET);
        return Ok(());
    }
    let home_score: i32 = parts[0].trim().parse()?;
    let away_score: i32 = parts[1].trim().parse()?;

    if home.is_none() || away.is_none() {
        println!("{}❌ Please provide both --home and --away. Match number lookup is omitted.{}", AnsiColors::RED, AnsiColors::RESET);
        return Ok(());
    }

    let ht_code = home.unwrap().trim().to_uppercase();
    let at_code = away.unwrap().trim().to_uppercase();

    let ht_opt = Q::tournament_teams().with_team_code_is(ht_code.as_str()).purpose("cli").execute_for_list(ctx).await?.data.pop();
    let at_opt = Q::tournament_teams().with_team_code_is(at_code.as_str()).purpose("cli").execute_for_list(ctx).await?.data.pop();

    if ht_opt.is_none() { println!("{}❌ Unknown team code: {}{}", AnsiColors::RED, ht_code, AnsiColors::RESET); return Ok(()); }
    if at_opt.is_none() { println!("{}❌ Unknown team code: {}{}", AnsiColors::RED, at_code, AnsiColors::RESET); return Ok(()); }

    let ht = ht_opt.unwrap();
    let at = at_opt.unwrap();
    let group_letter = ht.group_letter();

    let t_match_opt = Q::tournament_matches()
        .with_home_team_matching(Q::tournament_teams().with_id_is(ht.id()))
        .with_away_team_matching(Q::tournament_teams().with_id_is(at.id()))
        .purpose("cli").execute_for_list(ctx).await?.data.pop();

    let mut t_match = if let Some(m) = t_match_opt {
        m
    } else {
        let mut m = Q::tournament_matches().purpose("record").new_entity(ctx);
        m.update_home_team_id(ht.id());
        m.update_away_team_id(at.id());
        let mg = Q::match_groups().with_group_letter_is(group_letter.as_str()).purpose("cli").execute_for_list(ctx).await?.data.pop().unwrap();
        m.update_match_group_id(mg.id());
        m.update_tournament_id(ht.tournament_id());
        m
    };

    t_match.update_home_score(home_score);
    t_match.update_away_score(away_score);
    t_match.audit_as("Record match score").save(ctx).await?;

    let mg = Q::match_groups().with_group_letter_is(group_letter.as_str()).purpose("cli").execute_for_list(ctx).await?.data.pop().unwrap();
    let mut standings = Q::group_standings()
        .with_match_group_matching(Q::match_groups().with_id_is(mg.id()))
        .purpose("cli").execute_for_list(ctx).await?;

    for standing in &mut standings.data {
        let team_id = standing.tournament_team_id();
        let matches = Q::tournament_matches().with_match_group_matching(Q::match_groups().with_id_is(mg.id())).purpose("cli").execute_for_list(ctx).await?;
        
        let mut played = 0; let mut won = 0; let mut drawn = 0; let mut lost = 0;
        let mut gf = 0; let mut ga = 0;

        for m in matches.data {
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
        
        standing.update_played(played);
        standing.update_won(won);
        standing.update_drawn(drawn);
        standing.update_lost(lost);
        standing.update_goals_for(gf);
        standing.update_goals_against(ga);
        standing.update_goal_difference(gf - ga);
        standing.update_points((won * 3) + drawn);
        standing.clone().audit_as("Recalculate standing").save(ctx).await?;
    }
    
    println!("{}✅ Match recorded: {} {} - {} {}{}", AnsiColors::GREEN, ht.team_name(), home_score, away_score, at.team_name(), AnsiColors::RESET);

    Ok(())
}
