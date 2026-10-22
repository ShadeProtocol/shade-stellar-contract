//! Resource-cost measurements for the Shade hub contract.
//!
//! Regenerates the Shade rows of `docs/reference/resource-costs.md`:
//!
//! ```text
//! cargo test -p shade --test resource_costs -- --nocapture
//! ```
//!
//! Each measurement builds fresh state, resets the budget to unlimited (so
//! over-limit batches can still be measured), makes one top-level call, and
//! reads `env.cost_estimate().resources()`.

use shade::shade::{Shade, ShadeClient};
use shade::types::InvoiceFilter;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::StellarAssetClient;
use soroban_sdk::{vec, Address, Env, String, Vec};

const AMOUNT: i128 = 1_000;

// Per-transaction network limits used to decide whether a batch is safe.
// Source: Stellar network settings (Protocol 23). Update with the network.
const LIMIT_INSTRUCTIONS: i64 = 100_000_000;
const LIMIT_MEM_BYTES: i64 = 41_943_040;
const LIMIT_READ_ENTRIES: u32 = 100;
const LIMIT_WRITE_ENTRIES: u32 = 50;
const LIMIT_WRITE_BYTES: u32 = 132_096;
const LIMIT_EVENT_BYTES: u32 = 16_384;

struct Row {
    label: std::string::String,
    instructions: i64,
    mem_bytes: i64,
    read_entries: u32,
    write_entries: u32,
    write_bytes: u32,
    event_bytes: u32,
}

impl Row {
    fn within_limits(&self) -> bool {
        self.instructions <= LIMIT_INSTRUCTIONS
            && self.mem_bytes <= LIMIT_MEM_BYTES
            && self.read_entries <= LIMIT_READ_ENTRIES
            && self.write_entries <= LIMIT_WRITE_ENTRIES
            && self.write_bytes <= LIMIT_WRITE_BYTES
            && self.event_bytes <= LIMIT_EVENT_BYTES
    }

    fn print(&self) {
        std::println!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            self.label,
            self.instructions,
            self.mem_bytes,
            self.read_entries,
            self.write_entries,
            self.write_bytes,
            self.event_bytes,
            if self.within_limits() { "yes" } else { "no" }
        );
    }
}

fn header(first_column: &str) {
    std::println!(
        "| {} | instructions | memory bytes | read entries | write entries | write bytes | event bytes | within limits |",
        first_column
    );
    std::println!("|---|---|---|---|---|---|---|---|");
}

fn capture(env: &Env, label: std::string::String) -> Row {
    let r = env.cost_estimate().resources();
    Row {
        label,
        instructions: r.instructions,
        mem_bytes: r.mem_bytes,
        read_entries: r.disk_read_entries + r.memory_read_entries,
        write_entries: r.write_entries,
        write_bytes: r.write_bytes,
        event_bytes: r.contract_events_size_bytes,
    }
}

struct Fixture {
    env: Env,
    client: ShadeClient<'static>,
    token: Address,
    merchant: Address,
}

/// Shade initialised with one accepted token (5% fee) and one registered
/// merchant that has a payout account.
fn fixture() -> Fixture {
    let env = Env::default();
    env.mock_all_auths();
    let contract = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let token = env
        .register_stellar_asset_contract_v2(Address::generate(&env))
        .address();
    client.add_accepted_token(&admin, &token);
    client.set_fee(&admin, &token, &500);

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);
    client.set_merchant_account(&merchant, &Address::generate(&env));

    Fixture {
        env,
        client,
        token,
        merchant,
    }
}

fn new_invoice(f: &Fixture) -> u64 {
    f.client.create_invoice(
        &f.merchant,
        &String::from_str(&f.env, "Invoice"),
        &AMOUNT,
        &f.token,
        &None,
    )
}

fn measure_register_merchant() -> Row {
    let env = Env::default();
    env.mock_all_auths();
    let contract = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract);
    client.initialize(&Address::generate(&env));
    let merchant = Address::generate(&env);

    env.cost_estimate().budget().reset_unlimited();
    client.register_merchant(&merchant);
    capture(&env, "register_merchant".into())
}

fn measure_create_invoice() -> Row {
    let f = fixture();
    f.env.cost_estimate().budget().reset_unlimited();
    new_invoice(&f);
    capture(&f.env, "create_invoice".into())
}

fn measure_pay_invoice() -> Row {
    let f = fixture();
    let id = new_invoice(&f);
    let payer = Address::generate(&f.env);
    StellarAssetClient::new(&f.env, &f.token).mint(&payer, &AMOUNT);

    f.env.cost_estimate().budget().reset_unlimited();
    f.client.pay_invoice(&payer, &id);
    capture(&f.env, "pay_invoice".into())
}

fn measure_pay_invoices_batch(size: u32) -> Row {
    let f = fixture();
    let payer = Address::generate(&f.env);
    StellarAssetClient::new(&f.env, &f.token).mint(&payer, &(AMOUNT * size as i128));
    let mut ids: Vec<u64> = vec![&f.env];
    for _ in 0..size {
        ids.push_back(new_invoice(&f));
    }

    f.env.cost_estimate().budget().reset_unlimited();
    f.client.pay_invoices_batch(&payer, &ids);
    capture(&f.env, size.to_string())
}

fn measure_get_invoices(total: u32) -> Row {
    let f = fixture();
    for _ in 0..total {
        new_invoice(&f);
    }
    let filter = InvoiceFilter {
        status: None,
        merchant: None,
        min_amount: None,
        max_amount: None,
        start_date: None,
        end_date: None,
    };

    f.env.cost_estimate().budget().reset_unlimited();
    f.client.get_invoices(&filter);
    capture(&f.env, total.to_string())
}

#[test]
fn resource_costs_single_operations() {
    header("operation");
    for row in [
        measure_register_merchant(),
        measure_create_invoice(),
        measure_pay_invoice(),
    ] {
        row.print();
        assert!(
            row.within_limits(),
            "{} must fit in one transaction",
            row.label
        );
    }
}

#[test]
fn resource_costs_pay_invoices_batch() {
    let sizes = [1u32, 5, 10, 15, 16, 17, 18, 20, 30, 50];
    let mut max_safe = 0;
    header("batch size");
    for size in sizes {
        let row = measure_pay_invoices_batch(size);
        row.print();
        if row.within_limits() {
            max_safe = size;
        }
    }
    std::println!("largest measured batch within limits: {}", max_safe);
    assert!(
        max_safe >= 1,
        "a single payment must fit in one transaction"
    );
}

#[test]
fn resource_costs_get_invoices() {
    let totals = [1u32, 10, 25, 50, 75, 90, 96, 97, 98, 99, 100, 150];
    let mut max_safe = 0;
    header("stored invoices");
    for total in totals {
        let row = measure_get_invoices(total);
        row.print();
        if row.within_limits() {
            max_safe = total;
        }
    }
    std::println!("largest measured invoice count within limits: {}", max_safe);
    assert!(
        max_safe >= 1,
        "a query over one invoice must fit in one transaction"
    );
}
