import { GaussianF64, version } from '..';
const distribution = new GaussianF64(1, 2);
const bytes: Uint8Array = distribution.write();
const decoded = GaussianF64.read_prefix(bytes);
const restored: GaussianF64 = GaussianF64.read(bytes);
const packageVersion: string = version();
void [decoded, restored, packageVersion];
