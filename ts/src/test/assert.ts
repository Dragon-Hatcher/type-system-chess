type AssertCount<T, N extends UnionCount<T>> = N;
type AssertOutcome<S extends State, U extends CurOutcome<S>> = U;
type AssertMoveDoes<
  S extends State,
  M extends string,
  NS extends ApplyMove<S, Unwrap<ParseMove<S, M>>>
> = NS;
