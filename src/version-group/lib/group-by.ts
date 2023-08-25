/**
 * Convert an array of objects to an object, where each property of the new
 * object is an array whose members share the same value for the given key.
 */
export function groupBy<A extends any[]>(key: string, array: A): Record<string, A> {
  return array.reduce((objectsByKeyValue, obj) => {
    const value = obj[key];
    objectsByKeyValue[value] = (objectsByKeyValue[value] || []).concat(obj);
    return objectsByKeyValue;
  }, {});
}
