use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TeamSeed {
    pub team_id: u64,
    pub group_id: u64,
    pub group_letter: String,
    pub display_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchResult {
    pub home_team_id: u64,
    pub away_team_id: u64,
    pub match_group_id: u64,
    pub home_score: i32,
    pub away_score: i32,
    pub completed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupStandingRow {
    pub team_id: u64,
    pub group_id: u64,
    pub group_letter: String,
    pub display_name: String,
    pub played: i32,
    pub won: i32,
    pub drawn: i32,
    pub lost: i32,
    pub goals_for: i32,
    pub goals_against: i32,
    pub goal_difference: i32,
    pub points: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectedKnockoutMatch {
    pub label: String,
    pub home_seed: String,
    pub home_team: String,
    pub away_seed: String,
    pub away_team: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KnockoutTie {
    pub label: String,
    pub home_team: String,
    pub away_team: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KnockoutResult {
    pub tie: KnockoutTie,
    pub home_score: Option<i32>,
    pub away_score: Option<i32>,
    pub winner: Option<String>,
}

/// Represents a knockout match in the bracket, with enough info to compute advancement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KnockoutBracketMatch {
    pub match_number: i32,
    pub stage: KnockoutStage,
    pub home_team_id: u64,
    pub away_team_id: u64,
    pub home_score: i32,
    pub away_score: i32,
    pub penalty_home: i32,
    pub penalty_away: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum KnockoutStage {
    R32,
    R16,
    QF,
    SF,
    ThirdPlace,
    Final,
}

impl KnockoutStage {
    pub fn from_match_number(n: i32) -> Option<Self> {
        match n {
            73..=88 => Some(KnockoutStage::R32),
            89..=96 => Some(KnockoutStage::R16),
            97..=100 => Some(KnockoutStage::QF),
            101..=102 => Some(KnockoutStage::SF),
            103 => Some(KnockoutStage::ThirdPlace),
            104 => Some(KnockoutStage::Final),
            _ => None,
        }
    }
}

/// Determine the winner of a knockout match.
/// Returns Some(team_id) if there is a clear winner, None if the match is not yet decided.
pub fn knockout_match_winner(m: &KnockoutBracketMatch) -> Option<u64> {
    if m.home_team_id == 0 || m.away_team_id == 0 {
        return None;
    }
    // No goals at all → match not played
    if m.home_score == 0 && m.away_score == 0 && m.penalty_home == 0 && m.penalty_away == 0 {
        return None;
    }
    if m.home_score > m.away_score {
        return Some(m.home_team_id);
    }
    if m.away_score > m.home_score {
        return Some(m.away_team_id);
    }
    // Scores are equal → check penalties
    if m.penalty_home > m.penalty_away {
        return Some(m.home_team_id);
    }
    if m.penalty_away > m.penalty_home {
        return Some(m.away_team_id);
    }
    // Draw with no penalty resolution
    None
}

/// Determine the loser of a knockout match (for 3rd place match).
pub fn knockout_match_loser(m: &KnockoutBracketMatch) -> Option<u64> {
    let winner = knockout_match_winner(m)?;
    if winner == m.home_team_id {
        Some(m.away_team_id)
    } else {
        Some(m.home_team_id)
    }
}

/// Given a source match number in R32 (73..88), return which R16 match it feeds into,
/// and whether the winner becomes home or away in that R16 match.
///
/// R32 pairing → R16 slot:
///   R32 match 73 winner & R32 match 74 winner → R16 match 89
///   R32 match 75 winner & R32 match 76 winner → R16 match 90
///   ... etc.
///
/// The first (lower match number) feeds the home slot, the second feeds the away slot.
pub fn r32_feeds_r16(r32_match_number: i32) -> Option<(i32, bool)> {
    if !(73..=88).contains(&r32_match_number) {
        return None;
    }
    let offset = r32_match_number - 73; // 0..15
    let r16_index = offset / 2; // 0..7
    let is_home = offset % 2 == 0;
    Some((89 + r16_index, is_home))
}

/// R16 match feeds into QF.
pub fn r16_feeds_qf(r16_match_number: i32) -> Option<(i32, bool)> {
    if !(89..=96).contains(&r16_match_number) {
        return None;
    }
    let offset = r16_match_number - 89; // 0..7
    let qf_index = offset / 2; // 0..3
    let is_home = offset % 2 == 0;
    Some((97 + qf_index, is_home))
}

/// QF match feeds into SF.
pub fn qf_feeds_sf(qf_match_number: i32) -> Option<(i32, bool)> {
    if !(97..=100).contains(&qf_match_number) {
        return None;
    }
    let offset = qf_match_number - 97; // 0..3
    let sf_index = offset / 2; // 0..1
    let is_home = offset % 2 == 0;
    Some((101 + sf_index, is_home))
}

/// SF match feeds into Final (winners) and Third Place (losers).
pub fn sf_feeds_final_and_third(sf_match_number: i32) -> Option<(i32, i32, bool)> {
    if !(101..=102).contains(&sf_match_number) {
        return None;
    }
    let is_home = sf_match_number == 101;
    // Winner → Final (104), Loser → Third Place (103)
    Some((104, 103, is_home))
}

/// Given a completed match, determine where the winner should advance to.
/// Returns (target_match_number, is_home_slot) or None if the match doesn't feed forward.
pub fn advancement_target(match_number: i32) -> Option<(i32, bool)> {
    match KnockoutStage::from_match_number(match_number)? {
        KnockoutStage::R32 => r32_feeds_r16(match_number),
        KnockoutStage::R16 => r16_feeds_qf(match_number),
        KnockoutStage::QF => qf_feeds_sf(match_number),
        KnockoutStage::SF => {
            let (final_n, _third_n, is_home) = sf_feeds_final_and_third(match_number)?;
            Some((final_n, is_home))
        }
        KnockoutStage::ThirdPlace | KnockoutStage::Final => None,
    }
}

/// For SF losers, determine where they go (third place match).
pub fn third_place_target(match_number: i32) -> Option<(i32, bool)> {
    if let Some(KnockoutStage::SF) = KnockoutStage::from_match_number(match_number) {
        let (_final_n, third_n, is_home) = sf_feeds_final_and_third(match_number)?;
        Some((third_n, is_home))
    } else {
        None
    }
}

/// Compute all advancements for a set of knockout matches.
/// Returns a list of (target_match_number, is_home, team_id) tuples.
pub fn compute_advancements(matches: &[KnockoutBracketMatch]) -> Vec<(i32, bool, u64)> {
    let mut advancements = Vec::new();
    for m in matches {
        if let Some(winner_id) = knockout_match_winner(m) {
            if let Some((target, is_home)) = advancement_target(m.match_number) {
                advancements.push((target, is_home, winner_id));
            }
        }
        // SF losers → third place
        if let Some(loser_id) = knockout_match_loser(m) {
            if let Some((target, is_home)) = third_place_target(m.match_number) {
                advancements.push((target, is_home, loser_id));
            }
        }
    }
    advancements
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Qualifier {
    seed: String,
    team: String,
    points: i32,
    goal_difference: i32,
    goals_for: i32,
}

pub fn calculate_group_standings(
    teams: &[TeamSeed],
    matches: &[MatchResult],
) -> Vec<GroupStandingRow> {
    let teams_by_id: HashMap<u64, &TeamSeed> = teams.iter().map(|t| (t.team_id, t)).collect();

    let mut standings: Vec<_> = teams
        .iter()
        .map(|team| GroupStandingRow {
            team_id: team.team_id,
            group_id: team.group_id,
            group_letter: team.group_letter.clone(),
            display_name: team.display_name.clone(),
            played: 0,
            won: 0,
            drawn: 0,
            lost: 0,
            goals_for: 0,
            goals_against: 0,
            goal_difference: 0,
            points: 0,
        })
        .collect();

    for m in matches.iter().filter(|m| m.completed) {
        let Some(home) = teams_by_id.get(&m.home_team_id) else {
            continue;
        };
        let Some(away) = teams_by_id.get(&m.away_team_id) else {
            continue;
        };

        if home.group_id != m.match_group_id
            || away.group_id != m.match_group_id
            || home.group_id != away.group_id
        {
            continue;
        }

        apply_result(&mut standings, m.home_team_id, m.home_score, m.away_score);
        apply_result(&mut standings, m.away_team_id, m.away_score, m.home_score);
    }

    sort_group_standings(&mut standings);
    standings
}

pub fn projected_round_of_32(standings: &[GroupStandingRow]) -> Vec<ProjectedKnockoutMatch> {
    let mut sorted = standings.to_vec();
    sort_group_standings(&mut sorted);

    let mut group_winners: Vec<(String, String)> = Vec::new();
    let mut group_runners: Vec<(String, String)> = Vec::new();
    let mut third_place: Vec<(String, String, i32, i32, i32)> = Vec::new();
    let mut current_group = String::new();
    let mut group_rank = 0_usize;

    for standing in &sorted {
        if standing.group_letter != current_group {
            current_group = standing.group_letter.clone();
            group_rank = 0;
        }
        group_rank += 1;

        match group_rank {
            1 => group_winners.push((standing.group_letter.clone(), standing.display_name.clone())),
            2 => group_runners.push((standing.group_letter.clone(), standing.display_name.clone())),
            3 => third_place.push((
                standing.group_letter.clone(),
                standing.display_name.clone(),
                standing.points,
                standing.goal_difference,
                standing.goals_for,
            )),
            _ => {}
        }
    }

    // Sort third-place teams by strength to determine which 8 qualify
    third_place.sort_by(|a, b| {
        b.2.cmp(&a.2)
            .then(b.3.cmp(&a.3))
            .then(b.4.cmp(&a.4))
            .then(a.0.cmp(&b.0))
    });
    let qualified_thirds: Vec<(String, String)> =
        third_place.into_iter().take(8).map(|(g, t, _, _, _)| (g, t)).collect();

    // FIFA 48-team bracket: assign all 32 teams to positions 4..35
    // Group winners: positions 4-15 (A=4, B=5, ..., L=15)
    // Runners-up: positions 16-27 (A=16, B=17, ..., L=27)
    // Third-place (qualified): positions 28-35 sorted by group letter
    // Pairing: position i vs position (39 - i) for i = 4..19

    fn group_offset(g: &str) -> usize {
        g.chars()
            .next()
            .map(|c| (c as u8 - b'A') as usize)
            .unwrap_or(0)
    }

    let mut positions: std::collections::HashMap<usize, (String, String)> =
        std::collections::HashMap::new();

    // Group winners: positions 4..15
    for (g, team) in &group_winners {
        positions.insert(4 + group_offset(g), (format!("{}1", g), team.clone()));
    }
    // Runners-up: positions 16..27
    for (g, team) in &group_runners {
        positions.insert(16 + group_offset(g), (format!("{}2", g), team.clone()));
    }
    // Third-place (qualified, already sorted by group letter): positions 28..35
    for (idx, (g, team)) in qualified_thirds.iter().enumerate() {
        positions.insert(28 + idx, (format!("{}3", g), team.clone()));
    }

    let mut result = Vec::new();
    for i in 0..16 {
        let home_pos = 4 + i;
        let away_pos = 35usize.saturating_sub(i);
        let home = positions.get(&home_pos);
        let away = positions.get(&away_pos);
        result.push(ProjectedKnockoutMatch {
            label: format!("R32-{:02}", i + 1),
            home_seed: home
                .map(|(s, _)| s.clone())
                .unwrap_or_else(|| "TBD".to_string()),
            home_team: home
                .map(|(_, t)| t.clone())
                .unwrap_or_else(|| "TBD".to_string()),
            away_seed: away
                .map(|(s, _)| s.clone())
                .unwrap_or_else(|| "TBD".to_string()),
            away_team: away
                .map(|(_, t)| t.clone())
                .unwrap_or_else(|| "TBD".to_string()),
        });
    }
    result
}

pub fn teams_in_knockout_ties(ties: &[KnockoutTie]) -> Vec<String> {
    let mut teams: Vec<_> = ties
        .iter()
        .flat_map(|tie| [tie.home_team.clone(), tie.away_team.clone()])
        .collect();
    teams.sort();
    teams.dedup();
    teams
}

pub fn completed_knockout_winners(results: &[KnockoutResult]) -> Vec<String> {
    let mut winners: Vec<_> = results.iter().filter_map(knockout_winner).collect();
    winners.sort();
    winners
}

fn knockout_winner(result: &KnockoutResult) -> Option<String> {
    let home_score = result.home_score?;
    let away_score = result.away_score?;
    if home_score > away_score {
        Some(result.tie.home_team.clone())
    } else if away_score > home_score {
        Some(result.tie.away_team.clone())
    } else {
        result.winner.clone()
    }
}

fn apply_result(standings: &mut [GroupStandingRow], team_id: u64, gf: i32, ga: i32) {
    let Some(row) = standings.iter_mut().find(|s| s.team_id == team_id) else {
        return;
    };
    row.played += 1;
    row.goals_for += gf;
    row.goals_against += ga;
    row.goal_difference = row.goals_for - row.goals_against;
    if gf > ga {
        row.won += 1;
        row.points += 3;
    } else if gf == ga {
        row.drawn += 1;
        row.points += 1;
    } else {
        row.lost += 1;
    }
}

pub fn sort_group_standings(standings: &mut [GroupStandingRow]) {
    standings.sort_by(|a, b| {
        a.group_letter
            .cmp(&b.group_letter)
            .then(compare_standing_strength(a, b))
    });
}

fn compare_standing_strength(a: &GroupStandingRow, b: &GroupStandingRow) -> Ordering {
    b.points
        .cmp(&a.points)
        .then(b.goal_difference.cmp(&a.goal_difference))
        .then(b.goals_for.cmp(&a.goals_for))
        .then(a.display_name.cmp(&b.display_name))
}

#[allow(dead_code)]
fn compare_qualifier_strength(a: &Qualifier, b: &Qualifier) -> Ordering {
    b.points
        .cmp(&a.points)
        .then(b.goal_difference.cmp(&a.goal_difference))
        .then(b.goals_for.cmp(&a.goals_for))
        .then(a.seed.cmp(&b.seed))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn team(team_id: u64, group_id: u64, group: &str, name: &str) -> TeamSeed {
        TeamSeed {
            team_id,
            group_id,
            group_letter: group.to_string(),
            display_name: name.to_string(),
        }
    }

    fn standing(group: char, rank: usize, points: i32, gd: i32, gf: i32) -> GroupStandingRow {
        GroupStandingRow {
            team_id: ((group as u8 - b'A') as u64 * 10) + rank as u64,
            group_id: (group as u8 - b'A') as u64,
            group_letter: group.to_string(),
            display_name: format!("{}{}", group, rank),
            played: 3,
            won: 0,
            drawn: 0,
            lost: 0,
            goals_for: gf,
            goals_against: gf - gd,
            goal_difference: gd,
            points,
        }
    }

    fn replace_third_place(
        standings: &mut [GroupStandingRow],
        group: char,
        name: &str,
        points: i32,
        gd: i32,
        gf: i32,
    ) {
        let row = standings
            .iter_mut()
            .find(|s| s.group_letter == group.to_string() && s.display_name.ends_with('3'))
            .unwrap();
        row.display_name = name.to_string();
        row.points = points;
        row.goal_difference = gd;
        row.goals_for = gf;
        row.goals_against = gf - gd;
    }

    fn row<'a>(standings: &'a [GroupStandingRow], name: &str) -> &'a GroupStandingRow {
        standings.iter().find(|s| s.display_name == name).unwrap()
    }

    fn tie(label: &str, home: &str, away: &str) -> KnockoutTie {
        KnockoutTie {
            label: label.to_string(),
            home_team: home.to_string(),
            away_team: away.to_string(),
        }
    }

    fn result(
        label: &str,
        home: &str,
        home_score: i32,
        away: &str,
        away_score: i32,
    ) -> KnockoutResult {
        KnockoutResult {
            tie: tie(label, home, away),
            home_score: Some(home_score),
            away_score: Some(away_score),
            winner: None,
        }
    }

    fn result_after_penalties(
        label: &str,
        home: &str,
        home_score: i32,
        away: &str,
        away_score: i32,
        winner: &str,
    ) -> KnockoutResult {
        KnockoutResult {
            tie: tie(label, home, away),
            home_score: Some(home_score),
            away_score: Some(away_score),
            winner: Some(winner.to_string()),
        }
    }

    fn pending(label: &str, home: &str, away: &str) -> KnockoutResult {
        KnockoutResult {
            tie: tie(label, home, away),
            home_score: None,
            away_score: None,
            winner: None,
        }
    }

    fn sorted_names<const N: usize>(names: [&str; N]) -> Vec<String> {
        let mut result: Vec<_> = names.iter().map(|name| (*name).to_string()).collect();
        result.sort();
        result
    }

    fn reported_round_of_16_fixture() -> Vec<KnockoutTie> {
        vec![
            tie("R16-01", "France", "Paraguay"),
            tie("R16-02", "Canada", "Morocco"),
            tie("R16-03", "Portugal", "Spain"),
            tie("R16-04", "United States", "Belgium"),
            tie("R16-05", "Brazil", "Norway"),
            tie("R16-06", "Mexico", "England"),
            tie("R16-07", "Argentina", "Egypt"),
            tie("R16-08", "Switzerland", "Colombia"),
        ]
    }

    fn reported_round_of_16_results() -> Vec<KnockoutResult> {
        vec![
            result("R16-01", "France", 1, "Paraguay", 0),
            result("R16-02", "Canada", 0, "Morocco", 3),
            result("R16-03", "Portugal", 0, "Spain", 1),
            result("R16-04", "United States", 1, "Belgium", 4),
            result("R16-05", "Brazil", 1, "Norway", 2),
            result("R16-06", "Mexico", 2, "England", 3),
            result("R16-07", "Argentina", 3, "Egypt", 2),
            pending("R16-08", "Switzerland", "Colombia"),
        ]
    }

    fn reported_round_of_32_results() -> Vec<KnockoutResult> {
        vec![
            result("R32-01", "Canada", 1, "South Africa", 0),
            result("R32-02", "Brazil", 2, "Japan", 1),
            result_after_penalties("R32-03", "Germany", 1, "Paraguay", 1, "Paraguay"),
            result_after_penalties("R32-04", "Netherlands", 1, "Morocco", 1, "Morocco"),
            result("R32-05", "Norway", 2, "Ivory Coast", 1),
            result("R32-06", "France", 3, "Sweden", 0),
            result("R32-07", "Mexico", 2, "Ecuador", 0),
            result("R32-08", "England", 2, "DR Congo", 1),
            result("R32-09", "Belgium", 3, "Senegal", 2),
            result("R32-10", "United States", 2, "Bosnia and Herzegovina", 0),
            result("R32-11", "Spain", 3, "Austria", 0),
            result("R32-12", "Portugal", 2, "Croatia", 1),
            result("R32-13", "Switzerland", 2, "Algeria", 0),
            result_after_penalties("R32-14", "Egypt", 1, "Australia", 1, "Egypt"),
            result("R32-15", "Argentina", 3, "Cabo Verde", 2),
            result("R32-16", "Colombia", 1, "Ghana", 0),
        ]
    }

    fn reported_round_of_32_penalty_winners() -> Vec<String> {
        sorted_names(["Egypt", "Morocco", "Paraguay"])
    }

    fn bracket_match(
        match_number: i32,
        home_id: u64,
        away_id: u64,
        home_score: i32,
        away_score: i32,
    ) -> KnockoutBracketMatch {
        KnockoutBracketMatch {
            match_number,
            stage: KnockoutStage::from_match_number(match_number)
                .unwrap_or(KnockoutStage::R32),
            home_team_id: home_id,
            away_team_id: away_id,
            home_score,
            away_score,
            penalty_home: 0,
            penalty_away: 0,
        }
    }

    fn bracket_match_with_penalties(
        match_number: i32,
        home_id: u64,
        away_id: u64,
        home_score: i32,
        away_score: i32,
        penalty_home: i32,
        penalty_away: i32,
    ) -> KnockoutBracketMatch {
        KnockoutBracketMatch {
            match_number,
            stage: KnockoutStage::from_match_number(match_number)
                .unwrap_or(KnockoutStage::R32),
            home_team_id: home_id,
            away_team_id: away_id,
            home_score,
            away_score,
            penalty_home,
            penalty_away,
        }
    }

    fn empty_bracket_match(match_number: i32) -> KnockoutBracketMatch {
        bracket_match(match_number, 0, 0, 0, 0)
    }

    #[test]
    fn group_standings_ignore_cross_group_matches() {
        let teams = vec![
            team(1, 10, "C", "Brazil"),
            team(2, 10, "C", "Japan"),
            team(3, 90, "I", "Norway"),
            team(4, 90, "I", "France"),
        ];
        let matches = vec![
            MatchResult {
                home_team_id: 1,
                away_team_id: 2,
                match_group_id: 10,
                home_score: 2,
                away_score: 1,
                completed: true,
            },
            MatchResult {
                home_team_id: 1,
                away_team_id: 3,
                match_group_id: 10,
                home_score: 1,
                away_score: 7,
                completed: true,
            },
        ];

        let standings = calculate_group_standings(&teams, &matches);
        let brazil = row(&standings, "Brazil");
        let norway = row(&standings, "Norway");

        assert_eq!(brazil.played, 1);
        assert_eq!(brazil.points, 3);
        assert_eq!(norway.played, 0);
        assert_eq!(norway.points, 0);
    }

    #[test]
    fn projected_round_of_32_uses_best_eight_third_place_teams() {
        let mut standings = Vec::new();
        for group in 'A'..='L' {
            standings.push(standing(group, 1, 9, 5, 7));
            standings.push(standing(group, 2, 6, 2, 5));
            standings.push(standing(group, 3, 1, -5, 1));
            standings.push(standing(group, 4, 0, -7, 1));
        }

        replace_third_place(&mut standings, 'I', "Norway", 6, 2, 10);
        replace_third_place(&mut standings, 'A', "South Africa", 4, 1, 5);
        replace_third_place(&mut standings, 'C', "Scotland", 4, 0, 4);
        replace_third_place(&mut standings, 'D', "Australia", 4, 0, 3);
        replace_third_place(&mut standings, 'E', "Ecuador", 3, 1, 2);
        replace_third_place(&mut standings, 'F', "Japan", 3, 0, 3);
        replace_third_place(&mut standings, 'G', "Egypt", 3, 0, 2);
        replace_third_place(&mut standings, 'B', "Qatar", 3, -6, 1);
        replace_third_place(&mut standings, 'K', "Uzbekistan", 2, -1, 2);
        replace_third_place(&mut standings, 'L', "Ghana", 3, -1, 2);

        let bracket = projected_round_of_32(&standings);
        let teams: Vec<_> = bracket
            .iter()
            .flat_map(|m| [m.home_team.as_str(), m.away_team.as_str()])
            .collect();

        assert!(teams.contains(&"Norway"));
        assert!(teams.contains(&"Ghana"));
        assert!(!teams.contains(&"Uzbekistan"));
        assert_eq!(bracket.len(), 16);
    }

    #[test]
    fn reported_round_of_16_fixture_contains_the_actual_survivors() {
        let teams = teams_in_knockout_ties(&reported_round_of_16_fixture());

        assert_eq!(
            teams,
            sorted_names([
                "Argentina",
                "Belgium",
                "Brazil",
                "Canada",
                "Colombia",
                "Egypt",
                "England",
                "France",
                "Mexico",
                "Morocco",
                "Norway",
                "Paraguay",
                "Portugal",
                "Spain",
                "Switzerland",
                "United States",
            ])
        );
    }

    #[test]
    fn reported_round_of_32_winners_match_round_of_16_teams() {
        assert_eq!(
            completed_knockout_winners(&reported_round_of_32_results()),
            teams_in_knockout_ties(&reported_round_of_16_fixture())
        );
    }

    #[test]
    fn reported_completed_round_of_32_results_have_sixteen_winners() {
        assert_eq!(
            completed_knockout_winners(&reported_round_of_32_results()).len(),
            16
        );
        let mut penalty_winners = reported_round_of_32_results()
            .iter()
            .filter_map(|result| result.winner.clone())
            .collect::<Vec<_>>();
        penalty_winners.sort();
        assert_eq!(penalty_winners, reported_round_of_32_penalty_winners());
    }

    #[test]
    fn reported_completed_round_of_16_results_have_seven_confirmed_winners() {
        assert_eq!(
            completed_knockout_winners(&reported_round_of_16_results()),
            sorted_names([
                "Argentina",
                "Belgium",
                "England",
                "France",
                "Morocco",
                "Norway",
                "Spain",
            ])
        );
    }

    #[test]
    fn knockout_winner_home_wins() {
        let m = bracket_match(73, 1, 2, 3, 1);
        assert_eq!(knockout_match_winner(&m), Some(1));
    }

    #[test]
    fn knockout_winner_away_wins() {
        let m = bracket_match(73, 1, 2, 0, 2);
        assert_eq!(knockout_match_winner(&m), Some(2));
    }

    #[test]
    fn knockout_winner_draw_no_penalties_returns_none() {
        let m = bracket_match(73, 1, 2, 1, 1);
        assert_eq!(knockout_match_winner(&m), None);
    }

    #[test]
    fn knockout_winner_draw_with_penalties_home_wins() {
        let m = bracket_match_with_penalties(73, 1, 2, 1, 1, 5, 3);
        assert_eq!(knockout_match_winner(&m), Some(1));
    }

    #[test]
    fn knockout_winner_draw_with_penalties_away_wins() {
        let m = bracket_match_with_penalties(73, 1, 2, 2, 2, 3, 4);
        assert_eq!(knockout_match_winner(&m), Some(2));
    }

    #[test]
    fn knockout_winner_not_played_returns_none() {
        let m = bracket_match(73, 1, 2, 0, 0);
        assert_eq!(knockout_match_winner(&m), None);
    }

    #[test]
    fn knockout_winner_placeholder_match_returns_none() {
        let m = empty_bracket_match(89);
        assert_eq!(knockout_match_winner(&m), None);
    }

    #[test]
    fn knockout_loser_home_wins_loser_is_away() {
        let m = bracket_match(73, 10, 20, 3, 1);
        assert_eq!(knockout_match_loser(&m), Some(20));
    }

    #[test]
    fn knockout_loser_away_wins_loser_is_home() {
        let m = bracket_match(73, 10, 20, 0, 2);
        assert_eq!(knockout_match_loser(&m), Some(10));
    }

    #[test]
    fn r32_match_73_feeds_r16_match_89_home() {
        assert_eq!(r32_feeds_r16(73), Some((89, true)));
    }

    #[test]
    fn r32_match_74_feeds_r16_match_89_away() {
        assert_eq!(r32_feeds_r16(74), Some((89, false)));
    }

    #[test]
    fn r32_match_75_feeds_r16_match_90_home() {
        assert_eq!(r32_feeds_r16(75), Some((90, true)));
    }

    #[test]
    fn r32_match_88_feeds_r16_match_96_away() {
        assert_eq!(r32_feeds_r16(88), Some((96, false)));
    }

    #[test]
    fn r32_all_16_matches_map_to_8_r16_matches() {
        let mut r16_slots: Vec<(i32, bool)> = Vec::new();
        for n in 73..=88 {
            r16_slots.push(r32_feeds_r16(n).unwrap());
        }
        for r16_n in 89..=96 {
            let home_count = r16_slots.iter().filter(|s| s.0 == r16_n && s.1).count();
            let away_count = r16_slots.iter().filter(|s| s.0 == r16_n && !s.1).count();
            assert_eq!(home_count, 1, "R16 match {} should have 1 home feeder", r16_n);
            assert_eq!(away_count, 1, "R16 match {} should have 1 away feeder", r16_n);
        }
    }

    #[test]
    fn r16_all_8_matches_map_to_4_qf_matches() {
        let mut qf_slots: Vec<(i32, bool)> = Vec::new();
        for n in 89..=96 {
            qf_slots.push(r16_feeds_qf(n).unwrap());
        }
        for qf_n in 97..=100 {
            let home_count = qf_slots.iter().filter(|s| s.0 == qf_n && s.1).count();
            let away_count = qf_slots.iter().filter(|s| s.0 == qf_n && !s.1).count();
            assert_eq!(home_count, 1, "QF match {} should have 1 home feeder", qf_n);
            assert_eq!(away_count, 1, "QF match {} should have 1 away feeder", qf_n);
        }
    }

    #[test]
    fn qf_all_4_matches_map_to_2_sf_matches() {
        let mut sf_slots: Vec<(i32, bool)> = Vec::new();
        for n in 97..=100 {
            sf_slots.push(qf_feeds_sf(n).unwrap());
        }
        for sf_n in 101..=102 {
            let home_count = sf_slots.iter().filter(|s| s.0 == sf_n && s.1).count();
            let away_count = sf_slots.iter().filter(|s| s.0 == sf_n && !s.1).count();
            assert_eq!(home_count, 1, "SF match {} should have 1 home feeder", sf_n);
            assert_eq!(away_count, 1, "SF match {} should have 1 away feeder", sf_n);
        }
    }

    #[test]
    fn sf_winners_go_to_final_losers_go_to_third_place() {
        let (final_n, third_n, is_home) = sf_feeds_final_and_third(101).unwrap();
        assert_eq!(final_n, 104);
        assert_eq!(third_n, 103);
        assert!(is_home);

        let (final_n2, third_n2, is_home2) = sf_feeds_final_and_third(102).unwrap();
        assert_eq!(final_n2, 104);
        assert_eq!(third_n2, 103);
        assert!(!is_home2);
    }

    #[test]
    fn advancement_target_r32_to_r16() {
        assert_eq!(advancement_target(73), Some((89, true)));
        assert_eq!(advancement_target(74), Some((89, false)));
    }

    #[test]
    fn advancement_target_r16_to_qf() {
        assert_eq!(advancement_target(89), Some((97, true)));
        assert_eq!(advancement_target(90), Some((97, false)));
    }

    #[test]
    fn advancement_target_qf_to_sf() {
        assert_eq!(advancement_target(97), Some((101, true)));
        assert_eq!(advancement_target(98), Some((101, false)));
    }

    #[test]
    fn advancement_target_sf_to_final() {
        assert_eq!(advancement_target(101), Some((104, true)));
        assert_eq!(advancement_target(102), Some((104, false)));
    }

    #[test]
    fn advancement_target_final_returns_none() {
        assert_eq!(advancement_target(104), None);
    }

    #[test]
    fn advancement_target_third_place_returns_none() {
        assert_eq!(advancement_target(103), None);
    }

    #[test]
    fn third_place_target_for_sf_matches() {
        assert_eq!(third_place_target(101), Some((103, true)));
        assert_eq!(third_place_target(102), Some((103, false)));
    }

    #[test]
    fn third_place_target_for_non_sf_returns_none() {
        assert_eq!(third_place_target(73), None);
        assert_eq!(third_place_target(89), None);
        assert_eq!(third_place_target(97), None);
        assert_eq!(third_place_target(104), None);
    }

    #[test]
    fn compute_advancements_from_single_r32_match() {
        let matches = vec![bracket_match(73, 1, 32, 2, 0)];
        let adv = compute_advancements(&matches);
        assert_eq!(adv.len(), 1);
        assert_eq!(adv[0], (89, true, 1));
    }

    #[test]
    fn compute_advancements_from_two_r32_matches_filling_one_r16() {
        let matches = vec![
            bracket_match(73, 1, 32, 2, 0),
            bracket_match(74, 2, 31, 0, 3),
        ];
        let adv = compute_advancements(&matches);
        assert_eq!(adv.len(), 2);
        assert!(adv.contains(&(89, true, 1)));
        assert!(adv.contains(&(89, false, 31)));
    }

    #[test]
    fn compute_advancements_ignores_unplayed_matches() {
        let matches = vec![
            bracket_match(73, 1, 32, 2, 0),
            bracket_match(74, 2, 31, 0, 0),
        ];
        let adv = compute_advancements(&matches);
        assert_eq!(adv.len(), 1);
        assert_eq!(adv[0], (89, true, 1));
    }

    #[test]
    fn compute_advancements_penalty_win() {
        let matches = vec![
            bracket_match_with_penalties(73, 1, 32, 1, 1, 4, 2),
        ];
        let adv = compute_advancements(&matches);
        assert_eq!(adv.len(), 1);
        assert_eq!(adv[0], (89, true, 1));
    }

    #[test]
    fn compute_advancements_draw_without_penalties_no_advancement() {
        let matches = vec![bracket_match(73, 1, 32, 1, 1)];
        let adv = compute_advancements(&matches);
        assert!(adv.is_empty());
    }

    #[test]
    fn compute_advancements_sf_produces_both_final_and_third_place() {
        let matches = vec![
            bracket_match(101, 10, 20, 3, 1),
            bracket_match(102, 30, 40, 0, 2),
        ];
        let adv = compute_advancements(&matches);
        assert_eq!(adv.len(), 4);
        assert!(adv.contains(&(104, true, 10)));
        assert!(adv.contains(&(104, false, 40)));
        assert!(adv.contains(&(103, true, 20)));
        assert!(adv.contains(&(103, false, 30)));
    }

    #[test]
    fn compute_advancements_full_r32_produces_16_r16_slots() {
        let matches: Vec<_> = (0..16)
            .map(|i| {
                let match_number = 73 + i as i32;
                let home_id = (i + 1) as u64;
                let away_id = (32 - i) as u64;
                bracket_match(match_number, home_id, away_id, 2, 0)
            })
            .collect();
        let adv = compute_advancements(&matches);
        assert_eq!(adv.len(), 16);
        for r16_n in 89..=96 {
            let home_count = adv.iter().filter(|a| a.0 == r16_n && a.1).count();
            let away_count = adv.iter().filter(|a| a.0 == r16_n && !a.1).count();
            assert_eq!(home_count, 1, "R16-{} home slot", r16_n);
            assert_eq!(away_count, 1, "R16-{} away slot", r16_n);
        }
    }

    #[test]
    fn full_bracket_traversal_r32_to_final() {
        let mut all_matches = Vec::new();
        for i in 0..16 {
            let match_number = 73 + i as i32;
            let home_id = (i + 1) as u64;
            let away_id = (32 - i) as u64;
            all_matches.push(bracket_match(match_number, home_id, away_id, 2, 0));
        }
        let r32_adv = compute_advancements(&all_matches);
        let mut r16_matches: Vec<KnockoutBracketMatch> = (89..=96)
            .map(|n| empty_bracket_match(n))
            .collect();
        for (target, is_home, team_id) in &r32_adv {
            if let Some(m) = r16_matches.iter_mut().find(|m| m.match_number == *target) {
                if *is_home { m.home_team_id = *team_id; } else { m.away_team_id = *team_id; }
            }
        }
        for m in r16_matches.iter_mut() {
            if m.home_team_id != 0 && m.away_team_id != 0 {
                if m.home_team_id < m.away_team_id { m.home_score = 2; } else { m.away_score = 2; }
            }
        }
        all_matches.extend(r16_matches.iter().cloned());
        let r16_adv = compute_advancements(&r16_matches);
        let mut qf_matches: Vec<KnockoutBracketMatch> = (97..=100)
            .map(|n| empty_bracket_match(n))
            .collect();
        for (target, is_home, team_id) in &r16_adv {
            if let Some(m) = qf_matches.iter_mut().find(|m| m.match_number == *target) {
                if *is_home { m.home_team_id = *team_id; } else { m.away_team_id = *team_id; }
            }
        }
        for m in qf_matches.iter_mut() {
            if m.home_team_id != 0 && m.away_team_id != 0 {
                if m.home_team_id < m.away_team_id { m.home_score = 2; } else { m.away_score = 2; }
            }
        }
        all_matches.extend(qf_matches.iter().cloned());
        let qf_adv = compute_advancements(&qf_matches);
        let mut sf_matches: Vec<KnockoutBracketMatch> = (101..=102)
            .map(|n| empty_bracket_match(n))
            .collect();
        for (target, is_home, team_id) in &qf_adv {
            if let Some(m) = sf_matches.iter_mut().find(|m| m.match_number == *target) {
                if *is_home { m.home_team_id = *team_id; } else { m.away_team_id = *team_id; }
            }
        }
        for m in sf_matches.iter_mut() {
            if m.home_team_id != 0 && m.away_team_id != 0 {
                if m.home_team_id < m.away_team_id { m.home_score = 2; } else { m.away_score = 2; }
            }
        }
        all_matches.extend(sf_matches.iter().cloned());
        let sf_adv = compute_advancements(&sf_matches);
        let mut final_match = empty_bracket_match(104);
        let mut third_match = empty_bracket_match(103);
        for (target, is_home, team_id) in &sf_adv {
            let m = if *target == 104 { &mut final_match } else { &mut third_match };
            if *is_home { m.home_team_id = *team_id; } else { m.away_team_id = *team_id; }
        }
        assert!(final_match.home_team_id == 1 || final_match.away_team_id == 1,
            "Team 1 (best seed) should reach the final, got home={} away={}",
            final_match.home_team_id, final_match.away_team_id);
        // Both finalists must be filled
        assert!(final_match.home_team_id != 0 && final_match.away_team_id != 0,
            "Both final slots should be filled");
        // Third place match should have two different non-zero teams
        assert!(third_match.home_team_id != 0 && third_match.away_team_id != 0,
            "Both third place slots should be filled");
        assert_ne!(third_match.home_team_id, third_match.away_team_id,
            "Third place teams should be different");
        // The 4 semi-final outputs should be disjoint from each other
        let mut all_four = vec![
            final_match.home_team_id, final_match.away_team_id,
            third_match.home_team_id, third_match.away_team_id,
        ];
        all_four.sort();
        all_four.dedup();
        assert_eq!(all_four.len(), 4, "Final + 3rd place should have 4 distinct teams");
    }

    #[test]
    fn stage_from_match_number_covers_all_ranges() {
        for n in 73..=88 { assert_eq!(KnockoutStage::from_match_number(n), Some(KnockoutStage::R32)); }
        for n in 89..=96 { assert_eq!(KnockoutStage::from_match_number(n), Some(KnockoutStage::R16)); }
        for n in 97..=100 { assert_eq!(KnockoutStage::from_match_number(n), Some(KnockoutStage::QF)); }
        assert_eq!(KnockoutStage::from_match_number(101), Some(KnockoutStage::SF));
        assert_eq!(KnockoutStage::from_match_number(102), Some(KnockoutStage::SF));
        assert_eq!(KnockoutStage::from_match_number(103), Some(KnockoutStage::ThirdPlace));
        assert_eq!(KnockoutStage::from_match_number(104), Some(KnockoutStage::Final));
        assert_eq!(KnockoutStage::from_match_number(72), None);
        assert_eq!(KnockoutStage::from_match_number(105), None);
    }

    #[test]
    fn out_of_range_match_numbers_return_none() {
        assert_eq!(r32_feeds_r16(72), None);
        assert_eq!(r32_feeds_r16(89), None);
        assert_eq!(r16_feeds_qf(88), None);
        assert_eq!(r16_feeds_qf(97), None);
        assert_eq!(qf_feeds_sf(96), None);
        assert_eq!(qf_feeds_sf(101), None);
        assert_eq!(sf_feeds_final_and_third(100), None);
        assert_eq!(sf_feeds_final_and_third(103), None);
    }
}
