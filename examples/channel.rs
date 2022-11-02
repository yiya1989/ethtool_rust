use ethtool;

fn main() {
    let channel_info_result = ethtool::get_channel_info("eth0");

    let channel_info = match channel_info_result {
        Ok(channel_info) => {
            println!("channel_info1: {}", channel_info);
            channel_info
        }
        Err(e) => {
            panic!("get channel_info failed, {}", e)
        }
    };
    println!("current channel_info {}", channel_info);
    println!("max_combined1 {}", channel_info.max_combined);

    let mut new_set_combined =
        std::cmp::min(channel_info.max_combined, channel_info.combined_count) - 1;

    new_set_combined = std::cmp::max(1, new_set_combined);

    let channel_info2 = ethtool::set_channel_combined("eth0", new_set_combined);
    let max_combined2 = match channel_info2 {
        Ok(channel_info) => {
            println!("channel_info2: {}", channel_info);
            channel_info.combined_count
        }
        Err(e) => {
            println!("max_combined error {}", e);
            0
        }
    };
    println!("max_combined2 {}", max_combined2);

    let channel_info3 = ethtool::set_channel_combined("eth0", channel_info.max_combined);
    let max_combined3 = match channel_info3 {
        Ok(channel_info) => {
            println!("channel_info3: {}", channel_info);
            channel_info.combined_count
        }
        Err(e) => {
            println!("max_combined error {}", e);
            0
        }
    };
    println!("max_combined3 {}", max_combined3);
}
