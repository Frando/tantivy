use crate::query::bm25::{BM25Params, BM25Weight};
use crate::query::Explanation;
use crate::Score;
use crate::Searcher;
use crate::Term;

/// Scoring function trait
///
/// See (BM25Weight)[./struct.BM25Weight.html'] for an example.
pub trait ScoringFunction: Sync + Send + ScoringFunctionClone + 'static {
    /// Score a given document.
    fn score(&self, fieldnorm_id: u8, term_freq: u32) -> Score;

    /// Returns an `Explanation` for the given document.
    fn explain(&self, fieldnorm_id: u8, term_freq: u32) -> Explanation;
}

pub trait ScoringParams: Sync + Send + 'static {}

pub type ScoringFunctionForTerms = fn(
    searcher: &Searcher,
    terms: &[Term],
    params: Option<ScoringParams>,
) -> Box<dyn ScoringFunction>;

// Helper trait to allow cloning dyn trait objects.
// See https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object/30353928#30353928
// for why this is needed.
pub trait ScoringFunctionClone {
    fn clone_box(&self) -> Box<ScoringFunction>;
}

pub struct ScoringSettings {
    enabled: bool,
    scoringfunction: Option<Box<Fn(&Searcher, &[Term], Option<Box<dyn ScoringParams>>)>>,
    params: Option<Box<dyn ScoringParams>>,
}

impl ScoringSettings {
    pub fn bm25() -> ScoringSettings {
        ScoringSettings {
            enabled: true,
            scoringfunction: BM25Weight::for_terms,
            params: Box::new(BM25Params::default()),
        }
    }
}

impl Default for ScoringSettings {
    fn default() -> Self {
        Self::bm25()
    }
}

impl<T> ScoringFunctionClone for T
where
    T: 'static + ScoringFunction + Clone,
{
    fn clone_box(&self) -> Box<ScoringFunction> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<ScoringFunction> {
    fn clone(&self) -> Box<ScoringFunction> {
        self.clone_box()
    }
}
