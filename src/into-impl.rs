use std::convert::Into;

struct Money<T> {
    amount: T,
    currency: String
}

#[derive(Debug)]
struct CurrencylessMoney<T> {
    amount: T
}

// The First T == a declaration of the generic type T
// Second and Third are the usages of it.
impl<T> Into<CurrencylessMoney<t>> for Money<T> {
    fn into(self) -> CurrencylessMoney<T> {
        CurrencylessMoney { amount: self.amount }
    }
}

fn main() {
    let money = Money { amount 42, currency: "EUR".to_string() };
    let currencyless_money: CurrencylessMoney<u32> = money.into();

    println!("Money without currency: {:?}", currencyless_money);
}


