const path = require('path');
const venvNodePath = path.join(__dirname, '..', 'benchmarks', 'venv_node', 'node_modules');
const idsmith = require(path.join(venvNodePath, 'idsmith'));
const ibantools = require(path.join(venvNodePath, 'ibantools'));
const cardValidator = require(path.join(venvNodePath, 'card-validator'));
const { performance } = require('perf_hooks');

const ITERATIONS = 100000;

function benchType(name, idsmithFunc, otherFunc, testValue) {
    console.log(`\n--- Benchmarking ${name} ---`);

    // Accuracy check
    let isIdsmithValid;
    try {
        isIdsmithValid = idsmithFunc(testValue);
    } catch (e) {
        isIdsmithValid = `Error: ${e.message}`;
    }

    let isOtherValid;
    try {
        isOtherValid = otherFunc(testValue);
    } catch (e) {
        isOtherValid = `Error: ${e.message}`;
    }

    console.log(`Accuracy Check: idsmith=${isIdsmithValid}, other=${isOtherValid}`);
    if (isIdsmithValid !== isOtherValid) {
        console.warn(`WARNING: Discrepancy for ${name} with value ${testValue}`);
    }

    // idsmith bench
    let start = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
        idsmithFunc(testValue);
    }
    let end = performance.now();
    const idsmithOps = ITERATIONS / ((end - start) / 1000);
    console.log(`idsmith: ${idsmithOps.toFixed(2)} ops/sec`);

    // other bench
    start = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
        otherFunc(testValue);
    }
    end = performance.now();
    const otherOps = ITERATIONS / ((end - start) / 1000);
    console.log(`other:   ${otherOps.toFixed(2)} ops/sec`);
    console.log(`Speedup: ${(idsmithOps / otherOps).toFixed(2)}x`);
}

function runBenchmarks() {
    // IBAN
    const testIban = "DE47508562162522867909";
    benchType("IBAN (DE)", 
        (v) => idsmith.validateIban(v),
        (v) => ibantools.isValidIBAN(v),
        testIban
    );

    // Credit Card
    const testCc = "4152839405126374";
    benchType("Credit Card (Visa)",
        (v) => idsmith.CreditCard.validate(v),
        (v) => cardValidator.number(v).isValid,
        testCc
    );

    // US SSN
    const testSsn = "446-72-2445";
    benchType("US SSN",
        (v) => idsmith.PersonalId.validate("US", v),
        (v) => true, // Just a placeholder
        testSsn
    );
}

runBenchmarks();
