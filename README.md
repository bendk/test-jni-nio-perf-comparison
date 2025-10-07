# Performance comparision between different struct layouts

Result: Layout doesn't seem to affect performance

- `nio` uses the code from before, which lays out the struct exactly like Rust.
- `nio2` swaps the field order.
- The performance seems to be about the same, probably because we need to load things into registers
  in either case.

On my machine:

```
:::::::::: Test with repeatTimes = 1000000 ::::::::::
nio: mean = 72.373148 ns, stddev = 2793.6788202203775 ns
nio2: mean = 78.682876 ns, stddev = 7349.068359459517 ns

:::::::::: Test with repeatTimes = 2000000 ::::::::::
nio: mean = 62.9509375 ns, stddev = 46.53128626371213 ns
nio2: mean = 62.8966675 ns, stddev = 22.30623069678319 ns

:::::::::: Test with repeatTimes = 3000000 ::::::::::
nio: mean = 62.850941 ns, stddev = 18.344760807475133 ns
nio2: mean = 63.04918033333333 ns, stddev = 238.2759300896244 ns

:::::::::: Test with repeatTimes = 4000000 ::::::::::
nio: mean = 63.61167625 ns, stddev = 83.25218715122796 ns
nio2: mean = 63.29407825 ns, stddev = 22.70440136275744 ns

:::::::::: Test with repeatTimes = 5000000 ::::::::::
nio: mean = 63.800994 ns, stddev = 20.87771171866185 ns
nio2: mean = 63.7865986 ns, stddev = 25.906912831317218 ns

:::::::::: Test with repeatTimes = 6000000 ::::::::::
nio: mean = 61.56280233333333 ns, stddev = 20.966614561679904 ns
nio2: mean = 62.01736533333333 ns, stddev = 18.036530527835627 ns

:::::::::: Test with repeatTimes = 7000000 ::::::::::
nio: mean = 62.09478871428571 ns, stddev = 351.99357316676054 ns
nio2: mean = 63.923044857142855 ns, stddev = 312.35291224050064 ns

:::::::::: Test with repeatTimes = 8000000 ::::::::::
nio: mean = 62.672238625 ns, stddev = 53.745339507936706 ns
nio2: mean = 64.225707875 ns, stddev = 431.00850241776834 ns

:::::::::: Test with repeatTimes = 9000000 ::::::::::
nio: mean = 62.456520777777776 ns, stddev = 22.204213959392217 ns
nio2: mean = 63.64954211111111 ns, stddev = 250.6515181110457 ns

:::::::::: Test with repeatTimes = 10000000 ::::::::::
nio: mean = 62.9868806 ns, stddev = 22.966277064660314 ns
nio2: mean = 64.1415138 ns, stddev = 21.1885820925125 ns
```



