import chalk from 'chalk-template';
import { Effect, LogLevel, Logger } from 'effect';

export function withLogger(program: Effect.Effect<unknown>) {
  const logger = Logger.make(({ logLevel, message }) => {
    const args = Array.isArray(message) ? message : [message];
    if (logLevel === LogLevel.Info) {
      globalThis.console.info(...args);
    } else if (logLevel === LogLevel.Debug) {
      globalThis.console.info(chalk`{magenta ? %s}`, ...args);
    } else if (logLevel === LogLevel.Error) {
      globalThis.console.error(chalk`{red ! %s}`, ...args);
    } else if (logLevel === LogLevel.Warning) {
      globalThis.console.warn(chalk`{yellow ! %s}`, ...args);
    } else {
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
