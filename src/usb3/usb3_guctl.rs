#[doc = "Register `USB3_GUCTL` reader"]
pub type R = crate::R<Usb3GuctlSpec>;
#[doc = "Register `USB3_GUCTL` writer"]
pub type W = crate::W<Usb3GuctlSpec>;
#[doc = "Field `DTFT` reader - Device Timeout Fine Tuning\n\nThis field is a Host mode parameter which determines how long\n\nthe host waits for a response from device before considering a\n\ntimeout.\n\nFor the DTFT field to take effect, DTCT must be set to 2'b00.\n\nThe DTFT value is the number of 125 MHz clocks * 256 to count\n\nbefore considering a device timeout.\n\nThe minimum value of DTFT is 2.\n\nFor example, if the mac3_clk is 125 MHz clk (8 ns period), this is\n\ncalculated as follows:\n\n(DTFT value) * 256 * (8 ns)\n\nQuick Reference:\n\nif DTFT = 0x2, 2*256*8 = 4usec timeout\n\nif DTFT = 0x5, 5*256*8 = 10usec timeout\n\nif DTFT = 0xA, 10*256*8 = 20usec timeout\n\nif DTFT = 0x10, 16*256*8 = 32usec timeout\n\nif DTFT = 0x19, 25*256*8 = 51usec timeout\n\nif DTFT = 0x31, 49*256*8 = 100usec timeout\n\nif DTFT = 0x62, 98*256*8 = 200usec timeout"]
pub type DtftR = crate::FieldReader<u16>;
#[doc = "Field `DTFT` writer - Device Timeout Fine Tuning\n\nThis field is a Host mode parameter which determines how long\n\nthe host waits for a response from device before considering a\n\ntimeout.\n\nFor the DTFT field to take effect, DTCT must be set to 2'b00.\n\nThe DTFT value is the number of 125 MHz clocks * 256 to count\n\nbefore considering a device timeout.\n\nThe minimum value of DTFT is 2.\n\nFor example, if the mac3_clk is 125 MHz clk (8 ns period), this is\n\ncalculated as follows:\n\n(DTFT value) * 256 * (8 ns)\n\nQuick Reference:\n\nif DTFT = 0x2, 2*256*8 = 4usec timeout\n\nif DTFT = 0x5, 5*256*8 = 10usec timeout\n\nif DTFT = 0xA, 10*256*8 = 20usec timeout\n\nif DTFT = 0x10, 16*256*8 = 32usec timeout\n\nif DTFT = 0x19, 25*256*8 = 51usec timeout\n\nif DTFT = 0x31, 49*256*8 = 100usec timeout\n\nif DTFT = 0x62, 98*256*8 = 200usec timeout"]
pub type DtftW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Device Timeout Coarse Tuning\n\nThis field is a Host mode parameter which determines how long\n\nthe host waits for a response from device before considering a\n\ntimeout.\n\nThe core first checks the DTCT value. If it is 0, then the timeout\n\nvalue is defined by the DTFT. If it is non-zero, then it uses the\n\nfollowing timeout values:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtct {
    #[doc = "0: 0 usec -> use DTFT value instead"]
    B00 = 0,
    #[doc = "1: 500 usec"]
    B01 = 1,
    #[doc = "2: 1.5 msec"]
    B10 = 2,
    #[doc = "3: 6.5 msec"]
    B11 = 3,
}
impl From<Dtct> for u8 {
    #[inline(always)]
    fn from(variant: Dtct) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtct {
    type Ux = u8;
}
#[doc = "Field `DTCT` reader - Device Timeout Coarse Tuning\n\nThis field is a Host mode parameter which determines how long\n\nthe host waits for a response from device before considering a\n\ntimeout.\n\nThe core first checks the DTCT value. If it is 0, then the timeout\n\nvalue is defined by the DTFT. If it is non-zero, then it uses the\n\nfollowing timeout values:"]
pub type DtctR = crate::FieldReader<Dtct>;
impl DtctR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtct {
        match self.bits {
            0 => Dtct::B00,
            1 => Dtct::B01,
            2 => Dtct::B10,
            3 => Dtct::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "0 usec -> use DTFT value instead"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Dtct::B00
    }
    #[doc = "500 usec"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Dtct::B01
    }
    #[doc = "1.5 msec"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Dtct::B10
    }
    #[doc = "6.5 msec"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Dtct::B11
    }
}
#[doc = "Field `DTCT` writer - Device Timeout Coarse Tuning\n\nThis field is a Host mode parameter which determines how long\n\nthe host waits for a response from device before considering a\n\ntimeout.\n\nThe core first checks the DTCT value. If it is 0, then the timeout\n\nvalue is defined by the DTFT. If it is non-zero, then it uses the\n\nfollowing timeout values:"]
pub type DtctW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Dtct>;
impl<'a, REG> DtctW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 usec -> use DTFT value instead"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Dtct::B00)
    }
    #[doc = "500 usec"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Dtct::B01)
    }
    #[doc = "1.5 msec"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Dtct::B10)
    }
    #[doc = "6.5 msec"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Dtct::B11)
    }
}
#[doc = "Insert Extra Delay Between FS Bulk OUT\n\nSome FS devices are slow to receive Bulk OUT data and can get\n\nstuck when there are consecutive Bulk OUT transactions with\n\nshort inter-transaction delays. This bit is used to control whether\n\nthe host inserts extra delay between consecutive Bulk OUT\n\ntransactions to a FS Endpoint.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Insrtextrfsbodi {
    #[doc = "0: Host doesn't insert extra delay between consecutive Bulk OUT transactions to a FS Endpoint."]
    B0 = 0,
    #[doc = "1: Host inserts about 12us extra delay between consecutive Bulk OUT transactions to a FS Endpoint to work around the device issue. Note: Setting this bit to one will reduce the Bulk OUT transfer performance for most of the FS devices."]
    B1 = 1,
}
impl From<Insrtextrfsbodi> for bool {
    #[inline(always)]
    fn from(variant: Insrtextrfsbodi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INSRTEXTRFSBODI` reader - Insert Extra Delay Between FS Bulk OUT\n\nSome FS devices are slow to receive Bulk OUT data and can get\n\nstuck when there are consecutive Bulk OUT transactions with\n\nshort inter-transaction delays. This bit is used to control whether\n\nthe host inserts extra delay between consecutive Bulk OUT\n\ntransactions to a FS Endpoint."]
pub type InsrtextrfsbodiR = crate::BitReader<Insrtextrfsbodi>;
impl InsrtextrfsbodiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Insrtextrfsbodi {
        match self.bits {
            false => Insrtextrfsbodi::B0,
            true => Insrtextrfsbodi::B1,
        }
    }
    #[doc = "Host doesn't insert extra delay between consecutive Bulk OUT transactions to a FS Endpoint."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Insrtextrfsbodi::B0
    }
    #[doc = "Host inserts about 12us extra delay between consecutive Bulk OUT transactions to a FS Endpoint to work around the device issue. Note: Setting this bit to one will reduce the Bulk OUT transfer performance for most of the FS devices."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Insrtextrfsbodi::B1
    }
}
#[doc = "Field `INSRTEXTRFSBODI` writer - Insert Extra Delay Between FS Bulk OUT\n\nSome FS devices are slow to receive Bulk OUT data and can get\n\nstuck when there are consecutive Bulk OUT transactions with\n\nshort inter-transaction delays. This bit is used to control whether\n\nthe host inserts extra delay between consecutive Bulk OUT\n\ntransactions to a FS Endpoint."]
pub type InsrtextrfsbodiW<'a, REG> = crate::BitWriter<'a, REG, Insrtextrfsbodi>;
impl<'a, REG> InsrtextrfsbodiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Host doesn't insert extra delay between consecutive Bulk OUT transactions to a FS Endpoint."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Insrtextrfsbodi::B0)
    }
    #[doc = "Host inserts about 12us extra delay between consecutive Bulk OUT transactions to a FS Endpoint to work around the device issue. Note: Setting this bit to one will reduce the Bulk OUT transfer performance for most of the FS devices."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Insrtextrfsbodi::B1)
    }
}
#[doc = "Field `EXTCAPSUPPTEN` reader - External Extended Capability Support Enable\n\nWhen set, this field enables extended capabilities to be\n\nimplemented outside the core.\n\nWhen the ExtCapSupEN is set and the Debug Capability is\n\nenabled, the Next Capability pointer in Debug Capability returns\n\n16.\n\nA read to the first DWORD of the last internal extended capability\n\n(the \"xHCI Supported Protocol Capability for USB 3.0\" when the\n\nDebug Capability is not enabled) returns a value of 4 in the Next\n\nCapability Pointer field.\n\nThis indicates to software that there is another capability four\n\nDWORDs after this capability (for example, at address N+16\n\nwhere N is the address of this DWORD). If enabled, an external\n\naddress decoder that snoops the xHC slave interface must be\n\nimplemented.\n\nIf it sees an access to N+16 or greater, the slave access is re-\n\nrouted to a piece of hardware which returns the external\n\ncapability pointer register of the new capability and also handles\n\nreads/writes to this new capability and the side effects.\n\nIf disabled, a read to the first DWORD of the last internal\n\nextended capability returns 0 in the 'Next Capability Pointer' field.\n\nThis indicates there are no more capabilities."]
pub type ExtcapsupptenR = crate::BitReader;
#[doc = "Field `EXTCAPSUPPTEN` writer - External Extended Capability Support Enable\n\nWhen set, this field enables extended capabilities to be\n\nimplemented outside the core.\n\nWhen the ExtCapSupEN is set and the Debug Capability is\n\nenabled, the Next Capability pointer in Debug Capability returns\n\n16.\n\nA read to the first DWORD of the last internal extended capability\n\n(the \"xHCI Supported Protocol Capability for USB 3.0\" when the\n\nDebug Capability is not enabled) returns a value of 4 in the Next\n\nCapability Pointer field.\n\nThis indicates to software that there is another capability four\n\nDWORDs after this capability (for example, at address N+16\n\nwhere N is the address of this DWORD). If enabled, an external\n\naddress decoder that snoops the xHC slave interface must be\n\nimplemented.\n\nIf it sees an access to N+16 or greater, the slave access is re-\n\nrouted to a piece of hardware which returns the external\n\ncapability pointer register of the new capability and also handles\n\nreads/writes to this new capability and the side effects.\n\nIf disabled, a read to the first DWORD of the last internal\n\nextended capability returns 0 in the 'Next Capability Pointer' field.\n\nThis indicates there are no more capabilities."]
pub type ExtcapsupptenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Check for LFPS Overlap During Remote Ux\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enoverlapchk {
    #[doc = "1: The SuperSpeed link when exiting U1/U2/U3 waits for either the remote link LFPS or TS1/TS2 training symbols before it confirms that the LFPS handshake is complete. This is done to handle the case where the LFPS glitch causes the link to start exiting from the low power state. Looking for the LFPS overlap makes sure that the link partner also sees the LFPS."]
    B1 = 1,
    #[doc = "0: When the link exists U1/U2/U3 because of a remote exit, it does not look for an LFPS overlap."]
    B0 = 0,
}
impl From<Enoverlapchk> for bool {
    #[inline(always)]
    fn from(variant: Enoverlapchk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENOVERLAPCHK` reader - Enable Check for LFPS Overlap During Remote Ux"]
pub type EnoverlapchkR = crate::BitReader<Enoverlapchk>;
impl EnoverlapchkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enoverlapchk {
        match self.bits {
            true => Enoverlapchk::B1,
            false => Enoverlapchk::B0,
        }
    }
    #[doc = "The SuperSpeed link when exiting U1/U2/U3 waits for either the remote link LFPS or TS1/TS2 training symbols before it confirms that the LFPS handshake is complete. This is done to handle the case where the LFPS glitch causes the link to start exiting from the low power state. Looking for the LFPS overlap makes sure that the link partner also sees the LFPS."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Enoverlapchk::B1
    }
    #[doc = "When the link exists U1/U2/U3 because of a remote exit, it does not look for an LFPS overlap."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Enoverlapchk::B0
    }
}
#[doc = "Field `ENOVERLAPCHK` writer - Enable Check for LFPS Overlap During Remote Ux"]
pub type EnoverlapchkW<'a, REG> = crate::BitWriter<'a, REG, Enoverlapchk>;
impl<'a, REG> EnoverlapchkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SuperSpeed link when exiting U1/U2/U3 waits for either the remote link LFPS or TS1/TS2 training symbols before it confirms that the LFPS handshake is complete. This is done to handle the case where the LFPS glitch causes the link to start exiting from the low power state. Looking for the LFPS overlap makes sure that the link partner also sees the LFPS."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Enoverlapchk::B1)
    }
    #[doc = "When the link exists U1/U2/U3 because of a remote exit, it does not look for an LFPS overlap."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Enoverlapchk::B0)
    }
}
#[doc = "Host IN Auto Retry\n\nWhen set, this field enables the Auto Retry feature. For IN\n\ntransfers (non-isochronous) that encounter data packets with\n\nCRC errors or internal overrun scenarios, the auto retry feature\n\ncauses the Host core to reply to the device with a non-\n\nterminating retry ACK (that is, an ACK transaction packet with\n\nRetry = 1 and NumP != 0).\n\nIf the Auto Retry feature is disabled (default), the core will\n\nrespond with a terminating retry ACK (that is, an ACK transaction\n\npacket with Retry = 1 and NumP = 0).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbhstinautoretryen {
    #[doc = "0: Auto Retry Disabled"]
    B0 = 0,
    #[doc = "1: Auto Retry Enabled Note: This bit is also applicable to the device mode."]
    B1 = 1,
}
impl From<Usbhstinautoretryen> for bool {
    #[inline(always)]
    fn from(variant: Usbhstinautoretryen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHSTINAUTORETRYEN` reader - Host IN Auto Retry\n\nWhen set, this field enables the Auto Retry feature. For IN\n\ntransfers (non-isochronous) that encounter data packets with\n\nCRC errors or internal overrun scenarios, the auto retry feature\n\ncauses the Host core to reply to the device with a non-\n\nterminating retry ACK (that is, an ACK transaction packet with\n\nRetry = 1 and NumP != 0).\n\nIf the Auto Retry feature is disabled (default), the core will\n\nrespond with a terminating retry ACK (that is, an ACK transaction\n\npacket with Retry = 1 and NumP = 0)."]
pub type UsbhstinautoretryenR = crate::BitReader<Usbhstinautoretryen>;
impl UsbhstinautoretryenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbhstinautoretryen {
        match self.bits {
            false => Usbhstinautoretryen::B0,
            true => Usbhstinautoretryen::B1,
        }
    }
    #[doc = "Auto Retry Disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usbhstinautoretryen::B0
    }
    #[doc = "Auto Retry Enabled Note: This bit is also applicable to the device mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usbhstinautoretryen::B1
    }
}
#[doc = "Field `USBHSTINAUTORETRYEN` writer - Host IN Auto Retry\n\nWhen set, this field enables the Auto Retry feature. For IN\n\ntransfers (non-isochronous) that encounter data packets with\n\nCRC errors or internal overrun scenarios, the auto retry feature\n\ncauses the Host core to reply to the device with a non-\n\nterminating retry ACK (that is, an ACK transaction packet with\n\nRetry = 1 and NumP != 0).\n\nIf the Auto Retry feature is disabled (default), the core will\n\nrespond with a terminating retry ACK (that is, an ACK transaction\n\npacket with Retry = 1 and NumP = 0)."]
pub type UsbhstinautoretryenW<'a, REG> = crate::BitWriter<'a, REG, Usbhstinautoretryen>;
impl<'a, REG> UsbhstinautoretryenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto Retry Disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbhstinautoretryen::B0)
    }
    #[doc = "Auto Retry Enabled Note: This bit is also applicable to the device mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbhstinautoretryen::B1)
    }
}
#[doc = "Compliance Mode for Device Address\n\nWhen this bit is 1'b1, Slot ID may have different value than\n\nDevice Address if max_slot_enabled &lt; 128.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdevaddr {
    #[doc = "1: Increment Device Address on each Address Device command."]
    B1 = 1,
    #[doc = "0: Device Address is equal to Slot ID. The xHCI compliance requires this bit to be set to 1. The 0 mode is for debug purpose only. This allows you to easily identify a device connected to a port in the Lecroy or Eliisys trace during hardware debug. This bit is valid in Host and DRD configuration and is used in host mode operation only. Ignore this bit in device mode."]
    B0 = 0,
}
impl From<Cmdevaddr> for bool {
    #[inline(always)]
    fn from(variant: Cmdevaddr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDEVADDR` reader - Compliance Mode for Device Address\n\nWhen this bit is 1'b1, Slot ID may have different value than\n\nDevice Address if max_slot_enabled &lt; 128."]
pub type CmdevaddrR = crate::BitReader<Cmdevaddr>;
impl CmdevaddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdevaddr {
        match self.bits {
            true => Cmdevaddr::B1,
            false => Cmdevaddr::B0,
        }
    }
    #[doc = "Increment Device Address on each Address Device command."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmdevaddr::B1
    }
    #[doc = "Device Address is equal to Slot ID. The xHCI compliance requires this bit to be set to 1. The 0 mode is for debug purpose only. This allows you to easily identify a device connected to a port in the Lecroy or Eliisys trace during hardware debug. This bit is valid in Host and DRD configuration and is used in host mode operation only. Ignore this bit in device mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdevaddr::B0
    }
}
#[doc = "Field `CMDEVADDR` writer - Compliance Mode for Device Address\n\nWhen this bit is 1'b1, Slot ID may have different value than\n\nDevice Address if max_slot_enabled &lt; 128."]
pub type CmdevaddrW<'a, REG> = crate::BitWriter<'a, REG, Cmdevaddr>;
impl<'a, REG> CmdevaddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Increment Device Address on each Address Device command."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdevaddr::B1)
    }
    #[doc = "Device Address is equal to Slot ID. The xHCI compliance requires this bit to be set to 1. The 0 mode is for debug purpose only. This allows you to easily identify a device connected to a port in the Lecroy or Eliisys trace during hardware debug. This bit is valid in Host and DRD configuration and is used in host mode operation only. Ignore this bit in device mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdevaddr::B0)
    }
}
#[doc = "Field `RESBWHSEPS` reader - Reserving 85% Bandwidth for HS Periodic EPs\n\nBy default, HC reserves 80% of the bandwidth for periodic EPs. If\n\nthis bit is set, the bandwidth is relaxed to 85% to accommodate\n\ntwo high speed, high bandwidth ISOC EPs.\n\nUSB 2.0 required 80% bandwidth allocated for ISOC traffic. If two\n\nHigh-bandwidth ISOC devices (HD Webcams) are connected, and\n\nif each requires 1024-bytes X 3 packets per Micro-Frame, then\n\nthe bandwidth required is around 82%. If this bit is set, then it is\n\npossible to connect two Webcams of 1024bytes X 3 paylod per\n\nMicro-Frame each. Otherwise, you may have to reduce the\n\nresolution of the Webcams.\n\nThis bit is valid in Host and DRD configuration and is used in host\n\nmode operation only. Ignore this bit in device mode."]
pub type ResbwhsepsR = crate::BitReader;
#[doc = "Field `RESBWHSEPS` writer - Reserving 85% Bandwidth for HS Periodic EPs\n\nBy default, HC reserves 80% of the bandwidth for periodic EPs. If\n\nthis bit is set, the bandwidth is relaxed to 85% to accommodate\n\ntwo high speed, high bandwidth ISOC EPs.\n\nUSB 2.0 required 80% bandwidth allocated for ISOC traffic. If two\n\nHigh-bandwidth ISOC devices (HD Webcams) are connected, and\n\nif each requires 1024-bytes X 3 packets per Micro-Frame, then\n\nthe bandwidth required is around 82%. If this bit is set, then it is\n\npossible to connect two Webcams of 1024bytes X 3 paylod per\n\nMicro-Frame each. Otherwise, you may have to reduce the\n\nresolution of the Webcams.\n\nThis bit is valid in Host and DRD configuration and is used in host\n\nmode operation only. Ignore this bit in device mode."]
pub type ResbwhsepsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPRSCTRLTRANSEN` reader - Sparse Control Transaction Enable\n\nSome devices are slow in responding to Control transfers.\n\nScheduling multiple transactions in one microframe/frame can\n\ncause these devices to misbehave.\n\nIf this bit is set to 1'b1, the host controller schedules transactions\n\nfor a Control transfer in different microframes/frames."]
pub type SprsctrltransenR = crate::BitReader;
#[doc = "Field `SPRSCTRLTRANSEN` writer - Sparse Control Transaction Enable\n\nSome devices are slow in responding to Control transfers.\n\nScheduling multiple transactions in one microframe/frame can\n\ncause these devices to misbehave.\n\nIf this bit is set to 1'b1, the host controller schedules transactions\n\nfor a Control transfer in different microframes/frames."]
pub type SprsctrltransenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "No Extra Delay Between SOF and the First\n\nSome HS devices misbehave when the host sends a packet\n\nimmediately after a SOF. However, adding an extra delay\n\nbetween a SOF and the first packet can reduce the USB data rate\n\nand performance.\n\nThis bit is used to control whether the host must wait for 2\n\nmicroseconds before it sends the first packet after a SOF, or not.\n\nUser can set this bit to one to improve the performance if those\n\nproblematic devices are not a concern in the user's host\n\nenvironment.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Noextrdl {
    #[doc = "0: Host waits for 2 microseconds after a SOF before it sends the first USB packet."]
    B0 = 0,
    #[doc = "1: Host doesn't wait after a SOF before it sends the first USB packet."]
    B1 = 1,
}
impl From<Noextrdl> for bool {
    #[inline(always)]
    fn from(variant: Noextrdl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOEXTRDL` reader - No Extra Delay Between SOF and the First\n\nSome HS devices misbehave when the host sends a packet\n\nimmediately after a SOF. However, adding an extra delay\n\nbetween a SOF and the first packet can reduce the USB data rate\n\nand performance.\n\nThis bit is used to control whether the host must wait for 2\n\nmicroseconds before it sends the first packet after a SOF, or not.\n\nUser can set this bit to one to improve the performance if those\n\nproblematic devices are not a concern in the user's host\n\nenvironment."]
pub type NoextrdlR = crate::BitReader<Noextrdl>;
impl NoextrdlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Noextrdl {
        match self.bits {
            false => Noextrdl::B0,
            true => Noextrdl::B1,
        }
    }
    #[doc = "Host waits for 2 microseconds after a SOF before it sends the first USB packet."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Noextrdl::B0
    }
    #[doc = "Host doesn't wait after a SOF before it sends the first USB packet."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Noextrdl::B1
    }
}
#[doc = "Field `NOEXTRDL` writer - No Extra Delay Between SOF and the First\n\nSome HS devices misbehave when the host sends a packet\n\nimmediately after a SOF. However, adding an extra delay\n\nbetween a SOF and the first packet can reduce the USB data rate\n\nand performance.\n\nThis bit is used to control whether the host must wait for 2\n\nmicroseconds before it sends the first packet after a SOF, or not.\n\nUser can set this bit to one to improve the performance if those\n\nproblematic devices are not a concern in the user's host\n\nenvironment."]
pub type NoextrdlW<'a, REG> = crate::BitWriter<'a, REG, Noextrdl>;
impl<'a, REG> NoextrdlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Host waits for 2 microseconds after a SOF before it sends the first USB packet."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Noextrdl::B0)
    }
    #[doc = "Host doesn't wait after a SOF before it sends the first USB packet."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Noextrdl::B1)
    }
}
#[doc = "Field `REFCLKPER` reader - REFCLKPER\n\nThis field indicates in terms of nano seconds the period of ref_clk.\n\nThe default value of this register is set to 'h8 (8ns/125 MHz).\n\nThis field needs to be updated during power-on initialization, if\n\nGCTL.SOFITPSYNC or GFLADJ.GFLADJ_REFCLK_LPM_SEL is set to\n\n1. The programmable maximum value is 62ns, and the minimum\n\nvalue is 8ns.\n\nYou must use a reference clock with a period that is an integer\n\nmultiple, so that ITP can meet the jitter margin of 32ns. The\n\nallowable ref_clk frequencies whose period is not integer\n\nmultiples are 16/17/19.2/24/39.7MHz.\n\nThis field must not be set to 0 at any time. If you never plan to\n\nuse this feature, then set this field to 'h8, the default value."]
pub type RefclkperR = crate::FieldReader<u16>;
#[doc = "Field `REFCLKPER` writer - REFCLKPER\n\nThis field indicates in terms of nano seconds the period of ref_clk.\n\nThe default value of this register is set to 'h8 (8ns/125 MHz).\n\nThis field needs to be updated during power-on initialization, if\n\nGCTL.SOFITPSYNC or GFLADJ.GFLADJ_REFCLK_LPM_SEL is set to\n\n1. The programmable maximum value is 62ns, and the minimum\n\nvalue is 8ns.\n\nYou must use a reference clock with a period that is an integer\n\nmultiple, so that ITP can meet the jitter margin of 32ns. The\n\nallowable ref_clk frequencies whose period is not integer\n\nmultiples are 16/17/19.2/24/39.7MHz.\n\nThis field must not be set to 0 at any time. If you never plan to\n\nuse this feature, then set this field to 'h8, the default value."]
pub type RefclkperW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:8 - Device Timeout Fine Tuning\n\nThis field is a Host mode parameter which determines how long\n\nthe host waits for a response from device before considering a\n\ntimeout.\n\nFor the DTFT field to take effect, DTCT must be set to 2'b00.\n\nThe DTFT value is the number of 125 MHz clocks * 256 to count\n\nbefore considering a device timeout.\n\nThe minimum value of DTFT is 2.\n\nFor example, if the mac3_clk is 125 MHz clk (8 ns period), this is\n\ncalculated as follows:\n\n(DTFT value) * 256 * (8 ns)\n\nQuick Reference:\n\nif DTFT = 0x2, 2*256*8 = 4usec timeout\n\nif DTFT = 0x5, 5*256*8 = 10usec timeout\n\nif DTFT = 0xA, 10*256*8 = 20usec timeout\n\nif DTFT = 0x10, 16*256*8 = 32usec timeout\n\nif DTFT = 0x19, 25*256*8 = 51usec timeout\n\nif DTFT = 0x31, 49*256*8 = 100usec timeout\n\nif DTFT = 0x62, 98*256*8 = 200usec timeout"]
    #[inline(always)]
    pub fn dtft(&self) -> DtftR {
        DtftR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:10 - Device Timeout Coarse Tuning\n\nThis field is a Host mode parameter which determines how long\n\nthe host waits for a response from device before considering a\n\ntimeout.\n\nThe core first checks the DTCT value. If it is 0, then the timeout\n\nvalue is defined by the DTFT. If it is non-zero, then it uses the\n\nfollowing timeout values:"]
    #[inline(always)]
    pub fn dtct(&self) -> DtctR {
        DtctR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Insert Extra Delay Between FS Bulk OUT\n\nSome FS devices are slow to receive Bulk OUT data and can get\n\nstuck when there are consecutive Bulk OUT transactions with\n\nshort inter-transaction delays. This bit is used to control whether\n\nthe host inserts extra delay between consecutive Bulk OUT\n\ntransactions to a FS Endpoint."]
    #[inline(always)]
    pub fn insrtextrfsbodi(&self) -> InsrtextrfsbodiR {
        InsrtextrfsbodiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Extended Capability Support Enable\n\nWhen set, this field enables extended capabilities to be\n\nimplemented outside the core.\n\nWhen the ExtCapSupEN is set and the Debug Capability is\n\nenabled, the Next Capability pointer in Debug Capability returns\n\n16.\n\nA read to the first DWORD of the last internal extended capability\n\n(the \"xHCI Supported Protocol Capability for USB 3.0\" when the\n\nDebug Capability is not enabled) returns a value of 4 in the Next\n\nCapability Pointer field.\n\nThis indicates to software that there is another capability four\n\nDWORDs after this capability (for example, at address N+16\n\nwhere N is the address of this DWORD). If enabled, an external\n\naddress decoder that snoops the xHC slave interface must be\n\nimplemented.\n\nIf it sees an access to N+16 or greater, the slave access is re-\n\nrouted to a piece of hardware which returns the external\n\ncapability pointer register of the new capability and also handles\n\nreads/writes to this new capability and the side effects.\n\nIf disabled, a read to the first DWORD of the last internal\n\nextended capability returns 0 in the 'Next Capability Pointer' field.\n\nThis indicates there are no more capabilities."]
    #[inline(always)]
    pub fn extcapsuppten(&self) -> ExtcapsupptenR {
        ExtcapsupptenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Check for LFPS Overlap During Remote Ux"]
    #[inline(always)]
    pub fn enoverlapchk(&self) -> EnoverlapchkR {
        EnoverlapchkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Host IN Auto Retry\n\nWhen set, this field enables the Auto Retry feature. For IN\n\ntransfers (non-isochronous) that encounter data packets with\n\nCRC errors or internal overrun scenarios, the auto retry feature\n\ncauses the Host core to reply to the device with a non-\n\nterminating retry ACK (that is, an ACK transaction packet with\n\nRetry = 1 and NumP != 0).\n\nIf the Auto Retry feature is disabled (default), the core will\n\nrespond with a terminating retry ACK (that is, an ACK transaction\n\npacket with Retry = 1 and NumP = 0)."]
    #[inline(always)]
    pub fn usbhstinautoretryen(&self) -> UsbhstinautoretryenR {
        UsbhstinautoretryenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Compliance Mode for Device Address\n\nWhen this bit is 1'b1, Slot ID may have different value than\n\nDevice Address if max_slot_enabled &lt; 128."]
    #[inline(always)]
    pub fn cmdevaddr(&self) -> CmdevaddrR {
        CmdevaddrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reserving 85% Bandwidth for HS Periodic EPs\n\nBy default, HC reserves 80% of the bandwidth for periodic EPs. If\n\nthis bit is set, the bandwidth is relaxed to 85% to accommodate\n\ntwo high speed, high bandwidth ISOC EPs.\n\nUSB 2.0 required 80% bandwidth allocated for ISOC traffic. If two\n\nHigh-bandwidth ISOC devices (HD Webcams) are connected, and\n\nif each requires 1024-bytes X 3 packets per Micro-Frame, then\n\nthe bandwidth required is around 82%. If this bit is set, then it is\n\npossible to connect two Webcams of 1024bytes X 3 paylod per\n\nMicro-Frame each. Otherwise, you may have to reduce the\n\nresolution of the Webcams.\n\nThis bit is valid in Host and DRD configuration and is used in host\n\nmode operation only. Ignore this bit in device mode."]
    #[inline(always)]
    pub fn resbwhseps(&self) -> ResbwhsepsR {
        ResbwhsepsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Sparse Control Transaction Enable\n\nSome devices are slow in responding to Control transfers.\n\nScheduling multiple transactions in one microframe/frame can\n\ncause these devices to misbehave.\n\nIf this bit is set to 1'b1, the host controller schedules transactions\n\nfor a Control transfer in different microframes/frames."]
    #[inline(always)]
    pub fn sprsctrltransen(&self) -> SprsctrltransenR {
        SprsctrltransenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - No Extra Delay Between SOF and the First\n\nSome HS devices misbehave when the host sends a packet\n\nimmediately after a SOF. However, adding an extra delay\n\nbetween a SOF and the first packet can reduce the USB data rate\n\nand performance.\n\nThis bit is used to control whether the host must wait for 2\n\nmicroseconds before it sends the first packet after a SOF, or not.\n\nUser can set this bit to one to improve the performance if those\n\nproblematic devices are not a concern in the user's host\n\nenvironment."]
    #[inline(always)]
    pub fn noextrdl(&self) -> NoextrdlR {
        NoextrdlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - REFCLKPER\n\nThis field indicates in terms of nano seconds the period of ref_clk.\n\nThe default value of this register is set to 'h8 (8ns/125 MHz).\n\nThis field needs to be updated during power-on initialization, if\n\nGCTL.SOFITPSYNC or GFLADJ.GFLADJ_REFCLK_LPM_SEL is set to\n\n1. The programmable maximum value is 62ns, and the minimum\n\nvalue is 8ns.\n\nYou must use a reference clock with a period that is an integer\n\nmultiple, so that ITP can meet the jitter margin of 32ns. The\n\nallowable ref_clk frequencies whose period is not integer\n\nmultiples are 16/17/19.2/24/39.7MHz.\n\nThis field must not be set to 0 at any time. If you never plan to\n\nuse this feature, then set this field to 'h8, the default value."]
    #[inline(always)]
    pub fn refclkper(&self) -> RefclkperR {
        RefclkperR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Device Timeout Fine Tuning\n\nThis field is a Host mode parameter which determines how long\n\nthe host waits for a response from device before considering a\n\ntimeout.\n\nFor the DTFT field to take effect, DTCT must be set to 2'b00.\n\nThe DTFT value is the number of 125 MHz clocks * 256 to count\n\nbefore considering a device timeout.\n\nThe minimum value of DTFT is 2.\n\nFor example, if the mac3_clk is 125 MHz clk (8 ns period), this is\n\ncalculated as follows:\n\n(DTFT value) * 256 * (8 ns)\n\nQuick Reference:\n\nif DTFT = 0x2, 2*256*8 = 4usec timeout\n\nif DTFT = 0x5, 5*256*8 = 10usec timeout\n\nif DTFT = 0xA, 10*256*8 = 20usec timeout\n\nif DTFT = 0x10, 16*256*8 = 32usec timeout\n\nif DTFT = 0x19, 25*256*8 = 51usec timeout\n\nif DTFT = 0x31, 49*256*8 = 100usec timeout\n\nif DTFT = 0x62, 98*256*8 = 200usec timeout"]
    #[inline(always)]
    #[must_use]
    pub fn dtft(&mut self) -> DtftW<Usb3GuctlSpec> {
        DtftW::new(self, 0)
    }
    #[doc = "Bits 9:10 - Device Timeout Coarse Tuning\n\nThis field is a Host mode parameter which determines how long\n\nthe host waits for a response from device before considering a\n\ntimeout.\n\nThe core first checks the DTCT value. If it is 0, then the timeout\n\nvalue is defined by the DTFT. If it is non-zero, then it uses the\n\nfollowing timeout values:"]
    #[inline(always)]
    #[must_use]
    pub fn dtct(&mut self) -> DtctW<Usb3GuctlSpec> {
        DtctW::new(self, 9)
    }
    #[doc = "Bit 11 - Insert Extra Delay Between FS Bulk OUT\n\nSome FS devices are slow to receive Bulk OUT data and can get\n\nstuck when there are consecutive Bulk OUT transactions with\n\nshort inter-transaction delays. This bit is used to control whether\n\nthe host inserts extra delay between consecutive Bulk OUT\n\ntransactions to a FS Endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn insrtextrfsbodi(&mut self) -> InsrtextrfsbodiW<Usb3GuctlSpec> {
        InsrtextrfsbodiW::new(self, 11)
    }
    #[doc = "Bit 12 - External Extended Capability Support Enable\n\nWhen set, this field enables extended capabilities to be\n\nimplemented outside the core.\n\nWhen the ExtCapSupEN is set and the Debug Capability is\n\nenabled, the Next Capability pointer in Debug Capability returns\n\n16.\n\nA read to the first DWORD of the last internal extended capability\n\n(the \"xHCI Supported Protocol Capability for USB 3.0\" when the\n\nDebug Capability is not enabled) returns a value of 4 in the Next\n\nCapability Pointer field.\n\nThis indicates to software that there is another capability four\n\nDWORDs after this capability (for example, at address N+16\n\nwhere N is the address of this DWORD). If enabled, an external\n\naddress decoder that snoops the xHC slave interface must be\n\nimplemented.\n\nIf it sees an access to N+16 or greater, the slave access is re-\n\nrouted to a piece of hardware which returns the external\n\ncapability pointer register of the new capability and also handles\n\nreads/writes to this new capability and the side effects.\n\nIf disabled, a read to the first DWORD of the last internal\n\nextended capability returns 0 in the 'Next Capability Pointer' field.\n\nThis indicates there are no more capabilities."]
    #[inline(always)]
    #[must_use]
    pub fn extcapsuppten(&mut self) -> ExtcapsupptenW<Usb3GuctlSpec> {
        ExtcapsupptenW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Check for LFPS Overlap During Remote Ux"]
    #[inline(always)]
    #[must_use]
    pub fn enoverlapchk(&mut self) -> EnoverlapchkW<Usb3GuctlSpec> {
        EnoverlapchkW::new(self, 13)
    }
    #[doc = "Bit 14 - Host IN Auto Retry\n\nWhen set, this field enables the Auto Retry feature. For IN\n\ntransfers (non-isochronous) that encounter data packets with\n\nCRC errors or internal overrun scenarios, the auto retry feature\n\ncauses the Host core to reply to the device with a non-\n\nterminating retry ACK (that is, an ACK transaction packet with\n\nRetry = 1 and NumP != 0).\n\nIf the Auto Retry feature is disabled (default), the core will\n\nrespond with a terminating retry ACK (that is, an ACK transaction\n\npacket with Retry = 1 and NumP = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn usbhstinautoretryen(&mut self) -> UsbhstinautoretryenW<Usb3GuctlSpec> {
        UsbhstinautoretryenW::new(self, 14)
    }
    #[doc = "Bit 15 - Compliance Mode for Device Address\n\nWhen this bit is 1'b1, Slot ID may have different value than\n\nDevice Address if max_slot_enabled &lt; 128."]
    #[inline(always)]
    #[must_use]
    pub fn cmdevaddr(&mut self) -> CmdevaddrW<Usb3GuctlSpec> {
        CmdevaddrW::new(self, 15)
    }
    #[doc = "Bit 16 - Reserving 85% Bandwidth for HS Periodic EPs\n\nBy default, HC reserves 80% of the bandwidth for periodic EPs. If\n\nthis bit is set, the bandwidth is relaxed to 85% to accommodate\n\ntwo high speed, high bandwidth ISOC EPs.\n\nUSB 2.0 required 80% bandwidth allocated for ISOC traffic. If two\n\nHigh-bandwidth ISOC devices (HD Webcams) are connected, and\n\nif each requires 1024-bytes X 3 packets per Micro-Frame, then\n\nthe bandwidth required is around 82%. If this bit is set, then it is\n\npossible to connect two Webcams of 1024bytes X 3 paylod per\n\nMicro-Frame each. Otherwise, you may have to reduce the\n\nresolution of the Webcams.\n\nThis bit is valid in Host and DRD configuration and is used in host\n\nmode operation only. Ignore this bit in device mode."]
    #[inline(always)]
    #[must_use]
    pub fn resbwhseps(&mut self) -> ResbwhsepsW<Usb3GuctlSpec> {
        ResbwhsepsW::new(self, 16)
    }
    #[doc = "Bit 17 - Sparse Control Transaction Enable\n\nSome devices are slow in responding to Control transfers.\n\nScheduling multiple transactions in one microframe/frame can\n\ncause these devices to misbehave.\n\nIf this bit is set to 1'b1, the host controller schedules transactions\n\nfor a Control transfer in different microframes/frames."]
    #[inline(always)]
    #[must_use]
    pub fn sprsctrltransen(&mut self) -> SprsctrltransenW<Usb3GuctlSpec> {
        SprsctrltransenW::new(self, 17)
    }
    #[doc = "Bit 21 - No Extra Delay Between SOF and the First\n\nSome HS devices misbehave when the host sends a packet\n\nimmediately after a SOF. However, adding an extra delay\n\nbetween a SOF and the first packet can reduce the USB data rate\n\nand performance.\n\nThis bit is used to control whether the host must wait for 2\n\nmicroseconds before it sends the first packet after a SOF, or not.\n\nUser can set this bit to one to improve the performance if those\n\nproblematic devices are not a concern in the user's host\n\nenvironment."]
    #[inline(always)]
    #[must_use]
    pub fn noextrdl(&mut self) -> NoextrdlW<Usb3GuctlSpec> {
        NoextrdlW::new(self, 21)
    }
    #[doc = "Bits 22:31 - REFCLKPER\n\nThis field indicates in terms of nano seconds the period of ref_clk.\n\nThe default value of this register is set to 'h8 (8ns/125 MHz).\n\nThis field needs to be updated during power-on initialization, if\n\nGCTL.SOFITPSYNC or GFLADJ.GFLADJ_REFCLK_LPM_SEL is set to\n\n1. The programmable maximum value is 62ns, and the minimum\n\nvalue is 8ns.\n\nYou must use a reference clock with a period that is an integer\n\nmultiple, so that ITP can meet the jitter margin of 32ns. The\n\nallowable ref_clk frequencies whose period is not integer\n\nmultiples are 16/17/19.2/24/39.7MHz.\n\nThis field must not be set to 0 at any time. If you never plan to\n\nuse this feature, then set this field to 'h8, the default value."]
    #[inline(always)]
    #[must_use]
    pub fn refclkper(&mut self) -> RefclkperW<Usb3GuctlSpec> {
        RefclkperW::new(self, 22)
    }
}
#[doc = "Global User Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_guctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_guctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GuctlSpec;
impl crate::RegisterSpec for Usb3GuctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_guctl::R`](R) reader structure"]
impl crate::Readable for Usb3GuctlSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_guctl::W`](W) writer structure"]
impl crate::Writable for Usb3GuctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GUCTL to value 0x0200_8010"]
impl crate::Resettable for Usb3GuctlSpec {
    const RESET_VALUE: u32 = 0x0200_8010;
}
