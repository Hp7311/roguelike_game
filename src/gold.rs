/// 2 public functions to manage gold
use std::fs;

fn write_to_gold_file(amount: u32) -> std::io::Result<()> {
    fs::write("gold.txt", amount.to_string())?;
    Ok(())
}


fn get_gold_amount() -> std::io::Result<u32> {
    let gold_file = fs::read_to_string("gold.txt")?;
    
    let gold_amount: u32 = gold_file
        .trim()
        .parse()
        .expect("error converting file string to integer");
    
    Ok(gold_amount)
}

pub fn add_to_gold(amount: u32) -> std::io::Result<()> {
    let gold = get_gold();
    let total = gold + amount;
    println!("New gold amount: {}", total);
    write_to_gold_file(total)?;
    Ok(())
}

pub fn get_gold() -> u32 {
    match get_gold_amount() {
        Ok(gold) => gold,
        Err(_) => {
            fs::write("gold.txt", "0").unwrap();
            0
        },
    }
}

