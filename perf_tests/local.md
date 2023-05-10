# Performance Tests
## Environment
Both application and DB is hosted in a single machine. 

- MemTotal: 30734064 kB
- Processor: 4CPU @ 1.3GHz
## 2023-05-12T08:57:36+08:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 100
test_duration_ms: 18000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 50
num_producer_per_topic: 5
```
### Result
```
┌──────────────────────────┬────────┬────────┬────────┬────────────────────┐
│        event_type        │  p25   │  p50   │  p75   │        p95         │
│         varchar          │ double │ double │ double │       double       │
├──────────────────────────┼────────┼────────┼────────┼────────────────────┤
│ TASK_POLL_LATENCY        │    8.0 │   13.0 │   20.0 │               34.0 │
│ TASK_PUSH_LATENCY        │    5.0 │    8.0 │   12.0 │               23.0 │
│ TASK_DELETE_LATENCY      │    7.0 │   11.0 │   19.0 │               30.0 │
│ SLEEP                    │  159.5 │  178.0 │  211.0 │ 215.84999999999997 │
│ TASK_DELETED             │ 2296.5 │ 2413.0 │ 2790.0 │ 3163.2999999999993 │
│ TASK_POLLED              │ 2281.5 │ 2416.0 │ 2775.0 │ 3125.1999999999994 │
│ TASK_PUSHED              │ 2274.0 │ 2507.0 │ 2705.5 │             3088.2 │
│ PUSH_TO_POLL_LEAD_TIME   │   67.0 │  153.0 │  280.0 │              777.0 │
│ PUSH_TO_DELETE_LEAD_TIME │   78.0 │  166.0 │  296.0 │              797.0 │
└──────────────────────────┴────────┴────────┴────────┴────────────────────┘
```
## 2023-05-12T08:59:44+08:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 100
test_duration_ms: 18000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 15
num_producer_per_topic: 5
```
### Result
```
┌──────────────────────────┬────────┬────────┬────────┬───────────────────┐
│        event_type        │  p25   │  p50   │  p75   │        p95        │
│         varchar          │ double │ double │ double │      double       │
├──────────────────────────┼────────┼────────┼────────┼───────────────────┤
│ TASK_PUSH_LATENCY        │    6.0 │    8.0 │   11.0 │              20.0 │
│ TASK_POLL_LATENCY        │    8.0 │   11.0 │   15.0 │              28.0 │
│ TASK_DELETE_LATENCY      │    6.0 │    9.0 │   13.0 │              24.0 │
│ SLEEP                    │   11.0 │   14.0 │   20.0 │ 31.29999999999999 │
│ TASK_DELETED             │ 2162.5 │ 2599.0 │ 2928.5 │            3376.6 │
│ TASK_POLLED              │ 2155.0 │ 2606.0 │ 2934.0 │            3363.0 │
│ TASK_PUSHED              │ 2143.0 │ 2606.0 │ 2803.0 │            3539.2 │
│ PUSH_TO_POLL_LEAD_TIME   │   75.0 │  141.0 │  231.0 │             392.0 │
│ PUSH_TO_DELETE_LEAD_TIME │   86.0 │  152.0 │  242.0 │             403.0 │
└──────────────────────────┴────────┴────────┴────────┴───────────────────┘
```
## 2023-05-12T09:00:49+08:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 100
test_duration_ms: 18000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 12
num_producer_per_topic: 8
```
### Result
```
┌──────────────────────────┬─────────┬────────┬─────────┬────────────────────┐
│        event_type        │   p25   │  p50   │   p75   │        p95         │
│         varchar          │ double  │ double │ double  │       double       │
├──────────────────────────┼─────────┼────────┼─────────┼────────────────────┤
│ TASK_PUSH_LATENCY        │     7.0 │    9.0 │    12.0 │               20.0 │
│ TASK_POLL_LATENCY        │     8.0 │   11.0 │    15.0 │               26.0 │
│ TASK_DELETE_LATENCY      │     7.0 │   10.0 │    14.0 │               23.0 │
│ SLEEP                    │    17.0 │   17.0 │    17.0 │               17.0 │
│ TASK_DELETED             │  2099.5 │ 2370.0 │  2695.0 │ 2959.7999999999997 │
│ TASK_POLLED              │  2104.5 │ 2365.0 │  2699.5 │ 2953.8999999999996 │
│ TASK_PUSHED              │  3549.0 │ 3898.0 │  4098.0 │  4436.199999999999 │
│ PUSH_TO_POLL_LEAD_TIME   │ 1410.25 │ 2957.0 │  4859.0 │             6777.0 │
│ PUSH_TO_DELETE_LEAD_TIME │ 1420.25 │ 2968.0 │ 4870.75 │             6790.0 │
└──────────────────────────┴─────────┴────────┴─────────┴────────────────────┘
```
## 2023-05-12T09:01:30+08:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 100
test_duration_ms: 18000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 13
num_producer_per_topic: 7
```
### Result
```
┌──────────────────────────┬─────────┬────────┬─────────┬────────────────────┐
│        event_type        │   p25   │  p50   │   p75   │        p95         │
│         varchar          │ double  │ double │ double  │       double       │
├──────────────────────────┼─────────┼────────┼─────────┼────────────────────┤
│ TASK_POLL_LATENCY        │    10.0 │   14.0 │    18.0 │               28.0 │
│ TASK_PUSH_LATENCY        │     8.0 │   12.0 │    16.0 │               24.0 │
│ TASK_DELETE_LATENCY      │     9.0 │   12.0 │    17.0 │               25.0 │
│ SLEEP                    │   12.75 │   14.5 │   16.25 │              17.65 │
│ TASK_PUSHED              │  2290.0 │ 2539.0 │  2879.0 │ 3935.1999999999985 │
│ TASK_DELETED             │ 2022.25 │ 2142.0 │ 2375.75 │  2721.899999999999 │
│ TASK_POLLED              │ 2032.25 │ 2140.0 │ 2383.25 │  2726.499999999999 │
│ PUSH_TO_POLL_LEAD_TIME   │  1347.0 │ 1847.0 │  2650.0 │             3467.0 │
│ PUSH_TO_DELETE_LEAD_TIME │ 1361.75 │ 1861.0 │  2665.0 │             3481.0 │
└──────────────────────────┴─────────┴────────┴─────────┴────────────────────┘
```
## 2023-05-12T09:06:10+08:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 100
test_duration_ms: 18000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 14
num_producer_per_topic: 6
```
### Result
```
┌──────────────────────────┬────────┬────────┬────────┬────────────────────┐
│        event_type        │  p25   │  p50   │  p75   │        p95         │
│         varchar          │ double │ double │ double │       double       │
├──────────────────────────┼────────┼────────┼────────┼────────────────────┤
│ TASK_POLL_LATENCY        │    9.0 │   12.0 │   18.0 │               32.0 │
│ TASK_PUSH_LATENCY        │    7.0 │    9.0 │   13.0 │               24.0 │
│ TASK_DELETE_LATENCY      │    7.0 │   10.0 │   16.0 │               28.0 │
│ SLEEP                    │   34.0 │   34.0 │   34.0 │               34.0 │
│ TASK_DELETED             │ 1659.5 │ 2519.0 │ 2944.5 │             3414.6 │
│ TASK_POLLED              │ 1656.0 │ 2527.0 │ 2947.0 │ 3418.2999999999997 │
│ TASK_PUSHED              │ 1925.5 │ 2685.0 │ 3104.0 │ 3450.4999999999995 │
│ PUSH_TO_POLL_LEAD_TIME   │  462.0 │  578.0 │  884.0 │             1761.0 │
│ PUSH_TO_DELETE_LEAD_TIME │  473.0 │  588.0 │  899.0 │             1780.0 │
└──────────────────────────┴────────┴────────┴────────┴────────────────────┘
```
## 2023-05-12T09:13:34+08:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 100
test_duration_ms: 180000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 14
num_producer_per_topic: 6
```
### Result
```
┌──────────────────────────┬────────┬─────────┬─────────┬────────────────────┐
│        event_type        │  p25   │   p50   │   p75   │        p95         │
│         varchar          │ double │ double  │ double  │       double       │
├──────────────────────────┼────────┼─────────┼─────────┼────────────────────┤
│ TASK_POLL_LATENCY        │   11.0 │    16.0 │    24.0 │               43.0 │
│ TASK_PUSH_LATENCY        │    8.0 │    12.0 │    16.0 │               29.0 │
│ TASK_DELETE_LATENCY      │    9.0 │    14.0 │    21.0 │               38.0 │
│ SLEEP                    │    2.5 │     4.0 │    13.5 │               21.1 │
│ TASK_DELETED             │ 1655.0 │  1817.0 │  2017.0 │             2841.0 │
│ TASK_POLLED              │ 1652.0 │  1825.0 │  2014.0 │             2855.0 │
│ TASK_PUSHED              │ 1998.0 │  2154.0 │  2338.0 │             2876.0 │
│ PUSH_TO_POLL_LEAD_TIME   │ 6046.0 │ 10647.0 │ 17568.0 │ 22280.849999999977 │
│ PUSH_TO_DELETE_LEAD_TIME │ 6065.0 │ 10666.0 │ 17586.0 │            22299.0 │
└──────────────────────────┴────────┴─────────┴─────────┴────────────────────┘
```
## 2023-05-12T09:17:03+08:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 100
test_duration_ms: 180000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 15
num_producer_per_topic: 5
```
### Result
```
┌──────────────────────────┬────────┬─────────┬─────────┬──────────┐
│        event_type        │  p25   │   p50   │   p75   │   p95    │
│         varchar          │ double │ double  │ double  │  double  │
├──────────────────────────┼────────┼─────────┼─────────┼──────────┤
│ TASK_POLL_LATENCY        │    9.0 │    12.0 │    16.0 │     53.0 │
│ TASK_PUSH_LATENCY        │    6.0 │     9.0 │    13.0 │     24.0 │
│ TASK_DELETE_LATENCY      │    8.0 │    10.0 │    14.0 │     26.0 │
│ SLEEP                    │    7.0 │    13.0 │    16.0 │     33.0 │
│ TASK_DELETED             │  819.0 │  2312.0 │  3255.0 │   3447.4 │
│ TASK_POLLED              │  816.0 │  2282.5 │ 3253.25 │   3431.5 │
│ TASK_PUSHED              │ 1952.0 │  2318.0 │  2440.0 │   3152.0 │
│ PUSH_TO_POLL_LEAD_TIME   │  655.0 │ 10236.0 │ 32207.0 │  53477.0 │
│ PUSH_TO_DELETE_LEAD_TIME │  673.0 │ 10250.5 │ 32218.0 │ 53487.25 │
└──────────────────────────┴────────┴─────────┴─────────┴──────────┘
```
## 2023-05-12T09:21:15+08:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 100
test_duration_ms: 180000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 17
num_producer_per_topic: 3
```
### Result
```
┌──────────────────────────┬────────┬────────┬────────┬───────────────────┐
│        event_type        │  p25   │  p50   │  p75   │        p95        │
│         varchar          │ double │ double │ double │      double       │
├──────────────────────────┼────────┼────────┼────────┼───────────────────┤
│ TASK_POLL_LATENCY        │    6.0 │   10.0 │   15.0 │              35.0 │
│ TASK_PUSH_LATENCY        │    5.0 │    7.0 │   10.0 │              21.0 │
│ TASK_DELETE_LATENCY      │    5.0 │    8.0 │   13.0 │              25.0 │
│ SLEEP                    │  35.75 │   47.0 │   56.0 │ 62.04999999999998 │
│ TASK_DELETED             │ 1347.0 │ 1675.0 │ 1960.0 │            2716.0 │
│ TASK_POLLED              │ 1354.0 │ 1674.0 │ 1947.0 │            2709.0 │
│ TASK_PUSHED              │ 1359.0 │ 1636.0 │ 1968.0 │            2717.0 │
│ PUSH_TO_POLL_LEAD_TIME   │   52.0 │  128.0 │  242.0 │             508.0 │
│ PUSH_TO_DELETE_LEAD_TIME │   62.0 │  138.0 │  253.0 │             521.0 │
└──────────────────────────┴────────┴────────┴────────┴───────────────────┘
```
## 2023-05-12T09:24:30+08:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 100
test_duration_ms: 180000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 16
num_producer_per_topic: 4
```
### Result
```
┌──────────────────────────┬────────┬────────┬────────┬────────┐
│        event_type        │  p25   │  p50   │  p75   │  p95   │
│         varchar          │ double │ double │ double │ double │
├──────────────────────────┼────────┼────────┼────────┼────────┤
│ TASK_PUSH_LATENCY        │    6.0 │    9.0 │   13.0 │   24.0 │
│ TASK_POLL_LATENCY        │    8.0 │   12.0 │   19.0 │   35.0 │
│ TASK_DELETE_LATENCY      │    6.0 │   10.0 │   15.0 │   29.0 │
│ SLEEP                    │  18.75 │   29.5 │   38.0 │   46.0 │
│ TASK_DELETED             │ 1593.0 │ 1765.0 │ 2047.0 │ 2992.0 │
│ TASK_POLLED              │ 1594.0 │ 1764.0 │ 2049.0 │ 2978.0 │
│ TASK_PUSHED              │ 1536.0 │ 1751.0 │ 2077.0 │ 2959.0 │
│ PUSH_TO_POLL_LEAD_TIME   │   59.0 │  122.0 │  215.0 │  397.0 │
│ PUSH_TO_DELETE_LEAD_TIME │   71.0 │  135.0 │  228.0 │  411.0 │
└──────────────────────────┴────────┴────────┴────────┴────────┘
```
## 2023-05-12T09:28:05+08:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 100
test_duration_ms: 300000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 16
num_producer_per_topic: 4
```
### Result
```
┌──────────────────────────┬────────┬────────┬─────────┬───────────────────┐
│        event_type        │  p25   │  p50   │   p75   │        p95        │
│         varchar          │ double │ double │ double  │      double       │
├──────────────────────────┼────────┼────────┼─────────┼───────────────────┤
│ TASK_POLL_LATENCY        │    9.0 │   12.0 │    19.0 │              75.0 │
│ TASK_PUSH_LATENCY        │    6.0 │    9.0 │    13.0 │              24.0 │
│ TASK_DELETE_LATENCY      │    8.0 │   10.0 │    14.0 │              30.0 │
│ SLEEP                    │   21.0 │   29.0 │    36.0 │ 42.14999999999999 │
│ TASK_DELETED             │ 1045.0 │ 1704.0 │  2674.0 │            3777.0 │
│ TASK_POLLED              │ 1037.0 │ 1703.0 │  2675.0 │            3777.0 │
│ TASK_PUSHED              │ 1456.0 │ 1881.0 │  2028.0 │            2622.0 │
│ PUSH_TO_POLL_LEAD_TIME   │  260.0 │ 3841.0 │ 17040.0 │ 47577.39999999991 │
│ PUSH_TO_DELETE_LEAD_TIME │  272.0 │ 3856.0 │ 17056.0 │ 47592.39999999991 │
└──────────────────────────┴────────┴────────┴─────────┴───────────────────┘
```
## 2023-05-12T09:33:56+08:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 100
test_duration_ms: 300000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 17
num_producer_per_topic: 3
```
### Result
```
┌──────────────────────────┬────────┬────────┬────────┬────────┐
│        event_type        │  p25   │  p50   │  p75   │  p95   │
│         varchar          │ double │ double │ double │ double │
├──────────────────────────┼────────┼────────┼────────┼────────┤
│ TASK_POLL_LATENCY        │    7.0 │   10.0 │   15.0 │   35.0 │
│ TASK_PUSH_LATENCY        │    5.0 │    7.0 │   10.0 │   19.0 │
│ TASK_DELETE_LATENCY      │    6.0 │    8.0 │   12.0 │   24.0 │
│ SLEEP                    │   34.0 │   46.0 │   56.0 │  61.25 │
│ TASK_DELETED             │ 1505.0 │ 1648.0 │ 1955.0 │ 2518.0 │
│ TASK_POLLED              │ 1507.0 │ 1641.0 │ 1956.0 │ 2520.0 │
│ TASK_PUSHED              │ 1439.0 │ 1648.0 │ 1984.0 │ 2411.0 │
│ PUSH_TO_POLL_LEAD_TIME   │   52.0 │  127.0 │  245.0 │  773.0 │
│ PUSH_TO_DELETE_LEAD_TIME │   62.0 │  137.0 │  256.0 │  786.0 │
└──────────────────────────┴────────┴────────┴────────┴────────┘
```
## 2023-05-12T16:55:31+08:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 100
test_duration_ms: 300000
consumer_sleep_time_ms: 10000
num_consumer_per_topic: 17
num_producer_per_topic: 3
```
### Result
```
┌──────────────────────────┬────────┬────────┬────────┬────────┐
│        event_type        │  p25   │  p50   │  p75   │  p95   │
│         varchar          │ double │ double │ double │ double │
├──────────────────────────┼────────┼────────┼────────┼────────┤
│ TASK_POLL_LATENCY        │    7.0 │    9.0 │   14.0 │   27.0 │
│ TASK_PUSH_LATENCY        │    5.0 │    7.0 │   10.0 │   18.0 │
│ TASK_DELETE_LATENCY      │    6.0 │    8.0 │   12.0 │   23.0 │
│ SLEEP                    │   11.0 │   12.0 │   13.0 │   24.0 │
│ TASK_DELETED             │ 1552.0 │ 1712.0 │ 1929.0 │ 2473.0 │
│ TASK_POLLED              │ 1545.0 │ 1708.0 │ 1930.0 │ 2473.0 │
│ TASK_PUSHED              │ 1413.0 │ 1717.0 │ 2030.0 │ 2866.0 │
│ PUSH_TO_POLL_LEAD_TIME   │  480.0 │ 1086.0 │ 2111.0 │ 4795.0 │
│ PUSH_TO_DELETE_LEAD_TIME │  489.0 │ 1096.0 │ 2122.0 │ 4807.0 │
└──────────────────────────┴────────┴────────┴────────┴────────┘
```
## 2023-05-12T17:07:34+08:00
### Config
```
num_topic: 50
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 1000
test_duration_ms: 300000
consumer_sleep_time_ms: 10000
num_consumer_per_topic: 17
num_producer_per_topic: 3
```
### Result
```
```
## 2023-05-12T17:12:52+08:00
### Config
```
num_topic: 50
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 1000
test_duration_ms: 300000
consumer_sleep_time_ms: 10000
num_consumer_per_topic: 17
num_producer_per_topic: 3
```
### Result
```
┌──────────────────────────┬─────────┬────────┬────────┬───────────────────┐
│        event_type        │   p25   │  p50   │  p75   │        p95        │
│         varchar          │ double  │ double │ double │      double       │
├──────────────────────────┼─────────┼────────┼────────┼───────────────────┤
│ TASK_POLL_LATENCY        │    59.0 │  134.0 │  266.0 │             646.0 │
│ TASK_PUSH_LATENCY        │    12.0 │   23.0 │   56.0 │             196.0 │
│ TASK_DELETE_LATENCY      │    13.0 │   31.0 │   71.0 │             201.0 │
│ SLEEP                    │     8.0 │   20.5 │   38.5 │ 126.2999999999999 │
│ TASK_DELETED             │ 2021.25 │ 2273.5 │ 2584.5 │ 3168.399999999999 │
│ TASK_POLLED              │  2032.0 │ 2275.0 │ 2547.5 │            3178.2 │
│ TASK_PUSHED              │  1677.0 │ 2339.0 │ 3151.0 │            4396.0 │
│ PUSH_TO_POLL_LEAD_TIME   │  1559.0 │ 3130.0 │ 7132.0 │           20146.0 │
│ PUSH_TO_DELETE_LEAD_TIME │  1613.0 │ 3181.0 │ 7183.0 │           20231.0 │
└──────────────────────────┴─────────┴────────┴────────┴───────────────────┘
```
## 2023-05-12T17:19:14+08:00
### Config
```
num_topic: 50
message_size_bytes: 1000
database_url: postgres://ecky:ecky@localhost:5432/ecky
database_max_connections: 1000
test_duration_ms: 300000
consumer_sleep_time_ms: 30000
num_consumer_per_topic: 17
num_producer_per_topic: 3
```
### Result
```
┌──────────────────────────┬─────────┬─────────┬──────────┬────────────────────┐
│        event_type        │   p25   │   p50   │   p75    │        p95         │
│         varchar          │ double  │ double  │  double  │       double       │
├──────────────────────────┼─────────┼─────────┼──────────┼────────────────────┤
│ TASK_POLL_LATENCY        │    91.0 │   220.0 │    433.0 │             1165.0 │
│ TASK_PUSH_LATENCY        │     9.0 │    16.0 │     35.0 │              166.0 │
│ TASK_DELETE_LATENCY      │    13.0 │    28.0 │     63.0 │              192.0 │
│ SLEEP                    │   167.5 │   328.0 │    420.5 │ 494.49999999999994 │
│ TASK_PUSHED              │ 1542.25 │  2112.0 │  2918.75 │ 11407.050000000001 │
│ TASK_DELETED             │   739.5 │  2225.5 │  2708.25 │ 3272.3000000000006 │
│ TASK_POLLED              │  728.75 │  2224.0 │  2696.25 │             3246.0 │
│ PUSH_TO_POLL_LEAD_TIME   │ 58047.0 │ 94376.0 │ 108536.0 │ 118050.79999999993 │
│ PUSH_TO_DELETE_LEAD_TIME │ 58126.0 │ 94432.0 │ 108585.0 │           118084.0 │
└──────────────────────────┴─────────┴─────────┴──────────┴────────────────────┘
```
