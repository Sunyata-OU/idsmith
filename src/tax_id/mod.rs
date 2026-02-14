use rand::Rng;
#[cfg(feature = "json")]
use serde::Serialize;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct TaxIdResult {
    pub country_code: String,
    pub country_name: String,
    pub name: String,
    pub code: String,
    pub holder_type: Option<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GenOptions {
    pub country: Option<String>,
    pub holder_type: Option<String>,
}

/// Valid PAN holder-type characters.
static PAN_HOLDER_TYPES: &[char] = &['P', 'C', 'H', 'F', 'A', 'T', 'B', 'L', 'J', 'G'];

/// Spanish NIF check letter table (mod-23).
static ES_NIF_LETTERS: &[u8] = b"TRWAGMYFPDXBNJZSQVHLCKE";

/// USCI valid characters for CN (0-9, A-H, J-N, P-R, T-W, X-Y).
static CN_USCI_CHARS: &[u8] = b"0123456789ABCDEFGHJKLMNPQRTUWXY";

/// CN USCI weights for positions 1-17.
static CN_USCI_WEIGHTS: &[u32] = &[
    1, 3, 9, 27, 19, 26, 16, 17, 20, 29, 25, 13, 8, 24, 10, 30, 28,
];

/// MX RFC alphanumeric mapping for check digit.
static MX_RFC_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMN&OPQRSTUVWXYZ ";

static SPECIFIC_COUNTRIES: &[(&str, &str)] = &[
    ("AE", "TRN"),
    ("AR", "CUIL"),
    ("AT", "Steuernummer"),
    ("AU", "TFN"),
    ("BD", "TIN"),
    ("BE", "NN"),
    ("BG", "EGN"),
    ("BH", "CPR"),
    ("BR", "CPF"),
    ("CA", "SIN"),
    ("CH", "AHV"),
    ("CL", "RUT"),
    ("CN", "USCI"),
    ("CO", "NIT"),
    ("CZ", "Rodné číslo"),
    ("DE", "Steuer-IdNr"),
    ("DK", "CPR"),
    ("DZ", "NIF"),
    ("EC", "RUC"),
    ("EE", "Isikukood"),
    ("EG", "National ID"),
    ("ES", "NIF"),
    ("ET", "TIN"),
    ("FI", "HETU"),
    ("FR", "NIF"),
    ("GB", "UTR"),
    ("GH", "TIN"),
    ("GR", "AFM"),
    ("HK", "HKID"),
    ("HR", "OIB"),
    ("HU", "Adóazonosító"),
    ("ID", "NPWP"),
    ("IE", "PPS"),
    ("IL", "Mispar Zehut"),
    ("IN", "PAN"),
    ("IS", "Kennitala"),
    ("IT", "Partita IVA"),
    ("JP", "My Number"),
    ("KE", "KRA PIN"),
    ("KR", "BRN"),
    ("KW", "Civil ID"),
    ("LK", "NIC"),
    ("LT", "Asmens kodas"),
    ("LU", "Matricule"),
    ("LV", "Personas kods"),
    ("MA", "CIN"),
    ("MT", "TIN"),
    ("MX", "RFC"),
    ("MY", "MyKad"),
    ("NG", "TIN"),
    ("NL", "BSN"),
    ("NO", "Fødselsnummer"),
    ("NP", "PAN"),
    ("NZ", "IRD"),
    ("OM", "Tax ID"),
    ("PE", "RUC"),
    ("PH", "TIN"),
    ("PK", "NTN"),
    ("PL", "PESEL"),
    ("PT", "NIF"),
    ("QA", "QID"),
    ("RO", "CNP"),
    ("RS", "JMBG"),
    ("SA", "National ID"),
    ("SE", "Personnummer"),
    ("SG", "Tax Ref"),
    ("SI", "Davčna"),
    ("SK", "Rodné číslo"),
    ("TH", "Tax ID"),
    ("TN", "CIN"),
    ("TR", "TC Kimlik"),
    ("TW", "National ID"),
    ("TZ", "TIN"),
    ("UA", "IPN"),
    ("US", "TIN"),
    ("UY", "RUT"),
    ("VE", "RIF"),
    ("VN", "MST"),
    ("ZA", "Tax Number"),
];

pub struct Registry;

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, opts: &GenOptions, rng: &mut impl Rng) -> Option<TaxIdResult> {
        let country = opts
            .country
            .as_deref()
            .unwrap_or_else(|| {
                let countries = self.list_countries();
                countries[rng.gen_range(0..countries.len())].0
            })
            .to_uppercase();

        if let Some((name, code, holder)) = match country.as_str() {
            "AE" => Some(("TRN", self.generate_ae(rng), None)),
            "AR" => Some(("CUIL", self.generate_ar(rng), None)),
            "AT" => Some(("Steuernummer", self.generate_at(rng), None)),
            "AU" => Some(("TFN", self.generate_au(rng), None)),
            "BD" => Some(("TIN", self.generate_bd(rng), None)),
            "BE" => Some(("NN", self.generate_be(rng), None)),
            "BG" => Some(("EGN", self.generate_bg(rng), None)),
            "BH" => Some(("CPR", self.generate_bh(rng), None)),
            "BR" => Some(("CPF", self.generate_br(rng), None)),
            "CA" => Some(("SIN", self.generate_ca(rng), None)),
            "CH" => Some(("AHV", self.generate_ch(rng), None)),
            "CL" => Some(("RUT", self.generate_cl(rng), None)),
            "CN" => Some(("USCI", self.generate_cn(rng), None)),
            "CO" => Some(("NIT", self.generate_co(rng), None)),
            "CZ" => Some(("Rodné číslo", self.generate_cz(rng), None)),
            "DE" => Some(("Steuer-IdNr", self.generate_de(rng), None)),
            "DK" => Some(("CPR", self.generate_dk(rng), None)),
            "DZ" => Some(("NIF", self.generate_dz(rng), None)),
            "EC" => Some(("RUC", self.generate_ec(rng), None)),
            "EE" => Some(("Isikukood", self.generate_ee(rng), None)),
            "EG" => Some(("National ID", self.generate_eg(rng), None)),
            "ES" => Some(("NIF", self.generate_es(rng), None)),
            "ET" => Some(("TIN", self.generate_et(rng), None)),
            "FI" => Some(("HETU", self.generate_fi(rng), None)),
            "FR" => Some(("NIF", self.generate_fr(rng), None)),
            "GB" => Some(("UTR", self.generate_gb(rng), None)),
            "GH" => Some(("TIN", self.generate_gh(rng), None)),
            "GR" => Some(("AFM", self.generate_gr(rng), None)),
            "HK" => Some(("HKID", self.generate_hk(rng), None)),
            "HR" => Some(("OIB", self.generate_hr(rng), None)),
            "HU" => Some(("Adóazonosító", self.generate_hu(rng), None)),
            "ID" => Some(("NPWP", self.generate_id(rng), None)),
            "IE" => Some(("PPS", self.generate_ie(rng), None)),
            "IL" => Some(("Mispar Zehut", self.generate_il(rng), None)),
            "IN" => {
                let (code, ht) = self.generate_in(opts, rng);
                Some(("PAN", code, Some(ht)))
            }
            "IS" => Some(("Kennitala", self.generate_is(rng), None)),
            "IT" => Some(("Partita IVA", self.generate_it(rng), None)),
            "JP" => Some(("My Number", self.generate_jp(rng), None)),
            "KE" => Some(("KRA PIN", self.generate_ke(rng), None)),
            "KR" => Some(("BRN", self.generate_kr(rng), None)),
            "KW" => Some(("Civil ID", self.generate_kw(rng), None)),
            "LK" => Some(("NIC", self.generate_lk(rng), None)),
            "LT" => Some(("Asmens kodas", self.generate_lt(rng), None)),
            "LU" => Some(("Matricule", self.generate_lu(rng), None)),
            "LV" => Some(("Personas kods", self.generate_lv(rng), None)),
            "MA" => Some(("CIN", self.generate_ma(rng), None)),
            "MT" => Some(("TIN", self.generate_mt(rng), None)),
            "MX" => Some(("RFC", self.generate_mx(rng), None)),
            "MY" => Some(("MyKad", self.generate_my(rng), None)),
            "NG" => Some(("TIN", self.generate_ng(rng), None)),
            "NL" => Some(("BSN", self.generate_nl(rng), None)),
            "NO" => Some(("Fødselsnummer", self.generate_no(rng), None)),
            "NP" => Some(("PAN", self.generate_np(rng), None)),
            "NZ" => Some(("IRD", self.generate_nz(rng), None)),
            "OM" => Some(("Tax ID", self.generate_om(rng), None)),
            "PE" => Some(("RUC", self.generate_pe(rng), None)),
            "PH" => Some(("TIN", self.generate_ph(rng), None)),
            "PK" => Some(("NTN", self.generate_pk(rng), None)),
            "PL" => Some(("PESEL", self.generate_pl(rng), None)),
            "PT" => Some(("NIF", self.generate_pt(rng), None)),
            "QA" => Some(("QID", self.generate_qa(rng), None)),
            "RO" => Some(("CNP", self.generate_ro(rng), None)),
            "RS" => Some(("JMBG", self.generate_rs(rng), None)),
            "SA" => Some(("National ID", self.generate_sa(rng), None)),
            "SE" => Some(("Personnummer", self.generate_se(rng), None)),
            "SG" => Some(("Tax Ref", self.generate_sg(rng), None)),
            "SI" => Some(("Davčna", self.generate_si(rng), None)),
            "SK" => Some(("Rodné číslo", self.generate_sk(rng), None)),
            "TH" => Some(("Tax ID", self.generate_th(rng), None)),
            "TN" => Some(("CIN", self.generate_tn(rng), None)),
            "TR" => Some(("TC Kimlik", self.generate_tr(rng), None)),
            "TW" => Some(("National ID", self.generate_tw(rng), None)),
            "TZ" => Some(("TIN", self.generate_tz(rng), None)),
            "UA" => Some(("IPN", self.generate_ua(rng), None)),
            "US" => Some(("TIN", self.generate_us(rng), None)),
            "UY" => Some(("RUT", self.generate_uy(rng), None)),
            "VE" => Some(("RIF", self.generate_ve(rng), None)),
            "VN" => Some(("MST", self.generate_vn(rng), None)),
            "ZA" => Some(("Tax Number", self.generate_za(rng), None)),
            _ => None,
        } {
            let country_name = crate::countries::get_country_name(&country).unwrap_or("Unknown");
            return Some(TaxIdResult {
                country_code: country,
                country_name: country_name.to_string(),
                name: name.to_string(),
                code,
                holder_type: holder,
                valid: true,
            });
        }

        None
    }

    pub fn validate(&self, country: &str, code: &str) -> bool {
        match country.to_uppercase().as_str() {
            "AE" => self.validate_ae(code),
            "AR" => self.validate_ar(code),
            "AT" => self.validate_at(code),
            "AU" => self.validate_au(code),
            "BD" => self.validate_bd(code),
            "BE" => self.validate_be(code),
            "BG" => self.validate_bg(code),
            "BH" => self.validate_bh(code),
            "BR" => self.validate_br(code),
            "CA" => self.validate_ca(code),
            "CH" => self.validate_ch(code),
            "CL" => self.validate_cl(code),
            "CN" => self.validate_cn(code),
            "CO" => self.validate_co(code),
            "CZ" => self.validate_cz(code),
            "DE" => self.validate_de(code),
            "DK" => self.validate_dk(code),
            "DZ" => self.validate_dz(code),
            "EC" => self.validate_ec(code),
            "EE" => self.validate_ee(code),
            "EG" => self.validate_eg(code),
            "ES" => self.validate_es(code),
            "ET" => self.validate_et(code),
            "FI" => self.validate_fi(code),
            "FR" => self.validate_fr(code),
            "GB" => self.validate_gb(code),
            "GH" => self.validate_gh(code),
            "GR" => self.validate_gr(code),
            "HK" => self.validate_hk(code),
            "HR" => self.validate_hr(code),
            "HU" => self.validate_hu(code),
            "ID" => self.validate_id(code),
            "IE" => self.validate_ie(code),
            "IL" => self.validate_il(code),
            "IN" => self.validate_in(code),
            "IS" => self.validate_is(code),
            "IT" => self.validate_it(code),
            "JP" => self.validate_jp(code),
            "KE" => self.validate_ke(code),
            "KR" => self.validate_kr(code),
            "KW" => self.validate_kw(code),
            "LK" => self.validate_lk(code),
            "LT" => self.validate_lt(code),
            "LU" => self.validate_lu(code),
            "LV" => self.validate_lv(code),
            "MA" => self.validate_ma(code),
            "MT" => self.validate_mt(code),
            "MX" => self.validate_mx(code),
            "MY" => self.validate_my(code),
            "NG" => self.validate_ng(code),
            "NL" => self.validate_nl(code),
            "NO" => self.validate_no(code),
            "NP" => self.validate_np(code),
            "NZ" => self.validate_nz(code),
            "OM" => self.validate_om(code),
            "PE" => self.validate_pe(code),
            "PH" => self.validate_ph(code),
            "PK" => self.validate_pk(code),
            "PL" => self.validate_pl(code),
            "PT" => self.validate_pt(code),
            "QA" => self.validate_qa(code),
            "RO" => self.validate_ro(code),
            "RS" => self.validate_rs(code),
            "SA" => self.validate_sa(code),
            "SE" => self.validate_se(code),
            "SG" => self.validate_sg(code),
            "SI" => self.validate_si(code),
            "SK" => self.validate_sk(code),
            "TH" => self.validate_th(code),
            "TN" => self.validate_tn(code),
            "TR" => self.validate_tr(code),
            "TW" => self.validate_tw(code),
            "TZ" => self.validate_tz(code),
            "UA" => self.validate_ua(code),
            "US" => self.validate_us(code),
            "UY" => self.validate_uy(code),
            "VE" => self.validate_ve(code),
            "VN" => self.validate_vn(code),
            "ZA" => self.validate_za(code),
            _ => false,
        }
    }

    pub fn list_countries(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        let mut result: Vec<(&str, &str, &str)> = SPECIFIC_COUNTRIES
            .iter()
            .map(|(code, name)| {
                (
                    *code,
                    crate::countries::get_country_name(code).unwrap_or("Unknown"),
                    *name,
                )
            })
            .collect();
        result.sort_by_key(|(code, _, _)| *code);
        result
    }

    // ── AE TRN (Tax Registration Number) ──
    // Format: 15 digits
    fn generate_ae(&self, rng: &mut impl Rng) -> String {
        (0..15)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_ae(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 15 && !c.starts_with('0')
    }

    // ── AR CUIL/CUIT ──
    // Format: 11 digits, weights [5,4,3,2,7,6,5,4,3,2] mod 11 (stdnum.ar.cuit)
    // Check = '012345678990'[11 - (sum % 11)]
    fn generate_ar(&self, rng: &mut impl Rng) -> String {
        let lookup: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 0];
        let prefix = [20u8, 23, 24, 27, 30, 33, 34][rng.gen_range(0..7)];
        let body: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9u8)).collect();
        let weights = [5u32, 4, 3, 2, 7, 6, 5, 4, 3, 2];
        let mut digits = vec![prefix / 10, prefix % 10];
        digits.extend_from_slice(&body);
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let check = lookup[(11 - (sum % 11)) as usize];
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }
    fn validate_ar(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 11 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let lookup: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 0];
        let weights = [5u32, 4, 3, 2, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[..10]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let expected = lookup[(11 - (sum % 11)) as usize];
        digits[10] == expected
    }

    // ── AT Steuernummer ──
    // Format: 9 digits, Luhn-variant check digit (stdnum.at.tin)
    fn generate_at(&self, rng: &mut impl Rng) -> String {
        let luhn_map: [u8; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
        let digits: Vec<u8> = (0..8)
            .map(|i| {
                if i == 0 {
                    rng.gen_range(1..=9u8)
                } else {
                    rng.gen_range(0..=9u8)
                }
            })
            .collect();
        let s: u32 = digits
            .iter()
            .enumerate()
            .map(|(i, &d)| {
                if i % 2 != 0 {
                    luhn_map[d as usize] as u32
                } else {
                    d as u32
                }
            })
            .sum();
        let check = ((10 - (s % 10)) % 10) as u8;
        let mut result: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        result.push((b'0' + check) as char);
        result
    }
    fn validate_at(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 9 || c.starts_with('0') {
            return false;
        }
        let luhn_map: [u8; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let s: u32 = digits[..8]
            .iter()
            .enumerate()
            .map(|(i, &d)| {
                if i % 2 != 0 {
                    luhn_map[d as usize] as u32
                } else {
                    d as u32
                }
            })
            .sum();
        let check = ((10 - (s % 10)) % 10) as u8;
        digits[8] == check
    }

    // ── BD TIN ──
    // Format: 12 digits
    fn generate_bd(&self, rng: &mut impl Rng) -> String {
        (0..12)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_bd(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 12 && !c.starts_with('0')
    }

    // ── BE NN (National Number) ──
    // Format: 11 digits, mod-97 check (for 2000+ births, prepend "2")
    fn generate_be(&self, rng: &mut impl Rng) -> String {
        let yy: u8 = rng.gen_range(0..=99);
        let mm: u8 = rng.gen_range(1..=12);
        let dd: u8 = rng.gen_range(1..=28);
        let seq: u16 = rng.gen_range(1..=997);
        let base_str = format!("{:02}{:02}{:02}{:03}", yy, mm, dd, seq);
        let base_num: u64 = base_str.parse().unwrap();
        // Try born after 2000 first
        let check_2k = 97 - ((2_000_000_000u64 + base_num) % 97);
        let check_19 = 97 - (base_num % 97);
        // Pick one randomly
        let check = if rng.gen_bool(0.5) {
            check_2k
        } else {
            check_19
        };
        format!("{}{:02}", base_str, check)
    }
    fn validate_be(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 11 {
            return false;
        }
        let base: u64 = c[..9].parse().unwrap_or(0);
        let check: u64 = c[9..].parse().unwrap_or(0);
        let r19 = 97 - (base % 97);
        let r2k = 97 - ((2_000_000_000u64 + base) % 97);
        check == r19 || check == r2k
    }

    // ── BG EGN ──
    // Format: 10 digits, weights [2,4,8,5,10,9,7,3,6] mod 11
    fn generate_bg(&self, rng: &mut impl Rng) -> String {
        let yy: u8 = rng.gen_range(0..=99);
        let mm: u8 = rng.gen_range(1..=12);
        let dd: u8 = rng.gen_range(1..=28);
        let region: u16 = rng.gen_range(0..=999);
        let base = format!("{:02}{:02}{:02}{:03}", yy, mm, dd, region);
        let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
        let weights = [2u32, 4, 8, 5, 10, 9, 7, 3, 6];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        let check = if r == 10 { 0u8 } else { r as u8 };
        format!("{}{}", base, check)
    }
    fn validate_bg(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 10 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let weights = [2u32, 4, 8, 5, 10, 9, 7, 3, 6];
        let sum: u32 = digits[..9]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        let expected = if r == 10 { 0u8 } else { r as u8 };
        digits[9] == expected
    }

    // ── BH CPR ──
    // Format: 9 digits
    fn generate_bh(&self, rng: &mut impl Rng) -> String {
        (0..9)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_bh(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 9 && !c.starts_with('0')
    }

    // ── CH AHV ──
    // Format: 13 digits, prefix "756", EAN-13 checksum
    fn generate_ch(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = vec![7, 5, 6];
        for _ in 0..9 {
            digits.push(rng.gen_range(0..=9u8));
        }
        let sum: u32 = digits
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * if i % 2 == 0 { 1 } else { 3 })
            .sum();
        let check = ((10 - (sum % 10)) % 10) as u8;
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }
    fn validate_ch(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 13 || !c.starts_with("756") {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let sum: u32 = digits[..12]
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * if i % 2 == 0 { 1 } else { 3 })
            .sum();
        let expected = ((10 - (sum % 10)) % 10) as u8;
        digits[12] == expected
    }

    // ── CL RUT ──
    // Format: 8-9 body digits + check digit (0-9 or K), cycling weights mod 11
    fn generate_cl(&self, rng: &mut impl Rng) -> String {
        let num: u32 = rng.gen_range(1_000_000..=99_999_999);
        let s = format!("{}", num);
        let digits: Vec<u8> = s.bytes().map(|b| b - b'0').collect();
        let mut sum: u32 = 0;
        let mut w = 2u32;
        for &d in digits.iter().rev() {
            sum += d as u32 * w;
            w += 1;
            if w > 7 {
                w = 2;
            }
        }
        let r = 11 - (sum % 11);
        let check = if r == 11 {
            '0'
        } else if r == 10 {
            'K'
        } else {
            (b'0' + r as u8) as char
        };
        format!("{}{}", s, check)
    }
    fn validate_cl(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() < 8 || clean.len() > 10 {
            return false;
        }
        let body = &clean[..clean.len() - 1];
        let check_char = clean.chars().last().unwrap();
        if !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        let mut sum: u32 = 0;
        let mut w = 2u32;
        for &d in digits.iter().rev() {
            sum += d as u32 * w;
            w += 1;
            if w > 7 {
                w = 2;
            }
        }
        let r = 11 - (sum % 11);
        let expected = if r == 11 {
            '0'
        } else if r == 10 {
            'K'
        } else {
            (b'0' + r as u8) as char
        };
        check_char == expected
    }

    // ── CO NIT ──
    // Format: 10 digits, weights [41,37,29,23,19,17,13,7,3] mod 11
    fn generate_co(&self, rng: &mut impl Rng) -> String {
        let body: Vec<u8> = (0..9)
            .map(|i| {
                if i == 0 {
                    rng.gen_range(1..=9u8)
                } else {
                    rng.gen_range(0..=9u8)
                }
            })
            .collect();
        let weights = [41u32, 37, 29, 23, 19, 17, 13, 7, 3];
        let sum: u32 = body
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        let check = if r == 0 {
            0u8
        } else if r == 1 {
            1
        } else {
            (11 - r) as u8
        };
        let mut digits = body;
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }
    fn validate_co(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 10 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let weights = [41u32, 37, 29, 23, 19, 17, 13, 7, 3];
        let sum: u32 = digits[..9]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        let expected = if r == 0 {
            0u8
        } else if r == 1 {
            1
        } else {
            (11 - r) as u8
        };
        digits[9] == expected
    }

    // ── CZ Rodné číslo ──
    // Format: 10 digits, divisible by 11
    fn generate_cz(&self, rng: &mut impl Rng) -> String {
        let yy: u8 = rng.gen_range(0..=99);
        let mm: u8 = rng.gen_range(1..=12);
        let dd: u8 = rng.gen_range(1..=28);
        loop {
            let seq: u16 = rng.gen_range(0..=999);
            let first9 = format!("{:02}{:02}{:02}{:03}", yy, mm, dd, seq);
            let n: u64 = first9.parse().unwrap();
            let check = (n % 11) % 10;
            let full = format!("{}{}", first9, check);
            if full.len() == 10 {
                return full;
            }
        }
    }
    fn validate_cz(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 10 {
            return false;
        }
        let first9: u64 = match c[..9].parse() {
            Ok(n) => n,
            Err(_) => return false,
        };
        let last: u64 = (c.as_bytes()[9] - b'0') as u64;
        (first9 % 11) % 10 == last
    }

    // ── DK CPR ──
    // Format: 10 digits (DDMMYYXXXX), no checksum since 2007
    fn generate_dk(&self, rng: &mut impl Rng) -> String {
        let dd: u8 = rng.gen_range(1..=28);
        let mm: u8 = rng.gen_range(1..=12);
        let yy: u8 = rng.gen_range(0..=99);
        let seq: u16 = rng.gen_range(1..=9999);
        format!("{:02}{:02}{:02}{:04}", dd, mm, yy, seq)
    }
    fn validate_dk(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 10
    }

    // ── DZ NIF ──
    // Format: 15 digits
    fn generate_dz(&self, rng: &mut impl Rng) -> String {
        (0..15)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_dz(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 15 && !c.starts_with('0')
    }

    // ── EC RUC ──
    // Format: 13 digits. Province (01-24,30,50) + type digit + body + check + establishment
    // Natural (d3 0-5): CI checksum (Luhn-like, weights 2,1 alternating, fold>9)
    // Public (d3=6): weights (3,2,7,6,5,4,3,2), mod 11, check at pos 9
    // Juridical (d3=9): weights (4,3,2,7,6,5,4,3,2), mod 11, check at pos 10
    fn generate_ec(&self, rng: &mut impl Rng) -> String {
        // Generate natural person RUC (most common, d3 0-5)
        let province: u8 = rng.gen_range(1..=24);
        let d3: u8 = rng.gen_range(0..=5);
        let body: Vec<u8> = (0..6).map(|_| rng.gen_range(0..=9u8)).collect();
        let mut first9: Vec<u8> = vec![province / 10, province % 10, d3];
        first9.extend_from_slice(&body);
        // CI checksum: weights (2,1,2,1,...), fold x-9 if x>9, sum%10, check=(10-sum)%10
        let sum: u32 = first9
            .iter()
            .enumerate()
            .map(|(i, &d)| {
                let w = if i % 2 == 0 { 2 } else { 1 };
                let v = d as u32 * w;
                if v > 9 {
                    v - 9
                } else {
                    v
                }
            })
            .sum();
        let check = ((10 - (sum % 10)) % 10) as u8;
        let mut result: String = first9.iter().map(|d| (b'0' + d) as char).collect();
        result.push((b'0' + check) as char);
        result.push_str("001");
        result
    }
    fn validate_ec(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 13 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let prov: u8 = digits[0] * 10 + digits[1];
        if !((1..=24).contains(&prov) || prov == 30 || prov == 50) {
            return false;
        }
        match digits[2] {
            0..=5 => {
                // Natural: CI checksum on first 10 digits
                let sum: u32 = digits[..9]
                    .iter()
                    .enumerate()
                    .map(|(i, &d)| {
                        let w = if i % 2 == 0 { 2 } else { 1 };
                        let v = d as u32 * w;
                        if v > 9 {
                            v - 9
                        } else {
                            v
                        }
                    })
                    .sum();
                let check = ((10 - (sum % 10)) % 10) as u8;
                digits[9] == check
            }
            6 => {
                // Public: weights (3,2,7,6,5,4,3,2) on digits 0-7, check at digit 8
                let weights: [u32; 8] = [3, 2, 7, 6, 5, 4, 3, 2];
                let sum: u32 = digits[..8]
                    .iter()
                    .zip(weights.iter())
                    .map(|(&d, &w)| d as u32 * w)
                    .sum();
                let r = sum % 11;
                let check = if r == 0 { 0u8 } else { (11 - r) as u8 };
                digits[8] == check
            }
            9 => {
                // Juridical: weights (4,3,2,7,6,5,4,3,2) on digits 0-8, check at digit 9
                let weights: [u32; 9] = [4, 3, 2, 7, 6, 5, 4, 3, 2];
                let sum: u32 = digits[..9]
                    .iter()
                    .zip(weights.iter())
                    .map(|(&d, &w)| d as u32 * w)
                    .sum();
                let r = sum % 11;
                let check = if r == 0 { 0u8 } else { (11 - r) as u8 };
                digits[9] == check
            }
            _ => false,
        }
    }

    // ── EE Isikukood ──
    // Format: 11 digits, two-pass weighted mod 11
    fn generate_ee(&self, rng: &mut impl Rng) -> String {
        let gender: u8 = rng.gen_range(1..=6);
        let yy: u8 = rng.gen_range(0..=99);
        let mm: u8 = rng.gen_range(1..=12);
        let dd: u8 = rng.gen_range(1..=28);
        let seq: u16 = rng.gen_range(0..=999);
        let base = format!("{}{:02}{:02}{:02}{:03}", gender, yy, mm, dd, seq);
        let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
        let w1 = [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 1];
        let sum1: u32 = digits
            .iter()
            .zip(w1.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r1 = sum1 % 11;
        let check = if r1 < 10 {
            r1 as u8
        } else {
            let w2 = [3u32, 4, 5, 6, 7, 8, 9, 1, 2, 3];
            let sum2: u32 = digits
                .iter()
                .zip(w2.iter())
                .map(|(&d, &w)| d as u32 * w)
                .sum();
            let r2 = sum2 % 11;
            if r2 < 10 {
                r2 as u8
            } else {
                0
            }
        };
        format!("{}{}", base, check)
    }
    fn validate_ee(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 11 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let w1 = [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 1];
        let sum1: u32 = digits[..10]
            .iter()
            .zip(w1.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r1 = sum1 % 11;
        let expected = if r1 < 10 {
            r1 as u8
        } else {
            let w2 = [3u32, 4, 5, 6, 7, 8, 9, 1, 2, 3];
            let sum2: u32 = digits[..10]
                .iter()
                .zip(w2.iter())
                .map(|(&d, &w)| d as u32 * w)
                .sum();
            let r2 = sum2 % 11;
            if r2 < 10 {
                r2 as u8
            } else {
                0
            }
        };
        digits[10] == expected
    }

    // ── EG National ID ──
    // Format: 14 digits, starts with 2 or 3
    fn generate_eg(&self, rng: &mut impl Rng) -> String {
        let first = if rng.gen_bool(0.5) { '2' } else { '3' };
        let rest: String = (0..13)
            .map(|_| (b'0' + rng.gen_range(0..10u8)) as char)
            .collect();
        format!("{}{}", first, rest)
    }
    fn validate_eg(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 14 && (c.starts_with('2') || c.starts_with('3'))
    }

    // ── ET TIN ──
    // Format: 10 digits
    fn generate_et(&self, rng: &mut impl Rng) -> String {
        (0..10)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_et(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 10 && !c.starts_with('0')
    }

    // ── FI HETU ──
    // Format: DDMMYY{sep}NNN{check}, sep is - (1900s), A (2000s)
    // Check: 9-digit number (DDMMYYNNN) mod 31 → char table
    fn generate_fi(&self, rng: &mut impl Rng) -> String {
        static FI_CHECK: &[u8] = b"0123456789ABCDEFHJKLMNPRSTUVWXY";
        let dd: u8 = rng.gen_range(1..=28);
        let mm: u8 = rng.gen_range(1..=12);
        let yy: u8 = rng.gen_range(0..=99);
        let sep = if rng.gen_bool(0.5) { '-' } else { 'A' };
        let seq: u16 = rng.gen_range(2..=899);
        let num: u64 = format!("{:02}{:02}{:02}{:03}", dd, mm, yy, seq)
            .parse()
            .unwrap();
        let check = FI_CHECK[(num % 31) as usize] as char;
        format!("{:02}{:02}{:02}{}{:03}{}", dd, mm, yy, sep, seq, check)
    }
    fn validate_fi(&self, code: &str) -> bool {
        static FI_CHECK: &[u8] = b"0123456789ABCDEFHJKLMNPRSTUVWXY";
        let clean = code.trim().to_uppercase();
        if clean.len() != 11 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        if chars[6] != '-' && chars[6] != 'A' && chars[6] != '+' {
            return false;
        }
        let date_part = &clean[..6];
        let seq_part = &clean[7..10];
        if !date_part.chars().all(|c| c.is_ascii_digit())
            || !seq_part.chars().all(|c| c.is_ascii_digit())
        {
            return false;
        }
        let num: u64 = match format!("{}{}", date_part, seq_part).parse() {
            Ok(n) => n,
            Err(_) => return false,
        };
        let expected = FI_CHECK[(num % 31) as usize] as char;
        chars[10] == expected
    }

    // ── GH TIN ──
    // Format: letter + 10 digits, letter from [P,C,G,Q,V]
    fn generate_gh(&self, rng: &mut impl Rng) -> String {
        let prefix = b"PCGQV"[rng.gen_range(0..5)] as char;
        let digits: String = (0..10)
            .map(|_| (b'0' + rng.gen_range(0..10u8)) as char)
            .collect();
        format!("{}{}", prefix, digits)
    }
    fn validate_gh(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 11 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        "PCGQV".contains(chars[0]) && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── GR AFM ──
    // Format: 9 digits, powers of 2 weighted, mod 11 mod 10
    fn generate_gr(&self, rng: &mut impl Rng) -> String {
        let body: Vec<u8> = (0..8)
            .map(|i| {
                if i == 0 {
                    rng.gen_range(1..=9u8)
                } else {
                    rng.gen_range(0..=9u8)
                }
            })
            .collect();
        let sum: u32 = body
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (1 << (8 - i)))
            .sum();
        let check = ((sum % 11) % 10) as u8;
        let mut digits = body;
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }
    fn validate_gr(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 9 || c.starts_with('0') {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let sum: u32 = digits[..8]
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (1 << (8 - i)))
            .sum();
        let expected = ((sum % 11) % 10) as u8;
        digits[8] == expected
    }

    // ── HK HKID ──
    // Format: 1 letter + 6 digits + check digit (0-9 or A), mod 11 weighted
    // Standard single-prefix: space(36)*9 + letter_val*8 + d1*7 + ... + d6*2
    fn generate_hk(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: Vec<u8> = (0..6).map(|_| rng.gen_range(0..=9u8)).collect();
        let letter_val = (letter as u32) - ('A' as u32) + 1;
        let mut sum = 36 * 9 + letter_val * 8;
        for (i, &d) in digits.iter().enumerate() {
            sum += d as u32 * (7 - i as u32);
        }
        let r = sum % 11;
        let check = if r == 0 {
            '0'
        } else if r == 1 {
            'A'
        } else {
            (b'0' + (11 - r) as u8) as char
        };
        let digit_str: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        format!("{}{}{}", letter, digit_str, check)
    }
    fn validate_hk(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase().replace(['(', ')'], "");
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        if !chars[0].is_ascii_uppercase() {
            return false;
        }
        if !chars[1..7].iter().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let check_char = chars[7];
        if !check_char.is_ascii_digit() && check_char != 'A' {
            return false;
        }
        let letter_val = chars[0] as u32 - 'A' as u32 + 1;
        let mut sum = 36 * 9 + letter_val * 8;
        for (i, &c) in chars[1..7].iter().enumerate() {
            sum += c.to_digit(10).unwrap() * (7 - i as u32);
        }
        let r = sum % 11;
        let expected = if r == 0 {
            '0'
        } else if r == 1 {
            'A'
        } else {
            (b'0' + (11 - r) as u8) as char
        };
        check_char == expected
    }

    // ── HR OIB ──
    // Format: 11 digits, ISO 7064 mod 11,10
    fn generate_hr(&self, rng: &mut impl Rng) -> String {
        let base: Vec<u8> = (0..10)
            .map(|i| {
                if i == 0 {
                    rng.gen_range(1..=9u8)
                } else {
                    rng.gen_range(0..=9u8)
                }
            })
            .collect();
        let check = iso7064_mod11_10(&base);
        let mut result: String = base.iter().map(|d| (b'0' + d) as char).collect();
        result.push((b'0' + check) as char);
        result
    }
    fn validate_hr(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 11 || c.starts_with('0') {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let expected = iso7064_mod11_10(&digits[..10]);
        digits[10] == expected
    }

    // ── HU Adóazonosító ──
    // Format: 10 digits (starts "8"), weights [1..9] mod 11
    fn generate_hu(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = vec![8];
        for _ in 0..8 {
            digits.push(rng.gen_range(0..=9u8));
        }
        let sum: u32 = digits
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (i as u32 + 1))
            .sum();
        let check = (sum % 11) as u8;
        if check >= 10 {
            return self.generate_hu(rng);
        }
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }
    fn validate_hu(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 10 || !c.starts_with('8') {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let sum: u32 = digits[..9]
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (i as u32 + 1))
            .sum();
        let expected = (sum % 11) as u8;
        expected < 10 && digits[9] == expected
    }

    // ── ID NPWP ──
    // Format: 15 or 16 digits
    fn generate_id(&self, rng: &mut impl Rng) -> String {
        let len = if rng.gen_bool(0.5) { 15 } else { 16 };
        (0..len)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_id(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        (c.len() == 15 || c.len() == 16) && !c.starts_with('0')
    }

    // ── IE PPS ──
    // Format: 7 digits + 1-2 letters, weighted mod 23 → letter
    fn generate_ie(&self, rng: &mut impl Rng) -> String {
        static IE_CHECK: &[u8] = b"WABCDEFGHIJKLMNOPQRSTUV";
        let digits: Vec<u8> = (0..7)
            .map(|i| {
                if i == 0 {
                    rng.gen_range(1..=9u8)
                } else {
                    rng.gen_range(0..=9u8)
                }
            })
            .collect();
        let weights = [8u32, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let check = IE_CHECK[(sum % 23) as usize] as char;
        let digit_str: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        format!("{}{}", digit_str, check)
    }
    fn validate_ie(&self, code: &str) -> bool {
        static IE_CHECK: &[u8] = b"WABCDEFGHIJKLMNOPQRSTUV";
        let clean = code.trim().to_uppercase();
        if clean.len() < 8 || clean.len() > 9 {
            return false;
        }
        if !clean[..7].chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let check_char = clean.chars().nth(7).unwrap();
        if !check_char.is_ascii_uppercase() {
            return false;
        }
        let digits: Vec<u8> = clean[..7].bytes().map(|b| b - b'0').collect();
        let weights = [8u32, 7, 6, 5, 4, 3, 2];
        let mut sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        // If 9 chars, the second letter acts as a multiplier
        if clean.len() == 9 {
            let second = clean.chars().nth(8).unwrap();
            let val = (second as u32) - ('A' as u32) + 1;
            sum += val * 9;
        }
        let expected = IE_CHECK[(sum % 23) as usize] as char;
        check_char == expected
    }

    // ── IL Mispar Zehut ──
    // Format: 9 digits with Luhn
    fn generate_il(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9u8)).collect();
        let check = luhn_check_digit(&digits);
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }
    fn validate_il(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 9 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        luhn_validate(&digits)
    }

    // ── India PAN ──
    // Format: AAAAA0000A (10 chars)
    fn generate_in(&self, opts: &GenOptions, rng: &mut impl Rng) -> (String, String) {
        let c1 = (b'A' + rng.gen_range(0..26u8)) as char;
        let c2 = (b'A' + rng.gen_range(0..26u8)) as char;
        let c3 = (b'A' + rng.gen_range(0..26u8)) as char;
        let holder = opts
            .holder_type
            .as_deref()
            .and_then(|s| s.chars().next())
            .map(|c| c.to_ascii_uppercase())
            .unwrap_or_else(|| PAN_HOLDER_TYPES[rng.gen_range(0..PAN_HOLDER_TYPES.len())]);
        let c5 = (b'A' + rng.gen_range(0..26u8)) as char;
        let seq: u16 = rng.gen_range(1..=9999);
        let check = (b'A' + rng.gen_range(0..26u8)) as char;
        (
            format!("{}{}{}{}{}{:04}{}", c1, c2, c3, holder, c5, seq, check),
            holder.to_string(),
        )
    }

    fn validate_in(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 10 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..5].iter().all(|c| c.is_ascii_uppercase())
            && PAN_HOLDER_TYPES.contains(&chars[3])
            && chars[5..9].iter().all(|c| c.is_ascii_digit())
            && chars[9].is_ascii_uppercase()
    }

    // ── US TIN ──
    // Format: 9 digits, area number constraints (no 000, 666, 9xx prefix)
    fn generate_us(&self, rng: &mut impl Rng) -> String {
        let area: u16 = loop {
            let a = rng.gen_range(1..=899u16);
            if a != 666 {
                break a;
            }
        };
        let group: u8 = rng.gen_range(1..=99);
        let serial: u16 = rng.gen_range(1..=9999);
        format!("{:03}{:02}{:04}", area, group, serial)
    }

    fn validate_us(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 9 {
            return false;
        }
        let area: u16 = clean[..3].parse().unwrap_or(0);
        let group: u8 = clean[3..5].parse().unwrap_or(0);
        let serial: u16 = clean[5..9].parse().unwrap_or(0);
        (1..=899).contains(&area) && area != 666 && group >= 1 && serial >= 1
    }

    // ── GB UTR ──
    // Format: 10 digits, check digit at position 1 (stdnum.gb.utr)
    // Lookup: '21987654321'[weighted_sum % 11]
    fn generate_gb(&self, rng: &mut impl Rng) -> String {
        let weights = [6u32, 7, 8, 9, 10, 5, 4, 3, 2];
        let lookup: [u8; 11] = [2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let base: Vec<u8> = (0..9).map(|_| rng.gen_range(0..=9u8)).collect();
        let sum: u32 = base
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let check = lookup[(sum % 11) as usize];
        let mut result = String::with_capacity(10);
        result.push((b'0' + check) as char);
        for d in &base {
            result.push((b'0' + d) as char);
        }
        result
    }

    fn validate_gb(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 10 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        let weights = [6u32, 7, 8, 9, 10, 5, 4, 3, 2];
        let lookup: [u8; 11] = [2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let sum: u32 = digits[1..]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let expected = lookup[(sum % 11) as usize];
        digits[0] == expected
    }

    // ── DE Steuer-IdNr ──
    // Format: 11 digits, ISO 7064 mod 11,10 check digit
    fn generate_de(&self, rng: &mut impl Rng) -> String {
        let base: Vec<u8> = (0..10)
            .map(|i| {
                if i == 0 {
                    rng.gen_range(1..=9u8)
                } else {
                    rng.gen_range(0..=9u8)
                }
            })
            .collect();
        let check = iso7064_mod11_10(&base);
        let mut result: String = base.iter().map(|d| (b'0' + d) as char).collect();
        result.push((b'0' + check) as char);
        result
    }

    fn validate_de(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 11 || clean.starts_with('0') {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        let expected = iso7064_mod11_10(&digits[..10]);
        digits[10] == expected
    }

    // ── FR NIF ──
    // Format: 13 digits, first digit 0-3, last 3 digits = first_10 % 511 (stdnum.fr.nif)
    fn generate_fr(&self, rng: &mut impl Rng) -> String {
        let first = rng.gen_range(0..=3u8);
        let rest: u64 = rng.gen_range(0..1_000_000_000);
        let base = first as u64 * 1_000_000_000 + rest;
        let check = base % 511;
        format!("{:010}{:03}", base, check)
    }

    fn validate_fr(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 13 {
            return false;
        }
        if !matches!(clean.as_bytes()[0], b'0'..=b'3') {
            return false;
        }
        let base: u64 = match clean[..10].parse() {
            Ok(n) => n,
            Err(_) => return false,
        };
        let check: u64 = match clean[10..].parse() {
            Ok(n) => n,
            Err(_) => return false,
        };
        base % 511 == check
    }

    // ── BR CPF ──
    // Format: 11 digits with 2 check digits (mod-11)
    fn generate_br(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = loop {
            let d: Vec<u8> = (0..9).map(|_| rng.gen_range(0..=9u8)).collect();
            if d.iter().any(|&x| x != d[0]) {
                break d;
            }
        };
        let sum1: u32 = digits
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (10 - i as u32))
            .sum();
        let r1 = (sum1 * 10 % 11) as u8;
        digits.push(if r1 >= 10 { 0 } else { r1 });
        let sum2: u32 = digits
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (11 - i as u32))
            .sum();
        let r2 = (sum2 * 10 % 11) as u8;
        digits.push(if r2 >= 10 { 0 } else { r2 });
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_br(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 11 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        if digits.iter().all(|&x| x == digits[0]) {
            return false;
        }
        let sum1: u32 = digits[..9]
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (10 - i as u32))
            .sum();
        let r1 = (sum1 * 10 % 11) as u8;
        let c1 = if r1 >= 10 { 0 } else { r1 };
        if digits[9] != c1 {
            return false;
        }
        let sum2: u32 = digits[..10]
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (11 - i as u32))
            .sum();
        let r2 = (sum2 * 10 % 11) as u8;
        let c2 = if r2 >= 10 { 0 } else { r2 };
        digits[10] == c2
    }

    // ── AU TFN ──
    // Format: 9 digits with weighted checksum (weights: 1,4,3,7,5,8,6,9,10)
    fn generate_au(&self, rng: &mut impl Rng) -> String {
        let weights = [1u32, 4, 3, 7, 5, 8, 6, 9, 10];
        loop {
            let digits: Vec<u8> = (0..9)
                .map(|i| {
                    if i == 0 {
                        rng.gen_range(1..=9u8)
                    } else {
                        rng.gen_range(0..=9u8)
                    }
                })
                .collect();
            let sum: u32 = digits
                .iter()
                .zip(weights.iter())
                .map(|(&d, &w)| d as u32 * w)
                .sum();
            if sum.is_multiple_of(11) {
                return digits.iter().map(|d| (b'0' + d) as char).collect();
            }
        }
    }

    fn validate_au(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 9 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        let weights = [1u32, 4, 3, 7, 5, 8, 6, 9, 10];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        sum.is_multiple_of(11)
    }

    // ── CA SIN ──
    // Format: 9 digits with Luhn checksum
    fn generate_ca(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..8)
            .map(|i| {
                if i == 0 {
                    rng.gen_range(1..=9u8)
                } else {
                    rng.gen_range(0..=9u8)
                }
            })
            .collect();
        let check = luhn_check_digit(&digits);
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_ca(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 9 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        luhn_validate(&digits)
    }

    // ── JP My Number ──
    // Format: 12 digits with check digit (mod-11 weighted)
    fn generate_jp(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..11)
            .map(|i| {
                if i == 0 {
                    rng.gen_range(1..=9u8)
                } else {
                    rng.gen_range(0..=9u8)
                }
            })
            .collect();
        let weights: Vec<u32> = (0..11)
            .map(|i| {
                let p = i + 1;
                if p <= 6 {
                    p as u32 + 1
                } else {
                    p as u32 - 5
                }
            })
            .collect();
        let sum: u32 = digits
            .iter()
            .rev()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let remainder = sum % 11;
        let check = if remainder <= 1 {
            0u8
        } else {
            (11 - remainder) as u8
        };
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_jp(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 12 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        let weights: Vec<u32> = (0..11)
            .map(|i| {
                let p = i + 1;
                if p <= 6 {
                    p as u32 + 1
                } else {
                    p as u32 - 5
                }
            })
            .collect();
        let sum: u32 = digits[..11]
            .iter()
            .rev()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let remainder = sum % 11;
        let expected = if remainder <= 1 {
            0u8
        } else {
            (11 - remainder) as u8
        };
        digits[11] == expected
    }

    // ── CN USCI ──
    // Format: 18 chars (Unified Social Credit Identifier) with mod-31 check
    fn generate_cn(&self, rng: &mut impl Rng) -> String {
        let mut chars: Vec<u8> = Vec::with_capacity(17);
        // Registration management department (1 char: 1-9 or A-G)
        chars.push(rng.gen_range(1..=9u8));
        // Organization category (1 char: 1-9 or A-G)
        chars.push(rng.gen_range(1..=9u8));
        // Region code (6 digits)
        let region: u32 = rng.gen_range(110000..=659000);
        for c in format!("{:06}", region).bytes() {
            chars.push(cn_usci_char_to_val(c));
        }
        // Organization code (9 chars: digits or valid letters)
        for _ in 0..9 {
            let idx = rng.gen_range(0..CN_USCI_CHARS.len());
            chars.push(idx as u8);
        }
        // Calculate check character
        let sum: u32 = chars
            .iter()
            .zip(CN_USCI_WEIGHTS.iter())
            .map(|(&v, &w)| v as u32 * w)
            .sum();
        let remainder = 31 - (sum % 31);
        let check_idx = if remainder == 31 { 0 } else { remainder as u8 };
        // Build output string
        let mut result = String::with_capacity(18);
        for &v in &chars {
            result.push(CN_USCI_CHARS[v as usize] as char);
        }
        result.push(CN_USCI_CHARS[check_idx as usize] as char);
        result
    }

    fn validate_cn(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 18 {
            return false;
        }
        let vals: Vec<u8> = match clean
            .bytes()
            .map(|b| CN_USCI_CHARS.iter().position(|&c| c == b).map(|p| p as u8))
            .collect::<Option<Vec<u8>>>()
        {
            Some(v) => v,
            None => return false,
        };
        let sum: u32 = vals[..17]
            .iter()
            .zip(CN_USCI_WEIGHTS.iter())
            .map(|(&v, &w)| v as u32 * w)
            .sum();
        let remainder = 31 - (sum % 31);
        let expected = if remainder == 31 { 0 } else { remainder as u8 };
        vals[17] == expected
    }

    // ── IT Partita IVA ──
    // Format: 11 digits with Luhn-variant checksum
    fn generate_it(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..10).map(|_| rng.gen_range(0..=9u8)).collect();
        // Italian Partita IVA check: odd positions (1-indexed) summed directly,
        // even positions doubled then subtract 9 if >= 10, check = (10 - sum%10) % 10
        let mut sum: u32 = 0;
        for (i, &d) in digits.iter().enumerate() {
            if i % 2 == 0 {
                sum += d as u32;
            } else {
                let doubled = d as u32 * 2;
                sum += if doubled > 9 { doubled - 9 } else { doubled };
            }
        }
        let check = ((10 - (sum % 10)) % 10) as u8;
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_it(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 11 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        if digits.iter().all(|&d| d == 0) {
            return false;
        }
        let mut sum: u32 = 0;
        for (i, &d) in digits[..10].iter().enumerate() {
            if i % 2 == 0 {
                sum += d as u32;
            } else {
                let doubled = d as u32 * 2;
                sum += if doubled > 9 { doubled - 9 } else { doubled };
            }
        }
        let expected = ((10 - (sum % 10)) % 10) as u8;
        digits[10] == expected
    }

    // ── ES NIF ──
    // Format: 8 digits + check letter (mod-23 table)
    fn generate_es(&self, rng: &mut impl Rng) -> String {
        let num: u32 = rng.gen_range(1_000_000..=99_999_999);
        let check = ES_NIF_LETTERS[(num % 23) as usize] as char;
        format!("{:08}{}", num, check)
    }

    fn validate_es(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        if !chars[..8].iter().all(|c| c.is_ascii_digit()) {
            return false;
        }
        if !chars[8].is_ascii_uppercase() {
            return false;
        }
        let num: u32 = match clean[..8].parse() {
            Ok(n) => n,
            Err(_) => return false,
        };
        let expected = ES_NIF_LETTERS[(num % 23) as usize] as char;
        chars[8] == expected
    }

    // ── NL BSN ──
    // Format: 9 digits, 11-check (weighted sum: 9,8,7,6,5,4,3,2,-1, divisible by 11)
    fn generate_nl(&self, rng: &mut impl Rng) -> String {
        let weights = [9i32, 8, 7, 6, 5, 4, 3, 2, -1];
        loop {
            let digits: Vec<u8> = (0..9)
                .map(|i| {
                    if i == 0 {
                        rng.gen_range(1..=9u8)
                    } else {
                        rng.gen_range(0..=9u8)
                    }
                })
                .collect();
            let sum: i32 = digits
                .iter()
                .zip(weights.iter())
                .map(|(&d, &w)| d as i32 * w)
                .sum();
            if sum % 11 == 0 && sum != 0 {
                return digits.iter().map(|d| (b'0' + d) as char).collect();
            }
        }
    }

    fn validate_nl(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 9 || clean.starts_with('0') {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        let weights = [9i32, 8, 7, 6, 5, 4, 3, 2, -1];
        let sum: i32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as i32 * w)
            .sum();
        sum % 11 == 0 && sum != 0
    }

    // ── SE Personnummer ──
    // Format: 10 digits (YYMMDDXXXX) with Luhn check on last 10 digits
    fn generate_se(&self, rng: &mut impl Rng) -> String {
        let year: u8 = rng.gen_range(40..=99);
        let month: u8 = rng.gen_range(1..=12);
        let day: u8 = rng.gen_range(1..=28);
        let serial: u16 = rng.gen_range(1..=999);
        let base: Vec<u8> = format!("{:02}{:02}{:02}{:03}", year, month, day, serial)
            .bytes()
            .map(|b| b - b'0')
            .collect();
        let check = luhn_check_digit(&base);
        format!("{:02}{:02}{:02}{:03}{}", year, month, day, serial, check)
    }

    fn validate_se(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 10 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        luhn_validate(&digits)
    }

    // ── KR BRN (Business Registration Number) ──
    // Format: 10 digits (XXX-XX-XXXXX) with check digit
    fn generate_kr(&self, rng: &mut impl Rng) -> String {
        let weights = [1u32, 3, 7, 1, 3, 7, 1, 3, 5];
        let mut digits: Vec<u8> = (0..9)
            .map(|i| {
                if i == 0 {
                    rng.gen_range(1..=9u8)
                } else {
                    rng.gen_range(0..=9u8)
                }
            })
            .collect();
        let mut sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        // Add floor(d[8]*5/10)
        sum += (digits[8] as u32 * 5) / 10;
        let check = ((10 - (sum % 10)) % 10) as u8;
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_kr(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 10 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        let weights = [1u32, 3, 7, 1, 3, 7, 1, 3, 5];
        let mut sum: u32 = digits[..9]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        sum += (digits[8] as u32 * 5) / 10;
        let expected = ((10 - (sum % 10)) % 10) as u8;
        digits[9] == expected
    }

    // ── SG Tax Reference (NRIC/FIN format) ──
    // Format: 1 letter + 7 digits + 1 check letter
    fn generate_sg(&self, rng: &mut impl Rng) -> String {
        let prefix = if rng.gen_bool(0.5) { 'S' } else { 'T' };
        let weights = [2u32, 7, 6, 5, 4, 3, 2];
        let digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9u8)).collect();
        let mut sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        if prefix == 'T' {
            sum += 4;
        }
        let check_letters = b"JZIHGFEDCBA";
        let check = check_letters[(sum % 11) as usize] as char;
        let digit_str: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        format!("{}{}{}", prefix, digit_str, check)
    }

    fn validate_sg(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        let prefix = chars[0];
        if prefix != 'S' && prefix != 'T' && prefix != 'F' && prefix != 'G' {
            return false;
        }
        if !chars[1..8].iter().all(|c| c.is_ascii_digit()) {
            return false;
        }
        if !chars[8].is_ascii_uppercase() {
            return false;
        }
        let weights = [2u32, 7, 6, 5, 4, 3, 2];
        let digits: Vec<u8> = clean[1..8].bytes().map(|b| b - b'0').collect();
        let mut sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        if prefix == 'T' || prefix == 'G' {
            sum += 4;
        }
        let check_letters = if prefix == 'S' || prefix == 'T' {
            b"JZIHGFEDCBA"
        } else {
            b"XWUTRQPNMLK"
        };
        let expected = check_letters[(sum % 11) as usize] as char;
        chars[8] == expected
    }

    // ── ZA Tax Number ──
    // Format: 10 digits with Luhn checksum
    fn generate_za(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = Vec::with_capacity(10);
        // First digit is 0, 1, 2, or 9 (tax type)
        digits.push([0u8, 1, 2, 9][rng.gen_range(0..4)]);
        for _ in 1..9 {
            digits.push(rng.gen_range(0..=9u8));
        }
        let check = luhn_check_digit(&digits);
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_za(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 10 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        luhn_validate(&digits)
    }

    // ── MX RFC ──
    // Format: 12 chars (company) or 13 chars (individual) with check digit
    // Structure: 3-4 alpha + 6 digits (YYMMDD) + 2 alphanum + check
    fn generate_mx(&self, rng: &mut impl Rng) -> String {
        let is_company = rng.gen_bool(0.5);
        let prefix_len = if is_company { 3 } else { 4 };
        let prefix: String = (0..prefix_len)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let year: u8 = rng.gen_range(50..=99);
        let month: u8 = rng.gen_range(1..=12);
        let day: u8 = rng.gen_range(1..=28);
        let homoclave: String = (0..2)
            .map(|_| {
                let idx = rng.gen_range(0..36u8);
                if idx < 10 {
                    (b'0' + idx) as char
                } else {
                    (b'A' + idx - 10) as char
                }
            })
            .collect();
        let base = format!("{}{:02}{:02}{:02}{}", prefix, year, month, day, homoclave);
        let check = mx_rfc_check_digit(&base);
        format!("{}{}", base, check)
    }

    fn validate_mx(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 12 && clean.len() != 13 {
            return false;
        }
        let prefix_len = clean.len() - 9; // 3 or 4
        if !clean[..prefix_len].chars().all(|c| c.is_ascii_uppercase()) {
            return false;
        }
        if !clean[prefix_len..prefix_len + 6]
            .chars()
            .all(|c| c.is_ascii_digit())
        {
            return false;
        }
        let base = &clean[..clean.len() - 1];
        let expected = mx_rfc_check_digit(base);
        let last = clean.chars().last().unwrap();
        last == expected
    }

    // ── IS Kennitala ──
    // Format: 10 digits DDMMRRSCCY where DDMMRR=date parts, S=seq, C=check, CY=century+year
    // Weights [3,2,7,6,5,4,3,2] on first 8 digits (DDMMRRSC-like), check at position 9 (0-indexed 8)
    // Actually: DDMMYYNNCZ - DD day, MM month, YY year, NN random, C check, Z century
    fn generate_is(&self, rng: &mut impl Rng) -> String {
        loop {
            let dd: u8 = rng.gen_range(1..=28);
            let mm: u8 = rng.gen_range(1..=12);
            let yy: u8 = rng.gen_range(0..=99);
            let nn: u8 = rng.gen_range(20..=99);
            let century: u8 = if yy <= 30 { 0 } else { 9 };
            // Kennitala: DDMMYYNN + check + century = 10 digits
            let base = format!("{:02}{:02}{:02}{:02}", dd, mm, yy, nn);
            let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
            let weights = [3u32, 2, 7, 6, 5, 4, 3, 2];
            let sum: u32 = digits
                .iter()
                .zip(weights.iter())
                .map(|(&d, &w)| d as u32 * w)
                .sum();
            let r = sum % 11;
            if r == 1 {
                continue;
            } // invalid
            let check = if r == 0 { 0u8 } else { (11 - r) as u8 };
            return format!("{}{}{}", base, check, century);
        }
    }
    fn validate_is(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 10 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let weights = [3u32, 2, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[..8]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        if r == 1 {
            return false;
        }
        let expected = if r == 0 { 0u8 } else { (11 - r) as u8 };
        digits[8] == expected
    }

    // ── KE KRA PIN ──
    // Format: letter + 9 digits + letter
    fn generate_ke(&self, rng: &mut impl Rng) -> String {
        let prefix = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..9)
            .map(|_| (b'0' + rng.gen_range(0..10u8)) as char)
            .collect();
        let suffix = (b'A' + rng.gen_range(0..26u8)) as char;
        format!("{}{}{}", prefix, digits, suffix)
    }
    fn validate_ke(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 11 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_uppercase()
            && chars[1..10].iter().all(|c| c.is_ascii_digit())
            && chars[10].is_ascii_uppercase()
    }

    // ── KW Civil ID ──
    // Format: 12 digits, starts with 1, 2, or 3
    fn generate_kw(&self, rng: &mut impl Rng) -> String {
        let first = [1u8, 2, 3][rng.gen_range(0..3)];
        let rest: String = (0..11)
            .map(|_| (b'0' + rng.gen_range(0..10u8)) as char)
            .collect();
        format!("{}{}", first, rest)
    }
    fn validate_kw(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 12 && (c.starts_with('1') || c.starts_with('2') || c.starts_with('3'))
    }

    // ── LK NIC ──
    // Format: 9 digits + V/X or 12 digits
    fn generate_lk(&self, rng: &mut impl Rng) -> String {
        if rng.gen_bool(0.5) {
            // Old format
            let digits: String = (0..9)
                .map(|i| {
                    if i == 0 {
                        (b'1' + rng.gen_range(0..9u8)) as char
                    } else {
                        (b'0' + rng.gen_range(0..10u8)) as char
                    }
                })
                .collect();
            let suffix = if rng.gen_bool(0.5) { 'V' } else { 'X' };
            format!("{}{}", digits, suffix)
        } else {
            // New format
            (0..12)
                .map(|i| {
                    if i == 0 {
                        (b'1' + rng.gen_range(0..9u8)) as char
                    } else {
                        (b'0' + rng.gen_range(0..10u8)) as char
                    }
                })
                .collect()
        }
    }
    fn validate_lk(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() == 10 {
            clean[..9].chars().all(|c| c.is_ascii_digit())
                && (clean.ends_with('V') || clean.ends_with('X'))
        } else if clean.len() == 12 {
            clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
        } else {
            false
        }
    }

    // ── LT Asmens kodas ──
    // Format: 11 digits, two-pass weighted mod 11 (same as EE)
    fn generate_lt(&self, rng: &mut impl Rng) -> String {
        let gender: u8 = rng.gen_range(1..=6);
        let yy: u8 = rng.gen_range(0..=99);
        let mm: u8 = rng.gen_range(1..=12);
        let dd: u8 = rng.gen_range(1..=28);
        let seq: u16 = rng.gen_range(0..=999);
        let base = format!("{}{:02}{:02}{:02}{:03}", gender, yy, mm, dd, seq);
        let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
        let w1 = [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 1];
        let sum1: u32 = digits
            .iter()
            .zip(w1.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r1 = sum1 % 11;
        let check = if r1 < 10 {
            r1 as u8
        } else {
            let w2 = [3u32, 4, 5, 6, 7, 8, 9, 1, 2, 3];
            let sum2: u32 = digits
                .iter()
                .zip(w2.iter())
                .map(|(&d, &w)| d as u32 * w)
                .sum();
            let r2 = sum2 % 11;
            if r2 < 10 {
                r2 as u8
            } else {
                0
            }
        };
        format!("{}{}", base, check)
    }
    fn validate_lt(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 11 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let w1 = [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 1];
        let sum1: u32 = digits[..10]
            .iter()
            .zip(w1.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r1 = sum1 % 11;
        let expected = if r1 < 10 {
            r1 as u8
        } else {
            let w2 = [3u32, 4, 5, 6, 7, 8, 9, 1, 2, 3];
            let sum2: u32 = digits[..10]
                .iter()
                .zip(w2.iter())
                .map(|(&d, &w)| d as u32 * w)
                .sum();
            let r2 = sum2 % 11;
            if r2 < 10 {
                r2 as u8
            } else {
                0
            }
        };
        digits[10] == expected
    }

    // ── LU Matricule ──
    // Format: 13 digits, digit 12 = Luhn, digit 13 = Verhoeff
    fn generate_lu(&self, rng: &mut impl Rng) -> String {
        let yy: u8 = rng.gen_range(0..=99);
        let mm: u8 = rng.gen_range(1..=12);
        let dd: u8 = rng.gen_range(1..=28);
        let seq: u16 = rng.gen_range(0..=999);
        let base_str = format!("{:04}{:02}{:02}{:03}", 1900 + yy as u16, mm, dd, seq);
        let base: Vec<u8> = base_str.bytes().map(|b| b - b'0').collect();
        let luhn_c = luhn_check_digit(&base);
        let mut all = base.clone();
        all.push(luhn_c);
        let verh_c = crate::personal_id::checksum::verhoeff_check(&all);
        format!("{}{}{}", base_str, luhn_c, verh_c)
    }
    fn validate_lu(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 13 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let luhn_expected = luhn_check_digit(&digits[..11]);
        if digits[11] != luhn_expected {
            return false;
        }
        crate::personal_id::checksum::verhoeff_validate(&digits)
    }

    // ── LV Personas kods ──
    // Format: 11 digits, modern format starts with "32"
    fn generate_lv(&self, rng: &mut impl Rng) -> String {
        let rest: String = (0..9)
            .map(|_| (b'0' + rng.gen_range(0..10u8)) as char)
            .collect();
        format!("32{}", rest)
    }
    fn validate_lv(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 11 {
            return false;
        }
        // Accept both modern (32-prefix) and legacy formats
        true
    }

    // ── MA CIN ──
    // Format: 1-2 letters + 5-6 digits
    fn generate_ma(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..10u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }
    fn validate_ma(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() < 6 || clean.len() > 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        let alpha_count = chars.iter().take_while(|c| c.is_ascii_uppercase()).count();
        (1..=2).contains(&alpha_count)
            && chars[alpha_count..].iter().all(|c| c.is_ascii_digit())
    }

    // ── MT TIN ──
    // Format: 7 digits + letter or 9 digits
    fn generate_mt(&self, rng: &mut impl Rng) -> String {
        if rng.gen_bool(0.5) {
            let digits: String = (0..7)
                .map(|i| {
                    if i == 0 {
                        (b'1' + rng.gen_range(0..9u8)) as char
                    } else {
                        (b'0' + rng.gen_range(0..10u8)) as char
                    }
                })
                .collect();
            let letter = b"MGLBH"[rng.gen_range(0..5)] as char;
            format!("{}{}", digits, letter)
        } else {
            (0..9)
                .map(|i| {
                    if i == 0 {
                        (b'1' + rng.gen_range(0..9u8)) as char
                    } else {
                        (b'0' + rng.gen_range(0..10u8)) as char
                    }
                })
                .collect()
        }
    }
    fn validate_mt(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() == 8 {
            clean[..7].chars().all(|c| c.is_ascii_digit())
                && "MGLBH".contains(clean.chars().last().unwrap())
        } else if clean.len() == 9 {
            clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
        } else {
            false
        }
    }

    // ── MY MyKad ──
    // Format: 12 digits (YYMMDDBBSSSS)
    fn generate_my(&self, rng: &mut impl Rng) -> String {
        let yy: u8 = rng.gen_range(0..=99);
        let mm: u8 = rng.gen_range(1..=12);
        let dd: u8 = rng.gen_range(1..=28);
        let bp: u8 = rng.gen_range(1..=99);
        let serial: u16 = rng.gen_range(1..=9999);
        format!("{:02}{:02}{:02}{:02}{:04}", yy, mm, dd, bp, serial)
    }
    fn validate_my(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 12
    }

    // ── NG TIN ──
    // Format: 12 digits
    fn generate_ng(&self, rng: &mut impl Rng) -> String {
        (0..12)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_ng(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 12 && !c.starts_with('0')
    }

    // ── NO Fødselsnummer ──
    // Format: 11 digits, two weighted check digits
    fn generate_no(&self, rng: &mut impl Rng) -> String {
        loop {
            let dd: u8 = rng.gen_range(1..=28);
            let mm: u8 = rng.gen_range(1..=12);
            let yy: u8 = rng.gen_range(0..=99);
            let ind: u16 = rng.gen_range(0..=999);
            let base = format!("{:02}{:02}{:02}{:03}", dd, mm, yy, ind);
            let d: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
            let w1 = [3u32, 7, 6, 1, 8, 9, 4, 5, 2];
            let sum1: u32 = d.iter().zip(w1.iter()).map(|(&d, &w)| d as u32 * w).sum();
            let r1 = 11 - (sum1 % 11);
            let c1 = if r1 == 11 {
                0u8
            } else if r1 == 10 {
                continue;
            } else {
                r1 as u8
            };
            let w2 = [5u32, 4, 3, 2, 7, 6, 5, 4, 3, 2];
            let mut d2 = d.clone();
            d2.push(c1);
            let sum2: u32 = d2.iter().zip(w2.iter()).map(|(&d, &w)| d as u32 * w).sum();
            let r2 = 11 - (sum2 % 11);
            let c2 = if r2 == 11 {
                0u8
            } else if r2 == 10 {
                continue;
            } else {
                r2 as u8
            };
            return format!("{}{}{}", base, c1, c2);
        }
    }
    fn validate_no(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 11 {
            return false;
        }
        let d: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let w1 = [3u32, 7, 6, 1, 8, 9, 4, 5, 2];
        let sum1: u32 = d[..9]
            .iter()
            .zip(w1.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r1 = 11 - (sum1 % 11);
        let c1 = if r1 == 11 {
            0u8
        } else if r1 == 10 {
            return false;
        } else {
            r1 as u8
        };
        if d[9] != c1 {
            return false;
        }
        let w2 = [5u32, 4, 3, 2, 7, 6, 5, 4, 3, 2];
        let sum2: u32 = d[..10]
            .iter()
            .zip(w2.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r2 = 11 - (sum2 % 11);
        let c2 = if r2 == 11 {
            0u8
        } else if r2 == 10 {
            return false;
        } else {
            r2 as u8
        };
        d[10] == c2
    }

    // ── NP PAN ──
    // Format: 9 digits
    fn generate_np(&self, rng: &mut impl Rng) -> String {
        (0..9)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_np(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 9 && !c.starts_with('0')
    }

    // ── NZ IRD ──
    // Format: 8-9 digits, mod 11 weighted
    fn generate_nz(&self, rng: &mut impl Rng) -> String {
        let body: Vec<u8> = (0..8)
            .map(|i| {
                if i == 0 {
                    rng.gen_range(1..=9u8)
                } else {
                    rng.gen_range(0..=9u8)
                }
            })
            .collect();
        let w1 = [3u32, 2, 7, 6, 5, 4, 3, 2];
        let sum: u32 = body
            .iter()
            .zip(w1.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        let check = if r == 0 { 0u8 } else { (11 - r) as u8 };
        if check == 10 {
            // Use secondary weights
            let w2 = [7u32, 4, 3, 2, 5, 2, 7, 6];
            let sum2: u32 = body
                .iter()
                .zip(w2.iter())
                .map(|(&d, &w)| d as u32 * w)
                .sum();
            let r2 = sum2 % 11;
            let c2 = if r2 == 0 { 0u8 } else { (11 - r2) as u8 };
            if c2 == 10 {
                return self.generate_nz(rng);
            }
            let mut result = body;
            result.push(c2);
            return result.iter().map(|d| (b'0' + d) as char).collect();
        }
        let mut result = body;
        result.push(check);
        result.iter().map(|d| (b'0' + d) as char).collect()
    }
    fn validate_nz(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 8 && c.len() != 9 {
            return false;
        }
        // Pad to 9 if 8
        let padded = if c.len() == 8 { format!("0{}", c) } else { c };
        let digits: Vec<u8> = padded.bytes().map(|b| b - b'0').collect();
        let w1 = [3u32, 2, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[..8]
            .iter()
            .zip(w1.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        let check = if r == 0 { 0u8 } else { (11 - r) as u8 };
        if check < 10 {
            return digits[8] == check;
        }
        let w2 = [7u32, 4, 3, 2, 5, 2, 7, 6];
        let sum2: u32 = digits[..8]
            .iter()
            .zip(w2.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r2 = sum2 % 11;
        let c2 = if r2 == 0 { 0u8 } else { (11 - r2) as u8 };
        c2 < 10 && digits[8] == c2
    }

    // ── OM Tax ID ──
    // Format: 8 digits
    fn generate_om(&self, rng: &mut impl Rng) -> String {
        (0..8)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_om(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 8 && !c.starts_with('0')
    }

    // ── PE RUC ──
    // Format: 11 digits, mod 11 weighted
    fn generate_pe(&self, rng: &mut impl Rng) -> String {
        let prefix = [10u8, 15, 17, 20][rng.gen_range(0..4)];
        let body: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9u8)).collect();
        let weights = [5u32, 4, 3, 2, 7, 6, 5, 4, 3, 2];
        let mut digits = vec![prefix / 10, prefix % 10];
        digits.extend_from_slice(&body);
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        let check = if 11 - r > 9 {
            (11 - r - 10) as u8
        } else {
            (11 - r) as u8
        };
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }
    fn validate_pe(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 11 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let weights = [5u32, 4, 3, 2, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[..10]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        let expected = if 11 - r > 9 {
            (11 - r - 10) as u8
        } else {
            (11 - r) as u8
        };
        digits[10] == expected
    }

    // ── PH TIN ──
    // Format: 9 or 12 digits
    fn generate_ph(&self, rng: &mut impl Rng) -> String {
        let len = if rng.gen_bool(0.5) { 9 } else { 12 };
        (0..len)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_ph(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        (c.len() == 9 || c.len() == 12) && !c.starts_with('0')
    }

    // ── PK NTN ──
    // Format: 7 digits
    fn generate_pk(&self, rng: &mut impl Rng) -> String {
        (0..7)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_pk(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 7 && !c.starts_with('0')
    }

    // ── PL PESEL ──
    // Format: 11 digits, weights [1,3,7,9,1,3,7,9,1,3] mod 10
    fn generate_pl(&self, rng: &mut impl Rng) -> String {
        let yy: u8 = rng.gen_range(0..=99);
        let mm: u8 = rng.gen_range(1..=12);
        let dd: u8 = rng.gen_range(1..=28);
        let serial: u16 = rng.gen_range(0..=9999);
        let base = format!("{:02}{:02}{:02}{:04}", yy, mm, dd, serial);
        let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
        let weights = [1u32, 3, 7, 9, 1, 3, 7, 9, 1, 3];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let check = ((10 - (sum % 10)) % 10) as u8;
        format!("{}{}", base, check)
    }
    fn validate_pl(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 11 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let weights = [1u32, 3, 7, 9, 1, 3, 7, 9, 1, 3];
        let sum: u32 = digits[..10]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let expected = ((10 - (sum % 10)) % 10) as u8;
        digits[10] == expected
    }

    // ── PT NIF ──
    // Format: 9 digits, weights [9,8,7,6,5,4,3,2] mod 11
    fn generate_pt(&self, rng: &mut impl Rng) -> String {
        let first = [1u8, 2, 3, 5, 6, 8][rng.gen_range(0..6)];
        let body: Vec<u8> = std::iter::once(first)
            .chain((0..7).map(|_| rng.gen_range(0..=9u8)))
            .collect();
        let weights = [9u32, 8, 7, 6, 5, 4, 3, 2];
        let sum: u32 = body
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        let check = if r < 2 { 0u8 } else { (11 - r) as u8 };
        let mut digits = body;
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }
    fn validate_pt(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 9 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let weights = [9u32, 8, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[..8]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        let expected = if r < 2 { 0u8 } else { (11 - r) as u8 };
        digits[8] == expected
    }

    // ── QA QID ──
    // Format: 11 digits, starts with 2 or 3
    fn generate_qa(&self, rng: &mut impl Rng) -> String {
        let first = if rng.gen_bool(0.5) { '2' } else { '3' };
        let rest: String = (0..10)
            .map(|_| (b'0' + rng.gen_range(0..10u8)) as char)
            .collect();
        format!("{}{}", first, rest)
    }
    fn validate_qa(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 11 && (c.starts_with('2') || c.starts_with('3'))
    }

    // ── RO CNP ──
    // Format: 13 digits, weights [2,7,9,1,4,6,3,5,8,2,7,9] mod 11
    fn generate_ro(&self, rng: &mut impl Rng) -> String {
        let sex: u8 = rng.gen_range(1..=6);
        let yy: u8 = rng.gen_range(0..=99);
        let mm: u8 = rng.gen_range(1..=12);
        let dd: u8 = rng.gen_range(1..=28);
        let county: u8 = rng.gen_range(1..=52);
        let seq: u16 = rng.gen_range(1..=999);
        let base = format!("{}{:02}{:02}{:02}{:02}{:03}", sex, yy, mm, dd, county, seq);
        let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
        let weights = [2u32, 7, 9, 1, 4, 6, 3, 5, 8, 2, 7, 9];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        let check = if r == 10 { 1u8 } else { r as u8 };
        format!("{}{}", base, check)
    }
    fn validate_ro(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 13 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let weights = [2u32, 7, 9, 1, 4, 6, 3, 5, 8, 2, 7, 9];
        let sum: u32 = digits[..12]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        let expected = if r == 10 { 1u8 } else { r as u8 };
        digits[12] == expected
    }

    // ── RS JMBG ──
    // Format: 13 digits, weighted checksum
    fn generate_rs(&self, rng: &mut impl Rng) -> String {
        let dd: u8 = rng.gen_range(1..=28);
        let mm: u8 = rng.gen_range(1..=12);
        let yyy: u16 = rng.gen_range(0..=999);
        let region: u8 = rng.gen_range(70..=99); // Serbia region codes
        let seq: u16 = rng.gen_range(0..=999);
        let base = format!("{:02}{:02}{:03}{:02}{:03}", dd, mm, yyy, region, seq);
        let d: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
        let weights = [7u32, 6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
        let sum: u32 = d
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = 11 - (sum % 11);
        let check = if r > 9 { 0u8 } else { r as u8 };
        format!("{}{}", base, check)
    }
    fn validate_rs(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 13 {
            return false;
        }
        let d: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let weights = [7u32, 6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
        let sum: u32 = d[..12]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = 11 - (sum % 11);
        let expected = if r > 9 { 0u8 } else { r as u8 };
        d[12] == expected
    }

    // ── SA National ID ──
    // Format: 10 digits, Luhn, starts with 1 or 2
    fn generate_sa(&self, rng: &mut impl Rng) -> String {
        let first = if rng.gen_bool(0.5) { 1u8 } else { 2 };
        let mut digits: Vec<u8> = vec![first];
        for _ in 0..8 {
            digits.push(rng.gen_range(0..=9u8));
        }
        let check = luhn_check_digit(&digits);
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }
    fn validate_sa(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 10 {
            return false;
        }
        if !c.starts_with('1') && !c.starts_with('2') {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        luhn_validate(&digits)
    }

    // ── SI Davčna ──
    // Format: 8 digits, weighted mod 11
    fn generate_si(&self, rng: &mut impl Rng) -> String {
        loop {
            let body: Vec<u8> = (0..7)
                .map(|i| {
                    if i == 0 {
                        rng.gen_range(1..=9u8)
                    } else {
                        rng.gen_range(0..=9u8)
                    }
                })
                .collect();
            let weights = [8u32, 7, 6, 5, 4, 3, 2];
            let sum: u32 = body
                .iter()
                .zip(weights.iter())
                .map(|(&d, &w)| d as u32 * w)
                .sum();
            let r = sum % 11;
            let check = if r == 0 {
                continue;
            } else if r == 1 {
                0u8
            } else {
                (11 - r) as u8
            };
            let mut digits = body;
            digits.push(check);
            return digits.iter().map(|d| (b'0' + d) as char).collect();
        }
    }
    fn validate_si(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 8 || c.starts_with('0') {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let weights = [8u32, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[..7]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let r = sum % 11;
        if r == 0 {
            return false;
        }
        let expected = if r == 1 { 0u8 } else { (11 - r) as u8 };
        digits[7] == expected
    }

    // ── SK Rodné číslo ──
    // Format: 10 digits, divisible by 11 (same as CZ)
    fn generate_sk(&self, rng: &mut impl Rng) -> String {
        self.generate_cz(rng)
    }
    fn validate_sk(&self, code: &str) -> bool {
        self.validate_cz(code)
    }

    // ── TH Tax ID ──
    // Format: 13 digits, weights [13..2] mod 11
    fn generate_th(&self, rng: &mut impl Rng) -> String {
        let body: Vec<u8> = (0..12)
            .map(|i| {
                if i == 0 {
                    rng.gen_range(1..=9u8)
                } else {
                    rng.gen_range(0..=9u8)
                }
            })
            .collect();
        let sum: u32 = body
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (13 - i as u32))
            .sum();
        let r = sum % 11;
        let check = if r <= 1 {
            (1 - r) as u8
        } else {
            (11 - r) as u8
        };
        let mut digits = body;
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }
    fn validate_th(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 13 || c.starts_with('0') {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let sum: u32 = digits[..12]
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (13 - i as u32))
            .sum();
        let r = sum % 11;
        let expected = if r <= 1 {
            (1 - r) as u8
        } else {
            (11 - r) as u8
        };
        digits[12] == expected
    }

    // ── TN CIN ──
    // Format: 8 digits
    fn generate_tn(&self, rng: &mut impl Rng) -> String {
        (0..8)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_tn(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 8 && !c.starts_with('0')
    }

    // ── TR TC Kimlik ──
    // Format: 11 digits, custom formula
    fn generate_tr(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = vec![rng.gen_range(1..=9u8)];
        for _ in 0..8 {
            digits.push(rng.gen_range(0..=9u8));
        }
        let odd_sum: u32 = digits.iter().step_by(2).map(|&d| d as u32).sum();
        let even_sum: u32 = digits.iter().skip(1).step_by(2).map(|&d| d as u32).sum();
        let d10 = ((odd_sum * 7).wrapping_sub(even_sum) % 10) as u8;
        digits.push(d10);
        let total: u32 = digits.iter().map(|&d| d as u32).sum();
        let d11 = (total % 10) as u8;
        digits.push(d11);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }
    fn validate_tr(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 11 || c.starts_with('0') {
            return false;
        }
        let d: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let odd_sum: u32 = d[..9].iter().step_by(2).map(|&d| d as u32).sum();
        let even_sum: u32 = d[..9].iter().skip(1).step_by(2).map(|&d| d as u32).sum();
        let expected_10 = ((odd_sum * 7).wrapping_sub(even_sum) % 10) as u8;
        if d[9] != expected_10 {
            return false;
        }
        let total: u32 = d[..10].iter().map(|&d| d as u32).sum();
        d[10] == (total % 10) as u8
    }

    // ── TW National ID ──
    // Format: letter + 9 digits, letter→2-digit mapping, weighted mod 10
    fn generate_tw(&self, rng: &mut impl Rng) -> String {
        static TW_MAP: &[(char, u8, u8)] = &[
            ('A', 1, 0),
            ('B', 1, 1),
            ('C', 1, 2),
            ('D', 1, 3),
            ('E', 1, 4),
            ('F', 1, 5),
            ('G', 1, 6),
            ('H', 1, 7),
            ('I', 3, 4),
            ('J', 1, 8),
            ('K', 1, 9),
            ('L', 2, 0),
            ('M', 2, 1),
            ('N', 2, 2),
            ('O', 3, 5),
            ('P', 2, 3),
            ('Q', 2, 4),
            ('R', 2, 5),
            ('S', 2, 6),
            ('T', 2, 7),
            ('U', 2, 8),
            ('V', 2, 9),
            ('W', 3, 2),
            ('X', 3, 0),
            ('Y', 3, 1),
            ('Z', 3, 3),
        ];
        let idx = rng.gen_range(0..26);
        let (letter, d1, d2) = TW_MAP[idx];
        let gender: u8 = rng.gen_range(1..=2);
        let body: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9u8)).collect();
        let weights = [1u32, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1];
        let mut all = vec![d1, d2, gender];
        all.extend_from_slice(&body);
        // Sum first 10 with weights[0..10]
        let sum: u32 = all
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let check = ((10 - (sum % 10)) % 10) as u8;
        let body_str: String = body.iter().map(|d| (b'0' + d) as char).collect();
        format!("{}{}{}{}", letter, gender, body_str, check)
    }
    fn validate_tw(&self, code: &str) -> bool {
        static TW_MAP: &[(char, u8, u8)] = &[
            ('A', 1, 0),
            ('B', 1, 1),
            ('C', 1, 2),
            ('D', 1, 3),
            ('E', 1, 4),
            ('F', 1, 5),
            ('G', 1, 6),
            ('H', 1, 7),
            ('I', 3, 4),
            ('J', 1, 8),
            ('K', 1, 9),
            ('L', 2, 0),
            ('M', 2, 1),
            ('N', 2, 2),
            ('O', 3, 5),
            ('P', 2, 3),
            ('Q', 2, 4),
            ('R', 2, 5),
            ('S', 2, 6),
            ('T', 2, 7),
            ('U', 2, 8),
            ('V', 2, 9),
            ('W', 3, 2),
            ('X', 3, 0),
            ('Y', 3, 1),
            ('Z', 3, 3),
        ];
        let clean = code.trim().to_uppercase();
        if clean.len() != 10 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        if !chars[0].is_ascii_uppercase() {
            return false;
        }
        if !chars[1..].iter().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let (_, d1, d2) = match TW_MAP.iter().find(|(l, _, _)| *l == chars[0]) {
            Some(m) => *m,
            None => return false,
        };
        let digits: Vec<u8> = clean[1..].bytes().map(|b| b - b'0').collect();
        let weights = [1u32, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1];
        let mut all = vec![d1, d2];
        all.extend_from_slice(&digits);
        let sum: u32 = all
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        sum.is_multiple_of(10)
    }

    // ── TZ TIN ──
    // Format: 9 digits
    fn generate_tz(&self, rng: &mut impl Rng) -> String {
        (0..9)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_tz(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 9 && !c.starts_with('0')
    }

    // ── UA IPN (РНОКПП) ──
    // Format: 10 digits, weights (-1,5,7,9,4,6,10,5,7), check = (sum%11)%10 (stdnum.ua.rntrc)
    fn generate_ua(&self, rng: &mut impl Rng) -> String {
        let weights: [i32; 9] = [-1, 5, 7, 9, 4, 6, 10, 5, 7];
        let base: Vec<u8> = (0..9)
            .map(|i| {
                if i == 0 {
                    rng.gen_range(1..=9u8)
                } else {
                    rng.gen_range(0..=9u8)
                }
            })
            .collect();
        let total: i32 = base
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as i32 * w)
            .sum();
        let check = ((total % 11).rem_euclid(10)) as u8;
        let mut result: String = base.iter().map(|d| (b'0' + d) as char).collect();
        result.push((b'0' + check) as char);
        result
    }
    fn validate_ua(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 10 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let weights: [i32; 9] = [-1, 5, 7, 9, 4, 6, 10, 5, 7];
        let total: i32 = digits[..9]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as i32 * w)
            .sum();
        let check = ((total % 11).rem_euclid(10)) as u8;
        digits[9] == check
    }

    // ── UY RUT ──
    // Format: 12 digits
    fn generate_uy(&self, rng: &mut impl Rng) -> String {
        (0..12)
            .map(|i| {
                if i == 0 {
                    (b'1' + rng.gen_range(0..9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..10u8)) as char
                }
            })
            .collect()
    }
    fn validate_uy(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        c.len() == 12 && !c.starts_with('0')
    }

    // ── VE RIF ──
    // Format: letter + 9 digits, letter from [V,E,J,G,P,C]
    fn generate_ve(&self, rng: &mut impl Rng) -> String {
        let prefix = b"VEJGPC"[rng.gen_range(0..6)] as char;
        let digits: String = (0..9)
            .map(|_| (b'0' + rng.gen_range(0..10u8)) as char)
            .collect();
        format!("{}{}", prefix, digits)
    }
    fn validate_ve(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 10 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        "VEJGPC".contains(chars[0]) && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── VN MST ──
    // Format: 10 or 13 digits, weights (31,29,23,19,17,13,7,5,3), check = 10-(sum%11) (stdnum.vn.mst)
    fn generate_vn(&self, rng: &mut impl Rng) -> String {
        let weights: [u32; 9] = [31, 29, 23, 19, 17, 13, 7, 5, 3];
        let has_branch = rng.gen_bool(0.3);
        loop {
            let base: Vec<u8> = (0..9)
                .map(|i| {
                    if i == 0 {
                        rng.gen_range(1..=9u8)
                    } else {
                        rng.gen_range(0..=9u8)
                    }
                })
                .collect();
            let total: u32 = base
                .iter()
                .zip(weights.iter())
                .map(|(&d, &w)| d as u32 * w)
                .sum();
            let check = 10 - (total % 11);
            if check > 9 {
                continue;
            }
            let mut result: String = base.iter().map(|d| (b'0' + d) as char).collect();
            result.push((b'0' + check as u8) as char);
            if has_branch {
                let branch: u16 = rng.gen_range(1..=999);
                result.push_str(&format!("{:03}", branch));
            }
            return result;
        }
    }
    fn validate_vn(&self, code: &str) -> bool {
        let c: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if c.len() != 10 && c.len() != 13 {
            return false;
        }
        let digits: Vec<u8> = c.bytes().map(|b| b - b'0').collect();
        let weights: [u32; 9] = [31, 29, 23, 19, 17, 13, 7, 5, 3];
        let total: u32 = digits[..9]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let check = 10 - (total % 11);
        if check > 9 {
            return false;
        }
        digits[9] == check as u8
    }
}

/// ISO 7064 mod 11,10 check digit calculation.
fn iso7064_mod11_10(digits: &[u8]) -> u8 {
    let mut product = 10u32;
    for &d in digits {
        let sum = (d as u32 + product) % 10;
        let sum = if sum == 0 { 10 } else { sum };
        product = (sum * 2) % 11;
    }
    let check = (11 - product) % 10;
    check as u8
}

/// Luhn check digit for a sequence of digits (appends the check digit).
fn luhn_check_digit(digits: &[u8]) -> u8 {
    let mut sum: u32 = 0;
    for (i, &d) in digits.iter().rev().enumerate() {
        let mut val = d as u32;
        if i % 2 == 0 {
            val *= 2;
            if val > 9 {
                val -= 9;
            }
        }
        sum += val;
    }
    ((10 - (sum % 10)) % 10) as u8
}

/// Luhn validation for a complete number including check digit.
fn luhn_validate(digits: &[u8]) -> bool {
    let mut sum: u32 = 0;
    for (i, &d) in digits.iter().rev().enumerate() {
        let mut val = d as u32;
        if i % 2 == 1 {
            val *= 2;
            if val > 9 {
                val -= 9;
            }
        }
        sum += val;
    }
    sum.is_multiple_of(10)
}

/// Convert a byte to CN USCI character value index.
fn cn_usci_char_to_val(b: u8) -> u8 {
    CN_USCI_CHARS.iter().position(|&c| c == b).unwrap_or(0) as u8
}

/// MX RFC check digit calculation.
fn mx_rfc_check_digit(base: &str) -> char {
    let base = base.to_uppercase();
    // Pad to 12 characters for companies (add space at start)
    let padded = if base.len() == 11 {
        format!(" {}", base)
    } else {
        base.to_string()
    };
    let mut sum: u32 = 0;
    for (i, c) in padded.chars().enumerate() {
        let val = MX_RFC_CHARS
            .iter()
            .position(|&rc| rc == c as u8)
            .unwrap_or(0) as u32;
        sum += val * (13 - i as u32);
    }
    let remainder = sum % 11;
    if remainder == 0 {
        '0'
    } else {
        let check = 11 - remainder;
        if check == 10 {
            'A'
        } else {
            (b'0' + check as u8) as char
        }
    }
}
