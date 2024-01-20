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

```bash
cat examples/input.txt | xstdin -n 2 ruby examples/ruby-cat.rb
74048: line2
74036: line1
74048: line5
74048: line7
74036: line4
74048: line9
74036: line6
74048: line11
74036: line8
74048: line13
74036: line10
74048: line15
74036: line12
74036: line14
74036: line16
```