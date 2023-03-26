# Benchmark

Count restarts: 10 times
CPU: Intel i7-10850H

Table shows avg time

| count tables| 100 | 1000 |
|---|---|---|
| rust-postgres | 2.4s | 2.9s |
| sqlx offline (preparation)| 1.4s | 45.4s |
| sqlx offline | 40.2s | 11m 14s |
| sqlx | 43.5s | 12m |
| diesel | 26.3s | overtime |
| seaorm | 21.8s | 1m 3s |

