# Performance comparision of not sending the buffer in every call

Result: Not sending the buffer improves performance

- `nio` uses the code from before, which sends a `ByteBuffer` argument for each call
- `nio2` avoids the `ByteBuffer` argument by setting it once at startup time.  It also tries to
simulate how this would work in the real world by defining a "stack", where the current stack
position are stred in the first 8 bytes.

`nio2` performed better, I think this is partly because it doesn't need to send the `ByteBuffer`
argument but also because we don't have to call `env.get_direct_buffer_address` and
`env.get_direct_buffer_capacity`.

On my machine:

```
:::::::::: Test with repeatTimes = 1000000 ::::::::::
nio: mean = 70.455725 ns, stddev = 118.35507506515376 ns
nio2: mean = 70.827754 ns, stddev = 8279.39721239206 ns

:::::::::: Test with repeatTimes = 2000000 ::::::::::
nio: mean = 62.7488685 ns, stddev = 26.794918743784844 ns
nio2: mean = 44.280129 ns, stddev = 25.07001303824103 ns

:::::::::: Test with repeatTimes = 3000000 ::::::::::
nio: mean = 63.122681 ns, stddev = 27.04163607713918 ns
nio2: mean = 41.208624 ns, stddev = 19.731948021148952 ns

:::::::::: Test with repeatTimes = 4000000 ::::::::::
nio: mean = 62.79826575 ns, stddev = 30.69965794608892 ns
nio2: mean = 40.7110895 ns, stddev = 19.637297236305873 ns

:::::::::: Test with repeatTimes = 5000000 ::::::::::
nio: mean = 63.3150262 ns, stddev = 405.8374367812536 ns
nio2: mean = 42.6128214 ns, stddev = 21.928530204686318 ns

:::::::::: Test with repeatTimes = 6000000 ::::::::::
nio: mean = 63.40525316666667 ns, stddev = 22.951453231250007 ns
nio2: mean = 42.4046745 ns, stddev = 20.629978649279618 ns

:::::::::: Test with repeatTimes = 7000000 ::::::::::
nio: mean = 63.773667 ns, stddev = 299.4293644136117 ns
nio2: mean = 41.56246228571428 ns, stddev = 89.96852393147313 ns

:::::::::: Test with repeatTimes = 8000000 ::::::::::
nio: mean = 63.45844775 ns, stddev = 105.39012803794074 ns
nio2: mean = 41.554047125 ns, stddev = 15.846937185474804 ns

:::::::::: Test with repeatTimes = 9000000 ::::::::::
nio: mean = 63.100746111111114 ns, stddev = 22.632153628946057 ns
nio2: mean = 41.50981355555555 ns, stddev = 15.88294342026826 ns

:::::::::: Test with repeatTimes = 10000000 ::::::::::
nio: mean = 63.0169447 ns, stddev = 410.4743833025464 ns
nio2: mean = 41.1557025 ns, stddev = 65.51080168898153 ns
```



