import { Effect, pipe } from 'effect';
import { IoTag, type Io } from '.';
import type { Ctx } from '../get-context';

export function exitIfInvalid(ctx: Ctx): Effect.Effect<Io, never, Ctx> {
  return pipe(
    IoTag,
    Effect.tap((io) =>
      Effect.sync(() => {
        if (ctx.isInvalid) {
          io.process.exit(1);
        }
      }),
    ),
    Effect.map(() => ctx),
  );
}
