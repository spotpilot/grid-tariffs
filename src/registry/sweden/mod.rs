pub mod affarsverken_elnat_ab;
pub mod ale_el_elnat_ab;
pub mod bjarke_energi;
pub mod bodens_energi_nat_ab;
pub mod boo_energi_ek;
pub mod borlange_energi_elnat_ab;
pub mod btea;
pub mod c_4_elnat_ab;
pub mod dala_energi_elnat_ab;
pub mod ellevio;
pub mod eon;
pub mod eskilstuna_energi_miljo;
pub mod falbygdens_energi;
pub mod falkenberg_energi_elnat_ab;
pub mod falu_elnat_ab;
pub mod gislaved_energi;
pub mod goteborg_energi;
pub mod gotlands_elnat_ab;
pub mod halmstads_energi_och_miljo_nat_ab;
pub mod harjeans_nat_ab;
pub mod harnosand_elnat_ab;
pub mod harryda_energi_ab;
pub mod hoganas_energi_elnat_ab;
pub mod jamtkraft_elnat_ab;
pub mod jbf;
pub mod jonkoping_energi;
pub mod kalmar_energi_elnat_ab;
pub mod karlskoga_elnat_ab;
pub mod karlstads_energi;
pub mod kraftringen;
pub mod kristinehamns_energi;
pub mod kungalv_energi_ab;
pub mod landskrona_energi_ab;
pub mod lerum_nat_ab;
pub mod lidkoping_elnat_ab;
pub mod linde_energi;
pub mod lulea_energi_elnat_ab;
pub mod malarenergi;
pub mod malung_salens_elverk;
pub mod mjolby_kraftnat_ab;
pub mod molndal_energi_nat_ab;
pub mod nacka_energi;
pub mod natkraft_boras_infra_ab;
pub mod norrtalje_energi_ab;
pub mod olofstroms_kraft_nat_ab;
pub mod oskarshamn_energi_nat_ab;
pub mod partille_energi;
pub mod sandviken_energi_elnat_ab;
pub mod sevab_nat_ab;
pub mod skanska_energi_nat_ab;
pub mod skelleftea_kraft_elnat_ab;
pub mod skovde_energi;
pub mod sollentuna_energi_miljo_ab;
pub mod sundsvall_elnat_ab;
pub mod tekniska_verken;
pub mod telge_energi;
pub mod trelleborg_elnat_ab;
pub mod trollhattan_energi_elnat_ab;
pub mod uddevalla_energi_elnat_ab;
pub mod umea_energi_elnat_ab;
pub mod varberg_energi_elnat_ab;
pub mod vasterbergslagens_elnat_ab;
pub mod vattenfall;
pub mod vaxjo_energi;
pub(crate) static GRID_OPERATORS: &[crate::GridOperator] = &[
    affarsverken_elnat_ab::AFFÄRSVERKEN_ELNÄT_AB,
    ale_el_elnat_ab::ALE_EL_ELNÄT_AB,
    bjarke_energi::BJÄRKE_ENERGI,
    bodens_energi_nat_ab::BODENS_ENERGI_NÄT_AB,
    boo_energi_ek::BOO_ENERGI_EK,
    borlange_energi_elnat_ab::BORLÄNGE_ENERGI_ELNÄT_AB,
    btea::BTEA,
    c_4_elnat_ab::C_4_ELNÄT_AB,
    dala_energi_elnat_ab::DALA_ENERGI_ELNÄT_AB,
    ellevio::ELLEVIO,
    eon::EON,
    eskilstuna_energi_miljo::ESKILSTUNA_ENERGI_MILJÖ,
    falbygdens_energi::FALBYGDENS_ENERGI,
    falkenberg_energi_elnat_ab::FALKENBERG_ENERGI_ELNÄT_AB,
    falu_elnat_ab::FALU_ELNÄT_AB,
    gislaved_energi::GISLAVED_ENERGI,
    goteborg_energi::GÖTEBORG_ENERGI,
    gotlands_elnat_ab::GOTLANDS_ELNÄT_AB,
    halmstads_energi_och_miljo_nat_ab::HALMSTADS_ENERGI_OCH_MILJÖ_NÄT_AB,
    harjeans_nat_ab::HÄRJEÅNS_NÄT_AB,
    harnosand_elnat_ab::HÄRNÖSAND_ELNÄT_AB,
    harryda_energi_ab::HÄRRYDA_ENERGI_AB,
    hoganas_energi_elnat_ab::HÖGANÄS_ENERGI_ELNÄT_AB,
    jamtkraft_elnat_ab::JÄMTKRAFT_ELNÄT_AB,
    jbf::JBF,
    jonkoping_energi::JÖNKÖPING_ENERGI,
    kalmar_energi_elnat_ab::KALMAR_ENERGI_ELNÄT_AB,
    karlskoga_elnat_ab::KARLSKOGA_ELNÄT_AB,
    karlstads_energi::KARLSTADS_ENERGI,
    kraftringen::KRAFTRINGEN,
    kristinehamns_energi::KRISTINEHAMNS_ENERGI,
    kungalv_energi_ab::KUNGÄLV_ENERGI_AB,
    landskrona_energi_ab::LANDSKRONA_ENERGI_AB,
    lerum_nat_ab::LERUM_NÄT_AB,
    lidkoping_elnat_ab::LIDKÖPING_ELNÄT_AB,
    linde_energi::LINDE_ENERGI,
    lulea_energi_elnat_ab::LULEÅ_ENERGI_ELNÄT_AB,
    malarenergi::MÄLARENERGI,
    malung_salens_elverk::MALUNG_SÄLENS_ELVERK,
    mjolby_kraftnat_ab::MJÖLBY_KRAFTNÄT_AB,
    molndal_energi_nat_ab::MÖLNDAL_ENERGI_NÄT_AB,
    nacka_energi::NACKA_ENERGI,
    natkraft_boras_infra_ab::NÄTKRAFT_BORÅS_INFRA_AB,
    norrtalje_energi_ab::NORRTÄLJE_ENERGI_AB,
    olofstroms_kraft_nat_ab::OLOFSTRÖMS_KRAFT_NÄT_AB,
    oskarshamn_energi_nat_ab::OSKARSHAMN_ENERGI_NÄT_AB,
    partille_energi::PARTILLE_ENERGI,
    sandviken_energi_elnat_ab::SANDVIKEN_ENERGI_ELNÄT_AB,
    sevab_nat_ab::SEVAB_NÄT_AB,
    skanska_energi_nat_ab::SKÅNSKA_ENERGI_NÄT_AB,
    skelleftea_kraft_elnat_ab::SKELLEFTEÅ_KRAFT_ELNÄT_AB,
    skovde_energi::SKÖVDE_ENERGI,
    sollentuna_energi_miljo_ab::SOLLENTUNA_ENERGI_MILJÖ_AB,
    sundsvall_elnat_ab::SUNDSVALL_ELNÄT_AB,
    tekniska_verken::TEKNISKA_VERKEN_LINKÖPING,
    tekniska_verken::TEKNISKA_VERKEN_LINKÖPING,
    telge_energi::TELGE_ENERGI,
    trelleborg_elnat_ab::TRELLEBORG_ELNÄT_AB,
    trollhattan_energi_elnat_ab::TROLLHÄTTAN_ENERGI_ELNÄT_AB,
    uddevalla_energi_elnat_ab::UDDEVALLA_ENERGI_ELNÄT_AB,
    umea_energi_elnat_ab::UMEÅ_ENERGI_ELNÄT_AB,
    varberg_energi_elnat_ab::VARBERG_ENERGI_ELNÄT_AB,
    vasterbergslagens_elnat_ab::VÄSTERBERGSLAGENS_ELNÄT_AB,
    vattenfall::VATTENFALL,
    vaxjo_energi::VÄXJÖ_ENERGI,
];
