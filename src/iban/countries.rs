use super::types::CharType::{Alpha as A, Alphanumeric as C, Numeric as N};
use super::types::{f, BbanField, CountryFormat};

const AL: &[BbanField] = &[f(3, N), f(4, N), f(1, N), f(16, C)];
const AT: &[BbanField] = &[f(5, N), f(11, N)];
const BE: &[BbanField] = &[f(3, N), f(7, N), f(2, N)];
const BA: &[BbanField] = &[f(3, N), f(3, N), f(8, N), f(2, N)];
const BG: &[BbanField] = &[f(4, A), f(6, N), f(8, C)];
const HR: &[BbanField] = &[f(7, N), f(10, N)];
const CY: &[BbanField] = &[f(3, N), f(5, N), f(16, C)];
const CZ: &[BbanField] = &[f(4, N), f(16, N)];
const DK: &[BbanField] = &[f(4, N), f(9, N), f(1, N)];
const EE: &[BbanField] = &[f(2, N), f(2, N), f(11, N), f(1, N)];
const FI: &[BbanField] = &[f(3, N), f(10, N), f(1, N)];
const FR: &[BbanField] = &[f(5, N), f(5, N), f(11, C), f(2, N)];
const DE: &[BbanField] = &[f(8, N), f(10, N)];
const GR: &[BbanField] = &[f(3, N), f(4, N), f(16, C)];
const HU: &[BbanField] = &[f(3, N), f(4, N), f(16, N), f(1, N)];
const IS: &[BbanField] = &[f(4, N), f(18, N)];
const IE: &[BbanField] = &[f(4, A), f(6, N), f(8, N)];
const IT: &[BbanField] = &[f(1, A), f(5, N), f(5, N), f(12, C)];
const LV: &[BbanField] = &[f(4, A), f(13, C)];
const LI: &[BbanField] = &[f(5, N), f(12, C)];
const LT: &[BbanField] = &[f(5, N), f(11, N)];
const LU: &[BbanField] = &[f(3, N), f(13, C)];
const MT: &[BbanField] = &[f(4, A), f(5, N), f(18, C)];
const MD: &[BbanField] = &[f(2, C), f(18, C)];
const MC: &[BbanField] = &[f(5, N), f(5, N), f(11, C), f(2, N)];
const ME: &[BbanField] = &[f(3, N), f(13, N), f(2, N)];
const NL: &[BbanField] = &[f(4, A), f(10, N)];
const NO: &[BbanField] = &[f(4, N), f(6, N), f(1, N)];
const PL: &[BbanField] = &[f(3, N), f(4, N), f(1, N), f(16, N)];
const PT: &[BbanField] = &[f(4, N), f(4, N), f(11, N), f(2, N)];
const RO: &[BbanField] = &[f(4, A), f(16, C)];
const RS: &[BbanField] = &[f(3, N), f(13, N), f(2, N)];
const SK: &[BbanField] = &[f(4, N), f(16, N)];
const SI: &[BbanField] = &[f(2, N), f(3, N), f(8, N), f(2, N)];
const ES: &[BbanField] = &[f(4, N), f(4, N), f(2, N), f(10, N)];
const SE: &[BbanField] = &[f(3, N), f(16, N), f(1, N)];
const CH: &[BbanField] = &[f(5, N), f(12, C)];
const TR: &[BbanField] = &[f(5, N), f(1, N), f(16, C)];
const GB: &[BbanField] = &[f(4, A), f(6, N), f(8, N)];
const SA: &[BbanField] = &[f(2, N), f(18, C)];
const AE: &[BbanField] = &[f(3, N), f(16, N)];

const AD: &[BbanField] = &[f(8, N), f(12, C)];
const AX: &[BbanField] = &[f(14, N)];
const AZ: &[BbanField] = &[f(4, A), f(20, C)];
const BH: &[BbanField] = &[f(4, A), f(14, C)];
const BR: &[BbanField] = &[f(23, N), f(1, A), f(1, C)];
const BY: &[BbanField] = &[f(4, A), f(4, N), f(16, C)];
const CR: &[BbanField] = &[f(18, N)];
const DO: &[BbanField] = &[f(4, A), f(20, N)];
const EG: &[BbanField] = &[f(25, N)];
const FO: &[BbanField] = &[f(14, N)];
const GE: &[BbanField] = &[f(2, C), f(16, N)];
const GI: &[BbanField] = &[f(4, A), f(15, C)];
const GL: &[BbanField] = &[f(14, N)];
const GT: &[BbanField] = &[f(24, C)];
const IL: &[BbanField] = &[f(19, N)];
const IQ: &[BbanField] = &[f(4, A), f(15, N)];
const JO: &[BbanField] = &[f(4, A), f(4, N), f(18, C)];
const KW: &[BbanField] = &[f(4, A), f(22, C)];
const KZ: &[BbanField] = &[f(3, N), f(13, C)];
const LB: &[BbanField] = &[f(4, N), f(20, C)];
const LC: &[BbanField] = &[f(4, A), f(24, C)];
const LY: &[BbanField] = &[f(21, N)];
const MK: &[BbanField] = &[f(3, N), f(10, N), f(2, N)];
const MN: &[BbanField] = &[f(16, N)];
const MR: &[BbanField] = &[f(23, N)];
const MU: &[BbanField] = &[f(4, A), f(19, N), f(3, A)];
const NI: &[BbanField] = &[f(4, A), f(20, N)];
const OM: &[BbanField] = &[f(3, N), f(16, C)];
const PK: &[BbanField] = &[f(4, C), f(16, N)];
const PS: &[BbanField] = &[f(4, C), f(21, N)];
const QA: &[BbanField] = &[f(4, A), f(21, C)];
const RU: &[BbanField] = &[f(14, N), f(15, C)];
const SC: &[BbanField] = &[f(4, A), f(20, N), f(3, A)];
const SD: &[BbanField] = &[f(14, N)];
const SM: &[BbanField] = &[f(1, A), f(10, N), f(12, C)];
const SO: &[BbanField] = &[f(19, N)];
const ST: &[BbanField] = &[f(21, N)];
const SV: &[BbanField] = &[f(4, A), f(20, N)];
const TL: &[BbanField] = &[f(19, N)];
const TN: &[BbanField] = &[f(20, N)];
const UA: &[BbanField] = &[f(6, N), f(19, C)];
const VA: &[BbanField] = &[f(18, N)];
const VG: &[BbanField] = &[f(4, C), f(16, N)];
const XK: &[BbanField] = &[f(16, N)];

// African & Middle Eastern IBAN countries
const AO: &[BbanField] = &[f(21, N)];
// UEMOA zone (French-influenced, 1 alpha + 23 numeric)
const BF: &[BbanField] = &[f(1, A), f(23, N)];
const BI: &[BbanField] = &[f(5, N), f(5, N), f(11, N), f(2, N)];
const BJ: &[BbanField] = &[f(1, A), f(23, N)];
// CEMAC zone (all numeric, 5+5+11+2)
const CF: &[BbanField] = &[f(5, N), f(5, N), f(11, N), f(2, N)];
const CG: &[BbanField] = &[f(5, N), f(5, N), f(11, N), f(2, N)];
const CI: &[BbanField] = &[f(1, A), f(23, N)];
const CM: &[BbanField] = &[f(5, N), f(5, N), f(11, N), f(2, N)];
const CV: &[BbanField] = &[f(21, N)];
const DJ: &[BbanField] = &[f(5, N), f(5, N), f(11, N), f(2, N)];
const DZ: &[BbanField] = &[f(22, N)];
const GA: &[BbanField] = &[f(5, N), f(5, N), f(11, N), f(2, N)];
const GN: &[BbanField] = &[f(24, N)];
const GQ: &[BbanField] = &[f(5, N), f(5, N), f(11, N), f(2, N)];
const GW: &[BbanField] = &[f(2, C), f(19, N)];
const IR: &[BbanField] = &[f(22, N)];
const KM: &[BbanField] = &[f(5, N), f(5, N), f(11, N), f(2, N)];
const MA: &[BbanField] = &[f(24, N)];
const MG: &[BbanField] = &[f(5, N), f(5, N), f(11, N), f(2, N)];
const ML: &[BbanField] = &[f(1, A), f(23, N)];
const MZ: &[BbanField] = &[f(21, N)];
const NE: &[BbanField] = &[f(1, A), f(23, N)];
const SN: &[BbanField] = &[f(1, A), f(23, N)];
const TD: &[BbanField] = &[f(5, N), f(5, N), f(11, N), f(2, N)];
const TG: &[BbanField] = &[f(1, A), f(23, N)];
// Crown Dependencies (GB format)
const GG: &[BbanField] = &[f(4, A), f(6, N), f(8, N)];
const IM: &[BbanField] = &[f(4, A), f(6, N), f(8, N)];
const JE: &[BbanField] = &[f(4, A), f(6, N), f(8, N)];

pub(crate) const ALL_FORMATS: &[CountryFormat] = &[
    CountryFormat {
        code: "AD",
        fields: AD,
    },
    CountryFormat {
        code: "AE",
        fields: AE,
    },
    CountryFormat {
        code: "AL",
        fields: AL,
    },
    CountryFormat {
        code: "AO",
        fields: AO,
    },
    CountryFormat {
        code: "AT",
        fields: AT,
    },
    CountryFormat {
        code: "AX",
        fields: AX,
    },
    CountryFormat {
        code: "AZ",
        fields: AZ,
    },
    CountryFormat {
        code: "BA",
        fields: BA,
    },
    CountryFormat {
        code: "BE",
        fields: BE,
    },
    CountryFormat {
        code: "BF",
        fields: BF,
    },
    CountryFormat {
        code: "BG",
        fields: BG,
    },
    CountryFormat {
        code: "BH",
        fields: BH,
    },
    CountryFormat {
        code: "BI",
        fields: BI,
    },
    CountryFormat {
        code: "BJ",
        fields: BJ,
    },
    CountryFormat {
        code: "BR",
        fields: BR,
    },
    CountryFormat {
        code: "BY",
        fields: BY,
    },
    CountryFormat {
        code: "CF",
        fields: CF,
    },
    CountryFormat {
        code: "CG",
        fields: CG,
    },
    CountryFormat {
        code: "CH",
        fields: CH,
    },
    CountryFormat {
        code: "CI",
        fields: CI,
    },
    CountryFormat {
        code: "CM",
        fields: CM,
    },
    CountryFormat {
        code: "CV",
        fields: CV,
    },
    CountryFormat {
        code: "CR",
        fields: CR,
    },
    CountryFormat {
        code: "CY",
        fields: CY,
    },
    CountryFormat {
        code: "CZ",
        fields: CZ,
    },
    CountryFormat {
        code: "DE",
        fields: DE,
    },
    CountryFormat {
        code: "DJ",
        fields: DJ,
    },
    CountryFormat {
        code: "DK",
        fields: DK,
    },
    CountryFormat {
        code: "DZ",
        fields: DZ,
    },
    CountryFormat {
        code: "DO",
        fields: DO,
    },
    CountryFormat {
        code: "EE",
        fields: EE,
    },
    CountryFormat {
        code: "EG",
        fields: EG,
    },
    CountryFormat {
        code: "ES",
        fields: ES,
    },
    CountryFormat {
        code: "FI",
        fields: FI,
    },
    CountryFormat {
        code: "FO",
        fields: FO,
    },
    CountryFormat {
        code: "FR",
        fields: FR,
    },
    CountryFormat {
        code: "GA",
        fields: GA,
    },
    CountryFormat {
        code: "GB",
        fields: GB,
    },
    CountryFormat {
        code: "GE",
        fields: GE,
    },
    CountryFormat {
        code: "GG",
        fields: GG,
    },
    CountryFormat {
        code: "GF",
        fields: FR,
    },
    CountryFormat {
        code: "GI",
        fields: GI,
    },
    CountryFormat {
        code: "GL",
        fields: GL,
    },
    CountryFormat {
        code: "GN",
        fields: GN,
    },
    CountryFormat {
        code: "GP",
        fields: FR,
    },
    CountryFormat {
        code: "GQ",
        fields: GQ,
    },
    CountryFormat {
        code: "GR",
        fields: GR,
    },
    CountryFormat {
        code: "GT",
        fields: GT,
    },
    CountryFormat {
        code: "GW",
        fields: GW,
    },
    CountryFormat {
        code: "HR",
        fields: HR,
    },
    CountryFormat {
        code: "HU",
        fields: HU,
    },
    CountryFormat {
        code: "IE",
        fields: IE,
    },
    CountryFormat {
        code: "IL",
        fields: IL,
    },
    CountryFormat {
        code: "IM",
        fields: IM,
    },
    CountryFormat {
        code: "IR",
        fields: IR,
    },
    CountryFormat {
        code: "IQ",
        fields: IQ,
    },
    CountryFormat {
        code: "IS",
        fields: IS,
    },
    CountryFormat {
        code: "IT",
        fields: IT,
    },
    CountryFormat {
        code: "JE",
        fields: JE,
    },
    CountryFormat {
        code: "JO",
        fields: JO,
    },
    CountryFormat {
        code: "KM",
        fields: KM,
    },
    CountryFormat {
        code: "KW",
        fields: KW,
    },
    CountryFormat {
        code: "KZ",
        fields: KZ,
    },
    CountryFormat {
        code: "LB",
        fields: LB,
    },
    CountryFormat {
        code: "MA",
        fields: MA,
    },
    CountryFormat {
        code: "MG",
        fields: MG,
    },
    CountryFormat {
        code: "ML",
        fields: ML,
    },
    CountryFormat {
        code: "LC",
        fields: LC,
    },
    CountryFormat {
        code: "LI",
        fields: LI,
    },
    CountryFormat {
        code: "LT",
        fields: LT,
    },
    CountryFormat {
        code: "LU",
        fields: LU,
    },
    CountryFormat {
        code: "LV",
        fields: LV,
    },
    CountryFormat {
        code: "LY",
        fields: LY,
    },
    CountryFormat {
        code: "MC",
        fields: MC,
    },
    CountryFormat {
        code: "MD",
        fields: MD,
    },
    CountryFormat {
        code: "ME",
        fields: ME,
    },
    CountryFormat {
        code: "MF",
        fields: FR,
    },
    CountryFormat {
        code: "MK",
        fields: MK,
    },
    CountryFormat {
        code: "MN",
        fields: MN,
    },
    CountryFormat {
        code: "MQ",
        fields: FR,
    },
    CountryFormat {
        code: "MR",
        fields: MR,
    },
    CountryFormat {
        code: "MZ",
        fields: MZ,
    },
    CountryFormat {
        code: "MT",
        fields: MT,
    },
    CountryFormat {
        code: "MU",
        fields: MU,
    },
    CountryFormat {
        code: "NC",
        fields: FR,
    },
    CountryFormat {
        code: "NE",
        fields: NE,
    },
    CountryFormat {
        code: "NI",
        fields: NI,
    },
    CountryFormat {
        code: "NL",
        fields: NL,
    },
    CountryFormat {
        code: "NO",
        fields: NO,
    },
    CountryFormat {
        code: "OM",
        fields: OM,
    },
    CountryFormat {
        code: "PF",
        fields: FR,
    },
    CountryFormat {
        code: "PK",
        fields: PK,
    },
    CountryFormat {
        code: "PL",
        fields: PL,
    },
    CountryFormat {
        code: "PM",
        fields: FR,
    },
    CountryFormat {
        code: "PS",
        fields: PS,
    },
    CountryFormat {
        code: "PT",
        fields: PT,
    },
    CountryFormat {
        code: "QA",
        fields: QA,
    },
    CountryFormat {
        code: "RE",
        fields: FR,
    },
    CountryFormat {
        code: "RO",
        fields: RO,
    },
    CountryFormat {
        code: "RS",
        fields: RS,
    },
    CountryFormat {
        code: "RU",
        fields: RU,
    },
    CountryFormat {
        code: "SA",
        fields: SA,
    },
    CountryFormat {
        code: "SC",
        fields: SC,
    },
    CountryFormat {
        code: "SD",
        fields: SD,
    },
    CountryFormat {
        code: "SE",
        fields: SE,
    },
    CountryFormat {
        code: "SI",
        fields: SI,
    },
    CountryFormat {
        code: "SK",
        fields: SK,
    },
    CountryFormat {
        code: "SM",
        fields: SM,
    },
    CountryFormat {
        code: "SN",
        fields: SN,
    },
    CountryFormat {
        code: "SO",
        fields: SO,
    },
    CountryFormat {
        code: "ST",
        fields: ST,
    },
    CountryFormat {
        code: "SV",
        fields: SV,
    },
    CountryFormat {
        code: "TD",
        fields: TD,
    },
    CountryFormat {
        code: "TF",
        fields: FR,
    },
    CountryFormat {
        code: "TG",
        fields: TG,
    },
    CountryFormat {
        code: "TL",
        fields: TL,
    },
    CountryFormat {
        code: "TN",
        fields: TN,
    },
    CountryFormat {
        code: "TR",
        fields: TR,
    },
    CountryFormat {
        code: "UA",
        fields: UA,
    },
    CountryFormat {
        code: "VA",
        fields: VA,
    },
    CountryFormat {
        code: "VG",
        fields: VG,
    },
    CountryFormat {
        code: "WF",
        fields: FR,
    },
    CountryFormat {
        code: "XK",
        fields: XK,
    },
    CountryFormat {
        code: "YT",
        fields: FR,
    },
];

pub(crate) fn get_format(country: &str) -> Option<&'static [BbanField]> {
    ALL_FORMATS
        .iter()
        .find(|f| f.code == country)
        .map(|f| f.fields)
}

/// Returns all supported IBAN country codes.
pub fn supported_countries() -> Vec<&'static str> {
    ALL_FORMATS.iter().map(|f| f.code).collect()
}
