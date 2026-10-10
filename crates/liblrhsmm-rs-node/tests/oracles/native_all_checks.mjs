import {verifyNumericalBindings} from './native_numerics_checks.mjs';
import {verifyDurationBindings} from './native_duration_checks.mjs';
import {verifyGaussianBindings} from './native_gaussian_checks.mjs';
import {verifyContainerBindings} from './native_containers_checks.mjs';
import {verifyStatisticsBindings} from './native_statistics_checks.mjs';
import {verifyDataBindings} from './native_data_checks.mjs';
import {verifyDatasetBindings} from './native_datasets_checks.mjs';
import {verifyEmissionBindings} from './native_emissions_checks.mjs';
import {verifyGeometricBindings} from './native_geometric_checks.mjs';
import {verifyHsmmBindings} from './native_hsmm_checks.mjs';
import {verifyPosteriorStorage} from './native_posterior_storage_checks.mjs';
import {verifyPosteriorBindings} from './native_posteriors_checks.mjs';
import {verifyEstimationBindings} from './native_estimation_checks.mjs';
import {verifyGenerationBindings} from './native_generation_checks.mjs';
import {verifyMemoryPoolBindings} from './native_memory_pool_checks.mjs';
import {verifyOwnershipBindings} from './native_ownership_checks.mjs';
import {verifyConfigurationBindings} from './native_configuration_checks.mjs';
import {verifyBuilderResize} from './native_builder_resize_checks.mjs';

const families = [
    ['configuration', verifyConfigurationBindings],
    ['builderResize', verifyBuilderResize],
    ['numerics', verifyNumericalBindings],
    ['duration', verifyDurationBindings],
    ['gaussian', verifyGaussianBindings],
    ['containers', verifyContainerBindings],
    ['statistics', verifyStatisticsBindings],
    ['data', verifyDataBindings],
    ['datasets', verifyDatasetBindings],
    ['emissions', verifyEmissionBindings],
    ['geometric', verifyGeometricBindings],
    ['hsmm', verifyHsmmBindings],
    ['posteriorStorage', verifyPosteriorStorage],
    ['posteriors', verifyPosteriorBindings],
    ['estimation', verifyEstimationBindings],
    ['generation', verifyGenerationBindings],
    ['memoryPool', verifyMemoryPoolBindings],
    ['ownership', verifyOwnershipBindings],
];

/** Run every family twice on one initialized module, reversing the second pass. */
export async function verifyAllBindings(api, load, serialization = true) {
    const results = {};
    for (const [name, verify] of families) {
        results[name] = await verify(api, load, serialization);
    }
    for (const [name, verify] of [...families].reverse()) {
        const repeated = await verify(api, load, serialization);
        if (JSON.stringify(repeated) !== JSON.stringify(results[name])) {
            throw new Error('Repeated shared-module verification changed: ' + name);
        }
    }
    return {families: families.length, passes: 2, serialization, results};
}
