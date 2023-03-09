# Benchmark

Count restarts: 10 times

Table shows avg time

| count tables| 100 | 1000 |
|---|---|---|
| rust-postgres | 3.5s | 3.5s |
| sqlx offline (preparation)| 48s | 4m |
| sqlx offline | 34s | 9m |
| sqlx | 35s | 10m 19s |
| diesel | 50.6s | overtime |
| seaorm | 31s | 27m 43s |

