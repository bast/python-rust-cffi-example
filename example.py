import account

account1 = account.new(100.0)
account2 = account.new(100.0)

account.deposit(account1, 50.0)
account.deposit(account1, 50.0)

account.withdraw(account2, 25.0)

print('balance of account 1:', account.get_balance(account1))
print('balance of account 2:', account.get_balance(account2))

account.free(account1)
account.free(account2)
