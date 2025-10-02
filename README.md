# Performance comparision between passing struct arguments through JNI vs a NIO buffer

NIO performs better on my machine.

- `jni` uses JNI code to read fields from a JVM struct
- `nio` uses a byte buffert to pass each arguments.  It serializes them using the Rust struct
    layout. We should be able to know this layout at codegen time using the `offset_of!` macro.

On my machine:

```
:::::::::: Test with repeatTimes = 1000000 ::::::::::
jni: mean = 74.349455 ns, stddev = 56.64028810164809 ns
nio: mean = 72.862406 ns, stddev = 537.1399059044735 ns

:::::::::: Test with repeatTimes = 2000000 ::::::::::
jni: mean = 73.165969 ns, stddev = 24.65722403857947 ns
nio: mean = 63.6984065 ns, stddev = 76.45442753520823 ns

:::::::::: Test with repeatTimes = 3000000 ::::::::::
jni: mean = 71.627474 ns, stddev = 24.213712996046816 ns
nio: mean = 62.99610966666667 ns, stddev = 20.054069641744526 ns

:::::::::: Test with repeatTimes = 4000000 ::::::::::
jni: mean = 73.233867 ns, stddev = 22.08773414642361 ns
nio: mean = 63.648628 ns, stddev = 22.24502932561238 ns

:::::::::: Test with repeatTimes = 5000000 ::::::::::
jni: mean = 72.893694 ns, stddev = 20.8190021142703 ns
nio: mean = 63.1709446 ns, stddev = 37.76875677205083 ns

:::::::::: Test with repeatTimes = 6000000 ::::::::::
jni: mean = 72.3611035 ns, stddev = 24.224454313810483 ns
nio: mean = 62.823570333333336 ns, stddev = 469.54843571396265 ns

:::::::::: Test with repeatTimes = 7000000 ::::::::::
jni: mean = 73.72710157142858 ns, stddev = 20.14013182660072 ns
nio: mean = 64.25377357142857 ns, stddev = 278.8820137675058 ns

:::::::::: Test with repeatTimes = 8000000 ::::::::::
jni: mean = 72.787555625 ns, stddev = 206.36717280425907 ns
nio: mean = 62.871828875 ns, stddev = 18.44826424176973 ns

:::::::::: Test with repeatTimes = 9000000 ::::::::::
jni: mean = 73.52163811111112 ns, stddev = 19.528077381583483 ns
nio: mean = 63.46867011111111 ns, stddev = 56.03658515526874 ns

:::::::::: Test with repeatTimes = 10000000 ::::::::::
jni: mean = 72.3297581 ns, stddev = 67.8119772468978 ns
nio: mean = 62.6916091 ns, stddev = 19.538797008178776 ns
```

