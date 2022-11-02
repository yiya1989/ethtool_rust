mod ethtool_channel;
mod ethtool_info;
mod ethtool_trait;
mod ifreq;

use ethtool_channel::ethtool_channels;

pub fn get_channel_info(interface_name: &str) -> Result<ethtool_channels, ifreq::IfError> {
    let ethtool_command = ethtool_channels {
        cmd: ethtool_info::ETHTOOL_GCHANNELS,
        ..Default::default()
    };

    let mut ethtool = ethtool_info::EthoolInfo::from_name(interface_name.to_owned())?;

    let result_ethtool_channels = ethtool.ioctl(ethtool_command)?;
    Ok(result_ethtool_channels)
}

pub fn set_channel_combined(
    interface_name: &str,
    combined: u32,
) -> Result<ethtool_channels, ifreq::IfError> {
    let ethtool_command = ethtool_channels {
        cmd: ethtool_info::ETHTOOL_SCHANNELS,
        combined_count: combined,
        ..Default::default()
    };

    let mut ethtool = ethtool_info::EthoolInfo::from_name(interface_name.to_owned())?;

    let result_ethtool_channels = ethtool.ioctl(ethtool_command)?;
    Ok(result_ethtool_channels)
}
