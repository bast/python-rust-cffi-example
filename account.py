from cffi import FFI

_ffi = FFI()
_ffi.cdef("""
    struct Account* account_new(const double f);
    void account_deposit(const struct Account* account, const double f);
    void account_withdraw(const struct Account* account, const double f);
    double account_get_balance(const struct Account* account);
    void account_free(const struct Account* account);
""")
# hardcoded out of laziness
_lib = _ffi.dlopen("target/debug/libaccount.so")

new = _lib.account_new
deposit = _lib.account_deposit
withdraw = _lib.account_withdraw
get_balance = _lib.account_get_balance
free = _lib.account_free
