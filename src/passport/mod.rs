use rand::Rng;
#[cfg(feature = "json")]
use serde::Serialize;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct PassportResult {
    pub country_code: String,
    pub country_name: String,
    pub name: String,
    pub code: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GenOptions {
    pub country: Option<String>,
}

static SPECIFIC_COUNTRIES: &[(&str, &str)] = &[
    ("AE", "Passport"),
    ("AR", "Pasaporte"),
    ("AT", "Reisepass"),
    ("AU", "Passport"),
    ("BD", "Passport"),
    ("BE", "Paspoort"),
    ("BG", "Passport"),
    ("BH", "Passport"),
    ("BR", "Passaporte"),
    ("CA", "Passport"),
    ("CH", "Reisepass"),
    ("CL", "Pasaporte"),
    ("CN", "Passport"),
    ("CO", "Pasaporte"),
    ("CZ", "Cestovní pas"),
    ("DE", "Reisepass"),
    ("DK", "Pas"),
    ("DZ", "Passeport"),
    ("EC", "Pasaporte"),
    ("EE", "Pass"),
    ("EG", "Passport"),
    ("ES", "Pasaporte"),
    ("ET", "Passport"),
    ("FI", "Passi"),
    ("FR", "Passeport"),
    ("GB", "Passport"),
    ("GH", "Passport"),
    ("GR", "Passport"),
    ("HK", "Passport"),
    ("HR", "Putovnica"),
    ("HU", "Útlevél"),
    ("ID", "Passport"),
    ("IE", "Passport"),
    ("IL", "Passport"),
    ("IN", "Passport"),
    ("IS", "Vegabréf"),
    ("IT", "Passaporto"),
    ("JP", "Passport"),
    ("KE", "Passport"),
    ("KR", "여권"),
    ("KW", "Passport"),
    ("LK", "Passport"),
    ("LT", "Pasas"),
    ("LU", "Passeport"),
    ("LV", "Pase"),
    ("MA", "Passeport"),
    ("MT", "Passaport"),
    ("MX", "Pasaporte"),
    ("MY", "Passport"),
    ("NG", "Passport"),
    ("NL", "Paspoort"),
    ("NO", "Pass"),
    ("NP", "Passport"),
    ("NZ", "Passport"),
    ("OM", "Passport"),
    ("PE", "Pasaporte"),
    ("PH", "Passport"),
    ("PK", "Passport"),
    ("PL", "Paszport"),
    ("PT", "Passaporte"),
    ("QA", "Passport"),
    ("RO", "Pașaport"),
    ("RS", "Pasoš"),
    ("SA", "Passport"),
    ("SE", "Pass"),
    ("SG", "Passport"),
    ("SI", "Potni list"),
    ("SK", "Cestovný pas"),
    ("TH", "Passport"),
    ("TN", "Passeport"),
    ("TR", "Pasaport"),
    ("TW", "Passport"),
    ("TZ", "Passport"),
    ("UA", "Passport"),
    ("US", "Passport"),
    ("UY", "Pasaporte"),
    ("VE", "Pasaporte"),
    ("VN", "Passport"),
    ("ZA", "Passport"),
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

    pub fn generate(&self, opts: &GenOptions, rng: &mut impl Rng) -> Option<PassportResult> {
        let country = opts
            .country
            .as_deref()
            .unwrap_or_else(|| {
                let countries = self.list_countries();
                countries[rng.gen_range(0..countries.len())].0
            })
            .to_uppercase();

        if let Some((name, code)) = match country.as_str() {
            // ── Existing 18 countries ──
            "IN" => Some(("Passport", self.generate_in(rng))),
            "US" => Some(("Passport", self.generate_us(rng))),
            "GB" => Some(("Passport", self.generate_gb(rng))),
            "DE" => Some(("Reisepass", self.generate_de(rng))),
            "FR" => Some(("Passeport", self.generate_fr(rng))),
            "BR" => Some(("Passaporte", self.generate_br(rng))),
            "AU" => Some(("Passport", self.generate_au(rng))),
            "CA" => Some(("Passport", self.generate_ca(rng))),
            "JP" => Some(("Passport", self.generate_jp(rng))),
            "CN" => Some(("Passport", self.generate_cn(rng))),
            "IT" => Some(("Passaporto", self.generate_it(rng))),
            "ES" => Some(("Pasaporte", self.generate_es(rng))),
            "NL" => Some(("Paspoort", self.generate_nl(rng))),
            "SE" => Some(("Pass", self.generate_se(rng))),
            "KR" => Some(("여권", self.generate_kr(rng))),
            "SG" => Some(("Passport", self.generate_sg(rng))),
            "ZA" => Some(("Passport", self.generate_za(rng))),
            "MX" => Some(("Pasaporte", self.generate_mx(rng))),
            // ── Europe (new) ──
            "PL" => Some(("Paszport", self.generate_pl(rng))),
            "RO" => Some(("Pașaport", self.generate_ro(rng))),
            "CZ" => Some(("Cestovní pas", self.generate_cz(rng))),
            "GR" => Some(("Passport", self.generate_gr(rng))),
            "PT" => Some(("Passaporte", self.generate_pt(rng))),
            "BE" => Some(("Paspoort", self.generate_be(rng))),
            "AT" => Some(("Reisepass", self.generate_at(rng))),
            "CH" => Some(("Reisepass", self.generate_ch(rng))),
            "IE" => Some(("Passport", self.generate_ie(rng))),
            "DK" => Some(("Pas", self.generate_dk(rng))),
            "FI" => Some(("Passi", self.generate_fi(rng))),
            "NO" => Some(("Pass", self.generate_no(rng))),
            "BG" => Some(("Passport", self.generate_bg(rng))),
            "SK" => Some(("Cestovný pas", self.generate_sk(rng))),
            "HR" => Some(("Putovnica", self.generate_hr(rng))),
            "SI" => Some(("Potni list", self.generate_si(rng))),
            "HU" => Some(("Útlevél", self.generate_hu(rng))),
            "LT" => Some(("Pasas", self.generate_lt(rng))),
            "LV" => Some(("Pase", self.generate_lv(rng))),
            "EE" => Some(("Pass", self.generate_ee(rng))),
            "MT" => Some(("Passaport", self.generate_mt(rng))),
            "LU" => Some(("Passeport", self.generate_lu(rng))),
            "IS" => Some(("Vegabréf", self.generate_is(rng))),
            "RS" => Some(("Pasoš", self.generate_rs(rng))),
            "TR" => Some(("Pasaport", self.generate_tr(rng))),
            "UA" => Some(("Passport", self.generate_ua(rng))),
            // ── Asia-Pacific (new) ──
            "ID" => Some(("Passport", self.generate_id(rng))),
            "TH" => Some(("Passport", self.generate_th(rng))),
            "MY" => Some(("Passport", self.generate_my(rng))),
            "PH" => Some(("Passport", self.generate_ph(rng))),
            "VN" => Some(("Passport", self.generate_vn(rng))),
            "PK" => Some(("Passport", self.generate_pk(rng))),
            "BD" => Some(("Passport", self.generate_bd(rng))),
            "LK" => Some(("Passport", self.generate_lk(rng))),
            "NP" => Some(("Passport", self.generate_np(rng))),
            "NZ" => Some(("Passport", self.generate_nz(rng))),
            "HK" => Some(("Passport", self.generate_hk(rng))),
            "TW" => Some(("Passport", self.generate_tw(rng))),
            // ── Americas (new) ──
            "AR" => Some(("Pasaporte", self.generate_ar(rng))),
            "CL" => Some(("Pasaporte", self.generate_cl(rng))),
            "CO" => Some(("Pasaporte", self.generate_co(rng))),
            "PE" => Some(("Pasaporte", self.generate_pe(rng))),
            "EC" => Some(("Pasaporte", self.generate_ec(rng))),
            "UY" => Some(("Pasaporte", self.generate_uy(rng))),
            "VE" => Some(("Pasaporte", self.generate_ve(rng))),
            // ── Africa / Middle East (new) ──
            "NG" => Some(("Passport", self.generate_ng(rng))),
            "KE" => Some(("Passport", self.generate_ke(rng))),
            "EG" => Some(("Passport", self.generate_eg(rng))),
            "SA" => Some(("Passport", self.generate_sa(rng))),
            "AE" => Some(("Passport", self.generate_ae(rng))),
            "IL" => Some(("Passport", self.generate_il(rng))),
            "QA" => Some(("Passport", self.generate_qa(rng))),
            "KW" => Some(("Passport", self.generate_kw(rng))),
            "BH" => Some(("Passport", self.generate_bh(rng))),
            "OM" => Some(("Passport", self.generate_om(rng))),
            "GH" => Some(("Passport", self.generate_gh(rng))),
            "TZ" => Some(("Passport", self.generate_tz(rng))),
            "ET" => Some(("Passport", self.generate_et(rng))),
            "MA" => Some(("Passeport", self.generate_ma(rng))),
            "TN" => Some(("Passeport", self.generate_tn(rng))),
            "DZ" => Some(("Passeport", self.generate_dz(rng))),
            _ => None,
        } {
            let country_name = crate::countries::get_country_name(&country).unwrap_or("Unknown");
            return Some(PassportResult {
                country_code: country,
                country_name: country_name.to_string(),
                name: name.to_string(),
                code,
                valid: true,
            });
        }

        None
    }

    pub fn validate(&self, country: &str, code: &str) -> bool {
        match country.to_uppercase().as_str() {
            // ── Existing 18 countries ──
            "IN" => self.validate_in(code),
            "US" => self.validate_us(code),
            "GB" => self.validate_gb(code),
            "DE" => self.validate_de(code),
            "FR" => self.validate_fr(code),
            "BR" => self.validate_br(code),
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
            "PL" => self.validate_pl(code),
            "RO" => self.validate_ro(code),
            "CZ" => self.validate_cz(code),
            "GR" => self.validate_gr(code),
            "PT" => self.validate_pt(code),
            "BE" => self.validate_be(code),
            "AT" => self.validate_at(code),
            "CH" => self.validate_ch(code),
            "IE" => self.validate_ie(code),
            "DK" => self.validate_dk(code),
            "FI" => self.validate_fi(code),
            "NO" => self.validate_no(code),
            "BG" => self.validate_bg(code),
            "SK" => self.validate_sk(code),
            "HR" => self.validate_hr(code),
            "SI" => self.validate_si(code),
            "HU" => self.validate_hu(code),
            "LT" => self.validate_lt(code),
            "LV" => self.validate_lv(code),
            "EE" => self.validate_ee(code),
            "MT" => self.validate_mt(code),
            "LU" => self.validate_lu(code),
            "IS" => self.validate_is(code),
            "RS" => self.validate_rs(code),
            "TR" => self.validate_tr(code),
            "UA" => self.validate_ua(code),
            // ── Asia-Pacific (new) ──
            "ID" => self.validate_id(code),
            "TH" => self.validate_th(code),
            "MY" => self.validate_my(code),
            "PH" => self.validate_ph(code),
            "VN" => self.validate_vn(code),
            "PK" => self.validate_pk(code),
            "BD" => self.validate_bd(code),
            "LK" => self.validate_lk(code),
            "NP" => self.validate_np(code),
            "NZ" => self.validate_nz(code),
            "HK" => self.validate_hk(code),
            "TW" => self.validate_tw(code),
            // ── Americas (new) ──
            "AR" => self.validate_ar(code),
            "CL" => self.validate_cl(code),
            "CO" => self.validate_co(code),
            "PE" => self.validate_pe(code),
            "EC" => self.validate_ec(code),
            "UY" => self.validate_uy(code),
            "VE" => self.validate_ve(code),
            // ── Africa / Middle East (new) ──
            "NG" => self.validate_ng(code),
            "KE" => self.validate_ke(code),
            "EG" => self.validate_eg(code),
            "SA" => self.validate_sa(code),
            "AE" => self.validate_ae(code),
            "IL" => self.validate_il(code),
            "QA" => self.validate_qa(code),
            "KW" => self.validate_kw(code),
            "BH" => self.validate_bh(code),
            "OM" => self.validate_om(code),
            "GH" => self.validate_gh(code),
            "TZ" => self.validate_tz(code),
            "ET" => self.validate_et(code),
            "MA" => self.validate_ma(code),
            "TN" => self.validate_tn(code),
            "DZ" => self.validate_dz(code),
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

    // ════════════════════════════════════════════════════════════════════
    // Existing 18 country implementations (unchanged)
    // ════════════════════════════════════════════════════════════════════

    // ── India ── 1 alpha + 7 digits = 8 chars
    fn generate_in(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_in(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_uppercase() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── United States ── 9 digits
    fn generate_us(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_us(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9 && !clean.starts_with('0')
    }

    // ── United Kingdom ── 9 digits
    fn generate_gb(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_gb(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Germany ── C + 8 alphanumeric = 9 chars
    fn generate_de(&self, rng: &mut impl Rng) -> String {
        let prefix = 'C';
        let rest: String = (0..8)
            .map(|_| {
                if rng.gen_bool(0.5) {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                } else {
                    (b'A' + rng.gen_range(0..26u8)) as char
                }
            })
            .collect();
        format!("{}{}", prefix, rest)
    }

    fn validate_de(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        clean.len() == 9
            && clean.starts_with('C')
            && clean[1..].chars().all(|c| c.is_ascii_alphanumeric())
    }

    // ── France ── 2 alpha + 7 digits = 9 chars
    fn generate_fr(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_fr(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Brazil ── 2 alpha + 6 digits = 8 chars
    fn generate_br(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_br(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Australia ── 1 alpha + 7 digits = 8 chars
    fn generate_au(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_au(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Canada ── 2 alpha + 6 digits = 8 chars
    fn generate_ca(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_ca(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Japan ── 2 alpha + 7 digits = 9 chars
    fn generate_jp(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_jp(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── China ── E/G prefix + 8 digits = 9 chars
    fn generate_cn(&self, rng: &mut impl Rng) -> String {
        let prefix = if rng.gen_bool(0.5) { 'E' } else { 'G' };
        let digits: String = (0..8)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", prefix, digits)
    }

    fn validate_cn(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let first = clean.chars().next().unwrap();
        (first == 'E' || first == 'G') && clean[1..].chars().all(|c| c.is_ascii_digit())
    }

    // ── Italy ── 2 alpha + 7 digits = 9 chars
    fn generate_it(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_it(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Spain ── 3 alpha + 6 digits = 9 chars
    fn generate_es(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..3)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_es(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..3].iter().all(|c| c.is_ascii_alphabetic())
            && chars[3..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Netherlands ── 2 alpha + 7 digits = 9 chars
    fn generate_nl(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_nl(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Sweden ── 8 digits
    fn generate_se(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_se(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 8 && !clean.starts_with('0')
    }

    // ── South Korea ── M/S + 8 digits = 9 chars
    fn generate_kr(&self, rng: &mut impl Rng) -> String {
        let prefix = if rng.gen_bool(0.5) { 'M' } else { 'S' };
        let digits: String = (0..8)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", prefix, digits)
    }

    fn validate_kr(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let first = clean.chars().next().unwrap();
        (first == 'M' || first == 'S') && clean[1..].chars().all(|c| c.is_ascii_digit())
    }

    // ── Singapore ── E + 7 digits + 1 alpha = 9 chars
    fn generate_sg(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        let suffix = (b'A' + rng.gen_range(0..26u8)) as char;
        format!("E{}{}", digits, suffix)
    }

    fn validate_sg(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0] == 'E'
            && chars[1..8].iter().all(|c| c.is_ascii_digit())
            && chars[8].is_ascii_alphabetic()
    }

    // ── South Africa ── A + 8 digits = 9 chars
    fn generate_za(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..8)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_za(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Mexico ── 10 digits
    fn generate_mx(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..10)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_mx(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 10 && !clean.starts_with('0')
    }

    // ════════════════════════════════════════════════════════════════════
    // New country implementations — Europe
    // ════════════════════════════════════════════════════════════════════

    // ── Poland ── 2 letters + 7 digits = 9 chars
    fn generate_pl(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_pl(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Romania ── 9 digits
    fn generate_ro(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_ro(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Czech Republic ── 8 digits
    fn generate_cz(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_cz(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 8
    }

    // ── Greece ── 2 letters + 6 digits = 8 chars
    fn generate_gr(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_gr(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Portugal ── 2 letters + 6 digits = 8 chars
    fn generate_pt(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_pt(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Belgium ── 2 letters + 7 digits = 9 chars
    fn generate_be(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_be(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Austria ── 1 letter + 7 digits = 8 chars
    fn generate_at(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_at(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Switzerland ── 1 letter + 7 digits = 8 chars
    fn generate_ch(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_ch(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Ireland ── 2 letters + 7 digits = 9 chars
    fn generate_ie(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_ie(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Denmark ── 9 digits
    fn generate_dk(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_dk(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Finland ── 2 letters + 7 digits = 9 chars
    fn generate_fi(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_fi(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Norway ── 9 digits
    fn generate_no(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_no(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Bulgaria ── 9 digits
    fn generate_bg(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_bg(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Slovakia ── 2 letters + 6 digits = 8 chars
    fn generate_sk(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_sk(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Croatia ── 9 digits
    fn generate_hr(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_hr(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Slovenia ── 2 letters + 6 digits = 8 chars
    fn generate_si(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_si(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Hungary ── 2 letters + 6 digits = 8 chars
    fn generate_hu(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_hu(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Lithuania ── 8 digits
    fn generate_lt(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_lt(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 8
    }

    // ── Latvia ── 2 letters + 6 digits = 8 chars
    fn generate_lv(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_lv(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Estonia ── 2 letters + 6 digits = 8 chars
    fn generate_ee(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_ee(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Malta ── 8 digits
    fn generate_mt(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_mt(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 8
    }

    // ── Luxembourg ── 2 letters + 6 digits = 8 chars
    fn generate_lu(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_lu(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Iceland ── 1 letter + 7 digits = 8 chars
    fn generate_is(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_is(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Serbia ── 9 digits
    fn generate_rs(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_rs(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Turkey ── 1 letter + 8 digits = 9 chars
    fn generate_tr(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..8)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_tr(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Ukraine ── 2 letters + 6 digits = 8 chars
    fn generate_ua(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_ua(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ════════════════════════════════════════════════════════════════════
    // New country implementations — Asia-Pacific
    // ════════════════════════════════════════════════════════════════════

    // ── Indonesia ── 1 letter + 7 digits = 8 chars
    fn generate_id(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_id(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Thailand ── 2 letters + 7 digits = 9 chars
    fn generate_th(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_th(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Malaysia ── 1 letter + 7 digits = 8 chars
    fn generate_my(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_my(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Philippines ── 2 letters + 7 digits = 9 chars
    fn generate_ph(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_ph(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Vietnam ── 1 letter + 7 digits = 8 chars
    fn generate_vn(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_vn(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Pakistan ── 2 letters + 7 digits = 9 chars
    fn generate_pk(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_pk(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Bangladesh ── 2 letters + 7 digits = 9 chars
    fn generate_bd(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_bd(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Sri Lanka ── 1 letter + 7 digits = 8 chars
    fn generate_lk(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_lk(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Nepal ── 8 digits
    fn generate_np(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_np(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 8
    }

    // ── New Zealand ── 2 letters + 6 digits = 8 chars
    fn generate_nz(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_nz(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Hong Kong ── H prefix + 8 digits = 9 chars
    fn generate_hk(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..8)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("H{}", digits)
    }

    fn validate_hk(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        clean.len() == 9 && clean.starts_with('H') && clean[1..].chars().all(|c| c.is_ascii_digit())
    }

    // ── Taiwan ── 9 digits
    fn generate_tw(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_tw(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ════════════════════════════════════════════════════════════════════
    // New country implementations — Americas
    // ════════════════════════════════════════════════════════════════════

    // ── Argentina ── 3 letters + 6 digits = 9 chars
    fn generate_ar(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..3)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_ar(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..3].iter().all(|c| c.is_ascii_alphabetic())
            && chars[3..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Chile ── 9 digits
    fn generate_cl(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_cl(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Colombia ── 2 letters + 7 digits = 9 chars
    fn generate_co(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_co(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Peru ── 9 digits
    fn generate_pe(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_pe(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Ecuador ── 1 letter + 7 digits = 8 chars
    fn generate_ec(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_ec(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Uruguay ── 8 digits
    fn generate_uy(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_uy(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 8
    }

    // ── Venezuela ── 9 digits
    fn generate_ve(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_ve(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ════════════════════════════════════════════════════════════════════
    // New country implementations — Africa / Middle East
    // ════════════════════════════════════════════════════════════════════

    // ── Nigeria ── 1 letter + 8 digits = 9 chars
    fn generate_ng(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..8)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_ng(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Kenya ── 9 digits
    fn generate_ke(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_ke(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Egypt ── 9 digits
    fn generate_eg(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_eg(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Saudi Arabia ── 1 letter + 8 digits = 9 chars
    fn generate_sa(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..8)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_sa(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── United Arab Emirates ── 9 digits
    fn generate_ae(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_ae(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Israel ── 8 digits
    fn generate_il(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_il(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 8
    }

    // ── Qatar ── 9 digits
    fn generate_qa(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_qa(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Kuwait ── 9 digits
    fn generate_kw(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_kw(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Bahrain ── 9 digits
    fn generate_bh(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_bh(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }

    // ── Oman ── 8 digits
    fn generate_om(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_om(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 8
    }

    // ── Ghana ── 1 letter + 7 digits = 8 chars
    fn generate_gh(&self, rng: &mut impl Rng) -> String {
        let letter = (b'A' + rng.gen_range(0..26u8)) as char;
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", letter, digits)
    }

    fn validate_gh(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0].is_ascii_alphabetic() && chars[1..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Tanzania ── 2 letters + 7 digits = 9 chars
    fn generate_tz(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_tz(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Ethiopia ── 2 letters + 6 digits = 8 chars
    fn generate_et(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..6)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_et(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 8 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Morocco ── 2 letters + 7 digits = 9 chars
    fn generate_ma(&self, rng: &mut impl Rng) -> String {
        let alpha: String = (0..2)
            .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
            .collect();
        let digits: String = (0..7)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        format!("{}{}", alpha, digits)
    }

    fn validate_ma(&self, code: &str) -> bool {
        let clean = code.trim().to_uppercase();
        if clean.len() != 9 {
            return false;
        }
        let chars: Vec<char> = clean.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
            && chars[2..].iter().all(|c| c.is_ascii_digit())
    }

    // ── Tunisia ── 8 digits
    fn generate_tn(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..8)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_tn(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 8
    }

    // ── Algeria ── 9 digits
    fn generate_dz(&self, rng: &mut impl Rng) -> String {
        let digits: String = (0..9)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect();
        digits
    }

    fn validate_dz(&self, code: &str) -> bool {
        let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        clean.len() == 9
    }
}
