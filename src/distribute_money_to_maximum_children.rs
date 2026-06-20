fn dist_money(money: i32, children: i32) -> i32 {
    if money < children {return -1};
    if money == children || money < 8 {return 0};

    let amount = money / 8;
    let remaining_money = money-amount*8;

    if amount > children {
        return children-1;
    }

    if amount < children {
        if children-amount==1 && remaining_money==4 {return amount-1;}
        if remaining_money >= children-amount {return amount;}
        let a= ((children-remaining_money-amount) as f32 / 8f32).ceil() as i32;
        let b = ((children-remaining_money-amount+a) as f32 / 8f32).ceil() as i32;
        return amount-b;
    }

    amount-(money-amount*8>0) as i32
}

pub fn main() {
    let money = 17;
    let children = 10;
    println!("{}", dist_money(money, children));
}
