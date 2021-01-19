# Using `nom` parser-combinator

## Part 1 

This was fairly easy, but quite clunk during evaluation

## Part 2 

Part 2 was much more interesting, and much more general. Could re-write Part 1 using a variation of it pretty easily.

In this version, I build up an expression tree by using `fold` parsers.

In this strange world, addition takes precedence over multiplication. So we parse multiplication as the "outer" element, and it in turn parses addition expressions as "inner" elements. These, in turn, parse literal values and expressions in parens.

