import chalk from 'chalk-template';
import { Effect, Logger, LogLevel } from 'effect';

export function withLogger(program: Effect.Effect<never, never, unknown>) {
  const logger = Logger.make(({ logLevel, message }) => {
    if (logLevel === LogLevel.Info) {
      globalThis.console.info(message);
    } else if (logLevel === LogLevel.Debug) {
      globalThis.console.info(chalk`{magenta ? %s}`, message);
    } else if (logLevel === LogLevel.Error) {
      globalThis.console.error(chalk`{red ! %s}`, message);
    } else if (logLevel === LogLevel.Warning) {
      globalThis.console.warn(chalk`{yellow ! %s}`, message);
    } else {
      globalThis.console.log(chalk`{cyan [%s] %s}`, logLevel, message);
    }
  });
  const layer = Logger.replace(Logger.defaultLogger, logger);
  const logLevel =
    process.env.SYNCPACK_VERBOSE === 'true'
      ? LogLevel.Debug
      : process.env.NODE_ENV === 'test'
        ? LogLevel.None
        : LogLevel.Info;
  return Effect.provide(Logger.withMinimumLogLevel(program, logLevel), layer);
}
