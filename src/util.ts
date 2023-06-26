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
