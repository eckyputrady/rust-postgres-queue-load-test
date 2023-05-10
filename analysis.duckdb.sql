with base as (
	select column0 as worker_id, column1 as time, column2 as event_type, column3 as event_payload
	from read_csv_auto('run.log')
), latencies as (
	select
		event_type,
		quantile_cont(event_payload, 0.25) as p25,
		quantile_cont(event_payload, 0.50) as p50,
		quantile_cont(event_payload, 0.75) as p75,
		quantile_cont(event_payload, 0.95) as p95
	from base
	where event_type like '%_LATENCY'
	group by event_type
), throughputs_by_time as (
	select time/1000 as time_s, event_type, count(*) as event_payload
	from base
	where event_type in ('TASK_PUSHED', 'TASK_POLLED', 'TASK_DELETED', 'SLEEP')
	group by time/1000, event_type
	order by time_s, event_type
), throughputs as (
	select
		event_type,
		quantile_cont(event_payload, 0.25) as p25,
		quantile_cont(event_payload, 0.50) as p50,
		quantile_cont(event_payload, 0.75) as p75,
		quantile_cont(event_payload, 0.95) as p95
	from throughputs_by_time
	group by event_type
), lead_times as (
	select
		a.msg_id,
		b.time - a.time as push_to_poll_lead_time,
		c.time - a.time as push_to_delete_lead_time,
	from (
		select event_payload as msg_id, time
		from base
		where event_type = 'TASK_PUSHED'
	) a
	inner join (
		select event_payload as msg_id, time
		from base
		where event_type = 'TASK_POLLED'
	) b on (a.msg_id = b.msg_id)
	inner join (
		select event_payload as msg_id, time
		from base
		where event_type = 'TASK_DELETED'
	) c on (a.msg_id = c.msg_id)
), lead_times_agg as (
	select
		'PUSH_TO_POLL_LEAD_TIME' as event_type,
		quantile_cont(push_to_poll_lead_time, 0.25) as p25,
		quantile_cont(push_to_poll_lead_time, 0.50) as p50,
		quantile_cont(push_to_poll_lead_time, 0.75) as p75,
		quantile_cont(push_to_poll_lead_time, 0.95) as p95
	from lead_times
	union all
	select
		'PUSH_TO_DELETE_LEAD_TIME' as event_type,
		quantile_cont(push_to_delete_lead_time, 0.25) as p25,
		quantile_cont(push_to_delete_lead_time, 0.50) as p50,
		quantile_cont(push_to_delete_lead_time, 0.75) as p75,
		quantile_cont(push_to_delete_lead_time, 0.95) as p95
	from lead_times
), final_stats as (
	select * from latencies
	union all
	select * from throughputs
	union all
	select * from lead_times_agg
)
select * from final_stats