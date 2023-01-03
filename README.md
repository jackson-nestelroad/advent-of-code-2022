# Advent of Code 2022
This repository features solutions to the 2022 **Advent of Code** hosted on https://adventofcode.com/2022/about.

## About
For my fifth year of doing Advent of Code, I chose to implement my solutions using the **Rust** programming language. I used this language in 2021 as well, but I chose to use this language again, as I have not used it much since then and very much enjoy using it.

## Solutions
My solutions are primarily written to be readable, maintainable, and reasonably efficient rather than making things as short or quick to implement as possible. My self-imposed goal was for each solution to run in less than one half-second (500 ms) individually. I did very well on this goal, and in fact, only two solutions (19 B and 24 B) consistently run over 100 ms when compiled with optimizations. The total runtime of all of my solutions together is around 0.6 seconds with optimizations applied:

```
$ cargo run --release all
1 A: 70509 (139 us)
1 B: 208567 (103 us)
2 A: 11873 (76 us)
2 B: 12014 (60 us)
3 A: 8298 (214 us)
3 B: 2708 (331 us)
4 A: 602 (83 us)
4 B: 891 (75 us)
5 A: BSDMQFLSP (215 us)
5 B: PGSQBFLDP (126 us)
6 A: 1275 (9 us)
6 B: 3605 (52 us)
7 A: 1543140 (158 us)
7 B: 1117448 (153 us)
8 A: 1796 (129 us)
8 B: 288120 (490 us)
9 A: 6209 (756 us)
9 B: 2460 (817 us)
10 A: 16060 (14 us)
###...##...##..####.#..#.#....#..#.####.
#..#.#..#.#..#.#....#.#..#....#..#.#....
###..#..#.#....###..##...#....####.###..
#..#.####.#....#....#.#..#....#..#.#....
#..#.#..#.#..#.#....#.#..#....#..#.#....
###..#..#..##..####.#..#.####.#..#.#....
10 B: check stdout (21 us)
11 A: 120056 (29 us)
11 B: 21816744824 (6329 us)
12 A: 383 (1967 us)
12 B: 377 (2317 us)
13 A: 5208 (570 us)
13 B: 25792 (600 us)
14 A: 745 (442 us)
14 B: 27551 (6245 us)
15 A: 5878678 (9 us)
15 B: 11796491041245 (38 us)
16 A: 1737 (15236 us)
16 B: 2216 (62999 us)
17 A: 3137 (540 us)
17 B: 1564705882327 (528 us)
18 A: 4242 (881 us)
18 B: 2428 (3519 us)
19 A: 2193 (12575 us)
19 B: 7200 (97082 us)
20 A: 7225 (4486 us)
20 B: 548634267428 (56263 us)
21 A: 49288254556480 (675 us)
21 B: 3558714869436 (653 us)
22 A: 95384 (442 us)
22 B: 15426 (574 us)
23 A: 3766 (1657 us)
23 B: 954 (107142 us)
24 A: 343 (54915 us)
24 B: 960 (146325 us)
25 A: 2-1-110-=01-1-0-0==2 (113 us)
25 B: Start The Blender (0 us)
All solutions ran in 0.589198022 seconds (589198 us)
```