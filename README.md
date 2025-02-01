# Timer

defines 3 timer implementations to benchmark their tradeoffs.

implementations include:

- `SliceVectorTimer` which schedules actions in `[Vec<T>; M]`
- `VectorVectorTimer` which schedules actions in `Vec<Vec<T>>`
- `SliceSmallvecTimer` which schedules actions in `[SmallVec<T, N>; M]`

where `SmallVec<T, N>` is from the [`smallvec`](https://docs.rs/smallvec) crate, a vector-like
interface to a bounded vector which sits on the stack to remove heap reads/writes.

## Usage

run the following benchmark from a terminal, close all unnecessary applications for consistency.

```bash
cargo bench
```

## Viewing Results

The default output of the above command will be at the following path:

`target/criterion/report/index.html`

though a copy of the report is also available [here](./criterion/report/index.html)

