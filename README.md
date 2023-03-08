# Benchmark

Count restarts: 10 times

Table shows avg time

| count tables| 100 | 1000 |
|---|---|---|
| sqlx prepare| 48s | 4m |
| sqlx prepared build | 34s | 9m |
| sqlx build | 35s | 10m 19s |
| diesel build | 50.6s | overtime |
| postgres connector | 3.5s | 3.5s | 

