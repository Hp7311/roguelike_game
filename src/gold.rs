/// 2 public functions to manage gold
use std::{fs, io::Write};
use crossterm::{
    QueueableCommand, cursor::MoveTo, style::Print
};

fn write_to_gold_file(amount: u32) -> std::io::Result<()> {
    fs::write("gold.txt", amount.to_string())?;
    Ok(())
}


fn read_gold_file() -> std::io::Result<u32> {
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
    write_to_gold_file(total)?;
    Ok(())
}

fn get_gold() -> u32 {
    match read_gold_file() {
        Ok(gold) => gold,
        Err(_) => {
            fs::write("gold.txt", "0").unwrap();
            0
        },
    }
}


pub fn render_gold() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();

    stdout.queue(MoveTo(0, 0))?
        .queue(Print(
            format!("Gold: {}", get_gold())
        ))?;

    stdout.flush()?;

    Ok(())
}
