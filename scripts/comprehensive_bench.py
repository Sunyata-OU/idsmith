import time
import idsmith
from stdnum import iban, ee, br, it, fr, gb, de
from stdnum.us import ssn
from stdnum.luhn import is_valid as luhn_is_valid

ITERATIONS = 50_000

def bench_type(name, idsmith_func, stdnum_func, test_value):
    print(f"\n--- Benchmarking {name} ---")
    
    # Accuracy check
    try:
        is_idsmith_valid = idsmith_func(test_value)
    except Exception as e:
        is_idsmith_valid = f"Error: {e}"
        
    try:
        is_stdnum_valid = stdnum_func(test_value)
    except Exception as e:
        is_stdnum_valid = f"Error: {e}"

    print(f"Accuracy Check: idsmith={is_idsmith_valid}, stdnum={is_stdnum_valid}")
    if is_idsmith_valid != is_stdnum_valid:
        print(f"WARNING: Discrepancy for {name} with value {test_value}")

    # idsmith bench
    start = time.time()
    for _ in range(ITERATIONS):
        idsmith_func(test_value)
    end = time.time()
    idsmith_ops = ITERATIONS / (end - start)
    print(f"idsmith: {idsmith_ops:.2f} ops/sec")

    # stdnum bench
    start = time.time()
    for _ in range(ITERATIONS):
        stdnum_func(test_value)
    end = time.time()
    stdnum_ops = ITERATIONS / (end - start)
    print(f"stdnum:  {stdnum_ops:.2f} ops/sec")
    print(f"Speedup: {idsmith_ops / stdnum_ops:.2f}x")

def run_benchmarks():
    # IBAN
    test_iban = "DE47508562162522867909"
    bench_type("IBAN (DE)", 
               lambda v: idsmith.validate_iban(v), 
               lambda v: iban.is_valid(v), 
               test_iban)

    # Credit Card (Luhn)
    test_cc = "4152839405126374"
    bench_type("Credit Card (Luhn)", 
               lambda v: idsmith.CreditCard.validate(v), 
               lambda v: luhn_is_valid(v), 
               test_cc)

    # Personal ID - US SSN
    test_ssn = "446-72-2445"
    bench_type("US SSN", 
               lambda v: idsmith.PersonalId.validate("US", v), 
               lambda v: ssn.is_valid(v), 
               test_ssn)

    # Personal ID - EE (Estonia)
    test_ee = "37605030299"
    bench_type("Estonia Personal ID", 
               lambda v: idsmith.PersonalId.validate("EE", v), 
               lambda v: ee.personalid.is_valid(v), 
               test_ee)

    # Company ID - FR SIREN
    test_siren = "732829320"
    bench_type("France SIREN", 
               lambda v: idsmith.CompanyId.validate("FR", v), 
               lambda v: fr.siren.is_valid(v), 
               test_siren)

    # Tax ID - IN PAN (No stdnum comparison, just idsmith)
    print("\n--- Benchmarking Tax ID (IN PAN) ---")
    start = time.time()
    for _ in range(ITERATIONS):
        idsmith.TaxId.validate("IN", "ABCDE1234F")
    end = time.time()
    print(f"idsmith: {ITERATIONS / (end - start):.2f} ops/sec")

    # Driver License - US
    print("\n--- Benchmarking Driver License (US) ---")
    start = time.time()
    for _ in range(ITERATIONS):
        idsmith.DriverLicense.validate("US", "A123456789012")
    end = time.time()
    print(f"idsmith: {ITERATIONS / (end - start):.2f} ops/sec")

    # Passport - DE
    print("\n--- Benchmarking Passport (DE) ---")
    start = time.time()
    for _ in range(ITERATIONS):
        idsmith.Passport.validate("DE", "C01234567")
    end = time.time()
    print(f"idsmith: {ITERATIONS / (end - start):.2f} ops/sec")

if __name__ == "__main__":
    run_benchmarks()
