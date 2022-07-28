# Pygeohash-Fast

A Fast geohasher for python.

Created by wrapping the rust [geohash create](https://docs.rs/crate/geohash/latest) with pyo3.

Huge shout out to the georust community :)

Currently very WIP, only supports encoding geohashes.

# Speed Comparisons
Compared against the great python library [pygeohash](https://github.com/wdm0006/pygeohash/blob/master/pygeohash/geohash.py)
This isn't a perfect benchmark, but should illstrate what's possible.
Tested on 1 million randomly generated lat/long pairs.

Results for Pygeohash
```
In [14]: %%timeit
    ...: [encode(lats[i], lngs[i], 8) for i in range(len(lats))]
    ...: 
    ...: 
10.8 s ± 77.4 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)
```

Results for Pygeohash-fast encode:
```
In [16]: %%timeit
    ...: [fast_encode(lngs[i], lats[i], 8) for i in range(len(lats))]
    ...: 
    ...: 
3.44 s ± 11.4 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)
```

However, in order to improve things in further when encoding many points we have methods for encoding many points at once.
```
In [19]: %%timeit
    ...: encode_many(lngs, lats, 8)
    ...: 
    ...: 
1.25 s ± 8.44 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)
```

So for simple encoding this library is ~8.64 times faster.