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
cat examples/input.txt | xstdin -n 2 cat
line1
line4
line6
line8
line10
line12
line14
line16
line2
line5
line7
line9
line11
line13
line15
```
