use rand::Rng;
#[cfg(feature = "json")]
use serde::Serialize;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct DriverLicenseResult {
    pub country_code: String,
    pub country_name: String,
    pub name: String,
    pub code: String,
    pub state: Option<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GenOptions {
    pub country: Option<String>,
    pub state: Option<String>,
}

/// Valid Indian state/UT codes used on driving licences.
static IN_STATES: &[&str] = &[
    "AN", "AP", "AR", "AS", "BR", "CG", "CH", "DD", "DL", "GA", "GJ", "HP", "HR", "JH", "JK", "KA",
    "KL", "LA", "LD", "MH", "ML", "MN", "MP", "MZ", "NL", "OD", "PB", "PY", "RJ", "SK", "TN", "TS",
    "TR", "UK", "UP", "WB",
];

/// US state codes for driver's licenses.
static US_STATES: &[&str] = &[
    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN", "IA", "KS",
    "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY",
    "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV",
    "WI", "WY", "DC",
];

/// Japanese prefecture codes (01-47).
static JP_PREFECTURES: &[&str] = &[
    "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15", "16",
    "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32",
    "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47",
];

/// Australian state codes.
static AU_STATES: &[&str] = &["NSW", "VIC", "QLD", "WA", "SA", "TAS", "ACT", "NT"];

/// Canadian province codes.
static CA_PROVINCES: &[&str] = &[
    "AB", "BC", "MB", "NB", "NL", "NS", "NT", "NU", "ON", "PE", "QC", "SK", "YT",
];

/// German Führerschein issuing authority prefixes (simplified).
static DE_PREFIXES: &[&str] = &[
    "B", "M", "K", "D", "F", "S", "H", "N", "HH", "HB", "DO", "E", "DD", "L", "BN",
];

/// Spanish DNI check letter table (mod-23).
static ES_DNI_LETTERS: &[u8] = b"TRWAGMYFPDXBNJZSQVHLCKE";

/// South Korean region codes for driver's licenses.
static KR_REGIONS: &[&str] = &[
    "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26",
];

/// Singapore NRIC check letter weights.
static SG_NRIC_WEIGHTS: &[u32] = &[2, 7, 6, 5, 4, 3, 2];

static SPECIFIC_COUNTRIES: &[(&str, &str)] = &[
    // ── Africa / Middle East ──
    ("AE", "Driving Licence"),
    ("BH", "Driving Licence"),
    ("DZ", "Permis de Conduire"),
    ("EG", "Driving Licence"),
    ("ET", "Driving Licence"),
    ("GH", "Driver's Licence"),
    ("IL", "Driving Licence"),
    ("KE", "Driving Licence"),
    ("KW", "Driving Licence"),
    ("MA", "Permis de Conduire"),
    ("NG", "Driver's Licence"),
    ("OM", "Driving Licence"),
    ("QA", "Driving Licence"),
    ("SA", "Driving Licence"),
    ("TN", "Permis de Conduire"),
    ("TZ", "Driving Licence"),
    ("ZA", "Driver's Licence"),
    // ── Americas ──
    ("AR", "Licencia de Conducir"),
    ("BR", "CNH"),
    ("CA", "Driver's Licence"),
    ("CL", "Licencia de Conducir"),
    ("CO", "Licencia de Conducción"),
    ("EC", "Licencia de Conducir"),
    ("MX", "Licencia de Conducir"),
    ("PE", "Licencia de Conducir"),
    ("US", "Driver's License"),
    ("UY", "Libreta de Conducir"),
    ("VE", "Licencia de Conducir"),
    // ── Asia-Pacific ──
    ("AU", "Driver Licence"),
    ("BD", "Driving Licence"),
    ("CN", "驾驶证"),
    ("HK", "Driving Licence"),
    ("ID", "SIM"),
    ("IN", "Driving Licence"),
    ("JP", "運転免許証"),
    ("KR", "운전면허증"),
    ("LK", "Driving Licence"),
    ("MY", "Driving Licence"),
    ("NP", "Driving Licence"),
    ("NZ", "Driver Licence"),
    ("PH", "Driver's License"),
    ("PK", "Driving Licence"),
    ("SG", "Driving Licence"),
    ("TH", "Driving Licence"),
    ("TW", "Driving Licence"),
    ("VN", "Driving Licence"),
    // ── Europe ──
    ("AT", "Führerschein"),
    ("BE", "Rijbewijs"),
    ("BG", "Driving Licence"),
    ("CH", "Führerausweis"),
    ("CZ", "Řidičský průkaz"),
    ("DE", "Führerschein"),
    ("DK", "Kørekort"),
    ("EE", "Juhiluba"),
    ("ES", "Permiso de Conducir"),
    ("FI", "Ajokortti"),
    ("FR", "Permis de Conduire"),
    ("GB", "Driving Licence"),
    ("GR", "Driving Licence"),
    ("HR", "Vozačka dozvola"),
    ("HU", "Vezetői engedély"),
    ("IE", "Driving Licence"),
    ("IS", "Ökuskírteini"),
    ("IT", "Patente"),
    ("LT", "Vairuotojo pažymėjimas"),
    ("LU", "Permis de Conduire"),
    ("LV", "Vadītāja apliecība"),
    ("MT", "Driving Licence"),
    ("NL", "Rijbewijs"),
    ("NO", "Førerkort"),
    ("PL", "Prawo Jazdy"),
    ("PT", "Carta de Condução"),
    ("RO", "Permis de Conducere"),
    ("RS", "Vozačka dozvola"),
    ("SE", "Körkort"),
    ("SI", "Vozniško dovoljenje"),
    ("SK", "Vodičský preukaz"),
    ("TR", "Sürücü Belgesi"),
    ("UA", "Driving Licence"),
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

    pub fn generate(&self, opts: &GenOptions, rng: &mut impl Rng) -> Option<DriverLicenseResult> {
        let country = opts
            .country
            .as_deref()
            .unwrap_or_else(|| {
                let countries = self.list_countries();
                countries[rng.gen_range(0..countries.len())].0
            })
            .to_uppercase();

        if let Some((name, result)) = match country.as_str() {
            // ── Existing specific implementations ──
            "IN" => Some(("Driving Licence", self.generate_in(opts, rng))),
            "US" => Some(("Driver's License", self.generate_us(opts, rng))),
            "GB" => Some(("Driving Licence", self.generate_gb(rng))),
            "DE" => Some(("Führerschein", self.generate_de(rng))),
            "FR" => Some(("Permis de Conduire", self.generate_fr(rng))),
            "BR" => Some(("CNH", self.generate_br(rng))),
            "AU" => Some(("Driver Licence", self.generate_au(opts, rng))),
            "CA" => Some(("Driver's Licence", self.generate_ca(opts, rng))),
            "JP" => Some(("運転免許証", self.generate_jp(rng))),
            "CN" => Some(("驾驶证", self.generate_cn(rng))),
            "IT" => Some(("Patente", self.generate_it(rng))),
            "ES" => Some(("Permiso de Conducir", self.generate_es(rng))),
            "NL" => Some(("Rijbewijs", self.generate_nl(rng))),
            "SE" => Some(("Körkort", self.generate_se(rng))),
            "KR" => Some(("운전면허증", self.generate_kr(rng))),
            "SG" => Some(("Driving Licence", self.generate_sg(rng))),
            "ZA" => Some(("Driver's Licence", self.generate_za(rng))),
            "MX" => Some(("Licencia de Conducir", self.generate_mx(rng))),
            // ── Europe (new) ──
            "AT" => Some(("Führerschein", self.generate_at(rng))),
            "BE" => Some(("Rijbewijs", self.generate_be(rng))),
            "BG" => Some(("Driving Licence", self.generate_bg(rng))),
            "CH" => Some(("Führerausweis", self.generate_ch(rng))),
            "CZ" => Some(("Řidičský průkaz", self.generate_cz(rng))),
            "DK" => Some(("Kørekort", self.generate_dk(rng))),
            "EE" => Some(("Juhiluba", self.generate_ee(rng))),
            "FI" => Some(("Ajokortti", self.generate_fi(rng))),
            "GR" => Some(("Driving Licence", self.generate_gr(rng))),
            "HR" => Some(("Vozačka dozvola", self.generate_hr(rng))),
            "HU" => Some(("Vezetői engedély", self.generate_hu(rng))),
            "IE" => Some(("Driving Licence", self.generate_ie(rng))),
            "IS" => Some(("Ökuskírteini", self.generate_is(rng))),
            "LT" => Some(("Vairuotojo pažymėjimas", self.generate_lt(rng))),
            "LU" => Some(("Permis de Conduire", self.generate_lu(rng))),
            "LV" => Some(("Vadītāja apliecība", self.generate_lv(rng))),
            "MT" => Some(("Driving Licence", self.generate_mt(rng))),
            "NO" => Some(("Førerkort", self.generate_no(rng))),
            "PL" => Some(("Prawo Jazdy", self.generate_pl(rng))),
            "PT" => Some(("Carta de Condução", self.generate_pt(rng))),
            "RO" => Some(("Permis de Conducere", self.generate_ro(rng))),
            "RS" => Some(("Vozačka dozvola", self.generate_rs(rng))),
            "SI" => Some(("Vozniško dovoljenje", self.generate_si(rng))),
            "SK" => Some(("Vodičský preukaz", self.generate_sk(rng))),
            "TR" => Some(("Sürücü Belgesi", self.generate_tr(rng))),
            "UA" => Some(("Driving Licence", self.generate_ua(rng))),
            // ── Americas (new) ──
            "AR" => Some(("Licencia de Conducir", self.generate_ar(rng))),
            "CL" => Some(("Licencia de Conducir", self.generate_cl(rng))),
            "CO" => Some(("Licencia de Conducción", self.generate_co(rng))),
            "EC" => Some(("Licencia de Conducir", self.generate_ec(rng))),
            "PE" => Some(("Licencia de Conducir", self.generate_pe(rng))),
            "UY" => Some(("Libreta de Conducir", self.generate_uy(rng))),
            "VE" => Some(("Licencia de Conducir", self.generate_ve(rng))),
            // ── Asia-Pacific (new) ──
            "BD" => Some(("Driving Licence", self.generate_bd(rng))),
            "HK" => Some(("Driving Licence", self.generate_hk(rng))),
            "ID" => Some(("SIM", self.generate_id(rng))),
            "MY" => Some(("Driving Licence", self.generate_my(rng))),
            "NP" => Some(("Driving Licence", self.generate_np(rng))),
            "NZ" => Some(("Driver Licence", self.generate_nz(rng))),
            "PH" => Some(("Driver's License", self.generate_ph(rng))),
            "PK" => Some(("Driving Licence", self.generate_pk(rng))),
            "LK" => Some(("Driving Licence", self.generate_lk(rng))),
            "TH" => Some(("Driving Licence", self.generate_th(rng))),
            "TW" => Some(("Driving Licence", self.generate_tw(rng))),
            "VN" => Some(("Driving Licence", self.generate_vn(rng))),
            // ── Africa / Middle East (new) ──
            "AE" => Some(("Driving Licence", self.generate_ae(rng))),
            "BH" => Some(("Driving Licence", self.generate_bh(rng))),
            "DZ" => Some(("Permis de Conduire", self.generate_dz(rng))),
            "EG" => Some(("Driving Licence", self.generate_eg(rng))),
            "ET" => Some(("Driving Licence", self.generate_et(rng))),
            "GH" => Some(("Driver's Licence", self.generate_gh(rng))),
            "IL" => Some(("Driving Licence", self.generate_il(rng))),
            "KE" => Some(("Driving Licence", self.generate_ke(rng))),
            "KW" => Some(("Driving Licence", self.generate_kw(rng))),
            "MA" => Some(("Permis de Conduire", self.generate_ma(rng))),
            "NG" => Some(("Driver's Licence", self.generate_ng(rng))),
            "OM" => Some(("Driving Licence", self.generate_om(rng))),
            "QA" => Some(("Driving Licence", self.generate_qa(rng))),
            "SA" => Some(("Driving Licence", self.generate_sa(rng))),
            "TN" => Some(("Permis de Conduire", self.generate_tn(rng))),
            "TZ" => Some(("Driving Licence", self.generate_tz(rng))),
            _ => None,
        } {
            let country_name = crate::countries::get_country_name(&country).unwrap_or("Unknown");
            return Some(DriverLicenseResult {
                country_code: country,
                country_name: country_name.to_string(),
                name: name.to_string(),
                code: result.0,
                state: result.1,
                valid: true,
            });
        }

        None
    }

    pub fn validate(&self, country: &str, code: &str) -> bool {
        match country.to_uppercase().as_str() {
            // ── Existing specific implementations ──
            "IN" => self.validate_in(code),
            "US" => self.validate_us(code),
            "GB" => self.validate_gb_code(code),
            "DE" => self.validate_de_code(code),
            "FR" => self.validate_fr_code(code),
            "BR" => self.validate_br_code(code),
            "AU" => self.validate_au(code),
            "CA" => self.validate_ca(code),
            "JP" => self.validate_jp(code),
            "CN" => self.validate_cn(code),
            "IT" => self.validate_it(code),
            "ES" => self.validate_es(code),
            "NL" => self.validate_nl(code),
            "SE" => self.validate_se(code),
            "KR" => self.validate_kr(code),
            "SG" => self.validate_sg(code),
            "ZA" => self.validate_za(code),
            "MX" => self.validate_mx(code),
            // ── Europe (new) ──
            "AT" => self.validate_at(code),
            "BE" => self.validate_be(code),
            "BG" => self.validate_bg(code),
            "CH" => self.validate_ch(code),
            "CZ" => self.validate_cz(code),
            "DK" => self.validate_dk(code),
            "EE" => self.validate_ee(code),
            "FI" => self.validate_fi(code),
            "GR" => self.validate_gr(code),
            "HR" => self.validate_hr(code),
            "HU" => self.validate_hu(code),
            "IE" => self.validate_ie(code),
            "IS" => self.validate_is(code),
            "LT" => self.validate_lt(code),
            "LU" => self.validate_lu(code),
            "LV" => self.validate_lv(code),
            "MT" => self.validate_mt(code),
            "NO" => self.validate_no(code),
            "PL" => self.validate_pl(code),
            "PT" => self.validate_pt(code),
            "RO" => self.validate_ro(code),
            "RS" => self.validate_rs(code),
            "SI" => self.validate_si(code),
            "SK" => self.validate_sk(code),
            "TR" => self.validate_tr(code),
            "UA" => self.validate_ua(code),
            // ── Americas (new) ──
            "AR" => self.validate_ar(code),
            "CL" => self.validate_cl(code),
            "CO" => self.validate_co(code),
            "EC" => self.validate_ec(code),
            "PE" => self.validate_pe(code),
            "UY" => self.validate_uy(code),
            "VE" => self.validate_ve(code),
            // ── Asia-Pacific (new) ──
            "BD" => self.validate_bd(code),
            "HK" => self.validate_hk(code),
            "ID" => self.validate_id(code),
            "MY" => self.validate_my(code),
            "NP" => self.validate_np(code),
            "NZ" => self.validate_nz(code),
            "PH" => self.validate_ph(code),
            "PK" => self.validate_pk(code),
            "LK" => self.validate_lk(code),
            "TH" => self.validate_th(code),
            "TW" => self.validate_tw(code),
            "VN" => self.validate_vn(code),
            // ── Africa / Middle East (new) ──
            "AE" => self.validate_ae(code),
            "BH" => self.validate_bh(code),
            "DZ" => self.validate_dz(code),
            "EG" => self.validate_eg(code),
            "ET" => self.validate_et(code),
            "GH" => self.validate_gh(code),
            "IL" => self.validate_il(code),
            "KE" => self.validate_ke(code),
            "KW" => self.validate_kw(code),
            "MA" => self.validate_ma(code),
            "NG" => self.validate_ng(code),
            "OM" => self.validate_om(code),
            "QA" => self.validate_qa(code),
            "SA" => self.validate_sa(code),
            "TN" => self.validate_tn(code),
            "TZ" => self.validate_tz(code),
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

    // ══════════════════════════════════════════════════════════════════════
    // Existing specific implementations (18 countries)
    // ══════════════════════════════════════════════════════════════════════

    // ── India ──
    // Format: {STATE}{RTO}{YEAR}{SERIAL} = 15 chars, e.g. MH0220190000001
    fn generate_in(&self, opts: &GenOptions, rng: &mut impl Rng) -> (String, Option<String>) {
        let state = opts
            .state
            .as_deref()
            .unwrap_or_else(|| IN_STATES[rng.gen_range(0..IN_STATES.len())]);
        let rto: u8 = rng.gen_range(1..=99);
        let year: u16 = rng.gen_range(1990..=2025);
        let serial: u32 = rng.gen_range(1..=9999999);
        (
            format!("{}{:02}{}{:07}", state, rto, year, serial),
            Some(state.to_string()),
        )
    }

    fn validate_in(&self, code: &str) -> bool {
        let clean: String = code
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '-')
            .collect();
        if clean.len() != 15 {
            return false;
        }
        let state = &clean[0..2];
        if !IN_STATES.contains(&state) {
            return false;
        }
        let rto = &clean[2..4];
        if !rto.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let rto_num: u8 = rto.parse().unwrap_or(0);
        if !(1..=99).contains(&rto_num) {
            return false;
        }
        clean[4..8].chars().all(|c| c.is_ascii_digit())
            && clean[8..15].chars().all(|c| c.is_ascii_digit())
    }

    // ── United States ──
    // Format: 1 alpha + 12 digits = 13 chars (simplified; real formats vary by state)
    fn generate_us(&self, opts: &GenOptions, rng: &mut impl Rng) -> (String, Option<String>) {
        let state = opts
            .state
            .as_deref()
            .unwrap_or_else(|| US_STATES[rng.gen_range(0..US_STATES.len())]);
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..12)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", letter, digits), Some(state.to_string()))
    }

    fn validate_us(&self, code: &str) -> bool {
        let clean = code.trim();
        if clean.len() != 13 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── United Kingdom ──
    // DVLA format: 16 chars alphanumeric. Surname (5) + decade of birth + month/gender + day + year digit + 2 check + 2 alpha
    fn generate_gb(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        // DVLA format: SSSSS DDDDD DDDDD A  (16 chars)
        // Surname portion (5 chars, padded with 9s)
        let surname_len = rng.gen_range(2..=5usize);
        let mut surname: String = (0..surname_len)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        while surname.len() < 5 {
            surname.push('9');
        }
        let decade = rng.gen_range(5..=9u8); // 1950s-1990s
        let month = rng.gen_range(1..=12u8);
        let day = rng.gen_range(1..=28u8);
        let year_digit = rng.gen_range(0..=9u8);
        let check: String = (0..3)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        let suffix: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        // 5 + 1 + 2 + 2 + 1 + 3 + 2 = 16
        let code = format!(
            "{}{}{:02}{:02}{}{}{}",
            surname, decade, month, day, year_digit, check, suffix
        );
        (code, None)
    }

    fn validate_gb_code(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 16 && clean.chars().all(|c| c.is_ascii_alphanumeric())
    }

    // ── Germany ──
    // Format: 11 chars alphanumeric (issuing authority prefix + digits)
    fn generate_de(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let prefix = DE_PREFIXES[rng.gen_range(0..DE_PREFIXES.len())];
        let remaining = 11 - prefix.len();
        let digits: String = (0..remaining)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", prefix, digits), None)
    }

    fn validate_de_code(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 11 && clean.chars().all(|c| c.is_ascii_alphanumeric())
    }

    // ── France ──
    // Format: 12 chars alphanumeric (2 alpha + 10 digits)
    fn generate_fr(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let prefix: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..10)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", prefix, digits), None)
    }

    fn validate_fr_code(&self, code: &str) -> bool {
        let clean = code.trim();
        if clean.len() != 12 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Brazil ──
    // CNH: 11 digits (9 base + 2 check digits)
    fn generate_br(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let base: Vec<u8> = (0..9).map(|_| rng.gen_range(0..=9u8)).collect();
        // First check digit
        let sum1: u32 = base
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (9 - i as u32))
            .sum();
        let c1 = (sum1 % 11) as u8;
        let c1 = if c1 >= 10 { 0 } else { c1 };
        // Second check digit
        let sum2: u32 = base
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (1 + i as u32))
            .sum();
        let c2 = (sum2 % 11) as u8;
        let c2 = if c2 >= 10 { 0 } else { c2 };
        let code: String = base
            .iter()
            .chain(&[c1, c2])
            .map(|d| (b'0' + d) as char)
            .collect();
        (code, None)
    }

    fn validate_br_code(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 11 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        // Check digit 1
        let sum1: u32 = digits[..9]
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (9 - i as u32))
            .sum();
        let c1 = (sum1 % 11) as u8;
        let c1 = if c1 >= 10 { 0 } else { c1 };
        if digits[9] != c1 {
            return false;
        }
        // Check digit 2
        let sum2: u32 = digits[..9]
            .iter()
            .enumerate()
            .map(|(i, &d)| d as u32 * (1 + i as u32))
            .sum();
        let c2 = (sum2 % 11) as u8;
        let c2 = if c2 >= 10 { 0 } else { c2 };
        digits[10] == c2
    }

    // ── Australia ──
    // Format: state-based, typically 8-10 alphanumeric chars
    fn generate_au(&self, opts: &GenOptions, rng: &mut impl Rng) -> (String, Option<String>) {
        let state = opts
            .state
            .as_deref()
            .unwrap_or_else(|| AU_STATES[rng.gen_range(0..AU_STATES.len())]);
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (
            format!("{}{}", digits, rng.gen_range(100..=999)),
            Some(state.to_string()),
        )
    }

    fn validate_au(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() >= 8 && clean.len() <= 10 && clean.chars().all(|c| c.is_ascii_alphanumeric())
    }

    // ── Canada ──
    // Format: province-based, letter + digits, typically 13 chars
    fn generate_ca(&self, opts: &GenOptions, rng: &mut impl Rng) -> (String, Option<String>) {
        let province = opts
            .state
            .as_deref()
            .unwrap_or_else(|| CA_PROVINCES[rng.gen_range(0..CA_PROVINCES.len())]);
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..12)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", letter, digits), Some(province.to_string()))
    }

    fn validate_ca(&self, code: &str) -> bool {
        let clean = code.trim();
        if clean.len() != 13 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Japan ──
    // Format: 12 digits (prefecture 2 + year 2 + serial 6 + check 2)
    fn generate_jp(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let pref = JP_PREFECTURES[rng.gen_range(0..JP_PREFECTURES.len())];
        let year: u8 = rng.gen_range(1..=35);
        let serial: u32 = rng.gen_range(0..=999999);
        let check: u8 = rng.gen_range(0..=99);
        (
            format!("{}{:02}{:06}{:02}", pref, year, serial, check),
            None,
        )
    }

    fn validate_jp(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 12 && clean.chars().all(|c| c.is_ascii_digit())
    }

    // ── China ──
    // Format: 12 digits (region 6 + serial 6)
    fn generate_cn(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let region: u32 = rng.gen_range(110000..=820000);
        let serial: u32 = rng.gen_range(0..=999999);
        (format!("{:06}{:06}", region, serial), None)
    }

    fn validate_cn(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 12 && clean.chars().all(|c| c.is_ascii_digit())
    }

    // ── Italy ──
    // Format: 10 chars (2 alpha + 7 digits + 1 alpha)
    fn generate_it(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let prefix: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        let suffix = (b'A' + rng.gen_range(0..26u8)) as char;
        (format!("{}{}{}", prefix, digits, suffix), None)
    }

    fn validate_it(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 10 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..9].iter().all(|c| c.is_ascii_digit())
            && chars[9].is_ascii_uppercase()
    }

    // ── Spain ──
    // Format: 8 digits + check letter (same as DNI, mod-23)
    fn generate_es(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let num: u32 = rng.gen_range(1_000_000..=99_999_999);
        let check = ES_DNI_LETTERS[(num % 23) as usize] as char;
        (format!("{:08}{}", num, check), None)
    }

    fn validate_es(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        if !chars[..8].iter().all(|c| c.is_ascii_digit()) || !chars[8].is_ascii_uppercase() {
            return false;
        }
        let num: u32 = match clean[..8].parse() {
            Ok(n) => n,
            Err(_) => return false,
        };
        let expected = ES_DNI_LETTERS[(num % 23) as usize] as char;
        chars[8] == expected
    }

    // ── Netherlands ──
    // Format: 10 digits
    fn generate_nl(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_nl(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Sweden ──
    // Format: 10 digits (personnummer-based: YYMMDDXXXX)
    fn generate_se(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let year: u8 = rng.gen_range(40..=99);
        let month: u8 = rng.gen_range(1..=12);
        let day: u8 = rng.gen_range(1..=28);
        let serial: u16 = rng.gen_range(1..=999);
        let base: Vec<u8> = format!("{:02}{:02}{:02}{:03}", year, month, day, serial)
            .bytes()
            .map(|b| b - b'0')
            .collect();
        let check = luhn_check_digit(&base);
        (
            format!("{:02}{:02}{:02}{:03}{}", year, month, day, serial, check),
            None,
        )
    }

    fn validate_se(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 10 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        luhn_validate(&digits)
    }

    // ── South Korea ──
    // Format: 12 digits (region 2 + year 2 + serial 6 + check 2)
    fn generate_kr(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let region = KR_REGIONS[rng.gen_range(0..KR_REGIONS.len())];
        let year: u8 = rng.gen_range(1..=35);
        let serial: u32 = rng.gen_range(0..=999999);
        let check: u8 = rng.gen_range(0..=99);
        (
            format!("{}{:02}{:06}{:02}", region, year, serial, check),
            None,
        )
    }

    fn validate_kr(&self, code: &str) -> bool {
        let clean = code.trim();
        if clean.len() != 12 || !clean.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let region = &clean[0..2];
        KR_REGIONS.contains(&region)
    }

    // ── Singapore ──
    // Format: 9 chars (S/T + 7 digits + check letter, NRIC format)
    fn generate_sg(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let prefix = if rng.gen_bool(0.5) { 'S' } else { 'T' };
        let digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9u8)).collect();
        let mut sum: u32 = digits
            .iter()
            .zip(SG_NRIC_WEIGHTS.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        if prefix == 'T' {
            sum += 4;
        }
        let check_letters = b"JZIHGFEDCBA";
        let check = check_letters[(sum % 11) as usize] as char;
        let digit_str: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        (format!("{}{}{}", prefix, digit_str, check), None)
    }

    fn validate_sg(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        let prefix = chars[0];
        if prefix != 'S' && prefix != 'T' {
            return false;
        }
        if !chars[1..8].iter().all(|c| c.is_ascii_digit()) || !chars[8].is_ascii_uppercase() {
            return false;
        }
        let digits: Vec<u8> = clean[1..8].bytes().map(|b| b - b'0').collect();
        let mut sum: u32 = digits
            .iter()
            .zip(SG_NRIC_WEIGHTS.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        if prefix == 'T' {
            sum += 4;
        }
        let check_letters = b"JZIHGFEDCBA";
        let expected = check_letters[(sum % 11) as usize] as char;
        chars[8] == expected
    }

    // ── South Africa ──
    // Format: 13 digits (ID number format: YYMMDDGSSSCAZ with Luhn)
    fn generate_za(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let year: u8 = rng.gen_range(40..=99);
        let month: u8 = rng.gen_range(1..=12);
        let day: u8 = rng.gen_range(1..=28);
        let gender: u16 = rng.gen_range(0..=9999);
        let citizen: u8 = rng.gen_range(0..=1u8);
        let race: u8 = 8; // Usually 8
        let base: Vec<u8> = format!(
            "{:02}{:02}{:02}{:04}{}{}",
            year, month, day, gender, citizen, race
        )
        .bytes()
        .map(|b| b - b'0')
        .collect();
        let check = luhn_check_digit(&base);
        (
            format!(
                "{:02}{:02}{:02}{:04}{}{}{}",
                year, month, day, gender, citizen, race, check
            ),
            None,
        )
    }

    fn validate_za(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() != 13 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        luhn_validate(&digits)
    }

    // ── Mexico ──
    // Format: 12 chars alphanumeric (CURP-derived)
    fn generate_mx(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let prefix: String = (0..4)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let year: u8 = rng.gen_range(50..=99);
        let month: u8 = rng.gen_range(1..=12);
        let day: u8 = rng.gen_range(1..=28);
        let suffix: String = (0..2)
            .map(|_| {
                let idx = rng.gen_range(0..36u8);
                if idx < 10 {
                    (b'0' + idx) as char
                } else {
                    (b'A' + idx - 10) as char
                }
            })
            .collect();
        (
            format!("{}{:02}{:02}{:02}{}", prefix, year, month, day, suffix),
            None,
        )
    }

    fn validate_mx(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 12 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..4].iter().all(|c| c.is_ascii_uppercase())
            && chars[4..10].iter().all(|c| c.is_ascii_digit())
            && chars[10..12].iter().all(|c| c.is_ascii_alphanumeric())
    }

    // ══════════════════════════════════════════════════════════════════════
    // New specific implementations — Europe
    // ══════════════════════════════════════════════════════════════════════

    // ── Austria ──
    // Format: 8 digits
    fn generate_at(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_at(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 8 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Belgium ──
    // Format: 10 digits
    fn generate_be(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_be(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Bulgaria ──
    // Format: 9 digits
    fn generate_bg(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_bg(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 9 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Switzerland ──
    // Format: 2 alpha + 10 digits = 12 chars
    fn generate_ch(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..10)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_ch(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 12 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Czech Republic ──
    // Format: 2 alpha + 8 digits = 10 chars
    fn generate_cz(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..8)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_cz(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 10 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Denmark ──
    // Format: 8 digits
    fn generate_dk(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_dk(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 8 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Estonia ──
    // Format: 2 alpha + 6 digits = 8 chars
    fn generate_ee(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_ee(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Finland ──
    // Format: 2 alpha + 8 digits = 10 chars
    fn generate_fi(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..8)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_fi(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 10 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Greece ──
    // Format: 9 digits
    fn generate_gr(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_gr(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 9 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Croatia ──
    // Format: 9 digits
    fn generate_hr(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_hr(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 9 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Hungary ──
    // Format: 2 alpha + 6 digits = 8 chars
    fn generate_hu(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_hu(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Ireland ──
    // Format: 9 digits
    fn generate_ie(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_ie(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 9 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Iceland ──
    // Format: 2 alpha + 7 digits = 9 chars
    fn generate_is(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_is(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Lithuania ──
    // Format: 8 digits
    fn generate_lt(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_lt(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 8 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Luxembourg ──
    // Format: 6 digits
    fn generate_lu(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..6)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_lu(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 6 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Latvia ──
    // Format: 2 alpha + 6 digits = 8 chars
    fn generate_lv(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_lv(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Malta ──
    // Format: 9 digits
    fn generate_mt(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_mt(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 9 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Norway ──
    // Format: 11 digits
    fn generate_no(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..11)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_no(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 11 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Poland ──
    // Format: 12 digits (NNNNNNNNNNN)
    fn generate_pl(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..12)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_pl(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 12 && !clean.starts_with('0')
    }

    // ── Portugal ──
    // Format: 2 alpha + 7 digits = 9 chars
    fn generate_pt(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_pt(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Romania ──
    // Format: 2 alpha + 8 digits = 10 chars
    fn generate_ro(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..8)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_ro(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 10 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Serbia ──
    // Format: 9 digits
    fn generate_rs(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_rs(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 9 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Slovenia ──
    // Format: 9 digits
    fn generate_si(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_si(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 9 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Slovakia ──
    // Format: 2 alpha + 6 digits = 8 chars
    fn generate_sk(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_sk(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Turkey ──
    // Format: 10 digits (TC Kimlik number)
    fn generate_tr(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_tr(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Ukraine ──
    // Format: 3 alpha + 6 digits = 9 chars
    fn generate_ua(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..3)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_ua(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..3].iter().all(|c| c.is_ascii_uppercase())
            && chars[3..].iter().all(|c| c.is_ascii_digit())
    }

    // ══════════════════════════════════════════════════════════════════════
    // New specific implementations — Americas
    // ══════════════════════════════════════════════════════════════════════

    // ── Argentina ──
    // Format: 8 digits
    fn generate_ar(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_ar(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 8 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Chile ──
    // Format: 9 digits (RUT format)
    fn generate_cl(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_cl(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 9 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Colombia ──
    // Format: 10 digits
    fn generate_co(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_co(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Ecuador ──
    // Format: 10 digits
    fn generate_ec(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_ec(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Peru ──
    // Format: 1 alpha + 8 digits = 9 chars
    fn generate_pe(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..8)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", letter, digits), None)
    }

    fn validate_pe(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_uppercase() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Uruguay ──
    // Format: 8 digits
    fn generate_uy(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_uy(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 8 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Venezuela ──
    // Format: 10 digits
    fn generate_ve(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_ve(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ══════════════════════════════════════════════════════════════════════
    // New specific implementations — Asia-Pacific
    // ══════════════════════════════════════════════════════════════════════

    // ── Bangladesh ──
    // Format: 10 digits
    fn generate_bd(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_bd(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Hong Kong ──
    // Format: 2 alpha + 6 digits = 8 chars
    fn generate_hk(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_hk(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Indonesia ──
    // Format: 12 digits (SIM number)
    fn generate_id(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..12)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_id(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 12 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Malaysia ──
    // Format: 12 digits (MyKad number)
    fn generate_my(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..12)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_my(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 12 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Nepal ──
    // Format: 10 digits
    fn generate_np(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_np(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── New Zealand ──
    // Format: 2 alpha + 6 digits = 8 chars
    fn generate_nz(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_nz(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_uppercase())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Philippines ──
    // Format: 1 alpha + 10 digits = 11 chars
    fn generate_ph(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..10)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", letter, digits), None)
    }

    fn validate_ph(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 11 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_uppercase() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Pakistan ──
    // Format: 10 digits
    fn generate_pk(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_pk(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Sri Lanka ──
    // Format: 1 alpha + 7 digits = 8 chars
    fn generate_lk(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", letter, digits), None)
    }

    fn validate_lk(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_uppercase() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Thailand ──
    // Format: 8 digits
    fn generate_th(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_th(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 8 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Taiwan ──
    // Format: 1 alpha + 9 digits = 10 chars (National ID format)
    fn generate_tw(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..9)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", letter, digits), None)
    }

    fn validate_tw(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 10 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_uppercase() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Vietnam ──
    // Format: 12 digits
    fn generate_vn(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..12)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_vn(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 12 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ══════════════════════════════════════════════════════════════════════
    // New specific implementations — Africa / Middle East
    // ══════════════════════════════════════════════════════════════════════

    // ── United Arab Emirates ──
    // Format: 10 digits
    fn generate_ae(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_ae(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Bahrain ──
    // Format: 9 digits
    fn generate_bh(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_bh(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 9 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Algeria ──
    // Format: 10 digits
    fn generate_dz(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_dz(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Egypt ──
    // Format: 10 digits
    fn generate_eg(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_eg(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Ethiopia ──
    // Format: 8 digits
    fn generate_et(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_et(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 8 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Ghana ──
    // Format: 1 alpha + 9 digits = 10 chars
    fn generate_gh(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..9)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", letter, digits), None)
    }

    fn validate_gh(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 10 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_uppercase() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Israel ──
    // Format: 8 digits
    fn generate_il(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_il(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 8 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Kenya ──
    // Format: 8 digits
    fn generate_ke(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_ke(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 8 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Kuwait ──
    // Format: 9 digits
    fn generate_kw(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_kw(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 9 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Morocco ──
    // Format: 10 digits
    fn generate_ma(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_ma(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Nigeria ──
    // Format: 3 alpha + 9 digits = 12 chars
    fn generate_ng(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let alpha: String = (0..3)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..9)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        (format!("{}{}", alpha, digits), None)
    }

    fn validate_ng(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 12 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..3].iter().all(|c| c.is_ascii_uppercase())
            && chars[3..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Oman ──
    // Format: 8 digits
    fn generate_om(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_om(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 8 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Qatar ──
    // Format: 10 digits
    fn generate_qa(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_qa(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Saudi Arabia ──
    // Format: 10 digits
    fn generate_sa(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_sa(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Tunisia ──
    // Format: 8 digits
    fn generate_tn(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_tn(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 8 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }

    // ── Tanzania ──
    // Format: 10 digits
    fn generate_tz(&self, rng: &mut impl Rng) -> (String, Option<String>) {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        (digits, None)
    }

    fn validate_tz(&self, code: &str) -> bool {
        let clean = code.trim();
        clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit()) && !clean.starts_with('0')
    }
}

/// Luhn check digit for a sequence of digits.
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
