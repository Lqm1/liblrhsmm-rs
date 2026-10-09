import {verifyNumericalBindings} from './wasm_numerics_checks.mjs';
import {verifyDurationBindings} from './wasm_duration_checks.mjs';
import {verifyGaussianBindings} from './wasm_gaussian_checks.mjs';
import {verifyContainerBindings} from './wasm_containers_checks.mjs';
import {verifyStatisticsBindings} from './wasm_statistics_checks.mjs';
import {verifyDataBindings} from './wasm_data_checks.mjs';
import {verifyDatasetBindings} from './wasm_datasets_checks.mjs';
import {verifyEmissionBindings} from './wasm_emissions_checks.mjs';
import {verifyGeometricBindings} from './wasm_geometric_checks.mjs';
import {verifyHsmmBindings} from './wasm_hsmm_checks.mjs';
import {verifyPosteriorStorage} from './wasm_posterior_storage_checks.mjs';
import {verifyPosteriorBindings} from './wasm_posteriors_checks.mjs';
import {verifyEstimationBindings} from './wasm_estimation_checks.mjs';
import {verifyGenerationBindings} from './wasm_generation_checks.mjs';
import {verifyMemoryPoolBindings} from './wasm_memory_pool_checks.mjs';
import {verifyOwnershipBindings} from './wasm_ownership_checks.mjs';
import {verifyConfigurationBindings} from './wasm_configuration_checks.mjs';

const families = [
    ['configuration', verifyConfigurationBindings],
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
