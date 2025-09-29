# Performance comparision between passing primitive arguments through JNI vs a NIO buffer

JNI performs better, but NIO isn't that far behind.

On my machine:

```
:::::::::: Test with repeatTimes = 1000000 ::::::::::
jni: mean = 84.751886 ns, stddev = 2280.7731249608037 ns
nio: mean = 118.043559 ns, stddev = 6526.972036784532 ns

:::::::::: Test with repeatTimes = 2000000 ::::::::::
jni: mean = 115.695664 ns, stddev = 86.12123348676393 ns
nio: mean = 136.739708 ns, stddev = 38.27032871675182 ns

:::::::::: Test with repeatTimes = 3000000 ::::::::::
jni: mean = 74.173058 ns, stddev = 28.28874737916737 ns
nio: mean = 100.622228 ns, stddev = 29.010622404589558 ns

:::::::::: Test with repeatTimes = 4000000 ::::::::::
jni: mean = 77.2469375 ns, stddev = 22.94167606975704 ns
nio: mean = 103.71103 ns, stddev = 407.6401294914906 ns

:::::::::: Test with repeatTimes = 5000000 ::::::::::
jni: mean = 77.837555 ns, stddev = 25.983173736226984 ns
nio: mean = 103.1584368 ns, stddev = 30.265895800028638 ns

:::::::::: Test with repeatTimes = 6000000 ::::::::::
jni: mean = 117.6930505 ns, stddev = 30.254931958209237 ns
nio: mean = 142.68516216666666 ns, stddev = 31.674496307092785 ns

:::::::::: Test with repeatTimes = 7000000 ::::::::::
jni: mean = 150.44903014285714 ns, stddev = 29.832185472961235 ns
nio: mean = 175.64935557142857 ns, stddev = 101.88797508154133 ns

:::::::::: Test with repeatTimes = 8000000 ::::::::::
jni: mean = 76.503230125 ns, stddev = 24.415073409321845 ns
nio: mean = 102.17382625 ns, stddev = 21.90869397503551 ns

:::::::::: Test with repeatTimes = 9000000 ::::::::::
jni: mean = 80.22905733333333 ns, stddev = 22.539792055064968 ns
nio: mean = 104.4624691111111 ns, stddev = 26.147517729883546 ns

:::::::::: Test with repeatTimes = 10000000 ::::::::::
jni: mean = 79.8554643 ns, stddev = 435.916682281931 ns
nio: mean = 103.8540566 ns, stddev = 29.016043953565354 ns
```
