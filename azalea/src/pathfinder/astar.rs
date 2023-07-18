use std::{
    cmp::Reverse,
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    time::{Duration, Instant},
};

use priority_queue::PriorityQueue;

pub struct Path<P, M>
where
    P: Eq + Hash + Copy + Debug,
{
    pub movements: Vec<Movement<P, M>>,
    pub partial: bool,
}

pub fn a_star<P, M, HeuristicFn, SuccessorsFn, SuccessFn>(
    start: P,
    heuristic: HeuristicFn,
    successors: SuccessorsFn,
    success: SuccessFn,
    timeout: Duration,
) -> Path<P, M>
where
    P: Eq + Hash + Copy + Debug,
    HeuristicFn: Fn(P) -> f32,
    SuccessorsFn: Fn(P) -> Vec<Edge<P, M>>,
    SuccessFn: Fn(P) -> bool,
{
    let start_time = Instant::now();

    let mut open_set = PriorityQueue::new();
    open_set.push(start, Reverse(Weight(0.)));
    let mut nodes: HashMap<P, Node<P, M>> = HashMap::new();
    nodes.insert(
        start,
        Node {
            position: start,
            movement_data: None,
            came_from: None,
            g_score: f32::default(),
            f_score: f32::MAX,
        },
    );

    let mut best_node = start;
    let mut best_node_score = heuristic(start);

    while let Some((current_node, _)) = open_set.pop() {
        if success(current_node) {
            return Path {
                movements: reconstruct_path(nodes, current_node),
                partial: false,
            };
        }

        let current_g_score = nodes
            .get(&current_node)
            .map(|n| n.g_score)
            .unwrap_or(f32::MAX);

        for neighbor in successors(current_node) {
            let tentative_g_score = current_g_score + neighbor.cost;
            let neighbor_g_score = nodes
                .get(&neighbor.movement.target)
                .map(|n| n.g_score)
                .unwrap_or(f32::MAX);
            if tentative_g_score < neighbor_g_score {
                let heuristic = heuristic(neighbor.movement.target);
                let f_score = tentative_g_score + heuristic;
                nodes.insert(
                    neighbor.movement.target,
                    Node {
                        position: neighbor.movement.target,
                        movement_data: Some(neighbor.movement.data),
                        came_from: Some(current_node),
                        g_score: tentative_g_score,
                        f_score,
                    },
                );
                open_set.push(neighbor.movement.target, Reverse(Weight(f_score)));

                let node_score = heuristic + tentative_g_score / 1.5;
                if node_score < best_node_score {
                    best_node = neighbor.movement.target;
                    best_node_score = node_score;
                }
            }
        }

        if start_time.elapsed() > timeout {
            // timeout, just return the best path we have so far
            println!("timeout");
            break;
        }
    }

    Path {
        movements: reconstruct_path(nodes, best_node),
        partial: true,
    }
}

fn reconstruct_path<P, M>(mut nodes: HashMap<P, Node<P, M>>, current: P) -> Vec<Movement<P, M>>
where
    P: Eq + Hash + Copy + Debug,
{
    let mut path = Vec::new();
    let mut current = current;
    while let Some(node) = nodes.remove(&current) {
        if let Some(came_from) = node.came_from {
            current = came_from;
        } else {
            break;
        }
        path.push(Movement {
            target: node.position,
            data: node.movement_data.unwrap(),
        });
    }
    path.reverse();
    path
}

pub struct Node<P, M> {
    pub position: P,
    pub movement_data: Option<M>,
    pub came_from: Option<P>,
    pub g_score: f32,
    pub f_score: f32,
}

pub struct Edge<P: Hash + Copy, M> {
    pub movement: Movement<P, M>,
    pub cost: f32,
}

pub struct Movement<P: Hash + Copy, M> {
    pub target: P,
    pub data: M,
}

impl<P: Hash + Copy + Debug, M: Debug> Debug for Movement<P, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Movement")
            .field("target", &self.target)
            .field("data", &self.data)
            .finish()
    }
}
impl<P: Hash + Copy + Clone, M: Clone> Clone for Movement<P, M> {
    fn clone(&self) -> Self {
        Self {
            target: self.target.clone(),
            data: self.data.clone(),
        }
    }
}

#[derive(PartialEq)]
pub struct Weight(f32);
impl Ord for Weight {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}
impl Eq for Weight {}
impl PartialOrd for Weight {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
