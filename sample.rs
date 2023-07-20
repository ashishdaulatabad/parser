use std::collections::HashMap;

fn main () -> std::io::Result<()> {
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).expect("Some error");
    let mut trimmed = line.trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let (n, x) = (trimmed[0], trimmed[1]);
    line = String::new();
    std::io::stdin().read_line(&mut line).expect("Some error");
    let val = line.trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    
    let mut map: std::collections::HashMap<u32, u32> = HashMap::new();

    for index in 0..val.len() {
        let elem = val[index];
        if let Some(res) = map.get(&(x - elem)) {
            println!("{} {}", res + 1, index + 1);
            return Ok(());
        }
        map.insert(elem, index as u32);
    }
    println!("IMPOSSIBLE");
    Ok(())
}