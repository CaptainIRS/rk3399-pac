#[doc = "Register `MAC_CONF` reader"]
pub type R = crate::R<MacConfSpec>;
#[doc = "Register `MAC_CONF` writer"]
pub type W = crate::W<MacConfSpec>;
#[doc = "Field `RE` reader - Receiver Enable\n\nWhen this bit is set, the receiver state machine of the GMAC is\n\nenabled for receiving frames from the GMII/MII. When this bit is\n\nreset, the GMAC receive state machine is disabled after the\n\ncompletion of the reception of the current frame, and will not\n\nreceive any further frames from the GMII/MII."]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Receiver Enable\n\nWhen this bit is set, the receiver state machine of the GMAC is\n\nenabled for receiving frames from the GMII/MII. When this bit is\n\nreset, the GMAC receive state machine is disabled after the\n\ncompletion of the reception of the current frame, and will not\n\nreceive any further frames from the GMII/MII."]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter Enable\n\nWhen this bit is set, the transmission state machine of the GMAC\n\nis enabled for transmission on the GMII/MII. When this bit is\n\nreset, the GMAC transmit state machine is disabled after the\n\ncompletion of the transmission of the current frame, and will not\n\ntransmit any further frames."]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter Enable\n\nWhen this bit is set, the transmission state machine of the GMAC\n\nis enabled for transmission on the GMII/MII. When this bit is\n\nreset, the GMAC transmit state machine is disabled after the\n\ncompletion of the transmission of the current frame, and will not\n\ntransmit any further frames."]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` reader - Deferral Check\n\nWhen this bit is set, the deferral check function is enabled in the\n\nGMAC. The GMAC will issue a Frame Abort status, along with the\n\nexcessive deferral error bit set in the transmit frame status when\n\nthe transmission state machine is deferred for more than 24,288\n\nbit times in 10/100-Mbps mode. If the Core is configured for\n\n1000 Mbps operation, the threshold for deferral is 155,680 bits\n\ntimes. Deferral begins when the transmitter is ready to transmit,\n\nbut is prevented because of an active CRS (carrier sense) signal\n\non the GMII/MII. Defer time is not cumulative. If the transmitter\n\ndefers for 10,000 bit times, then transmits, collides, backs off,\n\nand then has to defer again after completion of back-off, the\n\ndeferral timer resets to 0 and restarts.\n\nWhen this bit is reset, the deferral check function is disabled and\n\nthe GMAC defers until the CRS signal goes inactive."]
pub type DcR = crate::BitReader;
#[doc = "Field `DC` writer - Deferral Check\n\nWhen this bit is set, the deferral check function is enabled in the\n\nGMAC. The GMAC will issue a Frame Abort status, along with the\n\nexcessive deferral error bit set in the transmit frame status when\n\nthe transmission state machine is deferred for more than 24,288\n\nbit times in 10/100-Mbps mode. If the Core is configured for\n\n1000 Mbps operation, the threshold for deferral is 155,680 bits\n\ntimes. Deferral begins when the transmitter is ready to transmit,\n\nbut is prevented because of an active CRS (carrier sense) signal\n\non the GMII/MII. Defer time is not cumulative. If the transmitter\n\ndefers for 10,000 bit times, then transmits, collides, backs off,\n\nand then has to defer again after completion of back-off, the\n\ndeferral timer resets to 0 and restarts.\n\nWhen this bit is reset, the deferral check function is disabled and\n\nthe GMAC defers until the CRS signal goes inactive."]
pub type DcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Back-Off Limit\n\nThe Back-Off limit determines the random integer number (r) of\n\nslot time delays (4,096 bit times for 1000 Mbps and 512 bit times\n\nfor 10/100 Mbps) the GMAC waits before rescheduling a\n\ntransmission attempt during retries after a collision. This bit is\n\napplicable only to Half-Duplex mode and is reserved (RO) in Full-\n\nDuplex-only configuration.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bl {
    #[doc = "0: k = min (n, 10)"]
    B00 = 0,
    #[doc = "1: k = min (n, 8)"]
    B01 = 1,
    #[doc = "2: k = min (n, 4)"]
    B10 = 2,
    #[doc = "3: k = min (n, 1), Where n = retransmission attempt. The random integer r takes the value in the range 0 = r &lt; 2^k"]
    B11 = 3,
}
impl From<Bl> for u8 {
    #[inline(always)]
    fn from(variant: Bl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bl {
    type Ux = u8;
}
#[doc = "Field `BL` reader - Back-Off Limit\n\nThe Back-Off limit determines the random integer number (r) of\n\nslot time delays (4,096 bit times for 1000 Mbps and 512 bit times\n\nfor 10/100 Mbps) the GMAC waits before rescheduling a\n\ntransmission attempt during retries after a collision. This bit is\n\napplicable only to Half-Duplex mode and is reserved (RO) in Full-\n\nDuplex-only configuration."]
pub type BlR = crate::FieldReader<Bl>;
impl BlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bl {
        match self.bits {
            0 => Bl::B00,
            1 => Bl::B01,
            2 => Bl::B10,
            3 => Bl::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "k = min (n, 10)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Bl::B00
    }
    #[doc = "k = min (n, 8)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Bl::B01
    }
    #[doc = "k = min (n, 4)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Bl::B10
    }
    #[doc = "k = min (n, 1), Where n = retransmission attempt. The random integer r takes the value in the range 0 = r &lt; 2^k"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Bl::B11
    }
}
#[doc = "Field `BL` writer - Back-Off Limit\n\nThe Back-Off limit determines the random integer number (r) of\n\nslot time delays (4,096 bit times for 1000 Mbps and 512 bit times\n\nfor 10/100 Mbps) the GMAC waits before rescheduling a\n\ntransmission attempt during retries after a collision. This bit is\n\napplicable only to Half-Duplex mode and is reserved (RO) in Full-\n\nDuplex-only configuration."]
pub type BlW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Bl>;
impl<'a, REG> BlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "k = min (n, 10)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Bl::B00)
    }
    #[doc = "k = min (n, 8)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Bl::B01)
    }
    #[doc = "k = min (n, 4)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Bl::B10)
    }
    #[doc = "k = min (n, 1), Where n = retransmission attempt. The random integer r takes the value in the range 0 = r &lt; 2^k"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Bl::B11)
    }
}
#[doc = "Field `ACS` reader - Automatic Pad/CRC Stripping\n\nWhen this bit is set, the GMAC strips the Pad/FCS field on\n\nincoming frames only if the length's field value is less than or\n\nequal to 1,500 bytes. All received frames with length field greater\n\nthan or equal to 1,501 bytes are passed to the application\n\nwithout stripping the Pad/FCS field.\n\nWhen this bit is reset, the GMAC will pass all incoming frames to\n\nthe Host unmodified."]
pub type AcsR = crate::BitReader;
#[doc = "Field `ACS` writer - Automatic Pad/CRC Stripping\n\nWhen this bit is set, the GMAC strips the Pad/FCS field on\n\nincoming frames only if the length's field value is less than or\n\nequal to 1,500 bytes. All received frames with length field greater\n\nthan or equal to 1,501 bytes are passed to the application\n\nwithout stripping the Pad/FCS field.\n\nWhen this bit is reset, the GMAC will pass all incoming frames to\n\nthe Host unmodified."]
pub type AcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Link Up/Down\n\nIndicates whether the link is up or down during the transmission\n\nof configuration in RGMII interface:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lud {
    #[doc = "0: Link Down"]
    B0 = 0,
    #[doc = "1: Link Up"]
    B1 = 1,
}
impl From<Lud> for bool {
    #[inline(always)]
    fn from(variant: Lud) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LUD` reader - Link Up/Down\n\nIndicates whether the link is up or down during the transmission\n\nof configuration in RGMII interface:"]
pub type LudR = crate::BitReader<Lud>;
impl LudR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lud {
        match self.bits {
            false => Lud::B0,
            true => Lud::B1,
        }
    }
    #[doc = "Link Down"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lud::B0
    }
    #[doc = "Link Up"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lud::B1
    }
}
#[doc = "Field `LUD` writer - Link Up/Down\n\nIndicates whether the link is up or down during the transmission\n\nof configuration in RGMII interface:"]
pub type LudW<'a, REG> = crate::BitWriter<'a, REG, Lud>;
impl<'a, REG> LudW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Link Down"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lud::B0)
    }
    #[doc = "Link Up"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lud::B1)
    }
}
#[doc = "Field `DR` reader - Disable Retry\n\nWhen this bit is set, the GMAC will attempt only 1 transmission.\n\nWhen a collision occurs on the GMII/MII, the GMAC will ignore\n\nthe current frame transmission and report a Frame Abort with\n\nexcessive collision error in the transmit frame status.\n\nWhen this bit is reset, the GMAC will attempt retries based on the\n\nsettings of BL."]
pub type DrR = crate::BitReader;
#[doc = "Field `DR` writer - Disable Retry\n\nWhen this bit is set, the GMAC will attempt only 1 transmission.\n\nWhen a collision occurs on the GMII/MII, the GMAC will ignore\n\nthe current frame transmission and report a Frame Abort with\n\nexcessive collision error in the transmit frame status.\n\nWhen this bit is reset, the GMAC will attempt retries based on the\n\nsettings of BL."]
pub type DrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPC` reader - Checksum Offload\n\nWhen this bit is set, the GMAC calculates the 16-bit one's\n\ncomplement of the one's complement sum of all received\n\nEthernet frame payloads. It also checks whether the IPv4 Header\n\nchecksum (assumed to be bytes 25-26 or 29-30 (VLAN-tagged)\n\nof the received Ethernet frame) is correct for the received frame\n\nand gives the status in the receive status word. The GMAC core\n\nalso appends the 16-bit checksum calculated for the IP header\n\ndatagram payload (bytes after the IPv4 header) and appends it\n\nto the Ethernet frame transferred to the application (when Type 2\n\nCOE is deselected).\n\nWhen this bit is reset, this function is disabled.\n\nWhen Type 2 COE is selected, this bit, when set, enables IPv4\n\nchecksum checking for received frame payloads TCP/UDP/ICMP\n\nheaders. When this bit is reset, the COE function in the receiver\n\nis disabled and the corresponding PCE and IP HCE status bits are\n\nalways cleared."]
pub type IpcR = crate::BitReader;
#[doc = "Field `IPC` writer - Checksum Offload\n\nWhen this bit is set, the GMAC calculates the 16-bit one's\n\ncomplement of the one's complement sum of all received\n\nEthernet frame payloads. It also checks whether the IPv4 Header\n\nchecksum (assumed to be bytes 25-26 or 29-30 (VLAN-tagged)\n\nof the received Ethernet frame) is correct for the received frame\n\nand gives the status in the receive status word. The GMAC core\n\nalso appends the 16-bit checksum calculated for the IP header\n\ndatagram payload (bytes after the IPv4 header) and appends it\n\nto the Ethernet frame transferred to the application (when Type 2\n\nCOE is deselected).\n\nWhen this bit is reset, this function is disabled.\n\nWhen Type 2 COE is selected, this bit, when set, enables IPv4\n\nchecksum checking for received frame payloads TCP/UDP/ICMP\n\nheaders. When this bit is reset, the COE function in the receiver\n\nis disabled and the corresponding PCE and IP HCE status bits are\n\nalways cleared."]
pub type IpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - Duplex Mode\n\nWhen this bit is set, the GMAC operates in a Full-Duplex mode\n\nwhere it can transmit and receive simultaneously. This bit is RO\n\nwith default value of 1'b1 in Full-Duplex-only configuration."]
pub type DmR = crate::BitReader;
#[doc = "Field `DM` writer - Duplex Mode\n\nWhen this bit is set, the GMAC operates in a Full-Duplex mode\n\nwhere it can transmit and receive simultaneously. This bit is RO\n\nwith default value of 1'b1 in Full-Duplex-only configuration."]
pub type DmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - Loopback Mode\n\nWhen this bit is set, the GMAC operates in loopback mode at\n\nGMII/MII. The (G)MII Receive clock input (clk_rx_i) is required\n\nfor the loopback to work properly, as the Transmit clock is not\n\nlooped-back internally."]
pub type LmR = crate::BitReader;
#[doc = "Field `LM` writer - Loopback Mode\n\nWhen this bit is set, the GMAC operates in loopback mode at\n\nGMII/MII. The (G)MII Receive clock input (clk_rx_i) is required\n\nfor the loopback to work properly, as the Transmit clock is not\n\nlooped-back internally."]
pub type LmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO` reader - Disable Receive Own\n\nWhen this bit is set, the GMAC disables the reception of frames\n\nwhen the gmii_txen_o is asserted in Half-Duplex mode.\n\nWhen this bit is reset, the GMAC receives all packets that are\n\ngiven by the PHY while transmitting."]
pub type DoR = crate::BitReader;
#[doc = "Field `DO` writer - Disable Receive Own\n\nWhen this bit is set, the GMAC disables the reception of frames\n\nwhen the gmii_txen_o is asserted in Half-Duplex mode.\n\nWhen this bit is reset, the GMAC receives all packets that are\n\ngiven by the PHY while transmitting."]
pub type DoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Speed\n\nIndicates the speed in Fast Ethernet (MII) mode:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fes {
    #[doc = "0: 10 Mbps"]
    B0 = 0,
    #[doc = "1: 100 Mbps"]
    B1 = 1,
}
impl From<Fes> for bool {
    #[inline(always)]
    fn from(variant: Fes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FES` reader - Speed\n\nIndicates the speed in Fast Ethernet (MII) mode:"]
pub type FesR = crate::BitReader<Fes>;
impl FesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fes {
        match self.bits {
            false => Fes::B0,
            true => Fes::B1,
        }
    }
    #[doc = "10 Mbps"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fes::B0
    }
    #[doc = "100 Mbps"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fes::B1
    }
}
#[doc = "Field `FES` writer - Speed\n\nIndicates the speed in Fast Ethernet (MII) mode:"]
pub type FesW<'a, REG> = crate::BitWriter<'a, REG, Fes>;
impl<'a, REG> FesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10 Mbps"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fes::B0)
    }
    #[doc = "100 Mbps"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fes::B1)
    }
}
#[doc = "Port Select\n\nSelects between GMII and MII:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ps {
    #[doc = "0: GMII (1000 Mbps)"]
    B0 = 0,
    #[doc = "1: MII (10/100 Mbps)"]
    B1 = 1,
}
impl From<Ps> for bool {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS` reader - Port Select\n\nSelects between GMII and MII:"]
pub type PsR = crate::BitReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            false => Ps::B0,
            true => Ps::B1,
        }
    }
    #[doc = "GMII (1000 Mbps)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ps::B0
    }
    #[doc = "MII (10/100 Mbps)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ps::B1
    }
}
#[doc = "Field `PS` writer - Port Select\n\nSelects between GMII and MII:"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG, Ps>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GMII (1000 Mbps)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B0)
    }
    #[doc = "MII (10/100 Mbps)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B1)
    }
}
#[doc = "Field `DCRS` reader - Disable Carrier Sense During Transmission\n\nWhen set high, this bit makes the MAC transmitter ignore the\n\n(G)MII CRS signal during frame transmission in Half-Duplex\n\nmode. This request results in no errors generated due to Loss of\n\nCarrier or No Carrier during such transmission. When this bit is\n\nlow, the MAC transmitter generates such errors due to Carrier\n\nSense and will even abort the transmissions."]
pub type DcrsR = crate::BitReader;
#[doc = "Field `DCRS` writer - Disable Carrier Sense During Transmission\n\nWhen set high, this bit makes the MAC transmitter ignore the\n\n(G)MII CRS signal during frame transmission in Half-Duplex\n\nmode. This request results in no errors generated due to Loss of\n\nCarrier or No Carrier during such transmission. When this bit is\n\nlow, the MAC transmitter generates such errors due to Carrier\n\nSense and will even abort the transmissions."]
pub type DcrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Inter-Frame Gap\n\nThese bits control the minimum IFG between frames during\n\ntransmission.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ifg {
    #[doc = "0: 96 bit times"]
    B000 = 0,
    #[doc = "1: 88 bit times"]
    B001 = 1,
    #[doc = "2: 80 bit times ..."]
    B010 = 2,
    #[doc = "7: 40 bit times"]
    B111 = 7,
}
impl From<Ifg> for u8 {
    #[inline(always)]
    fn from(variant: Ifg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ifg {
    type Ux = u8;
}
#[doc = "Field `IFG` reader - Inter-Frame Gap\n\nThese bits control the minimum IFG between frames during\n\ntransmission."]
pub type IfgR = crate::FieldReader<Ifg>;
impl IfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ifg> {
        match self.bits {
            0 => Some(Ifg::B000),
            1 => Some(Ifg::B001),
            2 => Some(Ifg::B010),
            7 => Some(Ifg::B111),
            _ => None,
        }
    }
    #[doc = "96 bit times"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Ifg::B000
    }
    #[doc = "88 bit times"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Ifg::B001
    }
    #[doc = "80 bit times ..."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Ifg::B010
    }
    #[doc = "40 bit times"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Ifg::B111
    }
}
#[doc = "Field `IFG` writer - Inter-Frame Gap\n\nThese bits control the minimum IFG between frames during\n\ntransmission."]
pub type IfgW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ifg>;
impl<'a, REG> IfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "96 bit times"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Ifg::B000)
    }
    #[doc = "88 bit times"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Ifg::B001)
    }
    #[doc = "80 bit times ..."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Ifg::B010)
    }
    #[doc = "40 bit times"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Ifg::B111)
    }
}
#[doc = "Field `BE` reader - Frame Burst Enable\n\nWhen this bit is set, the GMAC allows frame bursting during\n\ntransmission in GMII Half-Duplex mode."]
pub type BeR = crate::BitReader;
#[doc = "Field `BE` writer - Frame Burst Enable\n\nWhen this bit is set, the GMAC allows frame bursting during\n\ntransmission in GMII Half-Duplex mode."]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JD` reader - Jabber Disable\n\nWhen this bit is set, the GMAC disables the jabber timer on the\n\ntransmitter, and can transfer frames of up to 16,384 bytes.\n\nWhen this bit is reset, the GMAC cuts off the transmitter if the\n\napplication sends out more than 2,048 bytes of data (10,240 if JE\n\nis set high) during transmission."]
pub type JdR = crate::BitReader;
#[doc = "Field `JD` writer - Jabber Disable\n\nWhen this bit is set, the GMAC disables the jabber timer on the\n\ntransmitter, and can transfer frames of up to 16,384 bytes.\n\nWhen this bit is reset, the GMAC cuts off the transmitter if the\n\napplication sends out more than 2,048 bytes of data (10,240 if JE\n\nis set high) during transmission."]
pub type JdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD` reader - Watchdog Disable\n\nWhen this bit is set, the GMAC disables the watchdog timer on\n\nthe receiver, and can receive frames of up to 16,384 bytes.\n\nWhen this bit is reset, the GMAC allows no more than 2,048 bytes\n\n(10,240 if JE is set high) of the frame being received and cuts off\n\nany bytes received after that."]
pub type WdR = crate::BitReader;
#[doc = "Field `WD` writer - Watchdog Disable\n\nWhen this bit is set, the GMAC disables the watchdog timer on\n\nthe receiver, and can receive frames of up to 16,384 bytes.\n\nWhen this bit is reset, the GMAC allows no more than 2,048 bytes\n\n(10,240 if JE is set high) of the frame being received and cuts off\n\nany bytes received after that."]
pub type WdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Transmit Configuration in RGMII\n\nWhen set, this bit enables the transmission of duplex mode, link\n\nspeed, and link up/down information to the PHY in the RGMII\n\nports. When this bit is reset, no such information is driven to the\n\nPHY."]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - Transmit Configuration in RGMII\n\nWhen set, this bit enables the transmission of duplex mode, link\n\nspeed, and link up/down information to the PHY in the RGMII\n\nports. When this bit is reset, no such information is driven to the\n\nPHY."]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Receiver Enable\n\nWhen this bit is set, the receiver state machine of the GMAC is\n\nenabled for receiving frames from the GMII/MII. When this bit is\n\nreset, the GMAC receive state machine is disabled after the\n\ncompletion of the reception of the current frame, and will not\n\nreceive any further frames from the GMII/MII."]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable\n\nWhen this bit is set, the transmission state machine of the GMAC\n\nis enabled for transmission on the GMII/MII. When this bit is\n\nreset, the GMAC transmit state machine is disabled after the\n\ncompletion of the transmission of the current frame, and will not\n\ntransmit any further frames."]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral Check\n\nWhen this bit is set, the deferral check function is enabled in the\n\nGMAC. The GMAC will issue a Frame Abort status, along with the\n\nexcessive deferral error bit set in the transmit frame status when\n\nthe transmission state machine is deferred for more than 24,288\n\nbit times in 10/100-Mbps mode. If the Core is configured for\n\n1000 Mbps operation, the threshold for deferral is 155,680 bits\n\ntimes. Deferral begins when the transmitter is ready to transmit,\n\nbut is prevented because of an active CRS (carrier sense) signal\n\non the GMII/MII. Defer time is not cumulative. If the transmitter\n\ndefers for 10,000 bit times, then transmits, collides, backs off,\n\nand then has to defer again after completion of back-off, the\n\ndeferral timer resets to 0 and restarts.\n\nWhen this bit is reset, the deferral check function is disabled and\n\nthe GMAC defers until the CRS signal goes inactive."]
    #[inline(always)]
    pub fn dc(&self) -> DcR {
        DcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit\n\nThe Back-Off limit determines the random integer number (r) of\n\nslot time delays (4,096 bit times for 1000 Mbps and 512 bit times\n\nfor 10/100 Mbps) the GMAC waits before rescheduling a\n\ntransmission attempt during retries after a collision. This bit is\n\napplicable only to Half-Duplex mode and is reserved (RO) in Full-\n\nDuplex-only configuration."]
    #[inline(always)]
    pub fn bl(&self) -> BlR {
        BlR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic Pad/CRC Stripping\n\nWhen this bit is set, the GMAC strips the Pad/FCS field on\n\nincoming frames only if the length's field value is less than or\n\nequal to 1,500 bytes. All received frames with length field greater\n\nthan or equal to 1,501 bytes are passed to the application\n\nwithout stripping the Pad/FCS field.\n\nWhen this bit is reset, the GMAC will pass all incoming frames to\n\nthe Host unmodified."]
    #[inline(always)]
    pub fn acs(&self) -> AcsR {
        AcsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Link Up/Down\n\nIndicates whether the link is up or down during the transmission\n\nof configuration in RGMII interface:"]
    #[inline(always)]
    pub fn lud(&self) -> LudR {
        LudR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable Retry\n\nWhen this bit is set, the GMAC will attempt only 1 transmission.\n\nWhen a collision occurs on the GMII/MII, the GMAC will ignore\n\nthe current frame transmission and report a Frame Abort with\n\nexcessive collision error in the transmit frame status.\n\nWhen this bit is reset, the GMAC will attempt retries based on the\n\nsettings of BL."]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Checksum Offload\n\nWhen this bit is set, the GMAC calculates the 16-bit one's\n\ncomplement of the one's complement sum of all received\n\nEthernet frame payloads. It also checks whether the IPv4 Header\n\nchecksum (assumed to be bytes 25-26 or 29-30 (VLAN-tagged)\n\nof the received Ethernet frame) is correct for the received frame\n\nand gives the status in the receive status word. The GMAC core\n\nalso appends the 16-bit checksum calculated for the IP header\n\ndatagram payload (bytes after the IPv4 header) and appends it\n\nto the Ethernet frame transferred to the application (when Type 2\n\nCOE is deselected).\n\nWhen this bit is reset, this function is disabled.\n\nWhen Type 2 COE is selected, this bit, when set, enables IPv4\n\nchecksum checking for received frame payloads TCP/UDP/ICMP\n\nheaders. When this bit is reset, the COE function in the receiver\n\nis disabled and the corresponding PCE and IP HCE status bits are\n\nalways cleared."]
    #[inline(always)]
    pub fn ipc(&self) -> IpcR {
        IpcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode\n\nWhen this bit is set, the GMAC operates in a Full-Duplex mode\n\nwhere it can transmit and receive simultaneously. This bit is RO\n\nwith default value of 1'b1 in Full-Duplex-only configuration."]
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode\n\nWhen this bit is set, the GMAC operates in loopback mode at\n\nGMII/MII. The (G)MII Receive clock input (clk_rx_i) is required\n\nfor the loopback to work properly, as the Transmit clock is not\n\nlooped-back internally."]
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable Receive Own\n\nWhen this bit is set, the GMAC disables the reception of frames\n\nwhen the gmii_txen_o is asserted in Half-Duplex mode.\n\nWhen this bit is reset, the GMAC receives all packets that are\n\ngiven by the PHY while transmitting."]
    #[inline(always)]
    pub fn do_(&self) -> DoR {
        DoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Speed\n\nIndicates the speed in Fast Ethernet (MII) mode:"]
    #[inline(always)]
    pub fn fes(&self) -> FesR {
        FesR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Select\n\nSelects between GMII and MII:"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission\n\nWhen set high, this bit makes the MAC transmitter ignore the\n\n(G)MII CRS signal during frame transmission in Half-Duplex\n\nmode. This request results in no errors generated due to Loss of\n\nCarrier or No Carrier during such transmission. When this bit is\n\nlow, the MAC transmitter generates such errors due to Carrier\n\nSense and will even abort the transmissions."]
    #[inline(always)]
    pub fn dcrs(&self) -> DcrsR {
        DcrsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap\n\nThese bits control the minimum IFG between frames during\n\ntransmission."]
    #[inline(always)]
    pub fn ifg(&self) -> IfgR {
        IfgR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 21 - Frame Burst Enable\n\nWhen this bit is set, the GMAC allows frame bursting during\n\ntransmission in GMII Half-Duplex mode."]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Jabber Disable\n\nWhen this bit is set, the GMAC disables the jabber timer on the\n\ntransmitter, and can transfer frames of up to 16,384 bytes.\n\nWhen this bit is reset, the GMAC cuts off the transmitter if the\n\napplication sends out more than 2,048 bytes of data (10,240 if JE\n\nis set high) during transmission."]
    #[inline(always)]
    pub fn jd(&self) -> JdR {
        JdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog Disable\n\nWhen this bit is set, the GMAC disables the watchdog timer on\n\nthe receiver, and can receive frames of up to 16,384 bytes.\n\nWhen this bit is reset, the GMAC allows no more than 2,048 bytes\n\n(10,240 if JE is set high) of the frame being received and cuts off\n\nany bytes received after that."]
    #[inline(always)]
    pub fn wd(&self) -> WdR {
        WdR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit Configuration in RGMII\n\nWhen set, this bit enables the transmission of duplex mode, link\n\nspeed, and link up/down information to the PHY in the RGMII\n\nports. When this bit is reset, no such information is driven to the\n\nPHY."]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Receiver Enable\n\nWhen this bit is set, the receiver state machine of the GMAC is\n\nenabled for receiving frames from the GMII/MII. When this bit is\n\nreset, the GMAC receive state machine is disabled after the\n\ncompletion of the reception of the current frame, and will not\n\nreceive any further frames from the GMII/MII."]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> ReW<MacConfSpec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Enable\n\nWhen this bit is set, the transmission state machine of the GMAC\n\nis enabled for transmission on the GMII/MII. When this bit is\n\nreset, the GMAC transmit state machine is disabled after the\n\ncompletion of the transmission of the current frame, and will not\n\ntransmit any further frames."]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<MacConfSpec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral Check\n\nWhen this bit is set, the deferral check function is enabled in the\n\nGMAC. The GMAC will issue a Frame Abort status, along with the\n\nexcessive deferral error bit set in the transmit frame status when\n\nthe transmission state machine is deferred for more than 24,288\n\nbit times in 10/100-Mbps mode. If the Core is configured for\n\n1000 Mbps operation, the threshold for deferral is 155,680 bits\n\ntimes. Deferral begins when the transmitter is ready to transmit,\n\nbut is prevented because of an active CRS (carrier sense) signal\n\non the GMII/MII. Defer time is not cumulative. If the transmitter\n\ndefers for 10,000 bit times, then transmits, collides, backs off,\n\nand then has to defer again after completion of back-off, the\n\ndeferral timer resets to 0 and restarts.\n\nWhen this bit is reset, the deferral check function is disabled and\n\nthe GMAC defers until the CRS signal goes inactive."]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DcW<MacConfSpec> {
        DcW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Back-Off Limit\n\nThe Back-Off limit determines the random integer number (r) of\n\nslot time delays (4,096 bit times for 1000 Mbps and 512 bit times\n\nfor 10/100 Mbps) the GMAC waits before rescheduling a\n\ntransmission attempt during retries after a collision. This bit is\n\napplicable only to Half-Duplex mode and is reserved (RO) in Full-\n\nDuplex-only configuration."]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BlW<MacConfSpec> {
        BlW::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic Pad/CRC Stripping\n\nWhen this bit is set, the GMAC strips the Pad/FCS field on\n\nincoming frames only if the length's field value is less than or\n\nequal to 1,500 bytes. All received frames with length field greater\n\nthan or equal to 1,501 bytes are passed to the application\n\nwithout stripping the Pad/FCS field.\n\nWhen this bit is reset, the GMAC will pass all incoming frames to\n\nthe Host unmodified."]
    #[inline(always)]
    #[must_use]
    pub fn acs(&mut self) -> AcsW<MacConfSpec> {
        AcsW::new(self, 7)
    }
    #[doc = "Bit 8 - Link Up/Down\n\nIndicates whether the link is up or down during the transmission\n\nof configuration in RGMII interface:"]
    #[inline(always)]
    #[must_use]
    pub fn lud(&mut self) -> LudW<MacConfSpec> {
        LudW::new(self, 8)
    }
    #[doc = "Bit 9 - Disable Retry\n\nWhen this bit is set, the GMAC will attempt only 1 transmission.\n\nWhen a collision occurs on the GMII/MII, the GMAC will ignore\n\nthe current frame transmission and report a Frame Abort with\n\nexcessive collision error in the transmit frame status.\n\nWhen this bit is reset, the GMAC will attempt retries based on the\n\nsettings of BL."]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DrW<MacConfSpec> {
        DrW::new(self, 9)
    }
    #[doc = "Bit 10 - Checksum Offload\n\nWhen this bit is set, the GMAC calculates the 16-bit one's\n\ncomplement of the one's complement sum of all received\n\nEthernet frame payloads. It also checks whether the IPv4 Header\n\nchecksum (assumed to be bytes 25-26 or 29-30 (VLAN-tagged)\n\nof the received Ethernet frame) is correct for the received frame\n\nand gives the status in the receive status word. The GMAC core\n\nalso appends the 16-bit checksum calculated for the IP header\n\ndatagram payload (bytes after the IPv4 header) and appends it\n\nto the Ethernet frame transferred to the application (when Type 2\n\nCOE is deselected).\n\nWhen this bit is reset, this function is disabled.\n\nWhen Type 2 COE is selected, this bit, when set, enables IPv4\n\nchecksum checking for received frame payloads TCP/UDP/ICMP\n\nheaders. When this bit is reset, the COE function in the receiver\n\nis disabled and the corresponding PCE and IP HCE status bits are\n\nalways cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ipc(&mut self) -> IpcW<MacConfSpec> {
        IpcW::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex Mode\n\nWhen this bit is set, the GMAC operates in a Full-Duplex mode\n\nwhere it can transmit and receive simultaneously. This bit is RO\n\nwith default value of 1'b1 in Full-Duplex-only configuration."]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DmW<MacConfSpec> {
        DmW::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback Mode\n\nWhen this bit is set, the GMAC operates in loopback mode at\n\nGMII/MII. The (G)MII Receive clock input (clk_rx_i) is required\n\nfor the loopback to work properly, as the Transmit clock is not\n\nlooped-back internally."]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LmW<MacConfSpec> {
        LmW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable Receive Own\n\nWhen this bit is set, the GMAC disables the reception of frames\n\nwhen the gmii_txen_o is asserted in Half-Duplex mode.\n\nWhen this bit is reset, the GMAC receives all packets that are\n\ngiven by the PHY while transmitting."]
    #[inline(always)]
    #[must_use]
    pub fn do_(&mut self) -> DoW<MacConfSpec> {
        DoW::new(self, 13)
    }
    #[doc = "Bit 14 - Speed\n\nIndicates the speed in Fast Ethernet (MII) mode:"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FesW<MacConfSpec> {
        FesW::new(self, 14)
    }
    #[doc = "Bit 15 - Port Select\n\nSelects between GMII and MII:"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<MacConfSpec> {
        PsW::new(self, 15)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission\n\nWhen set high, this bit makes the MAC transmitter ignore the\n\n(G)MII CRS signal during frame transmission in Half-Duplex\n\nmode. This request results in no errors generated due to Loss of\n\nCarrier or No Carrier during such transmission. When this bit is\n\nlow, the MAC transmitter generates such errors due to Carrier\n\nSense and will even abort the transmissions."]
    #[inline(always)]
    #[must_use]
    pub fn dcrs(&mut self) -> DcrsW<MacConfSpec> {
        DcrsW::new(self, 16)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap\n\nThese bits control the minimum IFG between frames during\n\ntransmission."]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IfgW<MacConfSpec> {
        IfgW::new(self, 17)
    }
    #[doc = "Bit 21 - Frame Burst Enable\n\nWhen this bit is set, the GMAC allows frame bursting during\n\ntransmission in GMII Half-Duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BeW<MacConfSpec> {
        BeW::new(self, 21)
    }
    #[doc = "Bit 22 - Jabber Disable\n\nWhen this bit is set, the GMAC disables the jabber timer on the\n\ntransmitter, and can transfer frames of up to 16,384 bytes.\n\nWhen this bit is reset, the GMAC cuts off the transmitter if the\n\napplication sends out more than 2,048 bytes of data (10,240 if JE\n\nis set high) during transmission."]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JdW<MacConfSpec> {
        JdW::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog Disable\n\nWhen this bit is set, the GMAC disables the watchdog timer on\n\nthe receiver, and can receive frames of up to 16,384 bytes.\n\nWhen this bit is reset, the GMAC allows no more than 2,048 bytes\n\n(10,240 if JE is set high) of the frame being received and cuts off\n\nany bytes received after that."]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WdW<MacConfSpec> {
        WdW::new(self, 23)
    }
    #[doc = "Bit 24 - Transmit Configuration in RGMII\n\nWhen set, this bit enables the transmission of duplex mode, link\n\nspeed, and link up/down information to the PHY in the RGMII\n\nports. When this bit is reset, no such information is driven to the\n\nPHY."]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<MacConfSpec> {
        TcW::new(self, 24)
    }
}
#[doc = "MAC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacConfSpec;
impl crate::RegisterSpec for MacConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_conf::R`](R) reader structure"]
impl crate::Readable for MacConfSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_conf::W`](W) writer structure"]
impl crate::Writable for MacConfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_CONF to value 0"]
impl crate::Resettable for MacConfSpec {
    const RESET_VALUE: u32 = 0;
}
