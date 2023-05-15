copy (
    with base as (
        select column0 as time, column1 as table, column2 as event, column3 as latency
        from read_csv_auto('run.log')
    ), by_10_secs as (
        select floor(time / 10000) as time, "table" || '_' || left(event, 3) as event, latency
        from base
    ), grouped as (
        select time, event, quantile_cont(latency, 0.95) as latency_p95, count(*) as count
        from by_10_secs
        group by time, event
    ), stats as (
        select *
        from (
            select time, latency_p95 as short_url_GET_latency_p95, count as short_url_GET_count from grouped
            where event = 'short_url_GET'
        ) a  left join (
            select time, latency_p95 as short_url_partitioned_GET_latency_p95, count as short_url_partitioned_GET_count from grouped
            where event = 'short_url_partitioned_GET'
        ) d using(time) left join (
            select time, latency_p95 as short_url_ADD_latency_p95, count as short_url_ADD_count from grouped
            where event = 'short_url_ADD'
        ) b using (time) left join (
            select time, latency_p95 as short_url_partitioned_ADD_latency_p95, count as short_url_partitioned_ADD_count from grouped
            where event = 'short_url_partitioned_ADD'
        ) c using (time)
    )
    select * from stats
) to 'stats.tsv' (header, delimiter '\t')