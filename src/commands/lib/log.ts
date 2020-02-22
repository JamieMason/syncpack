export type Logger = (...args: string[]) => void;

export const log: Logger = (...args) => console.log(...args);
