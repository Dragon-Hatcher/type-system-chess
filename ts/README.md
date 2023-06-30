# Typescript Type Chess

A chess program written entirely in the Typescript type system.
TS's type system is actually turing complete so we can make
anything we want.

![A showcase of the program working. A chess board is shown in the VSCode hover window while focusing the type `PlayChess`](./showcase.png)

## Features

This chess program features all rules of chess except for draws
by repetition and the 50 move rule. That means it covers:

- Castling
- Promotion
- En Passant

You can play interactively using long algebraic notation, apply
moves to existing positions to modify them, or import positions
using the FEN parser.

Check `index.ts` for more on how to use.

## Bugs

I have tried to check for bugs though the slow speed of the program
makes this somewhat difficult. If you find a bug or add more tests
feel free to open an issue/PR.

## Inspiration

I read a number of different articles about programming in TS types
but this one specifically is where I drew most of my inspiration.

[Type System Game Engines](https://blog.joshuakgoldberg.com/type-system-game-engines/#the-final-product)

## Thoughts

As esoteric program languages go TS types aren't really that bad. I
would honestly hesitate to even call it a Turing-Tarpit; It's just
a slightly strange functional program language. With let bindings,
math, and a big speed increase it could be almost pleasant to use.
