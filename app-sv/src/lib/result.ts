export type Ok<T> = {
  data: T;
  error: null;
};

export type Err<E> = {
  data: null;
  error: E;
};

export async function Try<T, E>(fn: Promise<T>): Promise<Result<T, E>> {
  try {
    const result = await fn;
    return { data: result, error: null } as Ok<T>;
  } catch (error) {
    return { data: null, error: error as E } as Err<E>;
  }
}

export async function TrySync<T, E = Error>(
  fn: () => T,
): Promise<Result<T, E>> {
  try {
    const result = await fn();
    return { data: result, error: null } as Ok<T>;
  } catch (error) {
    return { data: null, error: error as E } as Err<E>;
  }
}

export type Result<T, E> = Ok<T> | Err<E>;
