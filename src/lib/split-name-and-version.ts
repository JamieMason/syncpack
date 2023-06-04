/**
 * Split string by first occurring "@" which is not the first character in the
 * string (used by scoped npm packages).
 */
export function splitNameAndVersion(value: string): [string, string] {
  const ix = value.search(/(?!^)@/);
  return [value.slice(0, ix), value.slice(ix + 1)];
}
