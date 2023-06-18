import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import type { Ctx } from '../get-context';
import { type Env } from './create-env';
import { EnvTag } from './tags';

// @TODO what's the effect-ts way to process.exit?
export function exitIfInvalid(ctx: Ctx): Effect.Effect<Env, never, Ctx> {
  return pipe(
    EnvTag,
    Effect.flatMap((env) => (ctx.isInvalid ? env.exitProcess(1) : Effect.unit())),
    Effect.map(() => ctx),
  );
}
