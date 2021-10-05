import { Installation } from './get-dependencies';
import { satisfies, intersects, validRange } from 'semver';

const protocolRegex = /(\w+:)?(.+)/;
const getProtocolAndVersion = (version: string): [version: string, protocol?: string] => {
  const matches = protocolRegex.exec(version)!;

  return [matches[2], matches[1]];
};

export const versionsMatch = (a: Installation, b: Installation, matchRanges: boolean): boolean => {
  if (!matchRanges) return a.version === b.version;

  let protocolsMatch = true;
  const [aVersion, aProtocol] = getProtocolAndVersion(a.version);
  const [bVersion, bProtocol] = getProtocolAndVersion(b.version);

  if (aProtocol != null || bProtocol != null) {
    protocolsMatch = aProtocol === bProtocol;
  }

  // If both versions are ranges we need to check if they intersect (satisfy each other)
  // as .satisfies() does not support two ranges.
  if (validRange(aVersion) && validRange(bVersion)) {
    return protocolsMatch && intersects(aVersion, bVersion);
  }

  // We have to check both ways due to satisfies only working one way - range against version/range
  // Otherwise it will fail if A is a version and B is a range
  return protocolsMatch && (satisfies(aVersion, bVersion) || satisfies(bVersion, aVersion));
};
