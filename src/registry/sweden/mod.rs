use crate::GridOperator;

pub mod bjarke_energi;
pub mod btea;
pub mod ellevio;
pub mod eon;
pub mod eskilstuna_energi_miljo;
pub mod falbygdens_energi;
pub mod gislaved_energi;
pub mod goteborg_energi;
pub mod jbf;
pub mod jonkoping_energi;
pub mod karlstads_energi;
pub mod kraftringen;
pub mod kristinehamns_energi;
pub mod linde_energi;
pub mod malarenergi;
pub mod malung_salens_elverk;
pub mod nacka_energi;
pub mod partille_energi;
pub mod skovde_energi;
pub mod sollentuna_energi_miljo_ab;
pub mod tekniska_verken;
pub mod telge_energi;
pub mod vattenfall;
pub mod vaxjo_energi;

pub(crate) static SE_GRID_OPERATORS: &[GridOperator] = &[
    bjarke_energi::BJÄRKE_ENERGI,
    ellevio::ELLEVIO,
    eon::NORD,
    eon::STOCKHOLM,
    eon::SYD,
    eskilstuna_energi_miljo::ESKILSTUNA_ENERGI_MILJÖ,
    falbygdens_energi::FALBYGDENS_ENERGI,
    gislaved_energi::GISLAVED_ENERGI,
    goteborg_energi::GÖTEBORG_ENERGI,
    jbf::JBF,
    jonkoping_energi::JÖNKÖPING_ENERGI,
    karlstads_energi::KARLSTADS_ENERGI,
    kraftringen::KRAFTRINGEN,
    kristinehamns_energi::KRISTINEHAMNS_ENERGI,
    linde_energi::LINDE_ENERGI,
    malarenergi::MÄLARENERGI,
    nacka_energi::NACKA_ENERGI,
    partille_energi::PARTILLE_ENERGI,
    skovde_energi::SKÖVDE_ENERGI,
    sollentuna_energi_miljo_ab::SOLLENTUNA_ENERGI_MILJÖ_AB,
    tekniska_verken::KATRINEHOLM_ALTERNATIV,
    tekniska_verken::KATRINEHOLM_STANDARD,
    tekniska_verken::LINKÖPING_ALTERNATIV,
    tekniska_verken::LINKÖPING_STANDARD,
    telge_energi::TELGE_ENERGI,
    vattenfall::VATTENFALL_E4,
    vattenfall::VATTENFALL_T4,
    vaxjo_energi::VÄXJÖ_ENERGI,
];
