# Performance comparision different ways to pass arguments

- `jni` uses "normal" JNI calls.
- `nio` uses JNI calls, but with 0 arguments and no return values.
  Instead, the code sets up a shared `ByteBuffer` to pass arguments with.
  This buffer is essentially a manually managed stack.
- `jna-nio` does the same thing, but using JNA direct mapping

`jni` performed the fastest
`nio` was only slightly behind `jni`.
`jna-nio` was significantly behind `jna`

My (BDK) feeling is that we should pursue the `nio` route.  For a slight decrease in performance, we
get a major win in simplicity.  Having all signatures be `Fn() -> ()`, simplifies a lot of things.
It also means we can probably create a scaffolding layer that Kotlin/Java/Python/JS can all share,
and keep the language-specific stuff to an absolute minimum.  Also, the `nio` route performed
significantly better when dealing with structs rather than primitive arguments.

I'm thinking that it's worth it to switch to JNI somehow over the JNA route, since it results in
significant preformance gains.

BTW, when I run the current benchmarks, I see ~22us for a kotlin call.  This is not completely
apples-to-apples, but it's clear that any of these approaches will be much faster than the current
system.  I also believe that this performance difference increases when records and enums are
involved.

Results my machine:

```
:::::::::: Test with repeatTimes = 1000000 ::::::::::
jni: mean = 38.532333 ns, stddev = 2466.985576446869 ns
nio: mean = 47.597483 ns, stddev = 284.50683722421365 ns
jna-nio: mean = 100.24607 ns, stddev = 237.57085014298437 ns

:::::::::: Test with repeatTimes = 2000000 ::::::::::
jni: mean = 33.8936685 ns, stddev = 25.35427467125202 ns
nio: mean = 42.810357 ns, stddev = 147.23586735813916 ns
jna-nio: mean = 86.7266135 ns, stddev = 198.7638950772463 ns

:::::::::: Test with repeatTimes = 3000000 ::::::::::
jni: mean = 40.94717633333333 ns, stddev = 146.77964417200022 ns
nio: mean = 56.72405233333333 ns, stddev = 27467.423739141363 ns
jna-nio: mean = 78.61947833333333 ns, stddev = 34.46538820706429 ns

:::::::::: Test with repeatTimes = 4000000 ::::::::::
jni: mean = 44.5093275 ns, stddev = 11220.125539812057 ns
nio: mean = 37.51436675 ns, stddev = 100.98953455883195 ns
jna-nio: mean = 82.1094655 ns, stddev = 5527.79297989799 ns

:::::::::: Test with repeatTimes = 5000000 ::::::::::
jni: mean = 34.6896766 ns, stddev = 18.631050222333986 ns
nio: mean = 44.0245922 ns, stddev = 313.361844114639 ns
jna-nio: mean = 78.8225686 ns, stddev = 29.721957120776693 ns

:::::::::: Test with repeatTimes = 6000000 ::::::::::
jni: mean = 38.088654 ns, stddev = 6740.068070091855 ns
nio: mean = 40.51859283333334 ns, stddev = 5866.556687654979 ns
jna-nio: mean = 88.32087516666667 ns, stddev = 163.2923409282288 ns

:::::::::: Test with repeatTimes = 7000000 ::::::::::
jni: mean = 34.52556728571429 ns, stddev = 19.60341186032824 ns
nio: mean = 41.38253942857143 ns, stddev = 10635.5606706582 ns
jna-nio: mean = 83.38344214285715 ns, stddev = 8203.829069728612 ns

:::::::::: Test with repeatTimes = 8000000 ::::::::::
jni: mean = 34.53803875 ns, stddev = 69.60842925440366 ns
nio: mean = 37.120244375 ns, stddev = 18.51628532983105 ns
jna-nio: mean = 80.129109 ns, stddev = 4550.571768203493 ns

:::::::::: Test with repeatTimes = 9000000 ::::::::::
jni: mean = 37.39180511111111 ns, stddev = 9158.964454301542 ns
nio: mean = 44.79688411111111 ns, stddev = 6569.902422806481 ns
jna-nio: mean = 79.55691911111111 ns, stddev = 53.536192270482175 ns

:::::::::: Test with repeatTimes = 10000000 ::::::::::
jni: mean = 38.7813579 ns, stddev = 12003.784356838858 ns
nio: mean = 41.7472073 ns, stddev = 10090.22763102756 ns
jna-nio: mean = 84.4977933 ns, stddev = 122.34571706129732 ns
```
