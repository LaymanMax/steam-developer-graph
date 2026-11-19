pub mod manager;
pub mod matcher;
pub mod normalizer;

pub use manager::EntityResolutionManager;
pub use matcher::{MatchCandidate, evaluate_similarity};
pub use normalizer::{
    candidate_stem, normalize_conservative, normalize_key, parse_delimited_entities,
};
