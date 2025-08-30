# hash-cpu-bench

Recently, while writing a registration system for my backend project, I decided to use [argon2](https://en.wikipedia.org/wiki/Argon2) to hash passwords. And when I ran test requests on my backend, I was a bit surprised to see that it took me 600ms to hash through argon2. Then my friend ran argon2 hashing on his PC and the identical hashing took 25ms. From this I realized that my processor was crap, but still I wanted to make a CLI benchmark tool based on hash functions.

## Supported Algorithms

| Algorithm      | Parameters               |
|----------------|--------------------------|
| Argon2idFast   | m=64MB, t=2, p=1        |
| Argon2idSecure | m=256MB, t=4, p=2       |
| Argon2i        | m=128MB, t=3, p=1       |
| Argon2d        | m=128MB, t=3, p=1       |
| Bcrypt         | cost=10                  |
| Bcrypt         | cost=12                  |
| Bcrypt         | cost=14                  |
---

## Already Done Benchmarks

| CPU         | Argon2idFast | Argon2idSecure | Argon2i | Argon2d | Bcrypt10 | Bcrypt12 | Bcrypt14 |
|-------------|--------------|----------------|---------|---------|----------|----------|----------|
| [Intel Xeon E5-2650v4 @ 2.20GHz](./benchmarks/Intel/Xeon/E5/2650v4.json) | 119ms        | 950ms         | 359ms   | 350ms   | 84ms    | 341ms    | 1398ms    |
---

## Building
> [!NOTE]
> You must have [Rust](https://rust-lang.org) installed on your computer.
```shell
git clone https://github.com/smokingplaya/hash-cpu-bench
cd hash-cpu-bench
cargo build --release
```

> [!TIP]
> The executable binary will be placed in ``./target/release/hash_cpu_bench``(.exe in Windows)

## Interactive Mode

You can use the CLI interactively to hash strings on-the-fly and see the results instantly:

```shell
hash_cpu_bench
```

### Features
* Select a hashing preset interactively:
```
Choose hashing preset (Argon2idFast, Argon2idSecure, Argon2i, Argon2d, Bcrypt10, Bcrypt12, Bcrypt14)
preset> Argon2idFast
```
* Hash any input by typing it:
```
> mypassword123
Argon2idFast: $argon2id$v=19$m=65536,t=2,p=1$... (659ms)
```
* Change the hashing preset at any time:
```
> preset Bcrypt12
Preset changed to Bcrypt12
```
* Exit the REPL:
```
> exit
```

## Benchmark
Run a full benchmark of all supported hash algorithms:

```shell
hash_cpu_bench --benchmark
```

By default, the benchmark will perform 15 iterations per algorithm and save results to ``./bench_<timestamp>.json``

### Options
* ``--alg <ALGORITHM>`` - Run benchmark only for the specified algorithm (e.g., ``Argon2idFast``, ``Argon2i``, ``Bcrypt10``).
```shell
hash_cpu_bench --benchmark --alg Argon2idFast
```
* ``--repeats <N>`` - Set the number of iterations per algorithm (default: ``15``).
```shell
hash_cpu_bench --benchmark --repeats 20
```

## Contribution
Contributions are welcome! You can help by:

* Adding new hashing algorithms or presets.
* Improving benchmark accuracy and reporting.
* Enhancing the CLI user experience.
* Fixing bugs or improving documentation.

To contribute, fork the repository, make your changes, and submit a pull request. Please make sure your code follows the existing style.