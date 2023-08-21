import * as Context from '@effect/data/Context';
import * as Data from '@effect/data/Data';
import type { Env } from './create-env';

export const EnvTag = Context.Tag<Env>();

export class AskForChoiceError extends Data.TaggedClass('AskForChoiceError')<{
  readonly error: string;
}> {}

export class AskForInputError extends Data.TaggedClass('AskForInputError')<{
  readonly error: string;
}> {}

export class GlobError extends Data.TaggedClass('GlobError')<{
  readonly error: string;
}> {}

export class ReadConfigFileError extends Data.TaggedClass('ReadConfigFileError')<{
  readonly filePath: string;
  readonly error: string;
}> {}

export class ReadFileError extends Data.TaggedClass('ReadFileError')<{
  readonly filePath: string;
  readonly error: string;
}> {}

export class ReadYamlFileError extends Data.TaggedClass('ReadYamlFileError')<{
  readonly filePath: string;
  readonly error: string;
}> {}

export class WriteFileError extends Data.TaggedClass('WriteFileError')<{
  readonly filePath: string;
  readonly error: string;
}> {}

export interface JsonFile<T> {
  /** absolute path on disk to the directory of this file */
  readonly dirPath: string;
  /** absolute path on disk to this file */
  readonly filePath: string;
  /** relative path on disk to this file */
  readonly shortPath: string;
  /** parsed JSON contents of the file */
  contents: T;
  /** raw file contents of the file */
  readonly json: string;
}

export class JsonParseError extends Data.TaggedClass('JsonParseError')<{
  readonly error: string;
  readonly filePath: string;
  readonly json: string;
}> {}
