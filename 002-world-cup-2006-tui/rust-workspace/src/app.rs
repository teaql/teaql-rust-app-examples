use fifa_world_cup_2026_service::*;
use teaql_runtime::UserContext;
use teaql_core::Entity;
use ratatui::widgets::{TableState, ListState};

pub trait UserContextHttpExt {
    fn http(&self) -> HttpBuilder;
}

impl UserContextHttpExt for UserContext {
    fn http(&self) -> HttpBuilder {
        HttpBuilder {
            purpose: None,
        }
    }
}

pub struct HttpBuilder {
    purpose: Option<String>,
}

impl HttpBuilder {
    pub fn purpose(mut self, purpose: &str) -> Self {
        self.purpose = Some(purpose.to_string());
        self
    }

    pub async fn get(self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        // In a real TeaQL plugin, this would write to the DB audit log.
        let client = reqwest::Client::new();
        let text = client.get(url).send().await?.text().await?;
        Ok(text)
    }
}

#[derive(Debug, Clone)]
pub enum View {
    Global,
    Group(String),
    Players,
    Logs,
}

pub struct App {
    pub view: View,
    pub input_buffer: String,
    pub logs: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    pub should_quit: bool,
    pub ctx: UserContext,
    
    // Cached data
    pub global_standings: Vec<GroupStanding>,
    pub top_players: Vec<(String, String, i32)>, // (Team, Player, Goals)
    pub recent_matches: Vec<TournamentMatch>,
    
    pub group_standings: Vec<GroupStanding>,
    pub group_players: Vec<(String, String, i32)>,
    pub group_matches: Vec<TournamentMatch>,
    
    pub all_players: Vec<(String, String, i32, Vec<String>)>, // Team, Player, Goals, Matches
    
    pub global_table_state: TableState,
    pub player_table_state: TableState,
    pub players_state: TableState,
    pub matches_state: TableState,
    pub logs_state: ListState,
    pub active_pane: usize, // 0: Standings, 1: Players, 2: Matches
}

impl App {
    pub fn new(ctx: UserContext, logs: std::sync::Arc<std::sync::Mutex<Vec<String>>>) -> Self {
        Self {
            view: View::Global,
            input_buffer: String::new(),
            logs,
            should_quit: false,
            ctx,
            global_standings: vec![],
            top_players: vec![],
            recent_matches: vec![],
            group_standings: vec![],
            group_players: vec![],
            group_matches: vec![],
            all_players: vec![],
            global_table_state: TableState::default(),
            player_table_state: TableState::default(),
            players_state: TableState::default(),
            matches_state: TableState::default(),
            logs_state: ListState::default(),
            active_pane: 0,
        }
    }

    pub fn log(&mut self, msg: &str) {
        let ts = chrono::Local::now().format("%H:%M:%S").to_string();
        if let Ok(mut logs) = self.logs.lock() {
            logs.push(format!("[{}] {}", ts, msg));
            if logs.len() > 500 {
                logs.remove(0);
            }
        }
    }

    pub async fn fetch_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.recalculate_all_standings().await?;
        match &self.view {
            View::Global => {
                self.global_standings = Q::group_standings()
                    .select_tournament_team_with(Q::tournament_teams().select_self())
                    .select_match_group_with(Q::match_groups().select_self())
                    .comment("Fetch global standings").purpose("Render global standings dashboard").execute_for_list(&self.ctx).await?.data;
                
                Self::sort_standings(&mut self.global_standings);
                
                let goals = Q::match_goals()
                    .select_tournament_team_with(Q::tournament_teams().select_self())
                    .comment("Fetch all match goals").purpose("Render global top players dashboard").execute_for_list(&self.ctx).await?.data;
                
                let mut players_map = std::collections::HashMap::new();
                for g in goals {
                    let team = g.tournament_team().map(|t| format!("{} {}", t.emoji_flag(), t.team_name())).unwrap_or_default();
                    let player = g.player_name().to_string();
                    *players_map.entry((team, player)).or_insert(0) += 1;
                }
                let mut tp: Vec<_> = players_map.into_iter().map(|((t, p), c)| (t, p, c)).collect();
                tp.sort_by_key(|b| std::cmp::Reverse(b.2));
                self.top_players = tp;

                self.recent_matches = Q::tournament_matches()
                    .select_home_team_with(Q::tournament_teams().select_self())
                    .select_away_team_with(Q::tournament_teams().select_self())
                    .order_by_id_desc()
                    .comment("Fetch recent matches").purpose("Render global recent matches dashboard").execute_for_list(&self.ctx).await?.data;
                self.recent_matches.dedup_by_key(|m| m.id());
            }
            View::Group(g_letter) => {
                let g_opt = Q::match_groups().with_group_letter_is(g_letter.as_str()).comment("Find group by letter").purpose("Render group dashboard").execute_for_list(&self.ctx).await?.data.pop();
                if let Some(g) = g_opt {
                    self.group_standings = Q::group_standings()
                        .select_tournament_team_with(Q::tournament_teams().select_self())
                        .select_match_group_with(Q::match_groups().select_self())
                        .with_match_group_matching(Q::match_groups().with_id_is(g.id()))
                        .comment("Fetch group standings").purpose("Render group standings dashboard").execute_for_list(&self.ctx).await?.data;

                    Self::sort_standings(&mut self.group_standings);

                    let goals = Q::match_goals()
                        .select_tournament_team_with(Q::tournament_teams().with_group_letter_is(g_letter.as_str()))
                        .comment("Fetch group goals").purpose("Render group top players dashboard").execute_for_list(&self.ctx).await?.data;
                    
                    let mut players_map = std::collections::HashMap::new();
                    for goal in goals {
                        if let Some(team) = goal.tournament_team() {
                            let t_name = format!("{} {}", team.emoji_flag(), team.team_name());
                            let p_name = goal.player_name().to_string();
                            *players_map.entry((t_name, p_name)).or_insert(0) += 1;
                        }
                    }
                    let mut tp: Vec<_> = players_map.into_iter().map(|((t, p), c)| (t, p, c)).collect();
                    tp.sort_by_key(|b| std::cmp::Reverse(b.2));
                    self.group_players = tp;

                    self.group_matches = Q::tournament_matches()
                        .with_match_group_matching(Q::match_groups().with_id_is(g.id()))
                        .select_home_team_with(Q::tournament_teams().select_self())
                        .select_away_team_with(Q::tournament_teams().select_self())
                        .order_by_id_desc()
                        .comment("Fetch group matches").purpose("Render group matches dashboard").execute_for_list(&self.ctx).await?.data;
                    self.group_matches.dedup_by_key(|m| m.id());
                }
            }
            View::Players => {
                let goals = Q::match_goals()
                    .select_tournament_team_with(Q::tournament_teams().select_self())
                    .select_tournament_match_with(Q::tournament_matches().select_home_team_with(Q::tournament_teams().select_self()).select_away_team_with(Q::tournament_teams().select_self()))
                    .comment("Fetch goals for player/team").purpose("Render player/team dashboard").execute_for_list(&self.ctx).await?.data;
                
                let mut players_map = std::collections::HashMap::new();
                let mut matches_map = std::collections::HashMap::new();
                for g in goals {
                    let team = g.tournament_team().map(|t| format!("{} {}", t.emoji_flag(), t.team_name())).unwrap_or_default();
                    let player = g.player_name().to_string();
                    let key = (team, player);
                    *players_map.entry(key.clone()).or_insert(0) += 1;
                    if let Some(m) = g.tournament_match() {
                        let home_flag = m.home_team().map(|t| t.emoji_flag().to_string()).unwrap_or_default();
                        let home_name = m.home_team().map(|t| t.team_name().to_string()).unwrap_or_default();
                        let away_flag = m.away_team().map(|t| t.emoji_flag().to_string()).unwrap_or_default();
                        let away_name = m.away_team().map(|t| t.team_name().to_string()).unwrap_or_default();
                        let match_str = format!("{} {}  {:>14} vs {:<14}", home_flag, away_flag, home_name, away_name);
                        matches_map.entry(key).or_insert_with(Vec::new).push(match_str);
                    }
                }
                let mut all: Vec<_> = players_map.into_iter().map(|(k, count)| {
                    let mut ms = matches_map.remove(&k).unwrap_or_default();
                    ms.sort();
                    ms.dedup();
                    (k.0, k.1, count, ms)
                }).collect();
                all.sort_by_key(|b| std::cmp::Reverse(b.2));
                self.all_players = all;
            }
            View::Logs => {}
        }
        Ok(())
    }

    pub async fn process_command(&mut self) {
        let cmd = self.input_buffer.trim().to_string();
        if cmd.is_empty() { return; }
        
        self.log(&format!("> {}", cmd));

        if cmd.eq_ignore_ascii_case("quit") || cmd.eq_ignore_ascii_case("exit") {
            self.should_quit = true;
            return;
        }

        if cmd.eq_ignore_ascii_case("global") {
            self.view = View::Global;
            self.log("Switched to Global View");
        } else if cmd.starts_with("group ") {
            let letter = cmd.trim_start_matches("group ").trim().to_uppercase();
            self.view = View::Group(letter.clone());
            self.log(&format!("Switched to Group {} View", letter));
        } else if cmd == "players" {
            self.view = View::Players;
            self.log("Switched to Players View");
        } else if cmd == "logs" {
            self.view = View::Logs;
            self.log("Switched to Logs View");
        } else if cmd.eq_ignore_ascii_case("sync live") {
            self.log("Fetching live events from the internet...");
            if let Err(e) = self.sync_live_events().await {
                self.log(&format!("Sync error: {}", e));
            } else {
                self.log("Live events synced successfully.");
            }
        } else {
            self.log("Unknown command. Try: global, group A, team BRA, player Vinicius, sync live, quit");
        }

        self.input_buffer.clear();
        let _ = self.fetch_data().await;
    }

    async fn sync_live_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        #[derive(serde::Deserialize)]
        struct GoalJson {
            name: String,
        }
        
        #[derive(serde::Deserialize)]
        struct MatchJson {
            team1: String,
            team2: String,
            goals1: Option<Vec<GoalJson>>,
            goals2: Option<Vec<GoalJson>>,
        }
        
        #[derive(serde::Deserialize)]
        struct WorldCupJson {
            matches: Vec<MatchJson>,
        }

        let text = self.ctx.http().purpose("Sync live events").get("https://raw.githubusercontent.com/openfootball/worldcup.json/master/2026/worldcup.json").await?;
        let wc_json: WorldCupJson = serde_json::from_str(&text)?;

        let mut goals = Vec::new();
        for m in wc_json.matches {
            if let Some(g1) = m.goals1 {
                for g in g1 {
                    goals.push((m.team1.clone(), g.name.clone(), m.team2.clone()));
                }
            }
            if let Some(g2) = m.goals2 {
                for g in g2 {
                    goals.push((m.team2.clone(), g.name.clone(), m.team1.clone()));
                }
            }
        }

        let mut current_counts = std::collections::HashMap::new();
        let all_goals = Q::match_goals().purpose("sync").execute_for_list(&self.ctx).await?.data;
        for g in all_goals {
            *current_counts.entry(g.player_name().to_string()).or_insert(0) += 1;
        }

        let mut target_counts = std::collections::HashMap::new();
        let mut team_map = std::collections::HashMap::new();
        let mut opp_map = std::collections::HashMap::new();
        for (t, p, o) in &goals {
            *target_counts.entry(p.to_string()).or_insert(0) += 1;
            team_map.insert(p.to_string(), t.to_string());
            opp_map.insert(p.to_string(), o.to_string());
        }

        for (player, target) in target_counts {
            let current = current_counts.get(&player).copied().unwrap_or(0);
            if current < target {
                let diff = target - current;
                let team = team_map.get(&player).unwrap();
                let opp = opp_map.get(&player).unwrap();
                for _ in 0..diff {
                    if let Err(e) = self.record_goal(team, &player, opp).await {
                        self.log(&format!("Failed to record goal for {}: {}", player, e));
                    }
                }
            }
        }

        Ok(())
    }

    async fn record_goal(&mut self, team_str: &str, player_name: &str, opponent_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let team_code = team_str.to_uppercase();
        let mut teams = Q::tournament_teams().with_team_code_is(team_code.as_str()).comment("Find team by code").purpose("Record live goal event").execute_for_list(&self.ctx).await?.data;
        if teams.is_empty() {
            teams = Q::tournament_teams().with_team_name_is(team_str).comment("Find team by name").purpose("Record live goal event").execute_for_list(&self.ctx).await?.data;
        }
        
        let team = if let Some(t) = teams.pop() {
            t
        } else {
            return Err(format!("Team '{}' not found", team_str).into());
        };

        let opp_code = opponent_str.to_uppercase();
        let mut opps = Q::tournament_teams().with_team_code_is(opp_code.as_str()).comment("Find opponent team by code").purpose("Record live goal event").execute_for_list(&self.ctx).await?.data;
        if opps.is_empty() {
            opps = Q::tournament_teams().with_team_name_is(opponent_str).comment("Find opponent team by name").purpose("Record live goal event").execute_for_list(&self.ctx).await?.data;
        }
        
        let opponent = if let Some(t) = opps.pop() {
            t
        } else {
            return Err(format!("Opponent '{}' not found", opponent_str).into());
        };

        let group_letter = team.group_letter();
        let mg = Q::match_groups().with_group_letter_is(group_letter.clone()).comment("Find match group for team").purpose("Record live goal event").execute_for_list(&self.ctx).await?.data.pop().unwrap();
        
        let mut match_opt = Q::tournament_matches()
            .with_home_team_matching(Q::tournament_teams().with_id_is(team.id()))
            .with_away_team_matching(Q::tournament_teams().with_id_is(opponent.id()))
            .comment("Find match by home/away teams").purpose("Record live goal event").execute_for_list(&self.ctx).await?.data.into_iter().next();
            
        if match_opt.is_none() {
            match_opt = Q::tournament_matches()
                .with_home_team_matching(Q::tournament_teams().with_id_is(opponent.id()))
                .with_away_team_matching(Q::tournament_teams().with_id_is(team.id()))
                .comment("Find match by away/home teams").purpose("Record live goal event").execute_for_list(&self.ctx).await?.data.into_iter().next();
        }

        let mut t_match = if let Some(m) = match_opt {
            m
        } else {
            let mut m = Q::tournament_matches().purpose("record").new_entity(&self.ctx);
            m.update_home_team_id(team.id());
            m.update_away_team_id(opponent.id());
            m.update_match_group_id(mg.id());
            m.update_tournament_id(team.tournament_id());
            m.update_home_score(0);
            m.update_away_score(0);
            m.clone().audit_as("Create match").save(&self.ctx).await?;
            m = Q::tournament_matches()
                .with_home_team_matching(Q::tournament_teams().with_id_is(m.home_team_id()))
                .with_away_team_matching(Q::tournament_teams().with_id_is(m.away_team_id()))
                .comment("Find Finished status").purpose("Record live goal event").execute_for_list(&self.ctx).await?.data.pop().unwrap();
            m
        };

        let mut goal = Q::match_goals().purpose("record").new_entity(&self.ctx);
        goal.update_player_name(player_name.to_string());
        goal.update_tournament_match_id(t_match.id());
        goal.update_tournament_team_id(team.id());
        goal.update_tournament_id(team.tournament_id());
        goal.update_minute_scored(1);
        goal.audit_as("Record goal").save(&self.ctx).await?;

        let old_hs = t_match.home_score();
        let old_as = t_match.away_score();
        if t_match.home_team_id() == team.id() {
            t_match.update_home_score(old_hs + 1);
        } else {
            t_match.update_away_score(old_as + 1);
        }
        let new_hs = t_match.home_score();
        let new_as = t_match.away_score();
        t_match.audit_as(&format!("Update match score: {} vs {} ({} - {} -> {} - {})", team.team_name(), opponent.team_name(), old_hs, old_as, new_hs, new_as)).save(&self.ctx).await?;

        self.recalculate_all_standings().await?;

        Ok(())
    }

    async fn recalculate_all_standings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let teams = Q::tournament_teams().comment("Fetch all teams").purpose("Recalculate all standings").execute_for_list(&self.ctx).await?.data;
        let matches = Q::tournament_matches().comment("Fetch all matches").purpose("Recalculate all standings").execute_for_list(&self.ctx).await?.data;
        let statuses = Q::match_statuses().comment("Fetch all match statuses").purpose("Recalculate all standings").execute_for_list(&self.ctx).await?.data;
        let scheduled_id = statuses.iter().find(|s| s.code() == "SCHEDULED").map(|s| s.id()).unwrap_or(0);
        let postponed_id = statuses.iter().find(|s| s.code() == "POSTPONED").map(|s| s.id()).unwrap_or(0);
        let mut standings = Q::group_standings().comment("Fetch all group standings").purpose("Recalculate all standings").execute_for_list(&self.ctx).await?.data;

        for team in teams {
            let team_id = team.id();
            let mut played = 0; let mut won = 0; let mut drawn = 0; let mut lost = 0;
            let mut gf = 0; let mut ga = 0;

            for m in &matches {
                if m.match_status_id() == scheduled_id || m.match_status_id() == postponed_id {
                    continue;
                }
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
                    let old_pts = s.points();
                    let old_gd = s.goal_difference();
                    s.update_played(played);
                    s.update_won(won);
                    s.update_drawn(drawn);
                    s.update_lost(lost);
                    s.update_goals_for(gf);
                    s.update_goals_against(ga);
                    s.update_goal_difference(gf - ga);
                    s.update_points((won * 3) + drawn);
                    s.clone().audit_as(&format!("Recalculate standing for {}: Pts {}->{}, GD {}->{}", team.team_name(), old_pts, s.points(), old_gd, s.goal_difference())).save(&self.ctx).await?;
                }
            } else {
                self.log(&format!("Error: Team '{}' (ID: {}) has no GroupStanding record. Skipping...", team.team_name(), team_id));
            }
        }
        Ok(())
    }

    fn sort_standings(standings: &mut [GroupStanding]) {
        standings.sort_by(|a, b| {
            let a_group = E::group_standing(a).get_match_group().eval().map(|g| g.group_letter().to_string()).unwrap_or_default();
            let b_group = E::group_standing(b).get_match_group().eval().map(|g| g.group_letter().to_string()).unwrap_or_default();
            
            a_group.cmp(&b_group)
                .then(b.points().cmp(&a.points()))
                .then(b.goal_difference().cmp(&a.goal_difference()))
                .then(b.goals_for().cmp(&a.goals_for()))
                .then({
                    let a_name = E::group_standing(a).get_tournament_team().eval().map(|t| t.team_name().to_string()).unwrap_or_default();
                    let b_name = E::group_standing(b).get_tournament_team().eval().map(|t| t.team_name().to_string()).unwrap_or_default();
                    a_name.cmp(&b_name)
                })
        });
    }


    pub fn next_pane(&mut self) {
        self.active_pane = (self.active_pane + 1) % 3;
    }

    pub fn prev_pane(&mut self) {
        if self.active_pane == 0 {
            self.active_pane = 2;
        } else {
            self.active_pane -= 1;
        }
    }

    pub fn next(&mut self) {
        match self.view {
            View::Global | View::Group(_) => {
                match self.active_pane {
                    0 => {
                        let len = if matches!(self.view, View::Global) { self.global_standings.len() } else { self.group_standings.len() };
                        let i = match self.global_table_state.selected() {
                            Some(i) => if i >= len.saturating_sub(1) { 0 } else { i + 1 },
                            None => 0,
                        };
                        self.global_table_state.select(Some(i));
                    }
                    1 => {
                        let len = if matches!(self.view, View::Global) { self.top_players.len() } else { self.group_players.len() };
                        let i = match self.players_state.selected() {
                            Some(i) => if i >= len.saturating_sub(1) { 0 } else { i + 1 },
                            None => 0,
                        };
                        self.players_state.select(Some(i));
                    }
                    2 => {
                        let len = if matches!(self.view, View::Global) { self.recent_matches.len() } else { self.group_matches.len() };
                        let i = match self.matches_state.selected() {
                            Some(i) => if i >= len.saturating_sub(1) { 0 } else { i + 1 },
                            None => 0,
                        };
                        self.matches_state.select(Some(i));
                    }
                    _ => {}
                }
            }
            View::Players => {
                let i = match self.player_table_state.selected() {
                    Some(i) => if i >= self.all_players.len().saturating_sub(1) { 0 } else { i + 1 },
                    None => 0,
                };
                self.player_table_state.select(Some(i));
            }
            View::Logs => {
                let len = self.logs.lock().map(|l| l.len()).unwrap_or(0);
                let i = match self.logs_state.selected() {
                    Some(i) => if i >= len.saturating_sub(1) { 0 } else { i + 1 },
                    None => 0,
                };
                self.logs_state.select(Some(i));
            }
        }
    }

    pub fn previous(&mut self) {
        match self.view {
            View::Global | View::Group(_) => {
                match self.active_pane {
                    0 => {
                        let len = if matches!(self.view, View::Global) { self.global_standings.len() } else { self.group_standings.len() };
                        let i = match self.global_table_state.selected() {
                            Some(i) => if i == 0 { len.saturating_sub(1) } else { i - 1 },
                            None => 0,
                        };
                        self.global_table_state.select(Some(i));
                    }
                    1 => {
                        let len = if matches!(self.view, View::Global) { self.top_players.len() } else { self.group_players.len() };
                        let i = match self.players_state.selected() {
                            Some(i) => if i == 0 { len.saturating_sub(1) } else { i - 1 },
                            None => 0,
                        };
                        self.players_state.select(Some(i));
                    }
                    2 => {
                        let len = if matches!(self.view, View::Global) { self.recent_matches.len() } else { self.group_matches.len() };
                        let i = match self.matches_state.selected() {
                            Some(i) => if i == 0 { len.saturating_sub(1) } else { i - 1 },
                            None => 0,
                        };
                        self.matches_state.select(Some(i));
                    }
                    _ => {}
                }
            }
            View::Players => {
                let i = match self.player_table_state.selected() {
                    Some(i) => if i == 0 { self.all_players.len().saturating_sub(1) } else { i - 1 },
                    None => 0,
                };
                self.player_table_state.select(Some(i));
            }
            View::Logs => {
                let len = self.logs.lock().map(|l| l.len()).unwrap_or(0);
                let i = match self.logs_state.selected() {
                    Some(i) => if i == 0 { len.saturating_sub(1) } else { i - 1 },
                    None => 0,
                };
                self.logs_state.select(Some(i));
            }
        }
    }
}
