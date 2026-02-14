// Country modules
pub mod ad;
pub mod ae;
pub mod af;
pub mod ag;
pub mod ai;
pub mod al;
pub mod am;
pub mod ao;
pub mod aq;
pub mod ar;
pub mod as_;
pub mod at;
pub mod au;
pub mod aw;
pub mod ax;
pub mod az;
pub mod ba;
pub mod bb;
pub mod bd;
pub mod be;
pub mod bf;
pub mod bg;
pub mod bh;
pub mod bi;
pub mod bj;
pub mod bl;
pub mod bm;
pub mod bn;
pub mod bo;
pub mod bq;
pub mod br;
pub mod bs;
pub mod bt;
pub mod bv;
pub mod bw;
pub mod by;
pub mod bz;
pub mod ca;
pub mod cc;
pub mod cd;
pub mod cf;
pub mod cg;
pub mod ch;
pub mod ci;
pub mod ck;
pub mod cl;
pub mod cm;
pub mod cn;
pub mod co;
pub mod cr;
pub mod cu;
pub mod cv;
pub mod cw;
pub mod cx;
pub mod cy;
pub mod cz;
pub mod de;
pub mod dj;
pub mod dk;
pub mod dm;
pub mod do_;
pub mod dz;
pub mod ec;
pub mod ee;
pub mod eg;
pub mod eh;
pub mod er;
pub mod es;
pub mod et;
pub mod fi;
pub mod fj;
pub mod fk;
pub mod fm;
pub mod fo;
pub mod fr;
pub mod ga;
pub mod gb;
pub mod gd;
pub mod ge;
pub mod gf;
pub mod gg;
pub mod gh;
pub mod gi;
pub mod gl;
pub mod gm;
pub mod gn;
pub mod gp;
pub mod gq;
pub mod gr;
pub mod gs;
pub mod gt;
pub mod gu;
pub mod gw;
pub mod gy;
pub mod hk;
pub mod hm;
pub mod hn;
pub mod hr;
pub mod ht;
pub mod hu;
pub mod id;
pub mod ie;
pub mod il;
pub mod im;
pub mod in_;
pub mod io;
pub mod iq;
pub mod ir;
pub mod is;
pub mod it;
pub mod je;
pub mod jm;
pub mod jo;
pub mod jp;
pub mod ke;
pub mod kg;
pub mod kh;
pub mod ki;
pub mod km;
pub mod kn;
pub mod kp;
pub mod kr;
pub mod kw;
pub mod ky;
pub mod kz;
pub mod la;
pub mod lb;
pub mod lc;
pub mod li;
pub mod lk;
pub mod lr;
pub mod ls;
pub mod lt;
pub mod lu;
pub mod lv;
pub mod ly;
pub mod ma;
pub mod mc;
pub mod md;
pub mod me;
pub mod mf;
pub mod mg;
pub mod mh;
pub mod mk;
pub mod ml;
pub mod mm;
pub mod mn;
pub mod mo;
pub mod mp;
pub mod mq;
pub mod mr;
pub mod ms;
pub mod mt;
pub mod mu;
pub mod mv;
pub mod mw;
pub mod mx;
pub mod my;
pub mod mz;
pub mod na;
pub mod nc;
pub mod ne;
pub mod nf;
pub mod ng;
pub mod ni;
pub mod nl;
pub mod no;
pub mod np;
pub mod nr;
pub mod nu;
pub mod nz;
pub mod om;
pub mod pa;
pub mod pe;
pub mod pf;
pub mod pg;
pub mod ph;
pub mod pk;
pub mod pl;
pub mod pm;
pub mod pn;
pub mod pr;
pub mod ps;
pub mod pt;
pub mod pw;
pub mod py;
pub mod qa;
pub mod re;
pub mod ro;
pub mod rs;
pub mod ru;
pub mod rw;
pub mod sa;
pub mod sb;
pub mod sc;
pub mod sd;
pub mod se;
pub mod sg;
pub mod sh;
pub mod si;
pub mod sj;
pub mod sk;
pub mod sl;
pub mod sm;
pub mod sn;
pub mod so;
pub mod sr;
pub mod ss;
pub mod st;
pub mod sv;
pub mod sx;
pub mod sy;
pub mod sz;
pub mod tc;
pub mod td;
pub mod tf;
pub mod tg;
pub mod th;
pub mod tj;
pub mod tk;
pub mod tl;
pub mod tm;
pub mod tn;
pub mod to;
pub mod tr;
pub mod tt;
pub mod tv;
pub mod tw;
pub mod tz;
pub mod ua;
pub mod ug;
pub mod um;
pub mod us;
pub mod uy;
pub mod uz;
pub mod va;
pub mod vc;
pub mod ve;
pub mod vg;
pub mod vi;
pub mod vn;
pub mod vu;
pub mod wf;
pub mod ws;
pub mod xk;
pub mod ye;
pub mod yt;
pub mod za;
pub mod zm;
pub mod zw;

use rand::Rng;
#[cfg(feature = "json")]
use serde::Serialize;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct CompanyResult {
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

type GenerateFn = fn(&mut rand::rngs::ThreadRng) -> String;
type ValidateFn = fn(&str) -> bool;

struct RegistryEntry {
    code: &'static str,
    name: &'static str,
    generate: GenerateFn,
    validate: ValidateFn,
}

pub struct Registry {
    entries: Vec<RegistryEntry>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        let entries = vec![
            RegistryEntry { code: "AD", name: "NRT", generate: ad::generate, validate: ad::validate },
            RegistryEntry { code: "AE", name: "TRN", generate: ae::generate, validate: ae::validate },
            RegistryEntry { code: "AF", name: "Business ID", generate: af::generate, validate: af::validate },
            RegistryEntry { code: "AG", name: "Business ID", generate: ag::generate, validate: ag::validate },
            RegistryEntry { code: "AI", name: "Business ID", generate: ai::generate, validate: ai::validate },
            RegistryEntry { code: "AL", name: "NIPT", generate: al::generate, validate: al::validate },
            RegistryEntry { code: "AM", name: "Business ID", generate: am::generate, validate: am::validate },
            RegistryEntry { code: "AO", name: "Business ID", generate: ao::generate, validate: ao::validate },
            RegistryEntry { code: "AQ", name: "Business ID", generate: aq::generate, validate: aq::validate },
            RegistryEntry { code: "AR", name: "CUIT", generate: ar::generate, validate: ar::validate },
            RegistryEntry { code: "AS", name: "Business ID", generate: as_::generate, validate: as_::validate },
            RegistryEntry { code: "AT", name: "UID", generate: at::generate, validate: at::validate },
            RegistryEntry { code: "AU", name: "ABN", generate: au::generate, validate: au::validate },
            RegistryEntry { code: "AW", name: "Business ID", generate: aw::generate, validate: aw::validate },
            RegistryEntry { code: "AX", name: "Business ID", generate: ax::generate, validate: ax::validate },
            RegistryEntry { code: "AZ", name: "VOEN", generate: az::generate, validate: az::validate },
            RegistryEntry { code: "BA", name: "Business ID", generate: ba::generate, validate: ba::validate },
            RegistryEntry { code: "BB", name: "Business ID", generate: bb::generate, validate: bb::validate },
            RegistryEntry { code: "BD", name: "Business ID", generate: bd::generate, validate: bd::validate },
            RegistryEntry { code: "BE", name: "TVA", generate: be::generate, validate: be::validate },
            RegistryEntry { code: "BF", name: "Business ID", generate: bf::generate, validate: bf::validate },
            RegistryEntry { code: "BG", name: "VAT", generate: bg::generate, validate: bg::validate },
            RegistryEntry { code: "BH", name: "Business ID", generate: bh::generate, validate: bh::validate },
            RegistryEntry { code: "BI", name: "Business ID", generate: bi::generate, validate: bi::validate },
            RegistryEntry { code: "BJ", name: "Business ID", generate: bj::generate, validate: bj::validate },
            RegistryEntry { code: "BL", name: "Business ID", generate: bl::generate, validate: bl::validate },
            RegistryEntry { code: "BM", name: "Business ID", generate: bm::generate, validate: bm::validate },
            RegistryEntry { code: "BN", name: "Business ID", generate: bn::generate, validate: bn::validate },
            RegistryEntry { code: "BO", name: "Business ID", generate: bo::generate, validate: bo::validate },
            RegistryEntry { code: "BQ", name: "Business ID", generate: bq::generate, validate: bq::validate },
            RegistryEntry { code: "BR", name: "CNPJ", generate: br::generate, validate: br::validate },
            RegistryEntry { code: "BS", name: "Business ID", generate: bs::generate, validate: bs::validate },
            RegistryEntry { code: "BT", name: "Business ID", generate: bt::generate, validate: bt::validate },
            RegistryEntry { code: "BV", name: "Business ID", generate: bv::generate, validate: bv::validate },
            RegistryEntry { code: "BW", name: "Business ID", generate: bw::generate, validate: bw::validate },
            RegistryEntry { code: "BY", name: "UNP", generate: by::generate, validate: by::validate },
            RegistryEntry { code: "BZ", name: "Business ID", generate: bz::generate, validate: bz::validate },
            RegistryEntry { code: "CA", name: "BN", generate: ca::generate, validate: ca::validate },
            RegistryEntry { code: "CC", name: "Business ID", generate: cc::generate, validate: cc::validate },
            RegistryEntry { code: "CD", name: "Business ID", generate: cd::generate, validate: cd::validate },
            RegistryEntry { code: "CF", name: "Business ID", generate: cf::generate, validate: cf::validate },
            RegistryEntry { code: "CG", name: "Business ID", generate: cg::generate, validate: cg::validate },
            RegistryEntry { code: "CH", name: "UID", generate: ch::generate, validate: ch::validate },
            RegistryEntry { code: "CI", name: "Business ID", generate: ci::generate, validate: ci::validate },
            RegistryEntry { code: "CK", name: "Business ID", generate: ck::generate, validate: ck::validate },
            RegistryEntry { code: "CL", name: "RUT", generate: cl::generate, validate: cl::validate },
            RegistryEntry { code: "CM", name: "Business ID", generate: cm::generate, validate: cm::validate },
            RegistryEntry { code: "CN", name: "USCC", generate: cn::generate, validate: cn::validate },
            RegistryEntry { code: "CO", name: "NIT", generate: co::generate, validate: co::validate },
            RegistryEntry { code: "CR", name: "CPJ", generate: cr::generate, validate: cr::validate },
            RegistryEntry { code: "CU", name: "NI", generate: cu::generate, validate: cu::validate },
            RegistryEntry { code: "CV", name: "Business ID", generate: cv::generate, validate: cv::validate },
            RegistryEntry { code: "CW", name: "Business ID", generate: cw::generate, validate: cw::validate },
            RegistryEntry { code: "CX", name: "Business ID", generate: cx::generate, validate: cx::validate },
            RegistryEntry { code: "CY", name: "VAT", generate: cy::generate, validate: cy::validate },
            RegistryEntry { code: "CZ", name: "DIC", generate: cz::generate, validate: cz::validate },
            RegistryEntry { code: "DE", name: "USt-IdNr", generate: de::generate, validate: de::validate },
            RegistryEntry { code: "DJ", name: "Business ID", generate: dj::generate, validate: dj::validate },
            RegistryEntry { code: "DK", name: "CVR", generate: dk::generate, validate: dk::validate },
            RegistryEntry { code: "DM", name: "Business ID", generate: dm::generate, validate: dm::validate },
            RegistryEntry { code: "DO", name: "RNC", generate: do_::generate, validate: do_::validate },
            RegistryEntry { code: "DZ", name: "NIF", generate: dz::generate, validate: dz::validate },
            RegistryEntry { code: "EC", name: "RUC", generate: ec::generate, validate: ec::validate },
            RegistryEntry { code: "EE", name: "KMKR", generate: ee::generate, validate: ee::validate },
            RegistryEntry { code: "EG", name: "Tax Card", generate: eg::generate, validate: eg::validate },
            RegistryEntry { code: "EH", name: "Business ID", generate: eh::generate, validate: eh::validate },
            RegistryEntry { code: "ER", name: "Business ID", generate: er::generate, validate: er::validate },
            RegistryEntry { code: "ES", name: "CIF", generate: es::generate, validate: es::validate },
            RegistryEntry { code: "ET", name: "Business ID", generate: et::generate, validate: et::validate },
            RegistryEntry { code: "FI", name: "ALV nro", generate: fi::generate, validate: fi::validate },
            RegistryEntry { code: "FJ", name: "Business ID", generate: fj::generate, validate: fj::validate },
            RegistryEntry { code: "FK", name: "Business ID", generate: fk::generate, validate: fk::validate },
            RegistryEntry { code: "FM", name: "Business ID", generate: fm::generate, validate: fm::validate },
            RegistryEntry { code: "FO", name: "V-number", generate: fo::generate, validate: fo::validate },
            RegistryEntry { code: "FR", name: "TVA Intracommunautaire", generate: fr::generate, validate: fr::validate },
            RegistryEntry { code: "GA", name: "Business ID", generate: ga::generate, validate: ga::validate },
            RegistryEntry { code: "GB", name: "VAT Number", generate: gb::generate, validate: gb::validate },
            RegistryEntry { code: "GD", name: "Business ID", generate: gd::generate, validate: gd::validate },
            RegistryEntry { code: "GE", name: "Business ID", generate: ge::generate, validate: ge::validate },
            RegistryEntry { code: "GF", name: "Business ID", generate: gf::generate, validate: gf::validate },
            RegistryEntry { code: "GG", name: "Business ID", generate: gg::generate, validate: gg::validate },
            RegistryEntry { code: "GH", name: "TIN", generate: gh::generate, validate: gh::validate },
            RegistryEntry { code: "GI", name: "Business ID", generate: gi::generate, validate: gi::validate },
            RegistryEntry { code: "GL", name: "Business ID", generate: gl::generate, validate: gl::validate },
            RegistryEntry { code: "GM", name: "Business ID", generate: gm::generate, validate: gm::validate },
            RegistryEntry { code: "GN", name: "NIFp", generate: gn::generate, validate: gn::validate },
            RegistryEntry { code: "GP", name: "Business ID", generate: gp::generate, validate: gp::validate },
            RegistryEntry { code: "GQ", name: "Business ID", generate: gq::generate, validate: gq::validate },
            RegistryEntry { code: "GR", name: "AFM", generate: gr::generate, validate: gr::validate },
            RegistryEntry { code: "GS", name: "Business ID", generate: gs::generate, validate: gs::validate },
            RegistryEntry { code: "GT", name: "NIT", generate: gt::generate, validate: gt::validate },
            RegistryEntry { code: "GU", name: "Business ID", generate: gu::generate, validate: gu::validate },
            RegistryEntry { code: "GW", name: "Business ID", generate: gw::generate, validate: gw::validate },
            RegistryEntry { code: "GY", name: "Business ID", generate: gy::generate, validate: gy::validate },
            RegistryEntry { code: "HK", name: "BR Number", generate: hk::generate, validate: hk::validate },
            RegistryEntry { code: "HM", name: "Business ID", generate: hm::generate, validate: hm::validate },
            RegistryEntry { code: "HN", name: "Business ID", generate: hn::generate, validate: hn::validate },
            RegistryEntry { code: "HR", name: "OIB", generate: hr::generate, validate: hr::validate },
            RegistryEntry { code: "HT", name: "Business ID", generate: ht::generate, validate: ht::validate },
            RegistryEntry { code: "HU", name: "ANUM", generate: hu::generate, validate: hu::validate },
            RegistryEntry { code: "ID", name: "NPWP", generate: id::generate, validate: id::validate },
            RegistryEntry { code: "IE", name: "VAT", generate: ie::generate, validate: ie::validate },
            RegistryEntry { code: "IL", name: "Company Number", generate: il::generate, validate: il::validate },
            RegistryEntry { code: "IM", name: "Business ID", generate: im::generate, validate: im::validate },
            RegistryEntry { code: "IN", name: "GSTIN", generate: in_::generate, validate: in_::validate },
            RegistryEntry { code: "IO", name: "Business ID", generate: io::generate, validate: io::validate },
            RegistryEntry { code: "IQ", name: "Business ID", generate: iq::generate, validate: iq::validate },
            RegistryEntry { code: "IR", name: "Business ID", generate: ir::generate, validate: ir::validate },
            RegistryEntry { code: "IS", name: "Business ID", generate: is::generate, validate: is::validate },
            RegistryEntry { code: "IT", name: "Partita IVA", generate: it::generate, validate: it::validate },
            RegistryEntry { code: "JE", name: "Business ID", generate: je::generate, validate: je::validate },
            RegistryEntry { code: "JM", name: "Business ID", generate: jm::generate, validate: jm::validate },
            RegistryEntry { code: "JO", name: "Business ID", generate: jo::generate, validate: jo::validate },
            RegistryEntry { code: "JP", name: "CN", generate: jp::generate, validate: jp::validate },
            RegistryEntry { code: "KE", name: "PIN", generate: ke::generate, validate: ke::validate },
            RegistryEntry { code: "KG", name: "Business ID", generate: kg::generate, validate: kg::validate },
            RegistryEntry { code: "KH", name: "Business ID", generate: kh::generate, validate: kh::validate },
            RegistryEntry { code: "KI", name: "Business ID", generate: ki::generate, validate: ki::validate },
            RegistryEntry { code: "KM", name: "Business ID", generate: km::generate, validate: km::validate },
            RegistryEntry { code: "KN", name: "Business ID", generate: kn::generate, validate: kn::validate },
            RegistryEntry { code: "KP", name: "Business ID", generate: kp::generate, validate: kp::validate },
            RegistryEntry { code: "KR", name: "BRN", generate: kr::generate, validate: kr::validate },
            RegistryEntry { code: "KW", name: "Business ID", generate: kw::generate, validate: kw::validate },
            RegistryEntry { code: "KY", name: "Business ID", generate: ky::generate, validate: ky::validate },
            RegistryEntry { code: "KZ", name: "Business ID", generate: kz::generate, validate: kz::validate },
            RegistryEntry { code: "LA", name: "Business ID", generate: la::generate, validate: la::validate },
            RegistryEntry { code: "LB", name: "Business ID", generate: lb::generate, validate: lb::validate },
            RegistryEntry { code: "LC", name: "Business ID", generate: lc::generate, validate: lc::validate },
            RegistryEntry { code: "LI", name: "PEID", generate: li::generate, validate: li::validate },
            RegistryEntry { code: "LK", name: "Business ID", generate: lk::generate, validate: lk::validate },
            RegistryEntry { code: "LR", name: "Business ID", generate: lr::generate, validate: lr::validate },
            RegistryEntry { code: "LS", name: "Business ID", generate: ls::generate, validate: ls::validate },
            RegistryEntry { code: "LT", name: "PVM", generate: lt::generate, validate: lt::validate },
            RegistryEntry { code: "LU", name: "TVA", generate: lu::generate, validate: lu::validate },
            RegistryEntry { code: "LV", name: "PVN", generate: lv::generate, validate: lv::validate },
            RegistryEntry { code: "LY", name: "Business ID", generate: ly::generate, validate: ly::validate },
            RegistryEntry { code: "MA", name: "ICE", generate: ma::generate, validate: ma::validate },
            RegistryEntry { code: "MC", name: "TVA", generate: mc::generate, validate: mc::validate },
            RegistryEntry { code: "MD", name: "IDNO", generate: md::generate, validate: md::validate },
            RegistryEntry { code: "ME", name: "PIB", generate: me::generate, validate: me::validate },
            RegistryEntry { code: "MF", name: "Business ID", generate: mf::generate, validate: mf::validate },
            RegistryEntry { code: "MG", name: "Business ID", generate: mg::generate, validate: mg::validate },
            RegistryEntry { code: "MH", name: "Business ID", generate: mh::generate, validate: mh::validate },
            RegistryEntry { code: "MK", name: "EDB", generate: mk::generate, validate: mk::validate },
            RegistryEntry { code: "ML", name: "Business ID", generate: ml::generate, validate: ml::validate },
            RegistryEntry { code: "MM", name: "Business ID", generate: mm::generate, validate: mm::validate },
            RegistryEntry { code: "MN", name: "Business ID", generate: mn::generate, validate: mn::validate },
            RegistryEntry { code: "MO", name: "Business ID", generate: mo::generate, validate: mo::validate },
            RegistryEntry { code: "MP", name: "Business ID", generate: mp::generate, validate: mp::validate },
            RegistryEntry { code: "MQ", name: "Business ID", generate: mq::generate, validate: mq::validate },
            RegistryEntry { code: "MR", name: "Business ID", generate: mr::generate, validate: mr::validate },
            RegistryEntry { code: "MS", name: "Business ID", generate: ms::generate, validate: ms::validate },
            RegistryEntry { code: "MT", name: "VAT", generate: mt::generate, validate: mt::validate },
            RegistryEntry { code: "MU", name: "NID", generate: mu::generate, validate: mu::validate },
            RegistryEntry { code: "MV", name: "Business ID", generate: mv::generate, validate: mv::validate },
            RegistryEntry { code: "MW", name: "Business ID", generate: mw::generate, validate: mw::validate },
            RegistryEntry { code: "MX", name: "RFC", generate: mx::generate, validate: mx::validate },
            RegistryEntry { code: "MY", name: "Business Reg", generate: my::generate, validate: my::validate },
            RegistryEntry { code: "MZ", name: "NUIT", generate: mz::generate, validate: mz::validate },
            RegistryEntry { code: "NA", name: "Business ID", generate: na::generate, validate: na::validate },
            RegistryEntry { code: "NC", name: "Business ID", generate: nc::generate, validate: nc::validate },
            RegistryEntry { code: "NE", name: "Business ID", generate: ne::generate, validate: ne::validate },
            RegistryEntry { code: "NF", name: "Business ID", generate: nf::generate, validate: nf::validate },
            RegistryEntry { code: "NG", name: "TIN", generate: ng::generate, validate: ng::validate },
            RegistryEntry { code: "NI", name: "Business ID", generate: ni::generate, validate: ni::validate },
            RegistryEntry { code: "NL", name: "BTW", generate: nl::generate, validate: nl::validate },
            RegistryEntry { code: "NO", name: "MVA", generate: no::generate, validate: no::validate },
            RegistryEntry { code: "NP", name: "Business ID", generate: np::generate, validate: np::validate },
            RegistryEntry { code: "NR", name: "Business ID", generate: nr::generate, validate: nr::validate },
            RegistryEntry { code: "NU", name: "Business ID", generate: nu::generate, validate: nu::validate },
            RegistryEntry { code: "NZ", name: "IRD", generate: nz::generate, validate: nz::validate },
            RegistryEntry { code: "OM", name: "Business ID", generate: om::generate, validate: om::validate },
            RegistryEntry { code: "PA", name: "Business ID", generate: pa::generate, validate: pa::validate },
            RegistryEntry { code: "PE", name: "RUC", generate: pe::generate, validate: pe::validate },
            RegistryEntry { code: "PF", name: "Business ID", generate: pf::generate, validate: pf::validate },
            RegistryEntry { code: "PG", name: "Business ID", generate: pg::generate, validate: pg::validate },
            RegistryEntry { code: "PH", name: "TIN", generate: ph::generate, validate: ph::validate },
            RegistryEntry { code: "PK", name: "CNIC", generate: pk::generate, validate: pk::validate },
            RegistryEntry { code: "PL", name: "NIP", generate: pl::generate, validate: pl::validate },
            RegistryEntry { code: "PM", name: "Business ID", generate: pm::generate, validate: pm::validate },
            RegistryEntry { code: "PN", name: "Business ID", generate: pn::generate, validate: pn::validate },
            RegistryEntry { code: "PR", name: "Business ID", generate: pr::generate, validate: pr::validate },
            RegistryEntry { code: "PS", name: "Business ID", generate: ps::generate, validate: ps::validate },
            RegistryEntry { code: "PT", name: "NIF", generate: pt::generate, validate: pt::validate },
            RegistryEntry { code: "PW", name: "Business ID", generate: pw::generate, validate: pw::validate },
            RegistryEntry { code: "PY", name: "RUC", generate: py::generate, validate: py::validate },
            RegistryEntry { code: "QA", name: "Business ID", generate: qa::generate, validate: qa::validate },
            RegistryEntry { code: "RE", name: "Business ID", generate: re::generate, validate: re::validate },
            RegistryEntry { code: "RO", name: "CUI", generate: ro::generate, validate: ro::validate },
            RegistryEntry { code: "RS", name: "PIB", generate: rs::generate, validate: rs::validate },
            RegistryEntry { code: "RU", name: "INN", generate: ru::generate, validate: ru::validate },
            RegistryEntry { code: "RW", name: "Business ID", generate: rw::generate, validate: rw::validate },
            RegistryEntry { code: "SA", name: "VAT", generate: sa::generate, validate: sa::validate },
            RegistryEntry { code: "SB", name: "Business ID", generate: sb::generate, validate: sb::validate },
            RegistryEntry { code: "SC", name: "Business ID", generate: sc::generate, validate: sc::validate },
            RegistryEntry { code: "SD", name: "Business ID", generate: sd::generate, validate: sd::validate },
            RegistryEntry { code: "SE", name: "VAT", generate: se::generate, validate: se::validate },
            RegistryEntry { code: "SG", name: "UEN", generate: sg::generate, validate: sg::validate },
            RegistryEntry { code: "SH", name: "Business ID", generate: sh::generate, validate: sh::validate },
            RegistryEntry { code: "SI", name: "DDV", generate: si::generate, validate: si::validate },
            RegistryEntry { code: "SJ", name: "Business ID", generate: sj::generate, validate: sj::validate },
            RegistryEntry { code: "SK", name: "IC DPH", generate: sk::generate, validate: sk::validate },
            RegistryEntry { code: "SL", name: "Business ID", generate: sl::generate, validate: sl::validate },
            RegistryEntry { code: "SM", name: "COE", generate: sm::generate, validate: sm::validate },
            RegistryEntry { code: "SN", name: "NINEA", generate: sn::generate, validate: sn::validate },
            RegistryEntry { code: "SO", name: "Business ID", generate: so::generate, validate: so::validate },
            RegistryEntry { code: "SR", name: "Business ID", generate: sr::generate, validate: sr::validate },
            RegistryEntry { code: "SS", name: "Business ID", generate: ss::generate, validate: ss::validate },
            RegistryEntry { code: "ST", name: "Business ID", generate: st::generate, validate: st::validate },
            RegistryEntry { code: "SV", name: "NIT", generate: sv::generate, validate: sv::validate },
            RegistryEntry { code: "SX", name: "Business ID", generate: sx::generate, validate: sx::validate },
            RegistryEntry { code: "SY", name: "Business ID", generate: sy::generate, validate: sy::validate },
            RegistryEntry { code: "SZ", name: "Business ID", generate: sz::generate, validate: sz::validate },
            RegistryEntry { code: "TC", name: "Business ID", generate: tc::generate, validate: tc::validate },
            RegistryEntry { code: "TD", name: "Business ID", generate: td::generate, validate: td::validate },
            RegistryEntry { code: "TF", name: "Business ID", generate: tf::generate, validate: tf::validate },
            RegistryEntry { code: "TG", name: "Business ID", generate: tg::generate, validate: tg::validate },
            RegistryEntry { code: "TH", name: "Tax ID", generate: th::generate, validate: th::validate },
            RegistryEntry { code: "TJ", name: "Business ID", generate: tj::generate, validate: tj::validate },
            RegistryEntry { code: "TK", name: "Business ID", generate: tk::generate, validate: tk::validate },
            RegistryEntry { code: "TL", name: "Business ID", generate: tl::generate, validate: tl::validate },
            RegistryEntry { code: "TM", name: "Business ID", generate: tm::generate, validate: tm::validate },
            RegistryEntry { code: "TN", name: "MF", generate: tn::generate, validate: tn::validate },
            RegistryEntry { code: "TO", name: "Business ID", generate: to::generate, validate: to::validate },
            RegistryEntry { code: "TR", name: "VKN", generate: tr::generate, validate: tr::validate },
            RegistryEntry { code: "TT", name: "Business ID", generate: tt::generate, validate: tt::validate },
            RegistryEntry { code: "TV", name: "Business ID", generate: tv::generate, validate: tv::validate },
            RegistryEntry { code: "TW", name: "UBN", generate: tw::generate, validate: tw::validate },
            RegistryEntry { code: "TZ", name: "Business ID", generate: tz::generate, validate: tz::validate },
            RegistryEntry { code: "UA", name: "EDRPOU", generate: ua::generate, validate: ua::validate },
            RegistryEntry { code: "UG", name: "Business ID", generate: ug::generate, validate: ug::validate },
            RegistryEntry { code: "UM", name: "Business ID", generate: um::generate, validate: um::validate },
            RegistryEntry { code: "US", name: "EIN", generate: us::generate, validate: us::validate },
            RegistryEntry { code: "UY", name: "RUT", generate: uy::generate, validate: uy::validate },
            RegistryEntry { code: "UZ", name: "Business ID", generate: uz::generate, validate: uz::validate },
            RegistryEntry { code: "VA", name: "Business ID", generate: va::generate, validate: va::validate },
            RegistryEntry { code: "VC", name: "Business ID", generate: vc::generate, validate: vc::validate },
            RegistryEntry { code: "VE", name: "RIF", generate: ve::generate, validate: ve::validate },
            RegistryEntry { code: "VG", name: "Business ID", generate: vg::generate, validate: vg::validate },
            RegistryEntry { code: "VI", name: "Business ID", generate: vi::generate, validate: vi::validate },
            RegistryEntry { code: "VN", name: "MST", generate: vn::generate, validate: vn::validate },
            RegistryEntry { code: "VU", name: "Business ID", generate: vu::generate, validate: vu::validate },
            RegistryEntry { code: "WF", name: "Business ID", generate: wf::generate, validate: wf::validate },
            RegistryEntry { code: "WS", name: "Business ID", generate: ws::generate, validate: ws::validate },
            RegistryEntry { code: "XK", name: "Business ID", generate: xk::generate, validate: xk::validate },
            RegistryEntry { code: "YE", name: "Business ID", generate: ye::generate, validate: ye::validate },
            RegistryEntry { code: "YT", name: "Business ID", generate: yt::generate, validate: yt::validate },
            RegistryEntry { code: "ZA", name: "VAT", generate: za::generate, validate: za::validate },
            RegistryEntry { code: "ZM", name: "Business ID", generate: zm::generate, validate: zm::validate },
            RegistryEntry { code: "ZW", name: "Business ID", generate: zw::generate, validate: zw::validate },
        ];
        Self { entries }
    }

    fn find(&self, country: &str) -> Option<&RegistryEntry> {
        self.entries.iter().find(|e| e.code == country)
    }

    pub fn generate(&self, opts: &GenOptions, rng: &mut rand::rngs::ThreadRng) -> Option<CompanyResult> {
        let country = opts
            .country
            .as_deref()
            .unwrap_or_else(|| {
                let countries = self.list_countries();
                countries[rng.gen_range(0..countries.len())].0
            })
            .to_uppercase();

        if !crate::countries::is_supported(&country) {
            return None;
        }

        if let Some(entry) = self.find(&country) {
            return Some(CompanyResult {
                country_code: entry.code.to_string(),
                country_name: crate::countries::get_country_name(&country).unwrap_or("Unknown").to_string(),
                name: entry.name.to_string(),
                code: (entry.generate)(rng),
                valid: true,
            });
        }

        // Fallback for territories mapped to parent countries but requested directly
        // (Though resolved aliases usually handled by caller or generic fallback in past)
        // With full 250 modules, we expect find() to succeed for all valid codes.
        
        None
    }

    pub fn validate(&self, country: &str, code: &str) -> bool {
        let country = country.to_uppercase();
        if !crate::countries::is_supported(&country) {
            return false;
        }
        if let Some(entry) = self.find(&country) {
            return (entry.validate)(code);
        }
        false
    }

    pub fn list_countries(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        let mut result = Vec::new();
        for entry in &self.entries {
            result.push((entry.code, crate::countries::get_country_name(entry.code).unwrap_or("Unknown"), entry.name));
        }
        result.sort_by_key(|(code, _, _)| *code);
        result
    }
}
