use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccuracyTracker {
    client_accuracy: BTreeMap<String, Accuracy>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Accuracy {
    pub num_blocks: usize,
    pub num_correct: usize,
    pub misclassifications: DetailedMisclassifications,
}

/*
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SuccinctAccuracy {
    pub num_blocks: usize,
    pub num_correct: usize,
    pub misclassifications: Misclassifications,
}
*/

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DetailedMisclassifications {
    by_id: BTreeMap<String, Misclassifications>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Misclassifications {
    by_client: BTreeMap<String, usize>,
}

impl AccuracyTracker {
    pub fn record_block(&mut self, node_id: String, true_label: String, classified_as: String) {
        let correct = true_label == classified_as;
        let mut accuracy = self
            .client_accuracy
            .entry(true_label)
            .or_insert_with(Accuracy::default);

        accuracy.num_blocks += 1;

        if correct {
            accuracy.num_correct += 1;
        } else {
            *accuracy
                .misclassifications
                .by_id
                .entry(node_id)
                .or_insert_with(Misclassifications::default)
                .by_client
                .entry(classified_as)
                .or_insert(0) += 1;
        }
    }
}
