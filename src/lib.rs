struct Account {
    balance: f64,
}

impl Account {
    fn new(balance: f64) -> Account {
        Account { balance: balance }
    }
}

#[no_mangle]
pub extern "C" fn account_new(balance: f64) -> *const libc::c_void {
    let account = Box::new(Account::new(balance));
    Box::into_raw(account) as *mut libc::c_void
}

#[no_mangle]
pub extern "C" fn account_get_balance(x: *const libc::c_void) -> f64 {
    let account: &mut Account = unsafe { &mut *(x as *mut Account) };
    return account.balance;
}

#[no_mangle]
pub extern "C" fn account_deposit(x: *const libc::c_void, f: f64) {
    let account: &mut Account = unsafe { &mut *(x as *mut Account) };
    account.balance += f;
}

#[no_mangle]
pub extern "C" fn account_withdraw(x: *const libc::c_void, f: f64) {
    let account: &mut Account = unsafe { &mut *(x as *mut Account) };
    account.balance -= f;
}

#[no_mangle]
pub extern "C" fn account_free(x: *const libc::c_void) {
    unsafe {
        drop(Box::from_raw(x as *mut Account));
    }
}
