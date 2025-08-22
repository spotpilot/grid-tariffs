#[derive(Clone)]
pub(super) struct Links {
    /// Website containing info about the fees that the company charges
    pub(super) fee_info: &'static str,
    /// Link to public Eltariff-API endpoint (https://github.com/RI-SE/Eltariff-API)
    pub(super) eltariff_api: Option<&'static str>,
}
