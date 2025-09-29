# Performance comparision between passing primitive arguments through JNI vs a NIO buffer

JNI performs better, but NIO isn't that far behind.

On my machine:

```
:::::::::: Test with repeatTimes = 1000000 ::::::::::
jni: mean = 75.598854 ns, stddev = 2611.721553176418 ns
nio: mean = 123.975689 ns, stddev = 9034.164432826763 ns

:::::::::: Test with repeatTimes = 2000000 ::::::::::
jni: mean = 116.244566 ns, stddev = 167.1807003023769 ns
nio: mean = 140.1840645 ns, stddev = 105.2874974190477 ns

:::::::::: Test with repeatTimes = 3000000 ::::::::::
jni: mean = 116.73438133333333 ns, stddev = 7495.316721519419 ns
nio: mean = 136.057449 ns, stddev = 49.39255812992721 ns

:::::::::: Test with repeatTimes = 4000000 ::::::::::
jni: mean = 84.54980075 ns, stddev = 10940.676798294107 ns
nio: mean = 103.1683005 ns, stddev = 41.70713386117052 ns

:::::::::: Test with repeatTimes = 5000000 ::::::::::
jni: mean = 81.1207192 ns, stddev = 227.76817666430426 ns
nio: mean = 102.7200516 ns, stddev = 42.333057211678 ns

:::::::::: Test with repeatTimes = 6000000 ::::::::::
jni: mean = 116.24358033333333 ns, stddev = 36.10852712909876 ns
nio: mean = 144.9869875 ns, stddev = 145.83807641092292 ns

:::::::::: Test with repeatTimes = 7000000 ::::::::::
jni: mean = 81.28336342857143 ns, stddev = 10657.319191096823 ns
nio: mean = 98.62485914285715 ns, stddev = 371.8601946635312 ns

:::::::::: Test with repeatTimes = 8000000 ::::::::::
jni: mean = 80.8613945 ns, stddev = 4353.313148332554 ns
nio: mean = 106.047989875 ns, stddev = 162.8947638028518 ns

:::::::::: Test with repeatTimes = 9000000 ::::::::::
jni: mean = 81.34867933333334 ns, stddev = 440.783998976681 ns
nio: mean = 108.32370333333333 ns, stddev = 8888.844821486626 ns

:::::::::: Test with repeatTimes = 10000000 ::::::::::
jni: mean = 80.0809942 ns, stddev = 155.45244784337862 ns
nio: mean = 97.9288711 ns, stddev = 28.032415289553068 ns
```
