use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use string_interner::{DefaultSymbol, StringInterner};

type Client = DefaultSymbol;

// Two weeks.
const LIMIT: usize = 225 * 32 * 14;

#[derive(Clone, Default)]
pub struct AccuracyTracker {
    nodes_by_client: BTreeMap<Client, BTreeMap<String, NodeAccuracy>>,
    interner: StringInterner,
}

#[derive(Clone)]
pub struct NodeAccuracy {
    observation_limit: usize,
    observations: BTreeSet<Observation>,
}

/// Observation for a block produced by a specific node.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Observation {
    /// The slot of the block produced.
    slot: u64,
    /// The client that the block was classified as.
    classified_as: Client,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Summary {
    clients: BTreeMap<String, AggregateSummary>,
    nodes: Vec<NodeSummary>,
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct AggregateSummary {
    true_positives: usize,
    true_negatives: usize,
    false_positives: usize,
    false_negatives: usize,
    false_negatives_detail: FalseNegatives,
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct NodeSummary {
    name: String,
    label: String,
    true_positives: usize,
    false_negatives: BTreeMap<String, usize>,
    latest_slot: Option<u64>,
}

type FalseNegatives = BTreeMap<String, usize>;

impl AccuracyTracker {
    pub fn record_block(
        &mut self,
        node_name: String,
        true_label: String,
        classified_as_name: String,
        slot: u64,
    ) {
        let true_client = self.interner.get_or_intern(true_label);
        let classified_as = self.interner.get_or_intern(classified_as_name);

        let node = self
            .nodes_by_client
            .entry(true_client)
            .or_default()
            .entry(node_name)
            .or_insert_with(|| NodeAccuracy::new(LIMIT));

        node.observations.insert(Observation {
            slot,
            classified_as,
        });

        // Prune.
        while node.observations.len() > node.observation_limit {
            node.observations.pop_first();
        }
    }

    pub fn summarise(&self) -> Option<Summary> {
        let mut nodes = vec![];
        for (client, node_name, node_accuracy) in self
            .nodes_by_client
            .iter()
            .flat_map(|(client, nodes)| nodes.iter().map(move |(name, node)| (*client, name, node)))
        {
            let label = self.interner.resolve(client)?.to_string();

            let mut true_positives = 0;
            let mut false_negatives = BTreeMap::new();
            let mut latest_slot = None;

            for obs in &node_accuracy.observations {
                if obs.classified_as == client {
                    true_positives += 1;
                } else {
                    let classified_as = self.interner.resolve(obs.classified_as)?;
                    let count = false_negatives
                        .entry(classified_as.to_string())
                        .or_default();
                    *count += 1;
                }
                latest_slot = Some(obs.slot);
            }

            nodes.push(NodeSummary {
                name: node_name.clone(),
                label,
                true_positives,
                false_negatives,
                latest_slot,
            });
        }

        let mut clients: BTreeMap<String, AggregateSummary> = BTreeMap::new();

        for node in &nodes {
            let client1 = &node.label;
            let client1_summary = clients.entry(client1.clone()).or_default();
            client1_summary.true_positives += node.true_positives;

            for (client2, count) in &node.false_negatives {
                // Update false negative count for client1.
                let client1_summary = clients.entry(client1.clone()).or_default();
                *client1_summary
                    .false_negatives_detail
                    .entry(client2.clone())
                    .or_default() += *count;
                client1_summary.false_negatives += *count;

                // Update false positive count for client2.
                let client2_summary = clients.entry(client2.clone()).or_default();
                client2_summary.false_positives += *count;
            }
        }

        // Compute true negative counts.
        let mut true_negatives = BTreeMap::new();
        for (client1, _) in &clients {
            let mut num_true_negatives = 0;

            for (client2, summary2) in &clients {
                if client1 == client2 {
                    continue;
                }
                // Every correct classification of client2 is a true negative for client1.
                num_true_negatives += summary2.true_positives;
                // Every misclassification of client2 as client3 (!= client1) is a true negative
                // for client1.
                num_true_negatives += summary2
                    .false_negatives_detail
                    .iter()
                    .filter(|(client3, _)| *client3 != client1)
                    .map(|(_, count)| count)
                    .sum::<usize>();
            }

            true_negatives.insert(client1.clone(), num_true_negatives);
        }

        for (client, summary) in &mut clients {
            summary.true_negatives = true_negatives[client];
        }

        Some(Summary { clients, nodes })
    }
}

impl NodeAccuracy {
    pub fn new(limit: usize) -> Self {
        Self {
            observation_limit: limit,
            observations: Default::default(),
        }
    }
}
