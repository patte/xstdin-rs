# xstdin

Like `xargs`, but for stdin. Like `parallel`, but keeps a set or workers running.
Imagined by [paddor](https://github.com/paddor/). Developed with help by GPT-4.

## Installation

```bash
cargo install --path .
```

## Usage
```
Usage: xstdin [-n NUM] <command> [<arg1> <arg2> ...]

Options:
    -n NUM              set number of workers (default is 4)
    -h, --help          print this help menu
```

## Examples
```bash
seq 1 10 | xstdin -n 2 cat
1
3
5
7
9
2
4
6
8
10
```

```bash
seq 1 10 | xstdin -n 2 -- ruby -e 'STDIN.each_line { |line| puts "#$$: #{line}" }'
23026: 2
23014: 1
23026: 4
23014: 3
23026: 6
23014: 5
23026: 8
23014: 7
23026: 10
23014: 9
```

MacBook Air, M2, 2023:
```bash
yes | pv --rate | cat > /dev/null
[3.79GiB/s]

# strict round robin
yes | pv --rate | xstdin -l cat > /dev/null
[1.55MiB/s]

# chunked round robin
yes | pv --rate | xstdin cat > /dev/null
[2.64GiB/s]

yes | pv --rate | xstdin -b 32000 cat > /dev/null
[3.29GiB/s]
``````