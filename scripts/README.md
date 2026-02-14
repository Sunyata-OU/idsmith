# Scripts

This directory contains utility scripts for verifying the accuracy and performance of the `idsmith` library across different ecosystems.

## Scripts

### `accuracy_check.py`
Cross-validates `idsmith` generated identifiers (IBAN, Personal ID, Company ID, Credit Card) against the `python-stdnum` library to ensure 100% algorithmic correctness.

**To run:**
```bash
../benchmarks/venv_python/bin/python accuracy_check.py
```

### `comprehensive_bench.py`
Measures the performance (ops/sec) of `idsmith` compared to `python-stdnum` in a Python environment.

**To run:**
```bash
../benchmarks/venv_python/bin/python comprehensive_bench.py
```

### `comprehensive_bench.js`
Measures the performance (ops/sec) of `idsmith` compared to `ibantools` and `card-validator` in a Node.js environment.

**To run:**
```bash
cd ../benchmarks/venv_node && node ../../scripts/comprehensive_bench.js
```
