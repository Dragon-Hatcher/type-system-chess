type AssertCount<T, N extends UnionCount<T>> = N;
type AssertOutcome<S extends State, U extends CurOutcome<S>> = U;
