import idsmith
from stdnum import iban
from stdnum.ee import personalid as ee_personalid
from stdnum.br import cpf as br_cpf
from stdnum.it import codicefiscale as it_cf
from stdnum.fr import siren as fr_siren
from stdnum.us import ssn as us_ssn

COUNTRIES = ["DE", "FR", "EE", "IT", "ES", "GB"]

def test_accuracy():
    print("Starting Accuracy Cross-Validation (idsmith vs python-stdnum)...")
    
    # 1. IBAN
    print("\n[IBAN]")
    for country in COUNTRIES:
        valid_count = 0
        for _ in range(50):
            code = idsmith.generate_iban(country)
            if iban.is_valid(code):
                valid_count += 1
        print(f"  {country}: {valid_count}/50 passed")

    # 2. US SSN
    print("\n[US SSN]")
    valid_count = 0
    for _ in range(50):
        code = idsmith.PersonalId.generate("US")
        if us_ssn.is_valid(code):
            valid_count += 1
    print(f"  US: {valid_count}/50 passed")

    # 3. Estonia Personal ID
    print("\n[EE Personal ID]")
    valid_count = 0
    for _ in range(50):
        code = idsmith.PersonalId.generate("EE")
        if ee_personalid.is_valid(code):
            valid_count += 1
    print(f"  EE: {valid_count}/50 passed")

    # 4. Brazil CPF
    print("\n[BR CPF]")
    valid_count = 0
    for _ in range(50):
        code = idsmith.PersonalId.generate("BR")
        if br_cpf.is_valid(code):
            valid_count += 1
    print(f"  BR: {valid_count}/50 passed")

    # 5. France SIREN
    print("\n[FR SIREN]")
    valid_count = 0
    for _ in range(50):
        res = idsmith.CompanyId.generate("FR")
        code = res['code']
        if fr_siren.is_valid(code):
            valid_count += 1
    print(f"  FR: {valid_count}/50 passed")

    # 6. Italy Codice Fiscale
    print("\n[IT Codice Fiscale]")
    valid_count = 0
    for _ in range(50):
        code = idsmith.PersonalId.generate("IT")
        if it_cf.is_valid(code):
            valid_count += 1
    print(f"  IT: {valid_count}/50 passed")

if __name__ == "__main__":
    try:
        test_accuracy()
    except Exception as e:
        print(f"Error during accuracy test: {e}")
        import traceback
        traceback.print_exc()
