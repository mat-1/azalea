use std::{cmp::Reverse, collections::HashMap, fmt::Debug, hash::Hash, ops::Add};

use priority_queue::PriorityQueue;

pub fn a_star<P, M, W, HeuristicFn, SuccessorsFn, SuccessFn>(
    start: P,
    heuristic: HeuristicFn,
    successors: SuccessorsFn,
    success: SuccessFn,
) -> Option<Vec<Movement<P, M>>>
where
    P: Eq + Hash + Copy + Debug,
    W: PartialOrd + Default + Copy + num_traits::Bounded + Debug + Add<Output = W>,
    HeuristicFn: Fn(P) -> W,
    SuccessorsFn: Fn(P) -> Vec<Edge<P, M, W>>,
    SuccessFn: Fn(P) -> bool,
{
    let mut open_set = PriorityQueue::new();
    open_set.push(start, Reverse(Weight(W::default())));
    let mut nodes: HashMap<P, Node<P, M, W>> = HashMap::new();
    nodes.insert(
        start,
        Node {
            position: start,
            movement_data: None,
            came_from: None,
            g_score: W::default(),
            f_score: W::max_value(),
        },
    );

    while let Some((current_node, _)) = open_set.pop() {
        if success(current_node) {
            return Some(reconstruct_path(nodes, current_node));
        }

        let current_g_score = nodes
            .get(&current_node)
            .map(|n| n.g_score)
            .unwrap_or(W::max_value());

        for neighbor in successors(current_node) {
            let tentative_g_score = current_g_score + neighbor.cost;
            let neighbor_g_score = nodes
                .get(&neighbor.movement.target)
                .map(|n| n.g_score)
                .unwrap_or(W::max_value());
            if tentative_g_score < neighbor_g_score {
                let f_score = tentative_g_score + heuristic(neighbor.movement.target);
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
            }
        }
    }

    None
}

fn reconstruct_path<P, M, W>(
    mut nodes: HashMap<P, Node<P, M, W>>,
    current: P,
) -> Vec<Movement<P, M>>
where
    P: Eq + Hash + Copy + Debug,
    W: PartialOrd + Default + Copy + num_traits::Bounded + Debug,
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

pub struct Node<P, M, W> {
    pub position: P,
    pub movement_data: Option<M>,
    pub came_from: Option<P>,
    pub g_score: W,
    pub f_score: W,
}

pub struct Edge<P: Hash + Copy, M, W: PartialOrd + Copy> {
    pub movement: Movement<P, M>,
    pub cost: W,
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
pub struct Weight<W: PartialOrd + Debug>(W);
impl<W: PartialOrd + Debug> Ord for Weight<W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}
impl<W: PartialOrd + Debug> Eq for Weight<W> {}
impl<W: PartialOrd + Debug> PartialOrd for Weight<W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
