import type { AnyResult } from 'tightrope/fn/types';
import type { Result } from 'tightrope/result';
import { Err, Ok } from 'tightrope/result';
import { isErr } from 'tightrope/result/is-err';
import { unwrap } from 'tightrope/result/unwrap';
import { logVerbose } from './log-verbose';

export const $R = {
  /**
   * Return a single `Ok<output[]>` containing an array of the output of every
   * `Result` returned by `getResult(input)` which succeeded, or a single
   * `Err<Error>` if none succeeded.
   */
  onlyOk<Input, Output = Input>(getResult: (value: Input) => Result<Output>) {
    return (inputs: Input[]): Result<Output[]> => {
      const outputs: Output[] = [];
      for (const value of inputs) {
        const result = getResult(value);
        if (isErr(result)) continue;
        outputs.push(unwrap(result));
      }
      return outputs.length > 0
        ? (new Ok<Output[]>(outputs) as Result<Output[]>)
        : new Err(new Error('No Ok() returned by $R.onlyOk'));
    };
  },
  /** Log verbose only when Result is an Err */
  tapErrVerbose<T extends AnyResult>(result: T) {
    if (isErr(result)) logVerbose(result.value);
    return result;
  },
};
