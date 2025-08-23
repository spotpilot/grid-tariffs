# Grid Tariffs Library

Grid Tariffs is a Rust library providing data about electrical grid fees and tariffs for Swedish energy operators. The library contains detailed pricing information, power tariff calculations, and monetary handling for 24+ Swedish grid operators.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Initial Setup and Build
- Install Rust (if needed): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Build the library: `cargo build` -- takes ~4-7 seconds on clean build, ~1-2 seconds incremental
- Quick check build: `cargo check` -- takes ~1-4 seconds, useful for syntax validation
- Run all tests: `cargo test` -- takes <1 second. NEVER CANCEL, but very fast.
- Full clean build cycle: `cargo clean && cargo build && cargo test` -- takes ~12-15 seconds total

### Code Quality and Linting  
- Format code: `cargo fmt` -- takes <1 second, fixes formatting issues automatically
- Check formatting: `cargo fmt --check` -- takes <1 second, exits with error if formatting needed
- Run linter: `cargo clippy` -- takes ~1-4 seconds, shows warnings but should have no errors
- Generate documentation: `cargo doc` -- takes ~6 seconds
- View documentation: `cargo doc --open` -- opens docs in browser

### Timing Expectations
- **NEVER CANCEL** any cargo commands - they complete quickly (under 15 seconds even for clean builds)
- Dependency download (first time only): ~3-5 seconds
- Clean build from scratch: ~7-15 seconds  
- Incremental builds: ~1-4 seconds
- Test suite: <1 second (22 unit tests)
- Clippy linting: ~1-4 seconds
- Code formatting: <1 second

## Validation

### Manual Testing Scenarios
Always validate library functionality after making changes:
```rust
// Create a test to validate GridOperator lookup works
use crate::{GridOperator, country::Country};

// Test 1: Basic operator lookup
if let Some(operator) = GridOperator::get(Country::SE, "Karlstads Energi") {
    println!("✓ Found grid operator: {}", "Karlstads Energi");
} else {
    panic!("✗ Could not find Karlstads Energi");
}

// Test 2: Power tariff access  
if let Some(operator) = GridOperator::get(Country::SE, "Karlstads Energi") {
    if let Some(_tariff) = operator.power_tariff() {
        println!("✓ Power tariff data accessible");
    }
}
```

### CI Validation Steps
Always run before committing changes:
1. `cargo fmt` -- fix formatting automatically
2. `cargo clippy` -- should show warnings but no errors  
3. `cargo test` -- all 22 tests must pass
4. `cargo build` -- must build without errors

### Expected Test Results
The test suite should show:
```
running 22 tests
...all tests pass...
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured
```

## Codebase Navigation

### Key Locations
- **Library entry point**: `src/lib.rs` - Main public API with GridOperator struct
- **Money handling**: `src/money.rs` - Currency and monetary calculations (SEK/öre)
- **Grid operators data**: `src/registry/sweden/` - Individual operator definitions (24+ files)
- **Cost structures**: `src/costs.rs` - Tariff and fee calculation logic
- **Power tariffs**: `src/power_tariffs.rs` - Power consumption tariff calculations

### Important Files for Common Changes
- Adding new operators: `src/registry/sweden/` + update `src/registry/sweden/mod.rs`
- Modifying cost calculations: `src/costs.rs`  
- Changing monetary logic: `src/money.rs`
- Updating main API: `src/lib.rs`

### Registry Structure
Swedish operators are in `src/registry/sweden/`:
- Each operator has its own `.rs` file (e.g., `karlstads_energi.rs`)
- All operators are listed in `SE_GRID_OPERATORS` array in `mod.rs`
- Use the prelude module for common imports: `use crate::registry::prelude::*;`

## Common Tasks

### Repo Structure
```
src/
├── lib.rs              # Main library API  
├── money.rs            # Currency handling (SEK/öre)
├── costs.rs            # Cost calculation logic
├── power_tariffs.rs    # Power tariff structures
├── country.rs          # Country enum (SE)
├── currency.rs         # Currency enum (SEK)
├── registry/
│   └── sweden/
│       ├── mod.rs      # SE_GRID_OPERATORS array
│       ├── karlstads_energi.rs
│       ├── sollentuna_energi_miljo_ab.rs
│       └── ... (24+ operator files)
├── defs.rs             # Type definitions
├── fees.rs             # Fee structures  
├── links.rs            # External links
└── revenues.rs         # Revenue structures
```

### Key Dependencies (Cargo.toml)
```toml
chrono = "0.4.41"           # Date/time handling
chrono-tz = "0.10.4"        # Timezone support  
thiserror = "2.0.16"        # Error handling
```

### Public API Summary
```rust
// Main entry point
pub struct GridOperator { ... }

impl GridOperator {
    // Look up operator by country and name
    pub fn get(country: Country, name: &str) -> Option<&'static Self>
    
    // Access power tariff if available
    pub fn power_tariff(&self) -> Option<&PowerTariff>
}
```

### Sample Operator Names  
- "Karlstads Energi"
- "Sollentuna Energi & Miljö AB"
- "Tekniska Verken Linköping, prislista standard"
- "E.ON Stockholm" 
- "Ellevio"
- "Göteborg Energi"
- "Bjärke Energi"
- "Jönköping Energi"
- (see `src/registry/sweden/mod.rs` for complete list of 54 operators)

### Adding New Operators
1. Create new file in `src/registry/sweden/your_operator.rs`
2. Use the prelude: `use crate::registry::prelude::*;`
3. Define const: `pub(super) const YOUR_OPERATOR: GridOperator = ...`  
4. Add to mod.rs: `mod your_operator;` and add to `SE_GRID_OPERATORS` array
5. Run tests to validate: `cargo test`

### Debugging Common Issues
- **Formatting errors**: Run `cargo fmt` to auto-fix
- **Clippy warnings**: Check `src/currency.rs` and `src/money.rs` for known warnings
- **Import errors**: Use `use crate::registry::prelude::*;` for operator files
- **Build failures**: Check that all new operators are added to `SE_GRID_OPERATORS` in `mod.rs`

### Testing Changes
Always test library functionality after changes:
1. Build: `cargo build`
2. Test: `cargo test`  
3. Validate grid operator lookup works with known operators
4. Check that power tariff data is accessible where expected
5. Verify monetary calculations work correctly