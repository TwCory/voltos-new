use bitflags::bitflags;
use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use slotmap::{SlotMap, new_key_type};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::SystemTime;

new_key_type! { pub struct IfKey; }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IfIndex(pub i32);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IfName(pub String);

#[derive(Clone, Debug)]
pub enum IfRoleType {
    Physical,
    Virtual,
}

#[derive(Clone, Debug)]
pub enum IfHardwareType {
    Unspec,
    NetRom,
    Ether,
    Eether,
    Ax25,
    ProNet,
    ChaosNet,
    Ieee802,
    ArcNet,
    Appletlk,
    Dlci,
    Atm,
    MetriCom,
    Ieee1394,
    Eui64,
    InfiniBand,
    Slip,
    Cslip,
    Slip6,
    Cslip6,
    Rsrvd,
    Adapt,
    Rose,
    X25,
    HwX25,
    Can,
    Mctp,
    Ppp,
    Cisco,
    Hdlc,
    Lapb,
    Ddcmp,
    RawHdlc,
    RawIp,
    Tunnel,
    Tunnel6,
    Frad,
    Skip,
    Loopback,
    Localtlk,
    Fddi,
    Bif,
    Sit,
    IpDdp,
    IpGre,
    PimReg,
    Hippi,
    Ash,
    EcoNet,
    Irda,
    FcPp,
    FcAl,
    FcPl,
    FcFabric,
    Ieee802Tr,
    Ieee80211,
    Ieee80211Prism,
    Ieee80211RadioTap,
    Ieee802154,
    Ieee802154Monitor,
    Phonet,
    PhonetPipe,
    Caif,
    Ip6Gre,
    Netlink,
    SixLoWpan
    VsockMon,
    Void,
    ZeroHdrLen,
}

#[derive(Clone, Debug)]
pub enum IfLinkType {
    Generic,
    BareUdp,
    BatAdv,
    Bond,
    Bridge,
    Dummy,
    Erspan,
    Fou,
    Geneve,
    Gre,
    GreTap,
    Ifb,
    Ip6Gre,
    Ip6GreTap,
    Ip6Tnl,
    IpIp,
    IpoIb,
    IpVlan,
    IpVtap,
    MacSec,
    MacVlan,
    MacVtap,
    NlMon,
    Sit,
    TunTap,
    Vcan,
    Veth,
    Vlan,
    Vrf,
    Vti,
    Vti6,
    VxCan,
    VxLan,
    WireGuard,
    Wlan,
    Xfrm,
}

#[derive(Clone, Debug)]
pub enum IfDuplexType {
    Unknown,
    Auto,
    Half,
    Full,
}

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct IfHardwareFlags: u64 {
        const SPEED                     = 1 << 0;
        const DUPLEX                    = 1 << 1;
        const MTU                       = 1 << 2;
        const AUTONEG                   = 1 << 3;
        const MAC_ADDR                  = 1 << 4;
        const BROADCAST                 = 1 << 5;
        const OFFLOAD_TX_CSUM           = 1 << 6;
        const OFFLOAD_RX_CSUM           = 1 << 7;
        const OFFLOAD_TSO               = 1 << 8;
        const OFFLOAD_UFO               = 1 << 9;
        const OFFLOAD_GSO               = 1 << 10;
        const OFFLOAD_GRO               = 1 << 11;
    }
}

#[derive(Clone, Debug)]
pub struct IfHardwareSet {
    pub flags:                          IfHardwareFlags,
    pub speed_current:                  Option<u32>,
    pub speed_value_support:            SmallVec<[u32; 8]>,
    pub duplex:                         Option<IfDuplexType>,
    pub autoneg:                        Option<bool>,
    pub mtu:                            Option<u32>,
    pub mtu_min_size:                   Option<u32>,
    pub mtu_max_size:                   Option<u32>,
    pub mac_addr:                       Option<[u8; 6]>,
}

// impl IfHardwareSet


bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct IfLinkFlags: u64 {
        const KEEPALIVE                 = 1 << 0;
        const LLDP                      = 1 << 1;
        const IP_ROUTING                = 1 << 2;
        const ARP                       = 1 << 3;
        const CDP                       = 1 << 4;
        const MPLS                      = 1 << 5;
        const CLNS_ROUTING              = 1 << 6;
        const DECNET_ROUTING            = 1 << 7;
        const DEVICE_SENSOR             = 1 << 8;
        const NAT64                     = 1 << 9;
        const NETBIOS                   = 1 << 10;
        const ONEP                      = 1 << 11;
        const ETHERNET_OAM              = 1 << 12;
        const LLC2                      = 1 << 13;
        const LOOPBACK                  = 1 << 14;
        const PPPOE                     = 1 << 15;
    }
}

#[derive(Clone, Debug)]
pub struct IfLinkSet {
    pub flags:                          IfLinkFlags,
    pub keepalive:                      Option<u32>,
}

// impl IfLinkSet

#[derive(Clone, Debug)]
pub struct IfAddrSet {
    pub ipv4:                           Vec<Ipv4Net>,
    pub ipv6:                           Vec<Ipv6Net>,
    pub mac_addr:                       Option<[u8; 6]>,
}

#[derive(Clone, Debug)]
pub struct Interface {
    pub key:                IfKey,
    pub name:               IfName,
    pub ifindex:            IfIndex,
    pub role:               IfRoleType,
    pub hw_type:            IfHardwareType,
    pub link_type:          IfLinkType,
    pub addr:               IfAddrSet,
    pub hw_set:             IfHardwareSet,
    pub link_set:           IfLinkSet,
    pub last_check:         SystemTime,
    pub up:                 bool,
}
