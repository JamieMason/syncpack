import coerce from 'semver/functions/coerce';
import valid from 'semver/functions/valid';

/** Convert eg "1" to "1.0.0" which the semver lib does not understand */
export function clean(v: string): string {
  return valid(coerce(v)) || '';
}
