type DropFirst<T extends unknown[]> = T extends [any, ...infer U] ? U : [];

type SpaceAppend<
  A extends string,
  B extends string,
  Sep extends string = ""
> = A extends "" ? B : `${A}${Sep}${B}`;

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
