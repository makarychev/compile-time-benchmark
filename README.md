# Benchmark

Count restarts: 10 times

Table shows avg time


| count tables | sqlx(offline) | sqlx(online) | diesel | postgres connector |
|---|---|---|---|---|
| 1000 | 4m - prepare</br>9m - build | 10m 19s - build | overtime | 3.5s - build |
| 100 | 48s - prepare</br>34s - build | 35s - build | 50.6s | 3.5s - build |
