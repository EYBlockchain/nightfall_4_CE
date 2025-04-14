# Plonk circuits generation/verification benchmark

## Desktop

- Processor: Apple M1 Pro 10 cores
- Memory: 16 GB
- OS: macOS Monterey Version 12.6.1
- rustc 1.68.0 (2c8cc3432 2023-03-06)
- `RAYON_NUM_THREADS=10 cargo bench`
- `RAYON_NUM_THREADS=10 cargo bench --bench bench_unified_circuit`

### Benchmark Metrics

- Constraint count before padding and after padding
- Witness generation
- Proving time
- Verification time
- Srs generation time
