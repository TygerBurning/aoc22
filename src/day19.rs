use std::{collections::HashSet, hash::Hash};

use regex::Regex;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
    Noop,
}

#[derive(Debug)]
struct Blueprint {
    ore_cost: u32,
    clay_cost: u32,
    obsidian_cost_ore_clay: (u32, u32),
    geode_cost_ore_obsidian: (u32, u32),
}

#[derive(Debug, Clone, Default)]
struct Workforce {
    ore_producers: u32,
    clay_producers: u32,
    obsidian_producers: u32,
    geode_producers: u32,
}

#[derive(Debug, Clone)]
struct Inventory {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let mut bs = vec![];
    let re = Regex::new(r"Blueprint \d*: Each ore robot costs (\d*) ore. Each clay robot costs (\d*) ore. Each obsidian robot costs (\d*) ore and (\d*) clay. Each geode robot costs (\d*) ore and (\d*) obsidian.").unwrap();
    for s in input.lines() {
        let cap = re.captures(s).unwrap();
        bs.push(Blueprint {
            ore_cost: cap[1].parse().unwrap(),
            clay_cost: cap[2].parse().unwrap(),
            obsidian_cost_ore_clay: (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            geode_cost_ore_obsidian: (cap[5].parse().unwrap(), cap[6].parse().unwrap()),
        });
    }
    bs
}

fn possible_actions(b: &Blueprint, i: &Inventory, w: &Workforce) -> HashSet<Material> {
    // We only need to build robots until we are producing a decent amount per turn.
    let ore_workers_needed = vec![
        b.ore_cost,
        b.clay_cost,
        b.obsidian_cost_ore_clay.0,
        b.geode_cost_ore_obsidian.0,
    ];
    let max_ore_workers_needed = ore_workers_needed.iter().max().unwrap();

    let mut hs = HashSet::new();
    if b.ore_cost <= i.ore && &w.ore_producers < max_ore_workers_needed {
        hs.insert(Material::Ore);
    }
    if b.clay_cost <= i.ore && w.clay_producers < b.obsidian_cost_ore_clay.1 {
        hs.insert(Material::Clay);
    }
    if b.obsidian_cost_ore_clay.0 <= i.ore
        && b.obsidian_cost_ore_clay.1 <= i.clay
        && w.obsidian_producers < b.geode_cost_ore_obsidian.1
    {
        hs.insert(Material::Obsidian);
    }
    if b.geode_cost_ore_obsidian.0 <= i.ore && b.geode_cost_ore_obsidian.1 <= i.obsidian {
        hs.insert(Material::Geode);
    }

    // It's a stupid idea to do nothing, if we can do everything possible.

    // We can always do ore robots and clay robots.
    let mut possible_actions = 2;
    if w.clay_producers > 1 {
        // If we have a clay producer, we can build obsidian too
        possible_actions += 1;
    }
    if w.obsidian_producers > 1 {
        // If we have obsidian producer, we can build geodes too
        possible_actions += 1;
    }
    if hs.len() < possible_actions {
        hs.insert(Material::Noop);
    }

    hs
}

fn take_action(b: &Blueprint, i: &mut Inventory, w: &mut Workforce, choice: &Material) {
    match choice {
        Material::Ore => i.ore -= b.ore_cost,
        Material::Clay => i.ore -= b.clay_cost,
        Material::Obsidian => {
            i.ore -= b.obsidian_cost_ore_clay.0;
            i.clay -= b.obsidian_cost_ore_clay.1;
        }
        Material::Geode => {
            i.ore -= b.geode_cost_ore_obsidian.0;
            i.obsidian -= b.geode_cost_ore_obsidian.1;
        }
        Material::Noop => {}
    }
    build(w, choice);
}

fn accumulate(i: &mut Inventory, w: &Workforce) {
    i.ore += w.ore_producers;
    i.clay += w.clay_producers;
    i.obsidian += w.obsidian_producers;
    i.geode += w.geode_producers;
}

fn build(w: &mut Workforce, m: &Material) {
    match m {
        Material::Ore => w.ore_producers += 1,
        Material::Clay => w.clay_producers += 1,
        Material::Obsidian => w.obsidian_producers += 1,
        Material::Geode => w.geode_producers += 1,
        Material::Noop => {}
    }
}

fn recurse(
    b: &Blueprint,
    w: &mut Workforce,
    i: &mut Inventory,
    turns: &mut u32,
    actions_processed: Vec<Material>,
    excluded_actions: HashSet<Material>,
    best_geode_seen: &mut u32,
) -> u32 {
    if *turns == 0 {
        return i.geode;
    }

    // How many geodes could we possible build, if we built a geode producer now and every turn?
    let possible_geodes = *turns * (*turns + 1) / 2 + i.geode + (w.geode_producers * *turns);
    if *best_geode_seen > possible_geodes {
        return 0;
    }
    // Work out possible actions before we accumulate.
    let mut actions = possible_actions(b, &i, &w);

    for a in &excluded_actions {
        actions.remove(a);
    }

    accumulate(i, &w);
    *turns -= 1;

    let mut max = 0;
    for action in &actions {
        let mut candidate_workforce = w.clone();
        let mut candidate_inventory = i.clone();

        // Don't let us do any action tomorrow if we chose not to do today (apart from No-op)
        let restricted_actions = if action != &Material::Noop {
            HashSet::new()
        } else {
            let mut ea = excluded_actions.clone();
            for a in &actions {
                ea.insert(a.clone());
            }
            ea.remove(&Material::Noop);
            ea
        };

        let mut candidate_actions = actions_processed.clone();
        candidate_actions.push(action.clone());

        take_action(
            b,
            &mut candidate_inventory,
            &mut candidate_workforce,
            action,
        );
        let max_geodes = recurse(
            b,
            &mut candidate_workforce,
            &mut candidate_inventory,
            &mut turns.clone(),
            candidate_actions,
            restricted_actions,
            best_geode_seen,
        );

        if max_geodes > max {
            *best_geode_seen = max_geodes;
            max = max_geodes;
        }
    }
    max
}

fn solve_blueprint(b: &Blueprint, turns: &mut u32) -> u32 {
    let mut w = Workforce {
        ore_producers: 1,
        clay_producers: 0,
        obsidian_producers: 0,
        geode_producers: 0,
    };

    let mut i = Inventory {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };

    let mut best_geode_seen = 0;

    recurse(
        b,
        &mut w,
        &mut i,
        turns,
        vec![],
        HashSet::new(),
        &mut best_geode_seen,
    )
}

pub fn day19() {
    let input = include_str!("../inputs/day19.txt");
    let bs = parse_input(input);

    let mut part_a = 0;
    for (i, b) in bs.iter().enumerate() {
        let mut turns = 24;
        println!("> Blueprint {}:", i + 1);
        println!("{:?}", b);
        let geodes = solve_blueprint(&b, &mut turns);
        part_a += i as u32 * geodes as u32;
    }
    println!("Part A is: {:?}", part_a);

    let mut part_b = 1;
    for (i, b) in bs.iter().take(3).enumerate() {
        let mut turns = 32;
        println!("> Blueprint {}:", i + 1);
        println!("{:?}", b);
        let geodes = solve_blueprint(&b, &mut turns);
        part_b *= geodes as u32;
    }
    assert!(part_b > 1664);
    println!("Part B is: {:?}", part_b)
}

#[test]
fn sample_input_1() {
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
    let b = parse_input(input);
    let mut turns = 24;
    let geodes = solve_blueprint(&b[0], &mut turns);
    assert_eq!(geodes, 9);

    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
    let b = parse_input(input);
    let mut turns = 32;
    let geodes = solve_blueprint(&b[0], &mut turns);
    assert_eq!(geodes, 56);
}

#[test]
fn sample_input_2() {
    let input = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    let b = parse_input(input);
    let mut turns = 24;
    let geodes = solve_blueprint(&b[0], &mut turns);
    assert_eq!(geodes, 12);
}

#[test]
fn real_input_30() {
    let input = "Blueprint 30: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 13 obsidian.";
    let b = parse_input(input);
    let mut turns = 24;
    let geodes = solve_blueprint(&b[0], &mut turns);
    assert_eq!(geodes, 0);
}
