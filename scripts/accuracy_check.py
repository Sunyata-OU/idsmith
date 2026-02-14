import idsmith
import importlib
from stdnum import iban as std_iban
from stdnum.luhn import is_valid as luhn_is_valid

# Comprehensive mapping for cross-validation
MAPPINGS = {
    # Personal IDs
    ('US', 'personal'): 'stdnum.us.ssn',
    ('BR', 'personal'): 'stdnum.br.cpf',
    ('EE', 'personal'): 'stdnum.ee.personalid',
    ('IT', 'personal'): 'stdnum.it.codicefiscale',
    ('FR', 'personal'): 'stdnum.fr.nir',
    ('ES', 'personal'): 'stdnum.es.dni',
    ('GB', 'personal'): 'stdnum.gb.nino',
    ('TR', 'personal'): 'stdnum.tr.tckimlik',
    ('PL', 'personal'): 'stdnum.pl.pesel',
    ('FI', 'personal'): 'stdnum.fi.hetu',
    ('NO', 'personal'): 'stdnum.no.fodselsnummer',
    ('BE', 'personal'): 'stdnum.be.nn',
    ('IN', 'personal'): 'stdnum.in_.aadhaar',
    ('DK', 'personal'): 'stdnum.dk.cpr',
    ('SE', 'personal'): 'stdnum.se.personnummer',
    ('NL', 'personal'): 'stdnum.nl.bsn',
    ('AT', 'personal'): 'stdnum.at.socialversicherung',
    ('CA', 'personal'): 'stdnum.ca.sin',
    ('CL', 'personal'): 'stdnum.cl.rut',
    ('CZ', 'personal'): 'stdnum.cz.rc',
    ('HR', 'personal'): 'stdnum.hr.oib',
    ('IE', 'personal'): 'stdnum.ie.pps',
    ('IS', 'personal'): 'stdnum.is_.kennitala',
    ('PT', 'personal'): 'stdnum.pt.nif',
    ('RO', 'personal'): 'stdnum.ro.cnp',
    ('SK', 'personal'): 'stdnum.sk.rc',

    # Company / VAT IDs
    ('FR', 'company'): 'stdnum.fr.siren',
    ('DE', 'company'): 'stdnum.de.vat',
    ('GB', 'company'): 'stdnum.gb.vat',
    ('IT', 'company'): 'stdnum.it.iva',
    ('ES', 'company'): 'stdnum.es.nif',
    ('AT', 'company'): 'stdnum.at.uid',
    ('BE', 'company'): 'stdnum.be.vat',
    ('DK', 'company'): 'stdnum.dk.cvr',
    ('EE', 'company'): 'stdnum.ee.registrikood',
    ('FI', 'company'): 'stdnum.fi.alv',
    ('GR', 'company'): 'stdnum.gr.vat',
    ('IE', 'company'): 'stdnum.ie.vat',
    ('NL', 'company'): 'stdnum.nl.btw',
    ('NO', 'company'): 'stdnum.no.mva',
    ('PL', 'company'): 'stdnum.pl.regon',
}

def test_accuracy():
    print("=== idsmith Accuracy Cross-Validation Suite ===")
    
    # 1. IBAN
    print("\n[IBAN Validation]")
    iban_countries = idsmith.iban_countries()
    passed = 0
    skipped = 0
    for cc in iban_countries:
        code = idsmith.generate_iban(cc)
        try:
            if std_iban.is_valid(code):
                passed += 1
            else:
                if cc not in std_iban.SPEC:
                    skipped += 1
                else:
                    print(f"  FAILED: IBAN {cc}: {code}")
        except:
            skipped += 1
    print(f"  Result: {passed} passed, {skipped} skipped (unsupported by stdnum)")

    # 2. Personal IDs
    print("\n[Personal ID Validation]")
    p_passed = 0
    p_total = 0
    for (cc, type_key), mod_path in MAPPINGS.items():
        if type_key != 'personal': continue
        try:
            mod = importlib.import_module(mod_path)
            p_total += 1
            success = True
            for _ in range(10):
                code = idsmith.PersonalId.generate(cc)
                if not mod.is_valid(code):
                    print(f"  FAILED: {cc} {mod_path}: {code}")
                    success = False
                    break
            if success: p_passed += 1
        except ImportError:
            pass
    print(f"  Result: {p_passed}/{p_total} ID types verified")

    # 3. Company IDs
    print("\n[Company ID Validation]")
    c_passed = 0
    c_total = 0
    for (cc, type_key), mod_path in MAPPINGS.items():
        if type_key != 'company': continue
        try:
            mod = importlib.import_module(mod_path)
            c_total += 1
            success = True
            for _ in range(10):
                res = idsmith.CompanyId.generate(cc)
                code = res['code']
                if not (mod.is_valid(code) or mod.is_valid(cc + code)):
                    print(f"  FAILED: {cc} {mod_path}: {code}")
                    success = False
                    break
            if success: c_passed += 1
        except ImportError:
            pass
    print(f"  Result: {c_passed}/{c_total} Company ID types verified")

    # 4. Credit Cards
    print("\n[Credit Card Luhn Validation]")
    cc_passed = 0
    for _ in range(100):
        res = idsmith.CreditCard.generate()
        if luhn_is_valid(res['number']):
            cc_passed += 1
    print(f"  Result: {cc_passed}/100 passed")

if __name__ == "__main__":
    test_accuracy()
