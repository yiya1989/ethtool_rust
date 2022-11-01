use ethtool::get_channel_max_combined;

fn main() {
    let max_channel = get_channel_max_combined("eth0");

    let max_combined = match max_channel {
        Ok(max_combined) => max_combined,
        Err(e) => {
            println!("max_combined error {}", e);
            0
        }
    };

    println!("max_combined {}", max_combined);
}
