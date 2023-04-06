import type { AnyResult } from 'tightrope/fn/types';
import type { Result } from 'tightrope/result';
import { Err, Ok } from 'tightrope/result';
import { isErr } from 'tightrope/result/is-err';
import { unwrap } from 'tightrope/result/unwrap';
import { BaseError } from '../lib/error';
import { verbose } from '../lib/log';

export const $R = {
  /**
   * Return a single `Ok<output[]>` containing an array of the output of every
   * `Result` returned by `getResult(input)` which succeeded, or a single
   * `Err<BaseError>` if none succeeded.
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
        : new Err(new BaseError('No Ok() returned by $R.onlyOk'));
    };
  },
  /** Log verbose only when Result is an Err */
  tapErrVerbose<T extends AnyResult>(result: T) {
    if (isErr(result)) verbose(result.value);
    return result;
  },
};
