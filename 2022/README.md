# Advent of Code 2022

This year, I'm going to brush up on my rust by completing the challenges in rust.

It's been quite a while since I've used it so I'm... not that great at it anymore.*

Hopefully the code quality will improve as the month progresses.

## Methodology

For each challenge I'll start a new rust project with `cargo new`, and copy over `main.rs` from `template/`. I'll save the challenge input as `input.txt` and then get my results with this line:

```bash
cat input.txt | cargo run
```

I've added the `input.txt` to .gitignore so I don't accidentally commit it along with my code. I believe the input is probably different per user anyway.

I guess I am doing unit tests for parts of the challenges, as it's a very convenient way to test parts of my code on the examples given.

---

\* FINE, I'm _rusty_

