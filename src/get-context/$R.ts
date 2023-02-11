import { R } from '@mobily/ts-belt';
import { BaseError } from '../lib/error';
import { verbose } from '../lib/log';

/** Additional helpers for https://mobily.github.io/ts-belt/api/result */
export const $R = {
  /**
   * Return a single `R.Ok<output[]>` containing an array of the output of every
   * `R.Result` returned by `getResult(input)` which succeeded, or a single
   * `R.Error<BaseError>` if none succeeded.
   */
  onlyOk<Input, Output = Input>(
    getResult: (value: Input) => R.Result<Output, BaseError>,
  ) {
    return (inputs: Input[]): R.Result<Output[], BaseError> => {
      const outputs: Output[] = [];
      for (const value of inputs) {
        const result = getResult(value);
        if (R.isError(result)) continue;
        outputs.push(R.getExn(result));
      }
      return outputs.length > 0
        ? (R.Ok<Output[]>(outputs) as R.Result<Output[], BaseError>)
        : R.Error(new BaseError('No R.Ok() returned by $R.onlyOk'));
    };
  },
  /** Log verbose only when R.Result is an R.Err */
  tapErrVerbose<T extends R.Result<unknown, BaseError>>(result: T) {
    if (R.isError(result)) verbose(result._0.message);
    return result;
  },
};
