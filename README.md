# dhist

Save and sort most often used dmenu-like input

```
USAGE:
    dhist [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information

SUBCOMMANDS:
    help         Print this message or the help of the given subcommand(s)
    increment    Increase usage of input by 1
    query        Print history
    sort         Sort input by history frequency
    wrap         Wrap a command to sort before and increment after
```

## Examples

```
# sort input of dmenu based on usage
printf "%s\n" hello world | dhist wrap -- dmenu

# same as above, but more verbose
# dhist increment also prints out it's input, so you can still use it for another program
printf "%s\n" hello world | dhist sort | dmenu | dhist increment
```
