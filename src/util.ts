type DropFirst<T extends unknown[]> = T extends [any, ...infer U] ? U : [];
type DropFirstChar<S extends string> = S extends `${infer _}${infer Rest}`
  ? Rest
  : "";

type SpaceAppend<
  A extends string,
  B extends string,
  Sep extends string = ""
> = A extends "" ? B : `${A}${Sep}${B}`;

type FirstChar<S extends string> = S extends `${infer C}${infer _}` ? C : "";
type StrChars<
  S extends string,
  A extends string = never
> = S extends `${infer C}${infer Rest}` ?  StrChars<Rest, A | C> : A;

type Concat<A extends string, B extends string> = `${A}${B}`;
type Concat3<
  A extends string,
  B extends string,
  C extends string
> = `${A}${B}${C}`;
type Concat4<
  A extends string,
  B extends string,
  C extends string,
  D extends string
> = `${A}${B}${C}${D}`;

type ReObj<T extends object> = { [k in keyof T]: T[k] };
type AddProp<T extends object, U extends object> = ReObj<T & U>;

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
