#[derive(Debug, Clone, Copy)]
pub enum CharType {
    Numeric,
    Alpha,
    Alphanumeric,
}

#[derive(Debug, Clone, Copy)]
pub struct BbanField {
    pub length: u8,
    pub char_type: CharType,
}

pub(crate) const fn f(length: u8, char_type: CharType) -> BbanField {
    BbanField { length, char_type }
}

pub(crate) struct CountryFormat {
    pub code: &'static str,
    pub fields: &'static [BbanField],
}
