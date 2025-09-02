use itertools::Itertools;
use quote::{ToTokens, format_ident, quote};
use std::{
    fs::{self, File, read_to_string},
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};
use syn::{Ident, Item, ItemConst, Visibility};

use convert_case::{Case, Casing};
use grid_tariffs::Country;
use tracing::{info, warn};

pub(crate) fn generate_grid_operator(
    country: Country,
    name: &str,
    vat_number: &str,
    fee_link: &str,
) -> anyhow::Result<()> {
    let mod_name = snakeify(name);
    let registry_filepath = workspace_dir()
        .join("src")
        .join("registry")
        .join(country.to_string().to_lowercase())
        .join(format!("{}.rs", mod_name));
    if registry_filepath.exists() {
        warn!(existing_path = %registry_filepath.to_string_lossy(), "already exists - doing nothing");
        return Ok(());
    }
    info!(%mod_name, "generating mod");
    let contents = grid_operator_contents(country, name, vat_number, fee_link);
    fs::write(&registry_filepath, contents)?;
    info!(saved_at = %registry_filepath.to_string_lossy());
    generate_mod(country)?;
    Ok(())
}

fn grid_operator_contents(
    country: Country,
    name: &str,
    vat_number: &str,
    fee_link: &str,
) -> String {
    let constant_name = name.replace(".", "").to_case(Case::Constant);
    let country_code = country.code();
    format!(
        r###"use crate::registry::prelude::*;

const FEE_LINK: &str = "{fee_link}";

pub const {constant_name}: GridOperator = GridOperator::builder()
    .name("{name}")
    .vat_number("{vat_number}")
    .country(Country::{country_code})
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(9999, 12, 31)
    .monthly_fee(Cost::Unverified)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::Unverified)
    .other_fees(OtherFees::Unverified)
    .links(Links::new(Link::builder(FEE_LINK).content_locator_default().build()))
    .power_tariff(PowerTariff::Unverified)
    .build();
"###
    )
}

pub(crate) fn generate_mod(country: Country) -> anyhow::Result<()> {
    tracing::info!("Generating code structure...");
    let registry_dir = registry_country_dir(country);
    let filepaths = filepaths_in_dir(&registry_dir)?
        .into_iter()
        .filter(|filepath| !filepath.ends_with("mod.rs"))
        .sorted()
        .collect_vec();
    let mod_includes = to_mod_idents(&filepaths)
        .into_iter()
        .map(|mod_ident| quote! { pub mod #mod_ident; });
    let grid_operator_entries = filepaths
        .iter()
        .map(|filepath| {
            let code = read_to_string(filepath).unwrap();
            let parsed = syn::parse_file(&code).unwrap();
            parsed
                .items
                .iter()
                .filter_map(|item| {
                    if let Item::Const(ItemConst { vis, ident, ty, .. }) = item {
                        if matches!(vis, Visibility::Public(_))
                            && ty.to_token_stream().to_string() == "GridOperator"
                        {
                            return Some((filepath, ident.to_owned()));
                        }
                    }
                    None
                })
                .collect_vec()
        })
        .flatten()
        .map(|(path, ident)| (format_ident!("{}", filename_to_mod_name(path)), ident))
        .map(|(mod_ident, const_ident)| {
            quote! {
                #mod_ident::#const_ident
            }
        })
        .collect_vec();
    let grid_operators = quote! {
        pub(crate) static GRID_OPERATORS: &[crate::GridOperator] = &[
            #(#grid_operator_entries),*
        ];
    };
    let mod_contents = quote! {
        #(#mod_includes)*
        #grid_operators
    };
    let mod_filepath = registry_dir.join("mod.rs");
    File::create(&mod_filepath)?.write_all(mod_contents.to_string().as_bytes())?;
    rustfmt(vec![&mod_filepath])?;
    info!(mod_filepath = %mod_filepath.to_string_lossy(), "written");
    Ok(())
}

fn registry_country_dir(country: Country) -> PathBuf {
    workspace_dir()
        .join("src")
        .join("registry")
        .join(country.to_string().to_lowercase())
}

fn workspace_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned()
}

fn rustfmt(filepaths: Vec<&Path>) -> anyhow::Result<()> {
    Command::new("rustfmt")
        .arg("--edition=2024")
        .args(filepaths)
        .status()?;
    Ok(())
}

fn filepaths_in_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut names = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if let Some(name) = entry.path().file_name() {
            names.push(dir.join(name));
        }
    }

    Ok(names)
}

fn filename_to_mod_name(filepath: &Path) -> &str {
    filepath
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .trim_end_matches(".rs")
}

fn to_mod_idents(filepaths: &[PathBuf]) -> Vec<Ident> {
    filepaths
        .iter()
        .map(|filepath| filename_to_mod_name(filepath))
        .map(|mod_name| format_ident!("{}", mod_name))
        .collect()
}

fn snakeify(operator_name: &str) -> String {
    slug::slugify(operator_name).to_case(Case::Snake)
}
