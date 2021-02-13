# Advent of Code 2020 

[Advent of Code 2020](https://adventofcode.com/2020) was a great challenge. I ran out of time to tackle it in December when it was live, so had fun with it in January instead. I'd highly recommend doing it. I'll definitely do it again in 2021. Donate too if you're able.

For an additional challenge, I completed the whole thing in Rust. Not particularly neat Rust, either -- it's not meant to be production code :) 

It looks like I wasn't the only one who took this approach, as the [unofficial survey](https://www.reddit.com/r/rust/comments/knyoej/rust_is_the_second_most_used_language_for_advent/) shows that Rust was the most-used language after Python. I think there are quite a few developers interested in it. Python is obviously going to be faster for solving these problems when competing for time.

I had a bit of time to play with some interesting Rust crates, so I'm noting these in case I want to use them again: 
- [nom](https://crates.io/crates/nom) parser combinator is really cool; days 18, 19, 21, 22, 24. 
- [regex](https://crates.io/crates/regex) of course, but I could have used `nom` a bit more earlier on.
- [itertools](https://crates.io/crates/itertools) is useful as always, particularly the assertions and `iproduct!` macro for generating cartesian products over multiple dimensions.
- [lazy_static](https://crates.io/crates/lazy_static) is always useful too
- [anyhow](https://crates.io/crates/anyhow) and [eyre](https://crates.io/crates/eyre) are great for neater error management in the main program
- [ndarray](https://crates.io/crates/ndarray) used for some multidimensional array stuff, but wasn't required. I generally find it useful, so the extra practice with it was useful. Check out [nalgebra](https://crates.io/crates/nalgebra) too.
- [strum](https://crates.io/crates/strum) is useful for enumerating through available elements in `enum` types
