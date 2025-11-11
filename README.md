# Rust Collections Benchmarks 🦀

Comprehensive performance benchmarks comparing Rust's standard collections and concurrent data structures.

## 📊 What's Benchmarked

### Collections
- **Vec** - Dynamic arrays
- **VecDeque** - Double-ended queues
- **HashMap** - Hash tables
- **HashSet** - Hash sets
- **BTreeMap** - Sorted maps
- **DashMap** - Concurrent hash map

### Scenarios
- Simple vs pre-allocated initialization
- Sequential vs parallel operations
- Different synchronization primitives (Mutex, RwLock)
- Insert operations (push vs insert)

## 🚀 Quick Start

### Prerequisites
```bash
# Rust 1.70 or later
rustup update
```

### Run Benchmarks
```bash
# Run all benchmarks
cargo bench

# Run specific group
cargo bench vec
cargo bench hashmap
cargo bench rayon

# Run specific benchmark
cargo bench vec_simple_push
```

### View Results
After running, open the HTML report:
```bash
open target/criterion/report/index.html
```

## 📈 Key Findings

### 1. Pre-allocation Matters
Pre-allocating collections provides **37-60% performance improvements**:
- `Vec::with_capacity()` → 37% faster
- `HashMap::with_capacity()` → 60% faster
- `HashSet::with_capacity()` → 59% faster

### 2. Choose the Right Collection
| Operation | Best Choice | Time (9000 items) |
|-----------|-------------|-------------------|
| Push to end | `Vec::push()` | 19.4 µs |
| Push to front | `VecDeque::push_front()` | 37.4 µs |
| Key-value lookup | `HashMap` | 444.6 µs |
| Sorted keys | `BTreeMap` | 991.3 µs |
| Parallel writes | `DashMap` | 433.4 µs |

### 3. Concurrent Performance
For parallel operations with 9000 insertions:
- **DashMap** (lock-free): 433 µs ⚡
- **RwLock reads**: 2,351 µs 📖
- **RwLock writes**: 4,699 µs ✍️
- **Mutex**: 4,808 µs 🔒

**Takeaway:** DashMap is **11x faster** than Mutex for parallel writes!

### 4. VecDeque Usage
- `push_front()` / `push_back()` → Fast (O(1))
- `insert(index, value)` → Slow (O(n)) - **avoid!**

## 🔧 Configuration

Benchmarks use:
- **Sample size**: 100 iterations per benchmark
- **Test size**: 9,000 operations per iteration
- **Measurement time**: ~5 seconds per benchmark

Modify `ITER` constant in `benches/benching.rs` to change test size:
```rust
const ITER: usize = 9000;  // Change this value
```

## 📁 Project Structure
```
benching/
├── src/
│   └── lib.rs              # Collection implementations
├── benches/
│   └── benching.rs         # Benchmark definitions
├── Cargo.toml
└── README.md
```

## 🛠️ Dependencies
```toml
[dependencies]
rayon = "1.10"      # Parallel iterators
dashmap = "6.1"     # Concurrent HashMap

[dev-dependencies]
criterion = "0.5"   # Benchmarking framework
```

## 📚 Learn More

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Rust Collections Documentation](https://doc.rust-lang.org/std/collections/)
- [Rayon Parallel Iterators](https://docs.rs/rayon/)
- [DashMap Documentation](https://docs.rs/dashmap/)

## 💡 Recommendations

### When to Pre-allocate
✅ **Always pre-allocate when:**
- You know the final size upfront
- Building from an iterator with known length
- Performance is critical

❌ **Don't pre-allocate when:**
- Size is completely unknown
- Collection will be very small (<10 items)

### Choosing HashMap vs BTreeMap
**Use HashMap when:**
- You need fast lookups (O(1))
- Order doesn't matter
- Most common use case

**Use BTreeMap when:**
- You need sorted keys
- You need range queries
- You need ordered iteration

### Parallel Collections
**Use DashMap when:**
- High write concurrency
- Many threads inserting/updating
- Lock-free performance matters

**Use RwLock<HashMap> when:**
- Read-heavy workload (many readers, few writers)
- Occasional writes with many reads
- Don't want external dependency

**Use Mutex<HashMap> when:**
- Simplicity over performance
- Low contention expected
- Reads and writes are balanced

## 🤝 Contributing

Feel free to:
- Add more collection benchmarks
- Test different scenarios
- Optimize existing implementations
- Report interesting findings

## 📄 License

MIT

## 🙏 Acknowledgments

Built with [Criterion.rs](https://github.com/bheisler/criterion.rs) - the best benchmarking framework for Rust!