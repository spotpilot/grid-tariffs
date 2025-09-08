pub mod affarsverken_elnat_ab;
pub mod ale_el_elnat_ab;
pub mod alem_energi_elnat_ab;
pub mod alingsas_energi_elnat_ab;
pub mod almnas_elnat_ab;
pub mod alvesta_elnat_ab;
pub mod arvika_elnat_ab;
pub mod asele_elnat_ab;
pub mod bengtsfors_energi_nat_ab;
pub mod bjare_kraft_elnat_ab;
pub mod bjarke_energi;
pub mod blasjon_nat_ab;
pub mod bodens_energi_nat_ab;
pub mod boo_energi_ek;
pub mod borgholm_energi_elnat_ab;
pub mod borlange_energi_elnat_ab;
pub mod brittedals_elnat_ek_for;
pub mod bromolla_energi_vatten_ab;
pub mod btea;
pub mod c_4_elnat_ab;
pub mod dala_energi_elnat_ab;
pub mod degerfors_elnat_ab;
pub mod eksjo_elnat_ab;
pub mod ellevio;
pub mod emmaboda_elnat_ab;
pub mod eon;
pub mod eskilstuna_energi_miljo;
pub mod falbygdens_energi;
pub mod falkenberg_energi_elnat_ab;
pub mod falu_elnat_ab;
pub mod filipstad_energinat_ab;
pub mod gavle_energi_elnat_ab;
pub mod gislaved_energi;
pub mod goteborg_energi;
pub mod gotene_elforening;
pub mod gotlands_elnat_ab;
pub mod grastorp_energi_ek_for;
pub mod habo_energi_kraft_ab;
pub mod hallstaviks_elverk_ek_for;
pub mod halmstads_energi_och_miljo_nat_ab;
pub mod harjeans_nat_ab;
pub mod harnosand_elnat_ab;
pub mod harryda_energi_ab;
pub mod hedemora_energi_ab;
pub mod herrljunga_elnat_ab;
pub mod hjartums_elforening_ek_for;
pub mod hjo_elnat_ab;
pub mod hoganas_energi_elnat_ab;
pub mod jamtkraft_elnat_ab;
pub mod jbf;
pub mod jonkoping_energi;
pub mod kalmar_energi_elnat_ab;
pub mod karlsborgs_elnat_ab;
pub mod karlshamn_elnat_ab;
pub mod karlskoga_elnat_ab;
pub mod karlstads_energi;
pub mod kraftringen;
pub mod kristinehamns_energi;
pub mod kungalv_energi_ab;
pub mod kvanumbygdens_energi_ek_for;
pub mod landskrona_energi_ab;
pub mod lerum_nat_ab;
pub mod leva_nat_ab;
pub mod lidkoping_elnat_ab;
pub mod linde_energi;
pub mod ljungby_energinat_ab;
pub mod ljusdal_elnat_ab;
pub mod lulea_energi_elnat_ab;
pub mod malarenergi;
pub mod malung_salens_elverk;
pub mod mellersta_skanes_kraft_ek_for;
pub mod mjolby_kraftnat_ab;
pub mod molndal_energi_nat_ab;
pub mod nacka_energi;
pub mod nackans_elnat_ab;
pub mod nassjo_affarsverk_elnat_ab;
pub mod natkraft_boras_infra_ab;
pub mod njudung_savsjo_elnat_ab;
pub mod njudung_vetlanda_elnat_ab;
pub mod norrtalje_energi_ab;
pub mod nossebroortens_energi_ekonomisk_forening;
pub mod nybro_elnat_ab;
pub mod olofstroms_kraft_nat_ab;
pub mod oresundskraft_elnat_ab;
pub mod oskarshamn_energi_nat_ab;
pub mod osterlens_kraft_elnat_ab;
pub mod ostra_kinds_elnat_ab;
pub mod ovik_energi_nat_ab;
pub mod partille_energi;
pub mod pite_energi_elnat_ab;
pub mod ronneby_miljoteknik_elnat_ab;
pub mod salaheby_energi_elnat_ab;
pub mod sandhultsandareds_elnat_ab;
pub mod sandviken_energi_elnat_ab;
pub mod sevab_nat_ab;
pub mod sidensjo_vindkraft_elnat_ab;
pub mod sjobo_elnat_ab;
pub mod skanska_energi_nat_ab;
pub mod skara_elnat_ab;
pub mod skelleftea_kraft_elnat_ab;
pub mod skovde_energi;
pub mod skurups_elverk_ab;
pub mod skyllbergs_elnat_ab;
pub mod smedjebacken_energi_nat_ab;
pub mod soderhamn_elnat_ab;
pub mod sollentuna_energi_miljo_ab;
pub mod solvesborg_elnat_ab;
pub mod sturefors_eldistribution_ab;
pub mod sundsvall_elnat_ab;
pub mod tekniska_verken;
pub mod telge_energi;
pub mod tidaholms_elnat_ab;
pub mod tranas_energi_elnat_ab;
pub mod trelleborg_elnat_ab;
pub mod trollhattan_energi_elnat_ab;
pub mod uddevalla_energi_elnat_ab;
pub mod ulricehamn_energi_elnat_ab;
pub mod umea_energi_elnat_ab;
pub mod varberg_energi_elnat_ab;
pub mod varbergsortens_elnat_ab;
pub mod varnamo_elnat_ab;
pub mod vasterbergslagens_elnat_ab;
pub mod vasterviks_kraft_elnat_ab;
pub mod vattenfall;
pub mod vaxjo_energi;
pub mod vimmerby_energi_nat_ab;
pub mod ystad_elnat_ab;
pub(crate) static GRID_OPERATORS: &[crate::GridOperator] = &[
    affarsverken_elnat_ab::AFFARSVERKEN_ELNAT_AB,
    ale_el_elnat_ab::ALE_EL_ELNAT_AB,
    alem_energi_elnat_ab::ALEM_ENERGI_ELNAT_AB,
    alingsas_energi_elnat_ab::ALINGSAS_ENERGI_ELNAT_AB,
    almnas_elnat_ab::ALMNAS_ELNAT_AB,
    alvesta_elnat_ab::ALVESTA_ELNAT_AB,
    arvika_elnat_ab::ARVIKA_ELNAT_AB,
    asele_elnat_ab::ASELE_ELNAT_AB,
    bengtsfors_energi_nat_ab::BENGTSFORS_ENERGI_NAT_AB,
    bjare_kraft_elnat_ab::BJARE_KRAFT_ELNAT_AB,
    bjarke_energi::BJARKE_ENERGI,
    blasjon_nat_ab::BLASJON_NAT_AB,
    bodens_energi_nat_ab::BODENS_ENERGI_NAT_AB,
    boo_energi_ek::BOO_ENERGI_EK,
    borgholm_energi_elnat_ab::BORGHOLM_ENERGI_ELNAT_AB,
    borlange_energi_elnat_ab::BORLANGE_ENERGI_ELNAT_AB,
    brittedals_elnat_ek_for::BRITTEDALS_ELNAT_EK_FOR,
    bromolla_energi_vatten_ab::BROMOLLA_ENERGI_VATTEN_AB,
    btea::BTEA,
    c_4_elnat_ab::C_4_ELNAT_AB,
    dala_energi_elnat_ab::DALA_ENERGI_ELNAT_AB,
    degerfors_elnat_ab::DEGERFORS_ELNAT_AB,
    eksjo_elnat_ab::EKSJO_ELNAT_AB,
    ellevio::ELLEVIO,
    emmaboda_elnat_ab::EMMABODA_ELNAT_AB,
    eon::EON,
    eskilstuna_energi_miljo::ESKILSTUNA_ENERGI_MILJO,
    falbygdens_energi::FALBYGDENS_ENERGI,
    falkenberg_energi_elnat_ab::FALKENBERG_ENERGI_ELNAT_AB,
    falu_elnat_ab::FALU_ELNAT_AB,
    filipstad_energinat_ab::FILIPSTAD_ENERGINAT_AB,
    gavle_energi_elnat_ab::GAVLE_ENERGI_ELNAT_AB,
    gislaved_energi::GISLAVED_ENERGI,
    goteborg_energi::GOTEBORG_ENERGI,
    gotene_elforening::GOTENE_ELFORENING,
    gotlands_elnat_ab::GOTLANDS_ELNAT_AB,
    grastorp_energi_ek_for::GRASTORP_ENERGI_EK_FOR,
    habo_energi_kraft_ab::HABO_ENERGI_KRAFT_AB,
    hallstaviks_elverk_ek_for::HALLSTAVIKS_ELVERK_EK_FOR,
    halmstads_energi_och_miljo_nat_ab::HALMSTADS_ENERGI_OCH_MILJO_NAT_AB,
    harjeans_nat_ab::HARJEANS_NAT_AB,
    harnosand_elnat_ab::HARNOSAND_ELNAT_AB,
    harryda_energi_ab::HARRYDA_ENERGI_AB,
    hedemora_energi_ab::HEDEMORA_ENERGI_AB,
    herrljunga_elnat_ab::HERRLJUNGA_ELNAT_AB,
    hjartums_elforening_ek_for::HJARTUMS_ELFORENING_EK_FOR,
    hjo_elnat_ab::HJO_ELNAT_AB,
    hoganas_energi_elnat_ab::HOGANAS_ENERGI_ELNAT_AB,
    jamtkraft_elnat_ab::JAMTKRAFT_ELNAT_AB,
    jbf::JBF,
    jonkoping_energi::JONKOPING_ENERGI,
    kalmar_energi_elnat_ab::KALMAR_ENERGI_ELNAT_AB,
    karlsborgs_elnat_ab::KARLSBORGS_ELNAT_AB,
    karlshamn_elnat_ab::KARLSHAMN_ELNAT_AB,
    karlskoga_elnat_ab::KARLSKOGA_ELNAT_AB,
    karlstads_energi::KARLSTADS_ENERGI,
    kraftringen::KRAFTRINGEN,
    kristinehamns_energi::KRISTINEHAMNS_ENERGI,
    kungalv_energi_ab::KUNGALV_ENERGI_AB,
    kvanumbygdens_energi_ek_for::KVANUMBYGDENS_ENERGI_EK_FOR,
    landskrona_energi_ab::LANDSKRONA_ENERGI_AB,
    lerum_nat_ab::LERUM_NAT_AB,
    leva_nat_ab::LEVA_NAT_AB,
    lidkoping_elnat_ab::LIDKOPING_ELNAT_AB,
    linde_energi::LINDE_ENERGI,
    ljungby_energinat_ab::LJUNGBY_ENERGINAT_AB,
    ljusdal_elnat_ab::LJUSDAL_ELNAT_AB,
    lulea_energi_elnat_ab::LULEA_ENERGI_ELNAT_AB,
    malarenergi::MALARENERGI,
    malung_salens_elverk::MALUNG_SALENS_ELVERK,
    mellersta_skanes_kraft_ek_for::MELLERSTA_SKANES_KRAFT_EK_FOR,
    mjolby_kraftnat_ab::MJOLBY_KRAFTNAT_AB,
    molndal_energi_nat_ab::MOLNDAL_ENERGI_NAT_AB,
    nacka_energi::NACKA_ENERGI,
    nackans_elnat_ab::NACKANS_ELNAT_AB,
    nassjo_affarsverk_elnat_ab::NASSJO_AFFARSVERK_ELNAT_AB,
    natkraft_boras_infra_ab::NATKRAFT_BORAS_INFRA_AB,
    njudung_savsjo_elnat_ab::NJUDUNG_SAVSJO_ELNAT_AB,
    njudung_vetlanda_elnat_ab::NJUDUNG_VETLANDA_ELNAT_AB,
    norrtalje_energi_ab::NORRTALJE_ENERGI_AB,
    nossebroortens_energi_ekonomisk_forening::NOSSEBROORTENS_ENERGI_EKONOMISK_FORENING,
    nybro_elnat_ab::NYBRO_ELNAT_AB,
    olofstroms_kraft_nat_ab::OLOFSTROMS_KRAFT_NAT_AB,
    oresundskraft_elnat_ab::ORESUNDSKRAFT_ELNAT_AB,
    oskarshamn_energi_nat_ab::OSKARSHAMN_ENERGI_NAT_AB,
    osterlens_kraft_elnat_ab::OSTERLENS_KRAFT_ELNAT_AB,
    ostra_kinds_elnat_ab::OSTRA_KINDS_ELNAT_AB,
    ovik_energi_nat_ab::OVIK_ENERGI_NAT_AB,
    partille_energi::PARTILLE_ENERGI,
    pite_energi_elnat_ab::PITE_ENERGI_ELNAT_AB,
    ronneby_miljoteknik_elnat_ab::RONNEBY_MILJOTEKNIK_ELNAT_AB,
    salaheby_energi_elnat_ab::SALAHEBY_ENERGI_ELNAT_AB,
    sandhultsandareds_elnat_ab::SANDHULTSANDAREDS_ELNAT_AB,
    sandviken_energi_elnat_ab::SANDVIKEN_ENERGI_ELNAT_AB,
    sevab_nat_ab::SEVAB_NAT_AB,
    sidensjo_vindkraft_elnat_ab::SIDENSJO_VINDKRAFT_ELNAT_AB,
    sjobo_elnat_ab::SJOBO_ELNAT_AB,
    skanska_energi_nat_ab::SKANSKA_ENERGI_NAT_AB,
    skara_elnat_ab::SKARA_ELNAT_AB,
    skelleftea_kraft_elnat_ab::SKELLEFTEA_KRAFT_ELNAT_AB,
    skovde_energi::SKOVDE_ENERGI,
    skurups_elverk_ab::SKURUPS_ELVERK_AB,
    skyllbergs_elnat_ab::SKYLLBERGS_ELNAT_AB,
    smedjebacken_energi_nat_ab::SMEDJEBACKEN_ENERGI_NAT_AB,
    soderhamn_elnat_ab::SODERHAMN_ELNAT_AB,
    sollentuna_energi_miljo_ab::SOLLENTUNA_ENERGI_MILJO_AB,
    solvesborg_elnat_ab::SOLVESBORG_ELNAT_AB,
    sturefors_eldistribution_ab::STUREFORS_ELDISTRIBUTION_AB,
    sundsvall_elnat_ab::SUNDSVALL_ELNAT_AB,
    tekniska_verken::TEKNISKA_VERKEN_LINKOPING,
    tekniska_verken::TEKNISKA_VERKEN_KATRINEHOLM,
    telge_energi::TELGE_ENERGI,
    tidaholms_elnat_ab::TIDAHOLMS_ELNAT_AB,
    tranas_energi_elnat_ab::TRANAS_ENERGI_ELNAT_AB,
    trelleborg_elnat_ab::TRELLEBORG_ELNAT_AB,
    trollhattan_energi_elnat_ab::TROLLHATTAN_ENERGI_ELNAT_AB,
    uddevalla_energi_elnat_ab::UDDEVALLA_ENERGI_ELNAT_AB,
    ulricehamn_energi_elnat_ab::ULRICEHAMN_ENERGI_ELNAT_AB,
    umea_energi_elnat_ab::UMEA_ENERGI_ELNAT_AB,
    varberg_energi_elnat_ab::VARBERG_ENERGI_ELNAT_AB,
    varbergsortens_elnat_ab::VARBERGSORTENS_ELNAT_AB,
    varnamo_elnat_ab::VARNAMO_ELNAT_AB,
    vasterbergslagens_elnat_ab::VASTERBERGSLAGENS_ELNAT_AB,
    vasterviks_kraft_elnat_ab::VASTERVIKS_KRAFT_ELNAT_AB,
    vattenfall::VATTENFALL,
    vaxjo_energi::VAXJO_ENERGI,
    vimmerby_energi_nat_ab::VIMMERBY_ENERGI_NAT_AB,
    ystad_elnat_ab::YSTAD_ELNAT_AB,
];
