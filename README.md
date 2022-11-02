# Run example:
```
# cargo run --example channel
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/channel`
current channel_info: ethtool_channels { cmd: 60, max_rx: 0, max_tx: 0, max_other: 0, max_combined: 8, rx_count: 0, tx_count: 0, other_count: 0, combined_count: 8 }
max_combined1: 8
channel_info2: ethtool_channels { cmd: 61, max_rx: 0, max_tx: 0, max_other: 0, max_combined: 0, rx_count: 0, tx_count: 0, other_count: 0, combined_count: 7 }
max_combined2 7
channel_info3: ethtool_channels { cmd: 61, max_rx: 0, max_tx: 0, max_other: 0, max_combined: 0, rx_count: 0, tx_count: 0, other_count: 0, combined_count: 8 }
max_combined3 8
```

# How to use ethtool lib:
```
# example look up examples channel.rs

use ethtool;

# get channel info
let channel_info_result = ethtool::get_channel_info("eth0");

# set combined
let channel_info_result_ = ethtool::set_channel_combined("eth0", new_combined_value);

```