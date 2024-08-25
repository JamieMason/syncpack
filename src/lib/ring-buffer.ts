/**
 * An Array with a maximum size which, once reached, will replace the oldest
 * item when a new one is added.
 */
export class RingBuffer<T> extends Array {
  cursor: number;
  fixedLength: number;

  constructor(fixedLength: number) {
    super(fixedLength);
    this.cursor = 0;
    this.fixedLength = fixedLength;
  }

  push(...values: T[]) {
    values.forEach(value => {
      this[this.cursor++] = value;
      this.cursor %= this.length;
    });
    return this.length;
  }
}
