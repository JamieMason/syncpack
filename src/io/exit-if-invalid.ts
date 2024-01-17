import { Effect, pipe } from 'effect';
import type { Ctx } from '../get-context/index.js';
import { IoTag, type Io } from './index.js';

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
