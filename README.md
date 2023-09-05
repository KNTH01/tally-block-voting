# tally-block-voting

This program generates contest and votes in a context of block voting with m for district magnitude, which means that there are m winners.

# Usage

1) Use `generate` command to generate input: contest and votes
2) Use `tally` command to generate output and get the result: winners

```sh
# Usage: tally-block-voting --input <INPUT> --output <OUTPUT> <COMMAND>
$> ./tally-block-voting generate -i input.json
$> ./tally-block-voting tally -i input.json -o output.json
$> ./tally-block-voting --help
```


