import { Either } from 'effect';
import { isFunction } from 'tightrope/guard/is-function.js';
import { isPrimitive } from 'tightrope/guard/is-primitive.js';
import { isReadable } from 'tightrope/guard/is-readable.js';

// prettier-ignore
export function get<
  T,
  P1 extends keyof NonNullable<T>
>(
  obj: T,
  prop1: P1
): Either.Either<Error, NonNullable<T>[P1]>;

// prettier-ignore
export function get<
  T,
  P1 extends keyof NonNullable<T>,
  P2 extends keyof NonNullable<NonNullable<T>[P1]>
>(
  obj: T,
  prop1: P1,
  prop2: P2
): Either.Either<Error, NonNullable<NonNullable<T>[P1]>[P2]>;

// prettier-ignore
export function get<
  T,
  P1 extends keyof NonNullable<T>,
  P2 extends keyof NonNullable<NonNullable<T>[P1]>,
  P3 extends keyof NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>
>(
  obj: T,
  prop1: P1,
  prop2: P2,
  prop3: P3
): Either.Either<Error, NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>;

// prettier-ignore
export function get<
  T,
  P1 extends keyof NonNullable<T>,
  P2 extends keyof NonNullable<NonNullable<T>[P1]>,
  P3 extends keyof NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>,
  P4 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>
>(
  obj: T,
  prop1: P1,
  prop2: P2,
  prop3: P3,
  prop4: P4
): Either.Either<Error, NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>;

// prettier-ignore
export function get<
  T,
  P1 extends keyof NonNullable<T>,
  P2 extends keyof NonNullable<NonNullable<T>[P1]>,
  P3 extends keyof NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>,
  P4 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>,
  P5 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>
>(
  obj: T,
  prop1: P1,
  prop2: P2,
  prop3: P3,
  prop4: P4,
  prop5: P5
): Either.Either<Error, NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>;

// prettier-ignore
export function get<
  T,
  P1 extends keyof NonNullable<T>,
  P2 extends keyof NonNullable<NonNullable<T>[P1]>,
  P3 extends keyof NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>,
  P4 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>,
  P5 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>,
  P6 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>
>(
  obj: T,
  prop1: P1,
  prop2: P2,
  prop3: P3,
  prop4: P4,
  prop5: P5,
  prop6: P6
): Either.Either<Error, NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>[P6]>;

// prettier-ignore
export function get<
  T,
  P1 extends keyof NonNullable<T>,
  P2 extends keyof NonNullable<NonNullable<T>[P1]>,
  P3 extends keyof NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>,
  P4 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>,
  P5 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>,
  P6 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>,
  P7 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>[P6]>
>(
  obj: T,
  prop1: P1,
  prop2: P2,
  prop3: P3,
  prop4: P4,
  prop5: P5,
  prop6: P6,
  prop7: P7
): Either.Either<Error, NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>[P6]>[P7]>;

// prettier-ignore
export function get<
  T,
  P1 extends keyof NonNullable<T>,
  P2 extends keyof NonNullable<NonNullable<T>[P1]>,
  P3 extends keyof NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>,
  P4 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>,
  P5 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>,
  P6 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>,
  P7 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>[P6]>,
  P8 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>[P6]>[P7]>
>(
  obj: T,
  prop1: P1,
  prop2: P2,
  prop3: P3,
  prop4: P4,
  prop5: P5,
  prop6: P6,
  prop7: P7,
  prop8: P8
): Either.Either<Error, NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>[P6]>[P7]>[P8]>;

// prettier-ignore
export function get<
  T,
  P1 extends keyof NonNullable<T>,
  P2 extends keyof NonNullable<NonNullable<T>[P1]>,
  P3 extends keyof NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>,
  P4 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>,
  P5 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>,
  P6 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>,
  P7 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>[P6]>,
  P8 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>[P6]>[P7]>,
  P9 extends keyof NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>[P6]>[P7]>[P8]>
>(
  obj: T,
  prop1: P1,
  prop2: P2,
  prop3: P3,
  prop4: P4,
  prop5: P5,
  prop6: P6,
  prop7: P7,
  prop8: P8,
  prop9: P9
): Either.Either<Error, NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<NonNullable<T>[P1]>[P2]>[P3]>[P4]>[P5]>[P6]>[P7]>[P8]>[P9]>;

export function get<T>(obj: T, ...props: any[]): Either.Either<Error, unknown>;

export function get(origin: any, ...props: any[]): Either.Either<Error, unknown> {
  return isReadable(origin) ? props.reduce(getChild, origin) : ERR_UNREADABLE_ORIGIN(props, origin);
}

/** Used internally by get to retrieve a single child property from a parent object. */
function getChild(parent: unknown, prop: string): unknown | Either.Either<Error, never> {
  // quit if any ancestor was already not found
  if (Either.isEither(parent) && Either.isLeft(parent)) return parent;
  // ensure we have a plain value and not an Ok
  const value =
    Either.isEither(parent) && Either.isRight(parent) ? Either.getOrThrow(parent) : parent;
  // quit if we can't read properties of value (eg value.likeThis)
  if (!isReadable(value)) return ERR_UNREADABLE_CHILD(prop, value);
  // quit if value is object/array/function etc but the child is not found
  if (!isPrimitive(value) && prop in (value as any) === false) return ERR_NOT_FOUND(prop, value);
  // quit if eg true.toFixed, 12.toUpperCase
  if (isPrimitive(value) && (value as any)[prop] === undefined) return ERR_NOT_FOUND(prop, value);
  // the value is present, return it
  const child = (value as any)[prop];
  return Either.right(isFunction(child) ? child.bind(value) : child);
}

function ERR_UNREADABLE_CHILD(child: string, value: unknown) {
  return Either.left(new Error(`Cannot read "${child}" from unreadable value: ${value}`));
}

function ERR_NOT_FOUND(child: string, value: any) {
  return Either.left(new Error(`Property "${child}" not found on value: ${value}`));
}

function ERR_UNREADABLE_ORIGIN(props: any[], origin: any) {
  return Either.left(
    new Error(`Cannot read "${props.join('.')}" from unreadable value: ${origin}`),
  );
}
