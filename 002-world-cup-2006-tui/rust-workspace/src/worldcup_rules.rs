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
}
