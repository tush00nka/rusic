# About
`rusic` is a cli tool that generates actual music from provided text file, which contains semitones and their durations in beats.

# Build
```
cargo build --release
```

# Usage
```
rusic <FILENAME> [OPTIONS]
OPTIONS:
    -o <filename>   set output filename
```

# Example
Darude - Sandstorm
```txt
0   0.25 
0   0.25 
0   0.25 
0   0.25 
0   0.5 
0   0.25 
0   0.25 
0   0.25 
0   0.25 
0   0.25 
0   0.25 
0   0.5 
5   0.25 
5   0.25 
5   0.25 
5   0.25 
5   0.25 
5   0.25 
5   0.5 
3   0.25 
3   0.25 
3   0.25 
3   0.25 
3   0.25 
3   0.25 
3   0.5 
-2  0.5 
```