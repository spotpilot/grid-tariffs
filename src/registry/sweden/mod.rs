use crate::GridOperator;

mod bjarke_energi;
mod brittedal;
mod btea;
mod ellevio;
mod eon;
mod eskilstuna_energi_miljo;
mod falbygdens_energi;
mod gislaved_energi;
mod goteborg_energi;
mod jbf;
mod jonkoping_energi;
mod karlstads_energi;
mod kraftringen;
mod kristinehamns_energi;
mod linde_energi;
mod malarenergi;
mod malung_salens_elverk;
mod nacka_energi;
mod partille_energi;
mod skovde_energi;
mod sollentuna_energi_miljo_ab;
mod tekniska_verken;
mod telge_energi;
mod vaxjo_energi;

pub(crate) static SE_GRID_OPERATORS: &[GridOperator] = &[
    eon::SYD,
    eon::STOCKHOLM,
    eon::NORD,
    tekniska_verken::KATRINEHOLM_STANDARD,
    tekniska_verken::KATRINEHOLM_ALTERNATIV,
    tekniska_verken::LINKÖPING_STANDARD,
    tekniska_verken::LINKÖPING_ALTERNATIV,
    ellevio::ELLEVIO,
    goteborg_energi::GÖTEBORG_ENERGI,
    jonkoping_energi::JÖNKÖPING_ENERGI,
    skovde_energi::SKÖVDE_ENERGI,
    nacka_energi::NACKA_ENERGI,
    sollentuna_energi_miljo_ab::SOLLENTUNA_ENERGI_MILJÖ_AB,
    bjarke_energi::BJÄRKE_ENERGI,
    brittedal::BRITTEDAL,
    btea::BTEA,
    partille_energi::PARTILLE_ENERGI,
    falbygdens_energi::FALBYGDENS_ENERGI,
    linde_energi::LINDE_ENERGI,
    kristinehamns_energi::KRISTINEHAMNS_ENERGI,
    karlstads_energi::KARLSTADS_ENERGI,
    jbf::JBF,
    vaxjo_energi::VÄXJÖ_ENERGI,
    telge_energi::TELGE_ENERGI,
    malarenergi::MÄLARENERGI,
    eskilstuna_energi_miljo::ESKILSTUNA_ENERGI_MILJÖ,
    gislaved_energi::GISLAVED_ENERGI,
    kraftringen::KRAFTRINGEN,
];
