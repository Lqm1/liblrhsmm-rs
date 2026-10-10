//! Python bindings to the native Rust implementation.
#[path = "bindings/binding_datasetf32.rs"]
mod binding_datasetf32;
pub use binding_datasetf32::DatasetF32;
#[path = "bindings/binding_datasetf64.rs"]
mod binding_datasetf64;
pub use binding_datasetf64::DatasetF64;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodeddurationf32.rs"]
mod binding_decodeddurationf32;
#[cfg(feature = "serialization")]
pub use binding_decodeddurationf32::DecodedDurationF32;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodeddurationf64.rs"]
mod binding_decodeddurationf64;
#[cfg(feature = "serialization")]
pub use binding_decodeddurationf64::DecodedDurationF64;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedgaussianf32.rs"]
mod binding_decodedgaussianf32;
#[cfg(feature = "serialization")]
pub use binding_decodedgaussianf32::DecodedGaussianF32;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedgaussianf64.rs"]
mod binding_decodedgaussianf64;
#[cfg(feature = "serialization")]
pub use binding_decodedgaussianf64::DecodedGaussianF64;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedmodelf32.rs"]
mod binding_decodedmodelf32;
#[cfg(feature = "serialization")]
pub use binding_decodedmodelf32::DecodedModelF32;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedmodelf64.rs"]
mod binding_decodedmodelf64;
#[cfg(feature = "serialization")]
pub use binding_decodedmodelf64::DecodedModelF64;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedobservationf32.rs"]
mod binding_decodedobservationf32;
#[cfg(feature = "serialization")]
pub use binding_decodedobservationf32::DecodedObservationF32;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedobservationf64.rs"]
mod binding_decodedobservationf64;
#[cfg(feature = "serialization")]
pub use binding_decodedobservationf64::DecodedObservationF64;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedobservationsetf32.rs"]
mod binding_decodedobservationsetf32;
#[cfg(feature = "serialization")]
pub use binding_decodedobservationsetf32::DecodedObservationSetF32;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedobservationsetf64.rs"]
mod binding_decodedobservationsetf64;
#[cfg(feature = "serialization")]
pub use binding_decodedobservationsetf64::DecodedObservationSetF64;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedsegmentationf32.rs"]
mod binding_decodedsegmentationf32;
#[cfg(feature = "serialization")]
pub use binding_decodedsegmentationf32::DecodedSegmentationF32;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedsegmentationf64.rs"]
mod binding_decodedsegmentationf64;
#[cfg(feature = "serialization")]
pub use binding_decodedsegmentationf64::DecodedSegmentationF64;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedsegmentationsetf32.rs"]
mod binding_decodedsegmentationsetf32;
#[cfg(feature = "serialization")]
pub use binding_decodedsegmentationsetf32::DecodedSegmentationSetF32;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedsegmentationsetf64.rs"]
mod binding_decodedsegmentationsetf64;
#[cfg(feature = "serialization")]
pub use binding_decodedsegmentationsetf64::DecodedSegmentationSetF64;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedstreamf32.rs"]
mod binding_decodedstreamf32;
#[cfg(feature = "serialization")]
pub use binding_decodedstreamf32::DecodedStreamF32;
#[cfg(feature = "serialization")]
#[path = "bindings/binding_decodedstreamf64.rs"]
mod binding_decodedstreamf64;
#[cfg(feature = "serialization")]
pub use binding_decodedstreamf64::DecodedStreamF64;
#[path = "bindings/binding_durationcollectionf32.rs"]
mod binding_durationcollectionf32;
pub use binding_durationcollectionf32::DurationCollectionF32;
#[path = "bindings/binding_durationcollectionf64.rs"]
mod binding_durationcollectionf64;
pub use binding_durationcollectionf64::DurationCollectionF64;
#[path = "bindings/binding_durationf32.rs"]
mod binding_durationf32;
pub use binding_durationf32::DurationF32;
#[path = "bindings/binding_durationf64.rs"]
mod binding_durationf64;
pub use binding_durationf64::DurationF64;
#[path = "bindings/binding_durationposterior.rs"]
mod binding_durationposterior;
pub use binding_durationposterior::DurationPosterior;
#[path = "bindings/binding_durationposteriors.rs"]
mod binding_durationposteriors;
pub use binding_durationposteriors::DurationPosteriors;
#[path = "bindings/binding_durationstatisticscollectionf32.rs"]
mod binding_durationstatisticscollectionf32;
pub use binding_durationstatisticscollectionf32::DurationStatisticsCollectionF32;
#[path = "bindings/binding_durationstatisticscollectionf64.rs"]
mod binding_durationstatisticscollectionf64;
pub use binding_durationstatisticscollectionf64::DurationStatisticsCollectionF64;
#[path = "bindings/binding_durationstatisticsf32.rs"]
mod binding_durationstatisticsf32;
pub use binding_durationstatisticsf32::DurationStatisticsF32;
#[path = "bindings/binding_durationstatisticsf64.rs"]
mod binding_durationstatisticsf64;
pub use binding_durationstatisticsf64::DurationStatisticsF64;
#[path = "bindings/binding_emissiondensitiesf32.rs"]
mod binding_emissiondensitiesf32;
pub use binding_emissiondensitiesf32::EmissionDensitiesF32;
#[path = "bindings/binding_emissiondensitiesf64.rs"]
mod binding_emissiondensitiesf64;
pub use binding_emissiondensitiesf64::EmissionDensitiesF64;
#[path = "bindings/binding_emissionpruningf32.rs"]
mod binding_emissionpruningf32;
pub use binding_emissionpruningf32::EmissionPruningF32;
#[path = "bindings/binding_emissionpruningf64.rs"]
mod binding_emissionpruningf64;
pub use binding_emissionpruningf64::EmissionPruningF64;
#[path = "bindings/binding_gaussiancollectionf32.rs"]
mod binding_gaussiancollectionf32;
pub use binding_gaussiancollectionf32::GaussianCollectionF32;
#[path = "bindings/binding_gaussiancollectionf64.rs"]
mod binding_gaussiancollectionf64;
pub use binding_gaussiancollectionf64::GaussianCollectionF64;
#[path = "bindings/binding_gaussianf32.rs"]
mod binding_gaussianf32;
pub use binding_gaussianf32::GaussianF32;
#[path = "bindings/binding_gaussianf64.rs"]
mod binding_gaussianf64;
pub use binding_gaussianf64::GaussianF64;
#[path = "bindings/binding_gaussianstatisticscollectionf32.rs"]
mod binding_gaussianstatisticscollectionf32;
pub use binding_gaussianstatisticscollectionf32::GaussianStatisticsCollectionF32;
#[path = "bindings/binding_gaussianstatisticscollectionf64.rs"]
mod binding_gaussianstatisticscollectionf64;
pub use binding_gaussianstatisticscollectionf64::GaussianStatisticsCollectionF64;
#[path = "bindings/binding_gaussianstatisticsf32.rs"]
mod binding_gaussianstatisticsf32;
pub use binding_gaussianstatisticsf32::GaussianStatisticsF32;
#[path = "bindings/binding_gaussianstatisticsf64.rs"]
mod binding_gaussianstatisticsf64;
pub use binding_gaussianstatisticsf64::GaussianStatisticsF64;
#[path = "bindings/binding_generatedobservationf32.rs"]
mod binding_generatedobservationf32;
pub use binding_generatedobservationf32::GeneratedObservationF32;
#[path = "bindings/binding_generatedobservationf64.rs"]
mod binding_generatedobservationf64;
pub use binding_generatedobservationf64::GeneratedObservationF64;
#[path = "bindings/binding_generationf32.rs"]
mod binding_generationf32;
pub use binding_generationf32::GenerationF32;
#[path = "bindings/binding_generationf64.rs"]
mod binding_generationf64;
pub use binding_generationf64::GenerationF64;
#[path = "bindings/binding_geometricexpectationf32.rs"]
mod binding_geometricexpectationf32;
pub use binding_geometricexpectationf32::GeometricExpectationF32;
#[path = "bindings/binding_geometricexpectationf64.rs"]
mod binding_geometricexpectationf64;
pub use binding_geometricexpectationf64::GeometricExpectationF64;
#[path = "bindings/binding_geometricoptionsf32.rs"]
mod binding_geometricoptionsf32;
pub use binding_geometricoptionsf32::GeometricOptionsF32;
#[path = "bindings/binding_geometricoptionsf64.rs"]
mod binding_geometricoptionsf64;
pub use binding_geometricoptionsf64::GeometricOptionsF64;
#[path = "bindings/binding_geometricpathf32.rs"]
mod binding_geometricpathf32;
pub use binding_geometricpathf32::GeometricPathF32;
#[path = "bindings/binding_geometricpathf64.rs"]
mod binding_geometricpathf64;
pub use binding_geometricpathf64::GeometricPathF64;
#[path = "bindings/binding_hsmmexpectationf32.rs"]
mod binding_hsmmexpectationf32;
pub use binding_hsmmexpectationf32::HsmmExpectationF32;
#[path = "bindings/binding_hsmmexpectationf64.rs"]
mod binding_hsmmexpectationf64;
pub use binding_hsmmexpectationf64::HsmmExpectationF64;
#[path = "bindings/binding_hsmmoptionsf32.rs"]
mod binding_hsmmoptionsf32;
pub use binding_hsmmoptionsf32::HsmmOptionsF32;
#[path = "bindings/binding_hsmmoptionsf64.rs"]
mod binding_hsmmoptionsf64;
pub use binding_hsmmoptionsf64::HsmmOptionsF64;
#[path = "bindings/binding_hsmmpathf32.rs"]
mod binding_hsmmpathf32;
pub use binding_hsmmpathf32::HsmmPathF32;
#[path = "bindings/binding_hsmmpathf64.rs"]
mod binding_hsmmpathf64;
pub use binding_hsmmpathf64::HsmmPathF64;
#[path = "bindings/binding_hsmmscoresf32.rs"]
mod binding_hsmmscoresf32;
pub use binding_hsmmscoresf32::HsmmScoresF32;
#[path = "bindings/binding_hsmmscoresf64.rs"]
mod binding_hsmmscoresf64;
pub use binding_hsmmscoresf64::HsmmScoresF64;
#[path = "bindings/binding_jumpgroupf32.rs"]
mod binding_jumpgroupf32;
pub use binding_jumpgroupf32::JumpGroupF32;
#[path = "bindings/binding_jumpgroupf64.rs"]
mod binding_jumpgroupf64;
pub use binding_jumpgroupf64::JumpGroupF64;
#[path = "bindings/binding_logemissionsf32.rs"]
mod binding_logemissionsf32;
pub use binding_logemissionsf32::LogEmissionsF32;
#[path = "bindings/binding_logemissionsf64.rs"]
mod binding_logemissionsf64;
pub use binding_logemissionsf64::LogEmissionsF64;
#[path = "bindings/binding_memorypool.rs"]
mod binding_memorypool;
pub use binding_memorypool::MemoryPool;
#[path = "bindings/binding_mixtureoccupancymodef32.rs"]
mod binding_mixtureoccupancymodef32;
pub use binding_mixtureoccupancymodef32::MixtureOccupancyModeF32;
#[path = "bindings/binding_mixtureoccupancymodef64.rs"]
mod binding_mixtureoccupancymodef64;
pub use binding_mixtureoccupancymodef64::MixtureOccupancyModeF64;
#[path = "bindings/binding_mixtureposteriorcollection.rs"]
mod binding_mixtureposteriorcollection;
pub use binding_mixtureposteriorcollection::MixturePosteriorCollection;
#[path = "bindings/binding_mixtureposteriors.rs"]
mod binding_mixtureposteriors;
pub use binding_mixtureposteriors::MixturePosteriors;
#[path = "bindings/binding_mixtureweights.rs"]
mod binding_mixtureweights;
pub use binding_mixtureweights::MixtureWeights;
#[path = "bindings/binding_modelbuilderf32.rs"]
mod binding_modelbuilderf32;
pub use binding_modelbuilderf32::ModelBuilderF32;
#[path = "bindings/binding_modelbuilderf64.rs"]
mod binding_modelbuilderf64;
pub use binding_modelbuilderf64::ModelBuilderF64;
#[path = "bindings/binding_modelf32.rs"]
mod binding_modelf32;
pub use binding_modelf32::ModelF32;
#[path = "bindings/binding_modelf64.rs"]
mod binding_modelf64;
pub use binding_modelf64::ModelF64;
#[path = "bindings/binding_modelstatisticsf32.rs"]
mod binding_modelstatisticsf32;
pub use binding_modelstatisticsf32::ModelStatisticsF32;
#[path = "bindings/binding_modelstatisticsf64.rs"]
mod binding_modelstatisticsf64;
pub use binding_modelstatisticsf64::ModelStatisticsF64;
#[path = "bindings/binding_numericaliteratorf32.rs"]
mod binding_numericaliteratorf32;
pub use binding_numericaliteratorf32::NumericalIteratorF32;
#[path = "bindings/binding_numericaliteratorf64.rs"]
mod binding_numericaliteratorf64;
pub use binding_numericaliteratorf64::NumericalIteratorF64;
#[path = "bindings/binding_numericaltablef32.rs"]
mod binding_numericaltablef32;
pub use binding_numericaltablef32::NumericalTableF32;
#[path = "bindings/binding_numericaltablef64.rs"]
mod binding_numericaltablef64;
pub use binding_numericaltablef64::NumericalTableF64;
#[path = "bindings/binding_observationcollectionf32.rs"]
mod binding_observationcollectionf32;
pub use binding_observationcollectionf32::ObservationCollectionF32;
#[path = "bindings/binding_observationcollectionf64.rs"]
mod binding_observationcollectionf64;
pub use binding_observationcollectionf64::ObservationCollectionF64;
#[path = "bindings/binding_observationf32.rs"]
mod binding_observationf32;
pub use binding_observationf32::ObservationF32;
#[path = "bindings/binding_observationf64.rs"]
mod binding_observationf64;
pub use binding_observationf64::ObservationF64;
#[path = "bindings/binding_observationsetbuilderf32.rs"]
mod binding_observationsetbuilderf32;
pub use binding_observationsetbuilderf32::ObservationSetBuilderF32;
#[path = "bindings/binding_observationsetbuilderf64.rs"]
mod binding_observationsetbuilderf64;
pub use binding_observationsetbuilderf64::ObservationSetBuilderF64;
#[path = "bindings/binding_observationstreamcollectionf32.rs"]
mod binding_observationstreamcollectionf32;
pub use binding_observationstreamcollectionf32::ObservationStreamCollectionF32;
#[path = "bindings/binding_observationstreamcollectionf64.rs"]
mod binding_observationstreamcollectionf64;
pub use binding_observationstreamcollectionf64::ObservationStreamCollectionF64;
#[path = "bindings/binding_observationstreamf32.rs"]
mod binding_observationstreamf32;
pub use binding_observationstreamf32::ObservationStreamF32;
#[path = "bindings/binding_observationstreamf64.rs"]
mod binding_observationstreamf64;
pub use binding_observationstreamf64::ObservationStreamF64;
#[path = "bindings/binding_poolallocation.rs"]
mod binding_poolallocation;
pub use binding_poolallocation::PoolAllocation;
#[path = "bindings/binding_poolpageusage.rs"]
mod binding_poolpageusage;
pub use binding_poolpageusage::PoolPageUsage;
#[path = "bindings/binding_poolpages.rs"]
mod binding_poolpages;
pub use binding_poolpages::PoolPages;
#[path = "bindings/binding_preparationoptionsf32.rs"]
mod binding_preparationoptionsf32;
pub use binding_preparationoptionsf32::PreparationOptionsF32;
#[path = "bindings/binding_preparationoptionsf64.rs"]
mod binding_preparationoptionsf64;
pub use binding_preparationoptionsf64::PreparationOptionsF64;
#[path = "bindings/binding_prepareddurationf32.rs"]
mod binding_prepareddurationf32;
pub use binding_prepareddurationf32::PreparedDurationF32;
#[path = "bindings/binding_prepareddurationf64.rs"]
mod binding_prepareddurationf64;
pub use binding_prepareddurationf64::PreparedDurationF64;
#[path = "bindings/binding_preparedgaussianf32.rs"]
mod binding_preparedgaussianf32;
pub use binding_preparedgaussianf32::PreparedGaussianF32;
#[path = "bindings/binding_preparedgaussianf64.rs"]
mod binding_preparedgaussianf64;
pub use binding_preparedgaussianf64::PreparedGaussianF64;
#[path = "bindings/binding_preparedmodelf32.rs"]
mod binding_preparedmodelf32;
pub use binding_preparedmodelf32::PreparedModelF32;
#[path = "bindings/binding_preparedmodelf64.rs"]
mod binding_preparedmodelf64;
pub use binding_preparedmodelf64::PreparedModelF64;
#[path = "bindings/binding_preparedstreamf32.rs"]
mod binding_preparedstreamf32;
pub use binding_preparedstreamf32::PreparedStreamF32;
#[path = "bindings/binding_preparedstreamf64.rs"]
mod binding_preparedstreamf64;
pub use binding_preparedstreamf64::PreparedStreamF64;
#[path = "bindings/binding_segmentationcollectionf32.rs"]
mod binding_segmentationcollectionf32;
pub use binding_segmentationcollectionf32::SegmentationCollectionF32;
#[path = "bindings/binding_segmentationcollectionf64.rs"]
mod binding_segmentationcollectionf64;
pub use binding_segmentationcollectionf64::SegmentationCollectionF64;
#[path = "bindings/binding_segmentationf32.rs"]
mod binding_segmentationf32;
pub use binding_segmentationf32::SegmentationF32;
#[path = "bindings/binding_segmentationf64.rs"]
mod binding_segmentationf64;
pub use binding_segmentationf64::SegmentationF64;
#[path = "bindings/binding_segmentationsetbuilderf32.rs"]
mod binding_segmentationsetbuilderf32;
pub use binding_segmentationsetbuilderf32::SegmentationSetBuilderF32;
#[path = "bindings/binding_segmentationsetbuilderf64.rs"]
mod binding_segmentationsetbuilderf64;
pub use binding_segmentationsetbuilderf64::SegmentationSetBuilderF64;
#[path = "bindings/binding_stateoccupanciesf32.rs"]
mod binding_stateoccupanciesf32;
pub use binding_stateoccupanciesf32::StateOccupanciesF32;
#[path = "bindings/binding_stateoccupanciesf64.rs"]
mod binding_stateoccupanciesf64;
pub use binding_stateoccupanciesf64::StateOccupanciesF64;
#[path = "bindings/binding_statescoresf32.rs"]
mod binding_statescoresf32;
pub use binding_statescoresf32::StateScoresF32;
#[path = "bindings/binding_statescoresf64.rs"]
mod binding_statescoresf64;
pub use binding_statescoresf64::StateScoresF64;
#[path = "bindings/binding_streambuilderf32.rs"]
mod binding_streambuilderf32;
pub use binding_streambuilderf32::StreamBuilderF32;
#[path = "bindings/binding_streambuilderf64.rs"]
mod binding_streambuilderf64;
pub use binding_streambuilderf64::StreamBuilderF64;
#[path = "bindings/binding_streamcollectionf32.rs"]
mod binding_streamcollectionf32;
pub use binding_streamcollectionf32::StreamCollectionF32;
#[path = "bindings/binding_streamcollectionf64.rs"]
mod binding_streamcollectionf64;
pub use binding_streamcollectionf64::StreamCollectionF64;
#[path = "bindings/binding_streamf32.rs"]
mod binding_streamf32;
pub use binding_streamf32::StreamF32;
#[path = "bindings/binding_streamf64.rs"]
mod binding_streamf64;
pub use binding_streamf64::StreamF64;
#[path = "bindings/binding_streamstatisticscollectionf32.rs"]
mod binding_streamstatisticscollectionf32;
pub use binding_streamstatisticscollectionf32::StreamStatisticsCollectionF32;
#[path = "bindings/binding_streamstatisticscollectionf64.rs"]
mod binding_streamstatisticscollectionf64;
pub use binding_streamstatisticscollectionf64::StreamStatisticsCollectionF64;
#[path = "bindings/binding_streamstatisticsf32.rs"]
mod binding_streamstatisticsf32;
pub use binding_streamstatisticsf32::StreamStatisticsF32;
#[path = "bindings/binding_streamstatisticsf64.rs"]
mod binding_streamstatisticsf64;
pub use binding_streamstatisticsf64::StreamStatisticsF64;
#[path = "bindings/binding_transitiongraphf32.rs"]
mod binding_transitiongraphf32;
pub use binding_transitiongraphf32::TransitionGraphF32;
#[path = "bindings/binding_transitiongraphf64.rs"]
mod binding_transitiongraphf64;
pub use binding_transitiongraphf64::TransitionGraphF64;
#[path = "bindings/binding_transitionposterior.rs"]
mod binding_transitionposterior;
pub use binding_transitionposterior::TransitionPosterior;
#[path = "bindings/binding_transitionposteriors.rs"]
mod binding_transitionposteriors;
pub use binding_transitionposteriors::TransitionPosteriors;
#[path = "bindings/binding_updateoptionsf32.rs"]
mod binding_updateoptionsf32;
pub use binding_updateoptionsf32::UpdateOptionsF32;
#[path = "bindings/binding_updateoptionsf64.rs"]
mod binding_updateoptionsf64;
pub use binding_updateoptionsf64::UpdateOptionsF64;
#[path = "bindings/binding_functions_choose.rs"]
mod binding_functions_choose;
pub use binding_functions_choose::*;
#[path = "bindings/binding_functions_gaussian.rs"]
mod binding_functions_gaussian;
pub use binding_functions_gaussian::*;
#[path = "bindings/binding_functions_numerical.rs"]
mod binding_functions_numerical;
pub use binding_functions_numerical::*;
#[path = "bindings/binding_functions_regroup.rs"]
mod binding_functions_regroup;
pub use binding_functions_regroup::*;
#[path = "bindings/binding_functions_uniform.rs"]
mod binding_functions_uniform;
pub use binding_functions_uniform::*;
pub use liblrhsmm_rs::*;
use pyo3::prelude::*;
mod api;
mod callbacks;
fn consumed() -> PyErr {
    pyo3::exceptions::PyRuntimeError::new_err("owner has been consumed")
}
pub use crate::api::NumericalTable;
#[pymodule]
fn _liblrhsmm_rs(module: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedStreamF32>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedStreamF64>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedModelF32>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedModelF64>()?;
    module.add_class::<GaussianStatisticsCollectionF32>()?;
    module.add_class::<StreamStatisticsCollectionF32>()?;
    module.add_class::<DurationStatisticsCollectionF32>()?;
    module.add_class::<StreamStatisticsF32>()?;
    module.add_class::<ModelStatisticsF32>()?;
    module.add_class::<GaussianStatisticsCollectionF64>()?;
    module.add_class::<StreamStatisticsCollectionF64>()?;
    module.add_class::<DurationStatisticsCollectionF64>()?;
    module.add_class::<StreamStatisticsF64>()?;
    module.add_class::<ModelStatisticsF64>()?;
    module.add_class::<StreamCollectionF32>()?;
    module.add_class::<StreamF32>()?;
    module.add_class::<StreamBuilderF32>()?;
    module.add_class::<PreparedStreamF32>()?;
    module.add_class::<ModelF32>()?;
    module.add_class::<ModelBuilderF32>()?;
    module.add_class::<PreparedModelF32>()?;
    module.add_class::<StreamCollectionF64>()?;
    module.add_class::<StreamF64>()?;
    module.add_class::<StreamBuilderF64>()?;
    module.add_class::<PreparedStreamF64>()?;
    module.add_class::<ModelF64>()?;
    module.add_class::<ModelBuilderF64>()?;
    module.add_class::<PreparedModelF64>()?;
    module.add_class::<ObservationStreamCollectionF32>()?;
    module.add_class::<ObservationCollectionF32>()?;
    module.add_class::<SegmentationCollectionF32>()?;
    module.add_class::<ObservationStreamF32>()?;
    module.add_class::<ObservationF32>()?;
    module.add_class::<JumpGroupF32>()?;
    module.add_class::<SegmentationF32>()?;
    module.add_class::<TransitionGraphF32>()?;
    module.add_class::<ObservationStreamCollectionF64>()?;
    module.add_class::<ObservationCollectionF64>()?;
    module.add_class::<SegmentationCollectionF64>()?;
    module.add_class::<ObservationStreamF64>()?;
    module.add_class::<ObservationF64>()?;
    module.add_class::<JumpGroupF64>()?;
    module.add_class::<SegmentationF64>()?;
    module.add_class::<TransitionGraphF64>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedObservationF32>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedObservationF64>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedSegmentationF32>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedSegmentationF64>()?;
    module.add_class::<ObservationSetBuilderF32>()?;
    module.add_class::<ObservationSetBuilderF64>()?;
    module.add_class::<SegmentationSetBuilderF32>()?;
    module.add_class::<SegmentationSetBuilderF64>()?;
    module.add_class::<DatasetF32>()?;
    module.add_function(wrap_pyfunction!(binding_regroup_data_f32, module)?)?;
    module.add_class::<DatasetF64>()?;
    module.add_function(wrap_pyfunction!(binding_regroup_data_f64, module)?)?;
    module.add_class::<PreparationOptionsF32>()?;
    module.add_class::<UpdateOptionsF32>()?;
    module.add_class::<DurationStatisticsF32>()?;
    module.add_class::<DurationF32>()?;
    module.add_class::<PreparedDurationF32>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedDurationF32>()?;
    module.add_class::<PreparationOptionsF64>()?;
    module.add_class::<UpdateOptionsF64>()?;
    module.add_class::<DurationStatisticsF64>()?;
    module.add_class::<DurationF64>()?;
    module.add_class::<PreparedDurationF64>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedDurationF64>()?;
    module.add_class::<EmissionPruningF32>()?;
    module.add_class::<LogEmissionsF32>()?;
    module.add_class::<EmissionDensitiesF32>()?;
    module.add_class::<EmissionPruningF64>()?;
    module.add_class::<LogEmissionsF64>()?;
    module.add_class::<EmissionDensitiesF64>()?;
    module.add_class::<MixturePosteriorCollection>()?;
    module.add_class::<HsmmExpectationF32>()?;
    module.add_class::<GeometricExpectationF32>()?;
    module.add_class::<HsmmExpectationF64>()?;
    module.add_class::<GeometricExpectationF64>()?;
    module.add_class::<DurationCollectionF32>()?;
    module.add_class::<DurationCollectionF64>()?;
    module.add_class::<GaussianCollectionF32>()?;
    module.add_class::<GaussianF32>()?;
    module.add_class::<PreparedGaussianF32>()?;
    module.add_class::<GaussianStatisticsF32>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedGaussianF32>()?;
    module.add_class::<GaussianCollectionF64>()?;
    module.add_class::<GaussianF64>()?;
    module.add_class::<PreparedGaussianF64>()?;
    module.add_class::<GaussianStatisticsF64>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedGaussianF64>()?;
    module.add_function(wrap_pyfunction!(binding_uniform_sample_f32, module)?)?;
    module.add_function(wrap_pyfunction!(binding_gaussian_sample_f32, module)?)?;
    module.add_function(wrap_pyfunction!(binding_choose_component_f32, module)?)?;
    module.add_class::<GeneratedObservationF32>()?;
    module.add_class::<GenerationF32>()?;
    module.add_function(wrap_pyfunction!(binding_uniform_sample_f64, module)?)?;
    module.add_function(wrap_pyfunction!(binding_gaussian_sample_f64, module)?)?;
    module.add_function(wrap_pyfunction!(binding_choose_component_f64, module)?)?;
    module.add_class::<GeneratedObservationF64>()?;
    module.add_class::<GenerationF64>()?;
    module.add_class::<GeometricOptionsF32>()?;
    module.add_class::<StateScoresF32>()?;
    module.add_class::<GeometricPathF32>()?;
    module.add_class::<GeometricOptionsF64>()?;
    module.add_class::<StateScoresF64>()?;
    module.add_class::<GeometricPathF64>()?;
    module.add_class::<HsmmOptionsF32>()?;
    module.add_class::<HsmmScoresF32>()?;
    module.add_class::<HsmmPathF32>()?;
    module.add_class::<HsmmOptionsF64>()?;
    module.add_class::<HsmmScoresF64>()?;
    module.add_class::<HsmmPathF64>()?;
    module.add_class::<PoolPageUsage>()?;
    module.add_class::<PoolPages>()?;
    module.add_class::<PoolAllocation>()?;
    module.add_class::<MemoryPool>()?;
    module.add_class::<NumericalTable>()?;
    module.add_function(wrap_pyfunction!(binding_numerical_constants, module)?)?;
    module.add_class::<NumericalTableF32>()?;
    module.add_class::<NumericalIteratorF32>()?;
    module.add_function(wrap_pyfunction!(binding_numerical_log_add_f32, module)?)?;
    module.add_function(wrap_pyfunction!(
        binding_numerical_negative_exponential_f32,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(binding_numerical_exponential_f32, module)?)?;
    module.add_function(wrap_pyfunction!(
        binding_numerical_log_add_table_f32,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(
        binding_numerical_exponential_table_f32,
        module
    )?)?;
    module.add_class::<NumericalTableF64>()?;
    module.add_class::<NumericalIteratorF64>()?;
    module.add_function(wrap_pyfunction!(binding_numerical_log_add_f64, module)?)?;
    module.add_function(wrap_pyfunction!(
        binding_numerical_negative_exponential_f64,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(binding_numerical_exponential_f64, module)?)?;
    module.add_function(wrap_pyfunction!(
        binding_numerical_log_add_table_f64,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(
        binding_numerical_exponential_table_f64,
        module
    )?)?;
    module.add_class::<TransitionPosterior>()?;
    module.add_class::<TransitionPosteriors>()?;
    module.add_class::<DurationPosterior>()?;
    module.add_class::<DurationPosteriors>()?;
    module.add_class::<MixtureWeights>()?;
    module.add_class::<MixturePosteriors>()?;
    module.add_class::<StateOccupanciesF32>()?;
    module.add_class::<MixtureOccupancyModeF32>()?;
    module.add_class::<StateOccupanciesF64>()?;
    module.add_class::<MixtureOccupancyModeF64>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedObservationSetF32>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedObservationSetF64>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedSegmentationSetF32>()?;
    #[cfg(feature = "serialization")]
    module.add_class::<DecodedSegmentationSetF64>()?;
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
