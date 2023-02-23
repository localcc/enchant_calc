use std::{
    borrow::Borrow,
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use serde::{Deserialize, Serialize};

use crate::registry;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Enchant {
    pub enchant: registry::Enchant,
    pub level: u32,
    score: u32,
}

impl Enchant {
    pub fn new(enchant: registry::Enchant, level: u32) -> Self {
        let score = enchant.weight * level;
        Self {
            enchant,
            level,
            score,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Path {
    pub cost: u32,
    pub max_cost: u32,
    pub remaining: Vec<Arc<ItemKeyCombination>>,
    pub steps: Vec<Step>,
}

impl Path {
    pub fn with_capacity(cost: u32, max_cost: u32, remaining: usize, steps: usize) -> Path {
        Path {
            cost,
            max_cost,
            remaining: Vec::with_capacity(remaining),
            steps: Vec::with_capacity(steps),
        }
    }

    pub fn is_more_effective(&self, other: &Path) -> bool {
        self.cost < other.cost || (self.cost == other.cost && self.max_cost < other.cost)
    }

    pub fn explode(&self) -> (Vec<Path>, u32) {
        let mut best_paths: HashMap<u64, Path> = HashMap::new();
        let mut tries = 0;

        let len = self.remaining.len();

        for left in 0..len {
            for right in 0..len {
                if right == left {
                    continue;
                }

                if self.remaining[right]
                    .combination
                    .first()
                    .map(|e| e == &ItemKey::Item)
                    .unwrap_or(false)
                {
                    continue;
                }

                let mut new_path =
                    Path::with_capacity(0, 0, self.remaining.capacity(), self.steps.capacity());

                for i in 0..len {
                    if i != left && i != right {
                        new_path.remaining.push(self.remaining[i].clone());
                    }
                }

                let left_item = &self.remaining[left];
                let right_item = &self.remaining[right];

                let combined = left_item.combine(right_item);

                new_path.remaining.push(Arc::new(combined));
                new_path.remaining.sort();

                new_path.steps.reserve(self.steps.len() + 1);
                new_path.steps.extend(self.steps.clone());
                new_path.steps.push(Step {
                    left: left_item.clone(),
                    right: right_item.clone(),
                });

                let step_cost_enchants = right_item.cost();
                let step_cost_penalties = left_item.use_penalty() + right_item.use_penalty();

                new_path.cost = self.cost + step_cost_enchants + step_cost_penalties;
                new_path.max_cost =
                    u32::max(step_cost_enchants + step_cost_penalties, self.max_cost);

                tries += 1;

                let mut hasher = DefaultHasher::new();
                new_path.remaining.hash(&mut hasher);

                let flat_key = hasher.finish();

                if best_paths.contains_key(&flat_key) {
                    if new_path.is_more_effective(&best_paths[&flat_key]) {
                        best_paths.remove(&flat_key);
                        best_paths.insert(flat_key, new_path);
                    }
                } else {
                    best_paths.insert(flat_key, new_path);
                }
            }
        }

        let mut out = Vec::new();
        for (_, path) in best_paths {
            out.push(path);
        }

        (out, tries)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum ItemKey {
    Item,
    Enchant(u32, u32),
}

impl PartialOrd for ItemKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self {
            ItemKey::Item => match *other == ItemKey::Item {
                true => std::cmp::Ordering::Equal,
                false => std::cmp::Ordering::Less,
            },
            ItemKey::Enchant(_, insertion_index) => match other {
                ItemKey::Item => std::cmp::Ordering::Greater,
                ItemKey::Enchant(_, other_insertion_index) => {
                    insertion_index.cmp(other_insertion_index)
                }
            },
        })
    }
}

impl Ord for ItemKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct ItemKeyCombination {
    pub combination: Vec<ItemKey>,
    pub anvil_use_count: u32,
}

impl ItemKeyCombination {
    pub fn single_item(key: ItemKey, anvil_use_count: u32) -> Self {
        ItemKeyCombination {
            combination: Vec::from([key]),
            anvil_use_count,
        }
    }

    pub fn combine(&self, other: &ItemKeyCombination) -> Self {
        let mut combined = self.clone();
        combined.combination.extend(other.combination.clone());
        combined.combination.sort();

        let new_use_count = u32::max(self.anvil_use_count, other.anvil_use_count) + 1;
        combined.anvil_use_count = new_use_count;

        combined
    }

    pub fn use_penalty(&self) -> u32 {
        2_u32.pow(self.anvil_use_count).saturating_sub(1)
    }

    pub fn cost(&self) -> u32 {
        let mut total = 0;
        for combined in &self.combination {
            if let ItemKey::Enchant(cost, _) = combined.borrow() {
                total += cost;
            } else {
                panic!("Item on the right side");
            }
        }
        total
    }
}

#[derive(Debug, Clone)]
struct Step {
    pub left: Arc<ItemKeyCombination>,
    pub right: Arc<ItemKeyCombination>,
}

pub struct Solver<'enchants> {
    enchants: &'enchants [Enchant],
    items: Vec<Arc<ItemKeyCombination>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ResolvedStepItem {
    Item,
    Enchant(Enchant),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResolvedStep {
    pub left: Vec<ResolvedStepItem>,
    pub right: Vec<ResolvedStepItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResolvedPath {
    pub cost: u32,
    pub steps: Vec<ResolvedStep>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SolverResult {
    pub path: Option<ResolvedPath>,
    pub paths_tried: u32,
}

impl<'enchants> Solver<'enchants> {
    pub fn new(enchants: &'enchants [Enchant]) -> Self {
        let mut items = Vec::from([Arc::new(ItemKeyCombination::single_item(ItemKey::Item, 0))]);

        for (index, enchant) in enchants.iter().enumerate() {
            items.push(Arc::new(ItemKeyCombination::single_item(
                ItemKey::Enchant(enchant.score, index as u32),
                0,
            )));
        }

        Solver { enchants, items }
    }

    fn resolve_item_key(&self, item_key: &ItemKey) -> ResolvedStepItem {
        match item_key {
            ItemKey::Item => ResolvedStepItem::Item,
            ItemKey::Enchant(_, index) => {
                let enchant = &self.enchants[*index as usize];
                ResolvedStepItem::Enchant(enchant.clone())
            }
        }
    }

    fn resolve_combination(&self, combination: &ItemKeyCombination) -> Vec<ResolvedStepItem> {
        combination
            .combination
            .iter()
            .map(|e| self.resolve_item_key(e))
            .collect::<Vec<_>>()
    }

    fn solve_inner(
        incomplete_path: &Path,
        best_path: &mut Option<Path>,
        path_explored_callback: &mut impl FnMut(u32),
    ) {
        let (paths, tried) = incomplete_path.explode();
        for path in paths {
            if path.remaining.len() > 1 {
                Solver::solve_inner(&path, best_path, path_explored_callback);
            } else {
                path_explored_callback(tried);

                if best_path
                    .as_ref()
                    .map(|e| path.is_more_effective(e))
                    .unwrap_or(true)
                {
                    *best_path = Some(path)
                }
            }
        }
    }

    pub fn solve(&self, mut path_explored_callback: impl FnMut(u32)) -> SolverResult {
        let incomplete_path = Path {
            cost: 0,
            max_cost: 0,
            remaining: Vec::from_iter(self.items.clone()),
            steps: Vec::with_capacity(self.items.len()),
        };
        let mut best_path: Option<Path> = None;
        let paths_tried = Arc::new(AtomicU32::new(0));

        Solver::solve_inner(
            &incomplete_path,
            &mut best_path,
            &mut path_explored_callback,
        );

        let resolved_path = if let Some(path) = best_path {
            let mut steps = Vec::new();

            for step in &path.steps {
                let left = self.resolve_combination(&step.left);
                let right = self.resolve_combination(&step.right);

                steps.push(ResolvedStep { left, right })
            }

            Some(ResolvedPath {
                cost: path.cost,
                steps,
            })
        } else {
            None
        };

        SolverResult {
            path: resolved_path,
            paths_tried: paths_tried.load(Ordering::Relaxed),
        }
    }
}
