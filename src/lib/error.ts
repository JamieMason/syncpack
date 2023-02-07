export class BaseError extends Error {
  name: 'SyncpackError';
  cause?: BaseError | Error | null;

  constructor(
    message: string,
    options?: {
      cause?: unknown;
      props?: { args: any[] };
    },
  ) {
    super(message);
    this.name = 'SyncpackError';
    this.cause = BaseError.normalize(options?.cause);
  }

  static normalize(value: unknown): BaseError | Error | null {
    if (value instanceof BaseError) return value;
    if (value instanceof Error) return value;
    if (typeof value === 'string') return new Error(value);
    return null;
  }

  static map(message: string) {
    return (cause: unknown): BaseError => {
      return new BaseError(message, { cause });
    };
  }
}
