use fifa_world_cup_2026_service::*;
use ratatui::widgets::{ListState, TableState};
use ratatui_tournament::BracketMatch;
use teaql_core::{Entity, UpdateCommand};
use teaql_runtime::UserContext;

pub trait UserContextHttpExt {
    fn http(&self) -> HttpBuilder;
}

impl UserContextHttpExt for UserContext {
    fn http(&self) -> HttpBuilder {
        HttpBuilder { purpose: None }
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
    pub knockout_matches: Vec<BracketMatch>,
    pub top_players: Vec<(String, String, i32)>, // (Team, Player, Goals)
    pub recent_matches: Vec<TournamentMatch>,

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
            knockout_matches: vec![],
            top_players: vec![],
            recent_matches: vec![],
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
        match &self.view {
            View::Global => {
                self.knockout_matches = self.fetch_knockout_matches().await?;

                let goals = Q::match_goals()
                    .select_tournament_team_with(Q::tournament_teams().select_self())
                    .comment("Fetch all match goals")
                    .purpose("Render global top players dashboard")
                    .execute_for_list(&self.ctx)
                    .await?
                    .data;

                let mut players_map = std::collections::HashMap::new();
                for g in goals {
                    let team = g
                        .tournament_team()
                        .map(|t| format!("{} {}", t.emoji_flag(), t.team_name()))
                        .unwrap_or_default();
                    let player = g.player_name().to_string();
                    *players_map.entry((team, player)).or_insert(0) += 1;
                }
                let mut tp: Vec<_> = players_map
                    .into_iter()
                    .map(|((t, p), c)| (t, p, c))
                    .collect();
                tp.sort_by_key(|b| std::cmp::Reverse(b.2));
                self.top_players = tp;

                self.recent_matches = Q::tournament_matches()
                    .select_home_team_with(Q::tournament_teams().select_self())
                    .select_away_team_with(Q::tournament_teams().select_self())
                    .select_match_stage_with(Q::match_stages().select_self())
                    .order_by_id_desc()
                    .comment("Fetch recent knockout matches")
                    .purpose("Render global knockout matches dashboard")
                    .execute_for_list(&self.ctx)
                    .await?
                    .data;
                let total_matches = self.recent_matches.len();
                self.recent_matches.retain(|m| {
                    (m.home_score() != 0 || m.away_score() != 0)
                        && Self::knockout_stage_for_match(m, m.id() as usize, total_matches)
                            .is_some()
                });
                self.recent_matches.dedup_by_key(|m| m.id());
            }
            View::Players => {
                let goals = Q::match_goals()
                    .select_tournament_team_with(Q::tournament_teams().select_self())
                    .select_tournament_match_with(
                        Q::tournament_matches()
                            .select_home_team_with(Q::tournament_teams().select_self())
                            .select_away_team_with(Q::tournament_teams().select_self()),
                    )
                    .comment("Fetch goals for player/team")
                    .purpose("Render player/team dashboard")
                    .execute_for_list(&self.ctx)
                    .await?
                    .data;

                let mut players_map = std::collections::HashMap::new();
                let mut matches_map = std::collections::HashMap::new();
                for g in goals {
                    let team = g
                        .tournament_team()
                        .map(|t| format!("{} {}", t.emoji_flag(), t.team_name()))
                        .unwrap_or_default();
                    let player = g.player_name().to_string();
                    let key = (team, player);
                    *players_map.entry(key.clone()).or_insert(0) += 1;
                    if let Some(m) = g.tournament_match() {
                        let home_flag = m
                            .home_team()
                            .map(|t| t.emoji_flag().to_string())
                            .unwrap_or_default();
                        let home_name = m
                            .home_team()
                            .map(|t| t.team_name().to_string())
                            .unwrap_or_default();
                        let away_flag = m
                            .away_team()
                            .map(|t| t.emoji_flag().to_string())
                            .unwrap_or_default();
                        let away_name = m
                            .away_team()
                            .map(|t| t.team_name().to_string())
                            .unwrap_or_default();
                        let match_str = format!(
                            "{} {}  {:>14} vs {:<14}",
                            home_flag, away_flag, home_name, away_name
                        );
                        matches_map
                            .entry(key)
                            .or_insert_with(Vec::new)
                            .push(match_str);
                    }
                }
                let mut all: Vec<_> = players_map
                    .into_iter()
                    .map(|(k, count)| {
                        let mut ms = matches_map.remove(&k).unwrap_or_default();
                        ms.sort();
                        ms.dedup();
                        (k.0, k.1, count, ms)
                    })
                    .collect();
                all.sort_by_key(|b| std::cmp::Reverse(b.2));
                self.all_players = all;
            }
            View::Logs => {}
        }
        Ok(())
    }

    async fn fetch_knockout_matches(
        &self,
    ) -> Result<Vec<BracketMatch>, Box<dyn std::error::Error>> {
        let mut matches = Q::tournament_matches()
            .select_home_team_with(Q::tournament_teams().select_self())
            .select_away_team_with(Q::tournament_teams().select_self())
            .select_match_stage_with(Q::match_stages().select_self())
            .order_by_id_asc()
            .comment("Fetch knockout bracket matches")
            .purpose("Render global knockout bracket")
            .execute_for_list(&self.ctx)
            .await?
            .data;

        matches.sort_by_key(|m| {
            let match_number = m.match_number();
            if match_number > 0 {
                (0_i32, match_number, m.id())
            } else {
                (1_i32, 0_i32, m.id())
            }
        });

        let total = matches.len();
        let mut result: Vec<_> = matches
            .into_iter()
            .enumerate()
            .filter_map(|(idx, m)| {
                let sequence = idx + 1;
                let (_stage, stage_rank) = Self::knockout_stage_for_match(&m, sequence, total)?;
                Some(Self::knockout_match_view(m, stage_rank, sequence))
            })
            .collect();

        // Deduplicate by match label: if fix_data created a real match with
        // the same match_number as a seed match, keep the later one
        // (fix_data entries have higher IDs and appear later in the list).
        let mut seen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for (i, m) in result.iter().enumerate() {
            let entry = seen.entry(m.label.clone()).or_insert(i);
            // Always prefer later entries (fix_data corrections come after seed)
            *entry = i;
        }
        let keep: std::collections::HashSet<usize> = seen.values().copied().collect();
        let mut deduped: Vec<BracketMatch> = result
            .into_iter()
            .enumerate()
            .filter(|(i, _)| keep.contains(i))
            .map(|(_, m)| m)
            .collect();

        deduped.sort_by(|a, b| {
            let num_a = a.label.trim_start_matches('M').parse::<u32>().unwrap_or(0);
            let num_b = b.label.trim_start_matches('M').parse::<u32>().unwrap_or(0);
            a.stage_rank
                .cmp(&b.stage_rank)
                .then_with(|| num_a.cmp(&num_b))
        });

        // Reorder within each stage to match bracket structure:
        // QF stays sorted by label. R16 is reordered so that for each QF,
        // the two R16 matches whose winners are the QF participants are adjacent.
        // R32 is similarly reordered based on R16 participants.
        use crate::ui::team_short;

        let mut by_stage: std::collections::BTreeMap<usize, Vec<BracketMatch>> =
            std::collections::BTreeMap::new();
        for m in deduped {
            by_stage.entry(m.stage_rank).or_default().push(m);
        }

        let qf = by_stage.remove(&3).unwrap_or_default();
        let r16_orig = by_stage.remove(&2).unwrap_or_default();
        let r32_orig = by_stage.remove(&1).unwrap_or_default();

        // Reorder R16 by QF bracket
        let mut r16: Vec<BracketMatch> = Vec::new();
        let mut r16_used = vec![false; r16_orig.len()];
        for q in &qf {
            for team in [&q.home, &q.away] {
                let ts = team_short(team);
                // First try: winner matches
                let idx = r16_orig.iter().enumerate().position(|(i, m)| {
                    !r16_used[i]
                        && m.winner
                            .as_ref()
                            .map(|w| team_short(w) == ts)
                            .unwrap_or(false)
                });
                // Fallback: home/away matches
                let idx = idx.or_else(|| {
                    r16_orig.iter().enumerate().position(|(i, m)| {
                        !r16_used[i] && (team_short(&m.home) == ts || team_short(&m.away) == ts)
                    })
                });
                if let Some(idx) = idx {
                    r16_used[idx] = true;
                    r16.push(r16_orig[idx].clone());
                }
            }
        }
        for (i, m) in r16_orig.into_iter().enumerate() {
            if !r16_used[i] {
                r16.push(m);
            }
        }

        // Reorder R32 by R16 bracket
        let mut r32: Vec<BracketMatch> = Vec::new();
        let mut r32_used = vec![false; r32_orig.len()];
        for r in &r16 {
            for team in [&r.home, &r.away] {
                let ts = team_short(team);
                let idx = r32_orig.iter().enumerate().position(|(i, m)| {
                    !r32_used[i]
                        && m.winner
                            .as_ref()
                            .map(|w| team_short(w) == ts)
                            .unwrap_or(false)
                });
                let idx = idx.or_else(|| {
                    r32_orig.iter().enumerate().position(|(i, m)| {
                        !r32_used[i] && (team_short(&m.home) == ts || team_short(&m.away) == ts)
                    })
                });
                if let Some(idx) = idx {
                    r32_used[idx] = true;
                    r32.push(r32_orig[idx].clone());
                }
            }
        }
        for (i, m) in r32_orig.into_iter().enumerate() {
            if !r32_used[i] {
                r32.push(m);
            }
        }

        // Reassemble in order: R32, R16, QF, then remaining stages (SF, 3rd, Final)
        let mut final_result = Vec::new();
        final_result.extend(r32);
        final_result.extend(r16);
        final_result.extend(qf);
        for (_, ms) in by_stage {
            final_result.extend(ms);
        }

        Ok(final_result)
    }

    fn knockout_match_view(
        m: TournamentMatch,
        stage_rank: usize,
        sequence: usize,
    ) -> BracketMatch {
        let home = m
            .home_team()
            .map(|t| format!("{} {}", t.emoji_flag(), t.team_name()))
            .unwrap_or_else(|| "TBD".to_string());
        let away = m
            .away_team()
            .map(|t| format!("{} {}", t.emoji_flag(), t.team_name()))
            .unwrap_or_else(|| "TBD".to_string());
        let completed = m.home_score() != 0
            || m.away_score() != 0
            || m.penalty_home() != 0
            || m.penalty_away() != 0;
        let mut score = if completed {
            format!("{} - {}", m.home_score(), m.away_score())
        } else {
            "vs".to_string()
        };
        if m.penalty_home() != 0 || m.penalty_away() != 0 {
            score = format!("{} p{}-{}", score, m.penalty_home(), m.penalty_away());
        }

        let winner = if completed {
            if m.home_score() > m.away_score()
                || (m.home_score() == m.away_score() && m.penalty_home() > m.penalty_away())
            {
                Some(home.clone())
            } else if m.away_score() > m.home_score()
                || (m.home_score() == m.away_score() && m.penalty_away() > m.penalty_home())
            {
                Some(away.clone())
            } else {
                None
            }
        } else {
            None
        };

        let label = format!("M{:02}", m.match_number().max(sequence as i32));
        let pending_text = if !completed && stage_rank > 1 {
            Some(format!("? {}", Self::match_date_for_label(&label, stage_rank)))
        } else {
            None
        };

        BracketMatch {
            stage_rank,
            label,
            home,
            away,
            score,
            winner,
            completed,
            pending_text,
        }
    }

    fn match_date_for_label(label: &str, stage_rank: usize) -> String {
        match stage_rank {
            2 => match label {
                "M89" | "M90" | "M91" | "M92" => "7/7".to_string(),
                "M93" | "M94" | "M95" | "M96" => "7/8".to_string(),
                _ => "TBD".to_string(),
            },
            3 => match label {
                "M97" => "7/9".to_string(),
                "M98" => "7/10".to_string(),
                "M99" | "M100" => "7/11".to_string(),
                _ => "TBD".to_string(),
            },
            4 => match label {
                "M101" | "M102" => "7/15".to_string(),
                _ => "TBD".to_string(),
            },
            6 => match label {
                "M104" => "7/19".to_string(),
                _ => "TBD".to_string(),
            },
            _ => "TBD".to_string(),
        }
    }

    pub fn knockout_stage_for_match(
        m: &TournamentMatch,
        _sequence: usize,
        total: usize,
    ) -> Option<(&'static str, usize)> {
        match m.match_stage_id() {
            1001 => return None,
            1002 => return Some(("Round of 32", 1)),
            1003 => return Some(("Round of 16", 2)),
            1004 => return Some(("Quarter Final", 3)),
            1005 => return Some(("Semi Final", 4)),
            1006 => return Some(("Third Place", 5)),
            1007 => return Some(("Final", 6)),
            _ => {}
        }

        if let Some(stage) = m.match_stage() {
            match stage.code().as_str() {
                "GROUP" => return None,
                "ROUND_OF_32" => return Some(("Round of 32", 1)),
                "ROUND_OF_16" => return Some(("Round of 16", 2)),
                "QUARTER_FINAL" => return Some(("Quarter Final", 3)),
                "SEMI_FINAL" => return Some(("Semi Final", 4)),
                "THIRD_PLACE" => return Some(("Third Place", 5)),
                "FINAL" => return Some(("Final", 6)),
                _ => {}
            }
        }

        if m.match_number() <= 0 {
            return None;
        }

        let n = m.match_number() as usize;

        if total >= 104 {
            match n {
                1..=72 => None,
                73..=88 => Some(("Round of 32", 1)),
                89..=96 => Some(("Round of 16", 2)),
                97..=100 => Some(("Quarter Final", 3)),
                101..=102 => Some(("Semi Final", 4)),
                103 => Some(("Third Place", 5)),
                104 => Some(("Final", 6)),
                _ => None,
            }
        } else if total > 48 {
            match n {
                1..=48 => None,
                49..=64 => Some(("Round of 32", 1)),
                65..=72 => Some(("Round of 16", 2)),
                73..=76 => Some(("Quarter Final", 3)),
                77..=78 => Some(("Semi Final", 4)),
                79 => Some(("Third Place", 5)),
                80 => Some(("Final", 6)),
                _ => None,
            }
        } else {
            None
        }
    }

    pub async fn process_command(&mut self) {
        let cmd = self.input_buffer.trim().to_string();
        if cmd.is_empty() {
            return;
        }

        self.log(&format!("> {}", cmd));

        if cmd.eq_ignore_ascii_case("quit") || cmd.eq_ignore_ascii_case("exit") {
            self.should_quit = true;
            return;
        }

        if cmd.eq_ignore_ascii_case("global") || cmd.eq_ignore_ascii_case("bracket") {
            self.view = View::Global;
            self.log("Switched to Knockout Bracket View");
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
            self.log("Unknown command. Try: bracket, players, sync live, logs, quit");
        }

        self.input_buffer.clear();
        let _ = self.fetch_data().await;
    }

    pub async fn sync_live_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
        let all_goals = Q::match_goals()
            .purpose("sync")
            .execute_for_list(&self.ctx)
            .await?
            .data;
        for g in all_goals {
            *current_counts
                .entry(g.player_name().to_string())
                .or_insert(0) += 1;
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
                        self.clear_pending_changes();
                        self.log(&format!("Failed to record goal for {}: {}", player, e));
                    }
                }
            }
        }

        Ok(())
    }

    async fn record_goal(
        &mut self,
        team_str: &str,
        player_name: &str,
        opponent_str: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_pending_changes();

        let team_code = team_str.to_uppercase();
        let mut teams = Q::tournament_teams()
            .with_team_code_is(team_code.as_str())
            .comment("Find team by code")
            .purpose("Record live goal event")
            .execute_for_list(&self.ctx)
            .await?
            .data;
        if teams.is_empty() {
            teams = Q::tournament_teams()
                .with_team_name_is(team_str)
                .comment("Find team by name")
                .purpose("Record live goal event")
                .execute_for_list(&self.ctx)
                .await?
                .data;
        }

        let team = if let Some(t) = teams.pop() {
            t
        } else {
            return Err(format!("Team '{}' not found", team_str).into());
        };

        let opp_code = opponent_str.to_uppercase();
        let mut opps = Q::tournament_teams()
            .with_team_code_is(opp_code.as_str())
            .comment("Find opponent team by code")
            .purpose("Record live goal event")
            .execute_for_list(&self.ctx)
            .await?
            .data;
        if opps.is_empty() {
            opps = Q::tournament_teams()
                .with_team_name_is(opponent_str)
                .comment("Find opponent team by name")
                .purpose("Record live goal event")
                .execute_for_list(&self.ctx)
                .await?
                .data;
        }

        let opponent = if let Some(t) = opps.pop() {
            t
        } else {
            return Err(format!("Opponent '{}' not found", opponent_str).into());
        };

        let group_letter = team.group_letter();
        let mg = Q::match_groups()
            .with_group_letter_is(group_letter.clone())
            .comment("Find match group for team")
            .purpose("Record live goal event")
            .execute_for_list(&self.ctx)
            .await?
            .data
            .pop()
            .unwrap();

        let mut match_opt = Q::tournament_matches()
            .with_home_team_matching(Q::tournament_teams().with_id_is(team.id()))
            .with_away_team_matching(Q::tournament_teams().with_id_is(opponent.id()))
            .comment("Find match by home/away teams")
            .purpose("Record live goal event")
            .execute_for_list(&self.ctx)
            .await?
            .data
            .into_iter()
            .next();

        if match_opt.is_none() {
            match_opt = Q::tournament_matches()
                .with_home_team_matching(Q::tournament_teams().with_id_is(opponent.id()))
                .with_away_team_matching(Q::tournament_teams().with_id_is(team.id()))
                .comment("Find match by away/home teams")
                .purpose("Record live goal event")
                .execute_for_list(&self.ctx)
                .await?
                .data
                .into_iter()
                .next();
        }

        self.clear_pending_changes();

        let mut t_match = if let Some(m) = match_opt {
            m
        } else {
            let mut m = Q::tournament_matches()
                .purpose("record")
                .new_entity(&self.ctx);
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
                .comment("Find Finished status")
                .purpose("Record live goal event")
                .execute_for_list(&self.ctx)
                .await?
                .data
                .pop()
                .unwrap();
            m
        };

        self.clear_pending_changes();

        let mut goal = Q::match_goals().purpose("record").new_entity(&self.ctx);
        goal.update_player_name(player_name.to_string());
        goal.update_tournament_match_id(t_match.id());
        goal.update_tournament_team_id(team.id());
        goal.update_tournament_id(team.tournament_id());
        goal.update_minute_scored(1);
        goal.audit_as("Record goal").save(&self.ctx).await?;

        self.clear_pending_changes();

        let old_hs = t_match.home_score();
        let old_as = t_match.away_score();
        if t_match.home_team_id() == team.id() {
            t_match.update_home_score(old_hs + 1);
        } else {
            t_match.update_away_score(old_as + 1);
        }
        let new_hs = t_match.home_score();
        let new_as = t_match.away_score();
        self.update_match_score(&t_match).await?;
        self.log(&format!(
            "Updated match score: {} vs {} ({} - {} -> {} - {})",
            team.team_name(),
            opponent.team_name(),
            old_hs,
            old_as,
            new_hs,
            new_as
        ));

        self.clear_pending_changes();
        Ok(())
    }

    fn clear_pending_changes(&self) {
        self.ctx.entity_root().clear_current_change_set();
    }

    async fn update_match_score(
        &self,
        t_match: &TournamentMatch,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data_service = self
            .ctx
            .entity_data_service::<DataServiceExecutor>("TournamentMatch")?;
        let cmd = UpdateCommand::new("TournamentMatch", t_match.id())
            .value("home_score", t_match.home_score())
            .value("away_score", t_match.away_score())
            .value("version", t_match.version() + 1);
        data_service.update(&cmd).await?;
        Ok(())
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
            View::Global => match self.active_pane {
                0 => {
                    let len = self.knockout_matches.len();
                    let i = match self.global_table_state.selected() {
                        Some(i) => {
                            if i >= len.saturating_sub(1) {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    self.global_table_state.select(Some(i));
                }
                1 => {
                    let len = self.top_players.len();
                    let i = match self.players_state.selected() {
                        Some(i) => {
                            if i >= len.saturating_sub(1) {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    self.players_state.select(Some(i));
                }
                2 => {
                    let len = self.recent_matches.len();
                    let i = match self.matches_state.selected() {
                        Some(i) => {
                            if i >= len.saturating_sub(1) {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    self.matches_state.select(Some(i));
                }
                _ => {}
            },
            View::Players => {
                let i = match self.player_table_state.selected() {
                    Some(i) => {
                        if i >= self.all_players.len().saturating_sub(1) {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.player_table_state.select(Some(i));
            }
            View::Logs => {
                let len = self.logs.lock().map(|l| l.len()).unwrap_or(0);
                let i = match self.logs_state.selected() {
                    Some(i) => {
                        if i >= len.saturating_sub(1) {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.logs_state.select(Some(i));
            }
        }
    }

    pub fn previous(&mut self) {
        match self.view {
            View::Global => match self.active_pane {
                0 => {
                    let len = self.knockout_matches.len();
                    let i = match self.global_table_state.selected() {
                        Some(i) => {
                            if i == 0 {
                                len.saturating_sub(1)
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    self.global_table_state.select(Some(i));
                }
                1 => {
                    let len = self.top_players.len();
                    let i = match self.players_state.selected() {
                        Some(i) => {
                            if i == 0 {
                                len.saturating_sub(1)
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    self.players_state.select(Some(i));
                }
                2 => {
                    let len = self.recent_matches.len();
                    let i = match self.matches_state.selected() {
                        Some(i) => {
                            if i == 0 {
                                len.saturating_sub(1)
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    self.matches_state.select(Some(i));
                }
                _ => {}
            },
            View::Players => {
                let i = match self.player_table_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.all_players.len().saturating_sub(1)
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.player_table_state.select(Some(i));
            }
            View::Logs => {
                let len = self.logs.lock().map(|l| l.len()).unwrap_or(0);
                let i = match self.logs_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            len.saturating_sub(1)
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.logs_state.select(Some(i));
            }
        }
    }
}
