## Build 

```bash
cargo build --release
```

## Running the performance test

Make sure you have DuckDB installed ([instruction](https://duckdb.org/docs/installation/))

```bash
./run_perf_test.sh | tee -a perf_test_history.md
```

Check the result in `perf_test_history.md` file.

## Performance Test Results

Please check `perf_tests` folder.