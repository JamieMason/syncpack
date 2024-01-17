import { Data, Effect, pipe } from 'effect';
import type { Io } from './index.js';
import { IoTag } from './index.js';

class AskForChoiceError extends Data.TaggedClass('AskForChoiceError')<{
  readonly error: string;
}> {}

export function askForChoice(opts: {
  message: string;
  choices: string[];
}): Effect.Effect<Io, AskForChoiceError, string> {
  return pipe(
    IoTag,
    Effect.flatMap((io) =>
      Effect.tryPromise({
        try: () =>
          io.enquirer
            .prompt({
              type: 'select',
              name: 'choice',
              message: opts.message,
              choices: opts.choices,
            })
            .then((res) => res.choice),
        catch: (err) => new AskForChoiceError({ error: String(err) }),
      }),
    ),
  );
}
