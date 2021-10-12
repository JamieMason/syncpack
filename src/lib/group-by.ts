export function groupBy<T>(key: string, array: T[]): Record<string, T[]> {
  return array.reduce((memo: any, obj: any) => {
    const value = obj[key];
    memo[value] = (memo[value] || []).concat(obj);
    return memo;
  }, {});
}
