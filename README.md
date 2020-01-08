# mdxlroptimizer
[![GitHub Actions Workflow](https://github.com/Nebulavenus/mdxlroptimizer/workflows/Build/badge.svg)](https://github.com/Nebulavenus/mdxlroptimizer/actions)
[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Nebulavenus/mdxlroptimizer/blob/master/LICENSE)

Tool for optimizing the size of .mdx files used in warcraft 3.
Currently works only for 1.26 patch. Support for new patches planned.

### Usage
```
USAGE:
    mdxlroptimizer.exe [FLAGS] [OPTIONS] <FILE>

FLAGS:
    -h, --help         Prints help information
        --linearize    Converts Hermite/Bezier interpolation to Linear
        --log          Log everything into a file
        --outside      Deletes keyframes outside of animation sequences
    -V, --version      Prints version information

OPTIONS:
    -o, --output <output>          Output file
    -t, --threshold <threshold>    Similar keyframes with a threshold difference [default: 0]

ARGS:
    <FILE>    File to process
```

### Example
```
mdxlroptimizer --linearize --threshold 0.01 <file_name> | druidcat.mdx
```
It produces a file with postfix <file_name>_optimized.mdx

### License
MIT
