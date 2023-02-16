export function printStrings(strings: string[]): string {
  return strings.map((str) => `"${str}"`).join(', ');
}
