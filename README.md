# mdxlroptimizer
[![GitHub Actions Workflow](https://github.com/Nebulavenus/mdxlroptimizer/workflows/Build/badge.svg)](https://github.com/Nebulavenus/mdxlroptimizer/actions)
[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Nebulavenus/mdxlroptimizer/blob/master/LICENSE)

Tool for optimizing the size of .mdx files used in warcraft 3.
Currently works only for 1.26 patch. Support for new patches planned.

### Usage
```
USAGE:
    mdxlroptimizer.exe [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help         Prints help information
        --linearize    Converts hermite/bezier to linear. Simplify keyframes
        --log          Writes everything into a log file
        --outside      Delete redundant frames but outside anim sequences
    -V, --version      Prints version information

OPTIONS:
    -t, --threshold <threshold>

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    optimize    Optimize mdl file
```

### Example
```
mdxlroptimizer --linearize --threshold 0.01 optimize <file_name> | druidcat.mdx
```
It produces a file with postfix <file_name>_optimized.mdx

### License
MIT
