# Type System Chess

This repo contains chess implemented entirely in the (stable) Rust and Typescript type systems. Both languages feature turing complete type systems making this possible. 

I believe that this is the most complicated program implemented in the Rust type system. If you know of any similarly complex Rust projects please let me know.

Other very complex type systems programs have been written in Typescript. For example, [HypeScript](https://github.com/ronami/HypeScript).

![A showcase of the typescript version of the program working. A chess board is shown in the VSCode hover window while focusing the type `PlayChess`](./showcase.png)

## Features

The programs implement all chess rules except for draws by repetition and the 50 move rule. That means they cover:

- Castling
- Promotion
- En Passant

Both versions allow you to play moves using long algebraic notation (specifying the start and end square). Because TS allows parsing strings in the type system I also implemented a FEN parser for the TS version.

Check `main.rs` and `index.ts` for more on how to use.

## Bugs

I have implemented a few tests for each program but it is likely that bugs still exist. If you find a bug or add more tests feel free to open an issue/PR.

## Performance

Both programs are rather slow. Running the TS test suite takes about 30 seconds on my computer and running the Rust test suite takes about 10 minutes.

Rust performance was a fair bit better up until the end when I added the various complex pawn moves. I suspect that the compiler might have some extra difficulties with how these are implemented.

I was almost able to get away without increasing the recursion limit for the rust program. It was fine after adding castling kingside but broke when I added queenside castling (i.e. it is very close to not needing the extra recursion).

## Inspiration

### Typescript 

I read a number of different articles about programming in TS types but this one specifically is where I drew most of my inspiration.

[Type System Game Engines](https://blog.joshuakgoldberg.com/type-system-game-engines/#the-final-product)

A few of the utility types I used are from this article.

### Rust

There aren't nearly as many resources about programming in the Rust typesystem (probably because, as I will explain shortly, it is *significantly* harder than typescript). The only one I found is this article which I believe is the original proof of the Turing Completeness of the Rust type system. I use the methodology described in this article.

[Rust's Type System is Turing-Complete](https://sdleffler.github.io/RustTypeSystemTuringComplete/)

## Thoughts

### Typescript

As esoteric program languages go TS types aren't really that bad. I would honestly hesitate to even call it a Turing-Tarpit; It's just a slightly strange functional program language. With let bindings, math, and a big speed increase it could be almost pleasant to use.

### Rust

Rust types are a real pain to work with. A few issues[^1] I encountered:

- Every Computation has to be performed twice.

  Each time you use a trait you have to rewrite the same thing twice, once in the where clause and once in the output type. In fact, if you nest layers you have to specify each layer again. So `Foo<Bar<Baz, Qaz>>` requires, roughly, this.

  ```rust
  where Baz: RunBar<Qaz>,
        Bar<Baz, Qaz>: RunFoo,
  type Output = Foo<Bar<Baz>>
  ```

  For some truly horifying examples of this look in `move_gen::castle`.

- There is no form of negative reasoning. The results in the type `RankEq` (which checks if two ranks are equal) being implemented as a lookup table for all 64 combinations of two ranks.

- Ironically there is no way to get the Rust compiler to understand that an enum is closed. So just becuase you implement something for colors `White` and `Black` doesn't mean the Rust compiler knows that you have implemented it for all implementers of the `ColorEn`. (Again[^1]).

- Since values can only exist in the top line of an impl block and not in the where clause matching on a value requires introducing a new nested type every time. You end up with a lot of types like `Foo` and then `FooWithValue` and `FooWithValueAndOtherValue`. It gets grating fast. 

- Compile times are really slow.

In summary: unlike Typescript I would never like to program in the Rust typesystem again.

[^1]: It is probably unfair to call something that makes it difficult to program chess in your type system a problem. These features exist for sensible reasons in normal Rust.