pub mod bjarke_energi;
pub mod btea;
pub mod ellevio;
pub mod eon;
pub mod eskilstuna_energi_miljo;
pub mod falbygdens_energi;
pub mod gislaved_energi;
pub mod goteborg_energi;
pub mod jamtkraft_elnat_ab;
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
pub(crate) static GRID_OPERATORS: &[crate::GridOperator] = &[
    bjarke_energi::BJÄRKE_ENERGI,
    btea::BTEA,
    ellevio::ELLEVIO,
    eon::SYD,
    eon::STOCKHOLM,
    eon::NORD,
    eskilstuna_energi_miljo::ESKILSTUNA_ENERGI_MILJÖ,
    falbygdens_energi::FALBYGDENS_ENERGI,
    gislaved_energi::GISLAVED_ENERGI,
    goteborg_energi::GÖTEBORG_ENERGI,
    jamtkraft_elnat_ab::JÄMTKRAFT_ELNÄT_AB,
    jbf::JBF,
    jonkoping_energi::JÖNKÖPING_ENERGI,
    karlstads_energi::KARLSTADS_ENERGI,
    kraftringen::KRAFTRINGEN,
    kristinehamns_energi::KRISTINEHAMNS_ENERGI,
    linde_energi::LINDE_ENERGI,
    malarenergi::MÄLARENERGI_FUSE_BASED,
    malarenergi::MÄLARENERGI_POWER_BASED,
    malung_salens_elverk::MALUNG_SÄLENS_ELVERK,
    nacka_energi::NACKA_ENERGI,
    partille_energi::PARTILLE_ENERGI,
    skovde_energi::SKÖVDE_ENERGI,
    sollentuna_energi_miljo_ab::SOLLENTUNA_ENERGI_MILJÖ_AB,
    tekniska_verken::LINKÖPING_STANDARD,
    tekniska_verken::LINKÖPING_ALTERNATIV,
    tekniska_verken::KATRINEHOLM_STANDARD,
    tekniska_verken::KATRINEHOLM_ALTERNATIV,
    telge_energi::TELGE_ENERGI,
    vattenfall::VATTENFALL_E4,
    vattenfall::VATTENFALL_T4,
    vaxjo_energi::VÄXJÖ_ENERGI,
];
