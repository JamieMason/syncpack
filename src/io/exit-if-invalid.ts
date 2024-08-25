import { Effect, pipe } from 'effect';
import type { Ctx } from '../get-context/index.js';
import { type Io, IoTag } from './index.js';

export function exitIfInvalid(ctx: Ctx): Effect.Effect<Ctx, never, Io> {
  return pipe(
    IoTag,
    Effect.tap(io =>
      Effect.sync(() => {
        if (ctx.isInvalid) {
          io.process.exit(1);
        }
      }),
    ),
    Effect.map(() => ctx),
  );
}
