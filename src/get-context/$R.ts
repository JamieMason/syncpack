import { R } from '@mobily/ts-belt';
import { BaseError } from '../lib/error';

/** Additional helpers for https://mobily.github.io/ts-belt/api/result */
export const $R = {
  /**
   * Return an R.Ok<output[]> for every R.Result which succeeded, or an
   * R.Error<BaseError> if none succeeded.
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
};
