# permutation-rs
Permutations in Rust

Takes two inputs from the console: n and r. n is the number of elements in the list to permute, r is the number of elements in the permuted lists. After running, a list of all the permuitations will be printed. Currently uses i32, but permutations can get very large, but such a large number is probably not necessary.

I just made this to mess around with the Rust language. This was originally intended to be a port of [permutation-py](https://github.com/stdonnelly/permutation-py). There turned out to be an issue with the logic of that program that caused it to not display all permutations if n > r. This one *should* work, though.
