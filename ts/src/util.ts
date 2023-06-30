type DropFirst<T extends unknown[]> = T extends [any, ...infer U] ? U : [];
type DropFirstChar<S extends string> = S extends `${infer _}${infer Rest}`
  ? Rest
  : "";

type FirstChar<S extends string> = S extends `${infer C}${infer _}` ? C : "";
type StrChars<
  S extends string,
  A extends string = never
> = S extends `${infer C}${infer Rest}` ? StrChars<Rest, A | C> : A;
type Split<
  Text extends string,
  Sep extends string = "\n",
  A extends string[] = []
> = Text extends ""
  ? A
  : Text extends `${infer Prefix}${Sep}${infer Suffix}`
  ? Split<Suffix, Sep, [Prefix, ...A]>
  : [Text];
type TrimStart<T extends string> = T extends `${" " | "\t"}${infer Text}`
  ? TrimStart<Text>
  : T;
type TrimEnd<T extends string> = T extends `${infer Text}${" " | "\t"}`
  ? TrimEnd<Text>
  : T;
type Trim<T extends string> = TrimStart<TrimEnd<T>>;
type FullUppercase<
  S extends string,
  A extends string = ""
> = S extends `${infer C}${infer Rest}`
  ? FullUppercase<Rest, `${A}${Uppercase<C>}`>
  : A;

type Concat<A extends string, B extends string> = `${A}${B}`;
type Concat3<
  A extends string,
  B extends string,
  C extends string
> = `${A}${B}${C}`;

type UnionToIntersection<U> = (
  U extends never ? never : (arg: U) => never
) extends (arg: infer I) => void
  ? I
  : never;

type UnionToTuple<T, A extends any[] = []> = UnionToIntersection<
  T extends never ? never : (t: T) => T
> extends (_: never) => infer W
  ? UnionToTuple<Exclude<T, W>, [...A, W]>
  : A;

type UnionCount<T, A extends any[] = []> = UnionToIntersection<
  T extends never ? never : (t: T) => T
> extends (_: never) => infer W
  ? UnionCount<Exclude<T, W>, [...A, W]>
  : A["length"];

type Unwrap<U> = Exclude<U, null>;

type GetNonemptyLines<S extends string[], A extends string[] = []> = S extends [
  infer S1 extends string,
  ...infer Rest extends string[]
]
  ? GetNonemptyLines<Rest, Trim<S1> extends "" ? A : [Trim<S1>, ...A]>
  : A;

type NeverToNull<T> = [T] extends [never] ? null : T;
