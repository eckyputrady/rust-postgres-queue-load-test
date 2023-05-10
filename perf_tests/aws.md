# Performance Tests
## Environment
- Worker machine: EC2 t2.small (1 CPU, 2 GBs RAM).
- DB machine: Aurora Postgres db.r5.large (2 CPUs, 16 GBs, 1000 connections max).
## 2023-05-12T04:40:51+00:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://postgres:pg-queue-test@pg-queue-test.cluster-czpzzsgh7mvn.ap-southeast-1.rds.amazonaws.com:5432/postgres
database_max_connections: 100
test_duration_ms: 30000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 17
num_producer_per_topic: 3
```
### Result
```
┌──────────────────────────┬────────┬────────┬────────┬────────────────────┐
│        event_type        │  p25   │  p50   │  p75   │        p95         │
│         varchar          │ double │ double │ double │       double       │
├──────────────────────────┼────────┼────────┼────────┼────────────────────┤
│ TASK_POLL_LATENCY        │   17.0 │   25.0 │   39.0 │               81.0 │
│ TASK_PUSH_LATENCY        │   12.0 │   17.0 │   24.0 │               41.0 │
│ TASK_DELETE_LATENCY      │   12.0 │   17.0 │   24.0 │               42.0 │
│ SLEEP                    │   31.0 │   49.0 │   55.0 │              61.55 │
│ TASK_POLLED              │  677.0 │  756.0 │  836.5 │              917.0 │
│ TASK_PUSHED              │  648.0 │  780.0 │  872.0 │              910.0 │
│ TASK_DELETED             │ 690.75 │  763.0 │ 829.25 │             924.55 │
│ PUSH_TO_POLL_LEAD_TIME   │   74.0 │  152.0 │  270.0 │ 502.59999999999854 │
│ PUSH_TO_DELETE_LEAD_TIME │   93.0 │  172.0 │  291.0 │              526.0 │
└──────────────────────────┴────────┴────────┴────────┴────────────────────┘
```
## 2023-05-12T04:41:35+00:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://postgres:pg-queue-test@pg-queue-test.cluster-czpzzsgh7mvn.ap-southeast-1.rds.amazonaws.com:5432/postgres
database_max_connections: 100
test_duration_ms: 30000
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
│ TASK_POLL_LATENCY        │   17.0 │   23.0 │   35.0 │              61.0 │
│ TASK_PUSH_LATENCY        │   14.0 │   18.0 │   23.0 │              37.0 │
│ TASK_DELETE_LATENCY      │   14.0 │   18.0 │   24.0 │              38.0 │
│ SLEEP                    │  36.25 │   50.5 │   60.0 │              62.0 │
│ TASK_DELETED             │ 683.75 │  771.0 │  814.0 │ 851.6499999999999 │
│ TASK_POLLED              │  702.0 │  769.5 │  820.0 │            855.95 │
│ TASK_PUSHED              │  684.0 │  748.5 │  804.0 │            859.05 │
│ PUSH_TO_POLL_LEAD_TIME   │   62.0 │  131.0 │  232.0 │             420.0 │
│ PUSH_TO_DELETE_LEAD_TIME │   81.0 │  150.0 │  255.0 │             444.0 │
└──────────────────────────┴────────┴────────┴────────┴───────────────────┘
```
## 2023-05-12T04:42:44+00:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://postgres:pg-queue-test@pg-queue-test.cluster-czpzzsgh7mvn.ap-southeast-1.rds.amazonaws.com:5432/postgres
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
│ TASK_POLL_LATENCY        │   19.0 │   27.0 │   41.0 │              77.0 │
│ TASK_PUSH_LATENCY        │   14.0 │   19.0 │   26.0 │              43.0 │
│ TASK_DELETE_LATENCY      │   14.0 │   19.0 │   26.0 │              43.0 │
│ SLEEP                    │   37.0 │   48.0 │   56.0 │ 62.04999999999998 │
│ TASK_DELETED             │ 651.75 │  713.0 │ 769.25 │            832.05 │
│ TASK_POLLED              │  650.5 │  714.5 │ 763.25 │             834.1 │
│ TASK_PUSHED              │  637.5 │  715.0 │  788.5 │ 852.0999999999999 │
│ PUSH_TO_POLL_LEAD_TIME   │   41.0 │  104.0 │  233.0 │             584.0 │
│ PUSH_TO_DELETE_LEAD_TIME │   62.0 │  126.0 │  256.0 │             611.0 │
└──────────────────────────┴────────┴────────┴────────┴───────────────────┘
```
## 2023-05-12T04:46:25+00:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://postgres:pg-queue-test@pg-queue-test.cluster-czpzzsgh7mvn.ap-southeast-1.rds.amazonaws.com:5432/postgres
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
│ TASK_POLL_LATENCY        │   19.0 │   28.0 │   43.0 │              73.0 │
│ TASK_PUSH_LATENCY        │   14.0 │   20.0 │   29.0 │              50.0 │
│ TASK_DELETE_LATENCY      │   14.0 │   20.0 │   30.0 │              50.0 │
│ SLEEP                    │   39.0 │   49.0 │   56.0 │              61.0 │
│ TASK_DELETED             │  530.5 │  697.5 │  759.0 │ 826.0500000000001 │
│ TASK_POLLED              │ 519.25 │  694.0 │  763.0 │            829.25 │
│ TASK_PUSHED              │ 519.75 │  704.0 │ 770.25 │ 839.3999999999999 │
│ PUSH_TO_POLL_LEAD_TIME   │   41.0 │  107.0 │  237.0 │             542.0 │
│ PUSH_TO_DELETE_LEAD_TIME │   66.0 │  130.0 │  260.0 │             569.0 │
└──────────────────────────┴────────┴────────┴────────┴───────────────────┘
```
## 2023-05-12T04:51:42+00:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://postgres:pg-queue-test@pg-queue-test.cluster-czpzzsgh7mvn.ap-southeast-1.rds.amazonaws.com:5432/postgres
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
│ TASK_POLL_LATENCY        │   18.0 │   26.0 │   40.0 │              77.0 │
│ TASK_PUSH_LATENCY        │   13.0 │   18.0 │   25.0 │              41.0 │
│ TASK_DELETE_LATENCY      │   13.0 │   18.0 │   26.0 │              42.0 │
│ SLEEP                    │   35.0 │   48.0 │   58.0 │              64.0 │
│ TASK_DELETED             │ 678.75 │  729.5 │  777.0 │             877.1 │
│ TASK_POLLED              │ 676.25 │  733.5 │  777.0 │ 868.3499999999999 │
│ TASK_PUSHED              │  665.0 │  736.0 │  799.0 │ 860.1999999999999 │
│ PUSH_TO_POLL_LEAD_TIME   │   51.0 │  134.0 │  273.0 │             570.0 │
│ PUSH_TO_DELETE_LEAD_TIME │   71.0 │  155.0 │  296.0 │             597.0 │
└──────────────────────────┴────────┴────────┴────────┴───────────────────┘
```
## 2023-05-12T05:39:01+00:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://postgres:pg-queue-test@pg-queue-test.cluster-czpzzsgh7mvn.ap-southeast-1.rds.amazonaws.com:5432/postgres
database_max_connections: 100
test_duration_ms: 180000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 17
num_producer_per_topic: 3
```
## 2023-05-12T05:40:36+00:00
### Config
```
num_topic: 50
message_size_bytes: 1000
database_url: postgres://postgres:pg-queue-test@pg-queue-test.cluster-czpzzsgh7mvn.ap-southeast-1.rds.amazonaws.com:5432/postgres
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
│ TASK_POLL_LATENCY        │  154.0 │  174.0 │  204.0 │             285.0 │
│ TASK_PUSH_LATENCY        │  154.0 │  171.0 │  197.0 │             266.0 │
│ TASK_DELETE_LATENCY      │  154.0 │  170.0 │  197.0 │             275.0 │
│ SLEEP                    │  440.0 │  465.0 │  487.0 │             515.0 │
│ TASK_DELETED             │  769.0 │  845.0 │  902.0 │             969.4 │
│ TASK_POLLED              │  763.0 │  843.0 │  906.0 │ 984.6999999999999 │
│ TASK_PUSHED              │  761.0 │  834.0 │  901.5 │             979.0 │
│ PUSH_TO_POLL_LEAD_TIME   │   43.0 │   67.0 │  102.0 │             197.0 │
│ PUSH_TO_DELETE_LEAD_TIME │  209.0 │  244.0 │  293.0 │             423.0 │
└──────────────────────────┴────────┴────────┴────────┴───────────────────┘
```
## 2023-05-12T05:45:01+00:00
### Config
```
num_topic: 50
message_size_bytes: 1000
database_url: postgres://postgres:pg-queue-test@pg-queue-test.cluster-czpzzsgh7mvn.ap-southeast-1.rds.amazonaws.com:5432/postgres
database_max_connections: 100
test_duration_ms: 1800000
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
│ TASK_POLL_LATENCY        │  160.0 │  181.0 │  209.0 │  281.0 │
│ TASK_PUSH_LATENCY        │  162.0 │  182.0 │  207.0 │  272.0 │
│ TASK_DELETE_LATENCY      │  162.0 │  182.0 │  207.0 │  272.0 │
│ SLEEP                    │  439.0 │  465.0 │  486.0 │  515.0 │
│ TASK_DELETED             │  727.0 │  802.0 │  856.0 │ 939.05 │
│ TASK_POLLED              │  724.0 │  799.0 │  858.0 │ 939.05 │
│ TASK_PUSHED              │  725.0 │  798.0 │  858.0 │ 944.05 │
│ PUSH_TO_POLL_LEAD_TIME   │   42.0 │   68.0 │  101.0 │  179.0 │
│ PUSH_TO_DELETE_LEAD_TIME │  220.0 │  254.0 │  302.0 │  407.0 │
└──────────────────────────┴────────┴────────┴────────┴────────┘
```
## 2023-05-12T06:22:12+00:00
### Config
```
num_topic: 50
message_size_bytes: 1000
database_url: postgres://postgres:pg-queue-test@pg-queue-test.cluster-czpzzsgh7mvn.ap-southeast-1.rds.amazonaws.com:5432/postgres
database_max_connections: 1000
test_duration_ms: 1800000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 17
num_producer_per_topic: 3
```
### Result
```
┌──────────────────────────┬────────┬────────┬────────┬────────────────────┐
│        event_type        │  p25   │  p50   │  p75   │        p95         │
│         varchar          │ double │ double │ double │       double       │
├──────────────────────────┼────────┼────────┼────────┼────────────────────┤
│ TASK_POLL_LATENCY        │   74.0 │  142.0 │  207.0 │              364.0 │
│ TASK_PUSH_LATENCY        │  139.0 │  174.0 │  225.0 │              371.0 │
│ TASK_DELETE_LATENCY      │  130.0 │  166.0 │  216.0 │              367.0 │
│ SLEEP                    │  489.0 │  515.0 │  538.0 │              558.0 │
│ TASK_PUSHED              │  669.5 │  771.0 │  873.0 │ 1008.0999999999999 │
│ TASK_POLLED              │  660.0 │  773.0 │  878.0 │ 1009.2999999999997 │
│ TASK_DELETED             │  666.0 │  772.0 │  875.0 │             1010.0 │
│ PUSH_TO_POLL_LEAD_TIME   │  152.0 │  198.0 │  278.0 │              437.0 │
│ PUSH_TO_DELETE_LEAD_TIME │  310.0 │  380.0 │  486.0 │              690.0 │
└──────────────────────────┴────────┴────────┴────────┴────────────────────┘
```
## 2023-05-12T07:05:01+00:00
### Config
```
num_topic: 5
message_size_bytes: 1000
database_url: postgres://postgres:pg-queue-test@pg-queue-test.cluster-czpzzsgh7mvn.ap-southeast-1.rds.amazonaws.com:5432/postgres
database_max_connections: 100
test_duration_ms: 1800000
consumer_sleep_time_ms: 1000
num_consumer_per_topic: 17
num_producer_per_topic: 3
```
### Result
```
┌──────────────────────────┬────────┬────────┬────────┬────────────────────┐
│        event_type        │  p25   │  p50   │  p75   │        p95         │
│         varchar          │ double │ double │ double │       double       │
├──────────────────────────┼────────┼────────┼────────┼────────────────────┤
│ TASK_POLL_LATENCY        │   19.0 │   23.0 │   30.0 │               55.0 │
│ TASK_PUSH_LATENCY        │   18.0 │   21.0 │   26.0 │               38.0 │
│ TASK_DELETE_LATENCY      │   17.0 │   21.0 │   25.0 │               37.0 │
│ SLEEP                    │   47.0 │   54.0 │   58.0 │ 60.049999999999955 │
│ TASK_DELETED             │  613.0 │  664.0 │  699.0 │              745.0 │
│ TASK_POLLED              │  612.0 │  663.0 │  699.0 │              743.0 │
│ TASK_PUSHED              │  610.0 │  664.0 │  700.0 │              744.0 │
│ PUSH_TO_POLL_LEAD_TIME   │   24.0 │   42.0 │  108.0 │              412.0 │
│ PUSH_TO_DELETE_LEAD_TIME │   45.0 │   64.0 │  131.0 │              437.0 │
└──────────────────────────┴────────┴────────┴────────┴────────────────────┘
```
