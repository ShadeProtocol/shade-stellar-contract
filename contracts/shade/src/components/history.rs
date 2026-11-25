use crate::types::{DataKey, PageInfo, Transaction, TransactionPage};
use soroban_sdk::{Address, Env, Vec};

/// Largest number of transactions one page can return. Larger `page_size`
/// values are clamped to this.
pub const MAX_TRANSACTION_PAGE_SIZE: u32 = 50;

/// Append `transaction` to the user's history.
///
/// Each transaction lives in its own entry (`UserTransaction(user, index)`,
/// indexed from 1) and a counter tracks how many exist, so a write touches two
/// small entries no matter how long the history is.
pub fn record_transaction(env: &Env, user: &Address, transaction: Transaction) {
    let index = get_user_transaction_count(env, user) + 1;
    env.storage()
        .persistent()
        .set(&DataKey::UserTransaction(user.clone(), index), &transaction);
    env.storage()
        .persistent()
        .set(&DataKey::UserTransactionCount(user.clone()), &index);
}

/// Number of transactions recorded for `user`.
pub fn get_user_transaction_count(env: &Env, user: &Address) -> u64 {
    env.storage()
        .persistent()
        .get(&DataKey::UserTransactionCount(user.clone()))
        .unwrap_or(0)
}

/// Return a page of `user`'s transactions, oldest first, starting after
/// `cursor` (exclusive).
///
/// Pass `cursor = 0` for the first page. A `page_size` of 0 or above
/// `MAX_TRANSACTION_PAGE_SIZE` is treated as `MAX_TRANSACTION_PAGE_SIZE`. `PageInfo.next_cursor` is the index of the
/// last transaction returned; use it as `cursor` in the next call. A
/// `next_cursor` of `0` means there are no more pages.
pub fn get_user_transactions_page(
    env: &Env,
    user: &Address,
    cursor: u64,
    page_size: u32,
) -> TransactionPage {
    let total = get_user_transaction_count(env, user);
    let size = if page_size == 0 || page_size > MAX_TRANSACTION_PAGE_SIZE {
        MAX_TRANSACTION_PAGE_SIZE
    } else {
        page_size
    };
    let limit = size as u64;
    let start = cursor.saturating_add(1);
    let end = start.saturating_add(limit - 1).min(total);

    let mut items: Vec<Transaction> = Vec::new(env);
    let mut index = start;
    while index <= end {
        if let Some(transaction) = env
            .storage()
            .persistent()
            .get::<_, Transaction>(&DataKey::UserTransaction(user.clone(), index))
        {
            items.push_back(transaction);
        }
        index += 1;
    }

    let has_next_page = start <= end && end < total;
    TransactionPage {
        page_info: PageInfo {
            count: items.len(),
            next_cursor: if has_next_page { end } else { 0 },
            has_next_page,
        },
        items,
    }
}

/// Deprecated: use [`get_user_transactions_page`].
///
/// Returns at most the first `MAX_TRANSACTION_PAGE_SIZE` transactions so the
/// call stays bounded.
pub fn get_user_transactions(env: &Env, user: Address) -> Vec<Transaction> {
    get_user_transactions_page(env, &user, 0, MAX_TRANSACTION_PAGE_SIZE).items
}
