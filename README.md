# Pygeohash-Fast

A Fast geohasher for python.

Created by wrapping the rust [geohash crate](https://docs.rs/crate/geohash/latest) with pyo3.

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

10.8 s ± 77.4 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)
```

Results for Pygeohash-fast encode:
```
In [16]: %%timeit
    ...: [fast_encode(lngs[i], lats[i], 8) for i in range(len(lats))]
3.44 s ± 11.4 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)
```

However, in order to improve things in further when encoding many points we have methods for encoding many points at once.
```
In [22]: %%timeit
    ...: encode_many(lngs, lats, 8)
155 ms ± 3.58 ms per loop (mean ± std. dev. of 7 runs, 10 loops each)
```

So for simple geohash encoding this library is ~69x times faster, nice.