import { Data, Effect, pipe } from 'effect';
import type { Io } from './index.js';
import { IoTag } from './index.js';

class AskForInputError extends Data.TaggedClass('AskForInputError')<{
  readonly error: string;
}> {}

export function askForInput(opts: {
  message: string;
}): Effect.Effect<string, AskForInputError, Io> {
  return pipe(
    IoTag,
    Effect.flatMap((io) =>
      Effect.tryPromise({
        try: () =>
          io.enquirer.prompt({
            name: 'version',
            type: 'input',
            message: opts.message,
          }),
        catch: (err) => new AskForInputError({ error: String(err) }),
      }),
    ),
  );
}
