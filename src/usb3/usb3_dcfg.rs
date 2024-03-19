#[doc = "Register `USB3_DCFG` reader"]
pub type R = crate::R<Usb3DcfgSpec>;
#[doc = "Register `USB3_DCFG` writer"]
pub type W = crate::W<Usb3DcfgSpec>;
#[doc = "Device Speed\n\nIndicates the speed at which the application requires the core to\n\nconnect, or the maximum speed the application can support.\n\nHowever, the actual bus speed is determined only after the chirp\n\nsequence is completed, and is based on the speed of the USB\n\nhost to which the core is connected.\n\nValue on reset: 4"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Devspd {
    #[doc = "4: SuperSpeed (USB 3.0 PHY clock is 125 MHz or 250 MHz)"]
    B100 = 4,
    #[doc = "0: High-speed (USB 2.0 PHY clock is 30 MHz or 60 MHz)"]
    B000 = 0,
    #[doc = "1: Full-speed (USB 2.0 PHY clock is 30 MHz or 60 MHz)"]
    B001 = 1,
}
impl From<Devspd> for u8 {
    #[inline(always)]
    fn from(variant: Devspd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Devspd {
    type Ux = u8;
}
#[doc = "Field `DEVSPD` reader - Device Speed\n\nIndicates the speed at which the application requires the core to\n\nconnect, or the maximum speed the application can support.\n\nHowever, the actual bus speed is determined only after the chirp\n\nsequence is completed, and is based on the speed of the USB\n\nhost to which the core is connected."]
pub type DevspdR = crate::FieldReader<Devspd>;
impl DevspdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Devspd> {
        match self.bits {
            4 => Some(Devspd::B100),
            0 => Some(Devspd::B000),
            1 => Some(Devspd::B001),
            _ => None,
        }
    }
    #[doc = "SuperSpeed (USB 3.0 PHY clock is 125 MHz or 250 MHz)"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Devspd::B100
    }
    #[doc = "High-speed (USB 2.0 PHY clock is 30 MHz or 60 MHz)"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Devspd::B000
    }
    #[doc = "Full-speed (USB 2.0 PHY clock is 30 MHz or 60 MHz)"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Devspd::B001
    }
}
#[doc = "Field `DEVSPD` writer - Device Speed\n\nIndicates the speed at which the application requires the core to\n\nconnect, or the maximum speed the application can support.\n\nHowever, the actual bus speed is determined only after the chirp\n\nsequence is completed, and is based on the speed of the USB\n\nhost to which the core is connected."]
pub type DevspdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Devspd>;
impl<'a, REG> DevspdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SuperSpeed (USB 3.0 PHY clock is 125 MHz or 250 MHz)"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Devspd::B100)
    }
    #[doc = "High-speed (USB 2.0 PHY clock is 30 MHz or 60 MHz)"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Devspd::B000)
    }
    #[doc = "Full-speed (USB 2.0 PHY clock is 30 MHz or 60 MHz)"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Devspd::B001)
    }
}
#[doc = "Field `DEVADDR` reader - Device Address.\n\nThe application must perform the following:\n\n1. Program this field after every SetAddress request.\n\n2. Reset this field to zero after USB reset."]
pub type DevaddrR = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device Address.\n\nThe application must perform the following:\n\n1. Program this field after every SetAddress request.\n\n2. Reset this field to zero after USB reset."]
pub type DevaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `INTRNUM` reader - Interrupt number\n\nIndicates interrupt/EventQ number on which non-endpoint-\n\nspecific device-related interrupts (see DEVT) are generated."]
pub type IntrnumR = crate::FieldReader;
#[doc = "Field `INTRNUM` writer - Interrupt number\n\nIndicates interrupt/EventQ number on which non-endpoint-\n\nspecific device-related interrupts (see DEVT) are generated."]
pub type IntrnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NUMP` reader - Number of Receive Buffers.\n\nThis bit indicates the number of receive buffers to be reported in\n\nthe ACK TP.\n\nThe DWC_usb3 controller uses this field if\n\nGRXTHRCFG.USBRxPktCntSel is set to 0. The application can\n\nprogram this value based on RxFIFO size, buffer sizes\n\nprogrammed in descriptors, and system latency.\n\nFor an OUT endpoint, this field controls the number of receive\n\nbuffers reported in the NumP field of the ACK TP transmitted by\n\nthe core.\n\nNote: This bit is used in host mode when Debug Capability is\n\nenabled."]
pub type NumpR = crate::FieldReader;
#[doc = "Field `NUMP` writer - Number of Receive Buffers.\n\nThis bit indicates the number of receive buffers to be reported in\n\nthe ACK TP.\n\nThe DWC_usb3 controller uses this field if\n\nGRXTHRCFG.USBRxPktCntSel is set to 0. The application can\n\nprogram this value based on RxFIFO size, buffer sizes\n\nprogrammed in descriptors, and system latency.\n\nFor an OUT endpoint, this field controls the number of receive\n\nbuffers reported in the NumP field of the ACK TP transmitted by\n\nthe core.\n\nNote: This bit is used in host mode when Debug Capability is\n\nenabled."]
pub type NumpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "LPM Capable\n\nThe application uses this bit to control the DWC_usb3 core LPM\n\ncapabilities. If the core operates as a non-LPM-capable device, it\n\ncannot respond to LPM transactions.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpmcap {
    #[doc = "0: LPM capability is not enabled."]
    B0 = 0,
    #[doc = "1: LPM capability is enabled."]
    B1 = 1,
}
impl From<Lpmcap> for bool {
    #[inline(always)]
    fn from(variant: Lpmcap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMCAP` reader - LPM Capable\n\nThe application uses this bit to control the DWC_usb3 core LPM\n\ncapabilities. If the core operates as a non-LPM-capable device, it\n\ncannot respond to LPM transactions."]
pub type LpmcapR = crate::BitReader<Lpmcap>;
impl LpmcapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpmcap {
        match self.bits {
            false => Lpmcap::B0,
            true => Lpmcap::B1,
        }
    }
    #[doc = "LPM capability is not enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lpmcap::B0
    }
    #[doc = "LPM capability is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lpmcap::B1
    }
}
#[doc = "Field `LPMCAP` writer - LPM Capable\n\nThe application uses this bit to control the DWC_usb3 core LPM\n\ncapabilities. If the core operates as a non-LPM-capable device, it\n\ncannot respond to LPM transactions."]
pub type LpmcapW<'a, REG> = crate::BitWriter<'a, REG, Lpmcap>;
impl<'a, REG> LpmcapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPM capability is not enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmcap::B0)
    }
    #[doc = "LPM capability is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmcap::B1)
    }
}
#[doc = "Field `IGNSTRMPP` reader - IgnoreStreamPP\n\nThis bit only affects stream-capable bulk endpoints.\n\nWhen this bit is set to 0 and the controller receives a Data Packet\n\nwith the Packet Pending (PP) bit set to 0 for OUT endpoints, or it\n\nreceives an ACK with the NumP field set to 0 and PP set to 0 for\n\nIN endpoints, the core attempts to search for another stream\n\n(CStream) to initiate to the host. However, there are two\n\nsituations where this behavior is not optimal:\n\n1. When the host is setting PP=0 even though it has not finished\n\nthe stream, or\n\n2. When the endpoint on the device is configured with one\n\ntransfer resource and therefore does not have any other streams\n\nto initiate to the host.\n\nWhen this bit is set to 1, the core ignores the Packet Pending bit\n\nfor the purposes of stream selection and does not search for\n\nanother stream when it receives DP(PP=0) or ACK(NumP=0,\n\nPP=0). This can enhance the performance when the device\n\nsystem bus bandwidth is low or the host responds to the core's\n\nERDY transmission very quickly."]
pub type IgnstrmppR = crate::BitReader;
#[doc = "Field `IGNSTRMPP` writer - IgnoreStreamPP\n\nThis bit only affects stream-capable bulk endpoints.\n\nWhen this bit is set to 0 and the controller receives a Data Packet\n\nwith the Packet Pending (PP) bit set to 0 for OUT endpoints, or it\n\nreceives an ACK with the NumP field set to 0 and PP set to 0 for\n\nIN endpoints, the core attempts to search for another stream\n\n(CStream) to initiate to the host. However, there are two\n\nsituations where this behavior is not optimal:\n\n1. When the host is setting PP=0 even though it has not finished\n\nthe stream, or\n\n2. When the endpoint on the device is configured with one\n\ntransfer resource and therefore does not have any other streams\n\nto initiate to the host.\n\nWhen this bit is set to 1, the core ignores the Packet Pending bit\n\nfor the purposes of stream selection and does not search for\n\nanother stream when it receives DP(PP=0) or ACK(NumP=0,\n\nPP=0). This can enhance the performance when the device\n\nsystem bus bandwidth is low or the host responds to the core's\n\nERDY transmission very quickly."]
pub type IgnstrmppW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Device Speed\n\nIndicates the speed at which the application requires the core to\n\nconnect, or the maximum speed the application can support.\n\nHowever, the actual bus speed is determined only after the chirp\n\nsequence is completed, and is based on the speed of the USB\n\nhost to which the core is connected."]
    #[inline(always)]
    pub fn devspd(&self) -> DevspdR {
        DevspdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:9 - Device Address.\n\nThe application must perform the following:\n\n1. Program this field after every SetAddress request.\n\n2. Reset this field to zero after USB reset."]
    #[inline(always)]
    pub fn devaddr(&self) -> DevaddrR {
        DevaddrR::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bits 12:16 - Interrupt number\n\nIndicates interrupt/EventQ number on which non-endpoint-\n\nspecific device-related interrupts (see DEVT) are generated."]
    #[inline(always)]
    pub fn intrnum(&self) -> IntrnumR {
        IntrnumR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Number of Receive Buffers.\n\nThis bit indicates the number of receive buffers to be reported in\n\nthe ACK TP.\n\nThe DWC_usb3 controller uses this field if\n\nGRXTHRCFG.USBRxPktCntSel is set to 0. The application can\n\nprogram this value based on RxFIFO size, buffer sizes\n\nprogrammed in descriptors, and system latency.\n\nFor an OUT endpoint, this field controls the number of receive\n\nbuffers reported in the NumP field of the ACK TP transmitted by\n\nthe core.\n\nNote: This bit is used in host mode when Debug Capability is\n\nenabled."]
    #[inline(always)]
    pub fn nump(&self) -> NumpR {
        NumpR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - LPM Capable\n\nThe application uses this bit to control the DWC_usb3 core LPM\n\ncapabilities. If the core operates as a non-LPM-capable device, it\n\ncannot respond to LPM transactions."]
    #[inline(always)]
    pub fn lpmcap(&self) -> LpmcapR {
        LpmcapR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IgnoreStreamPP\n\nThis bit only affects stream-capable bulk endpoints.\n\nWhen this bit is set to 0 and the controller receives a Data Packet\n\nwith the Packet Pending (PP) bit set to 0 for OUT endpoints, or it\n\nreceives an ACK with the NumP field set to 0 and PP set to 0 for\n\nIN endpoints, the core attempts to search for another stream\n\n(CStream) to initiate to the host. However, there are two\n\nsituations where this behavior is not optimal:\n\n1. When the host is setting PP=0 even though it has not finished\n\nthe stream, or\n\n2. When the endpoint on the device is configured with one\n\ntransfer resource and therefore does not have any other streams\n\nto initiate to the host.\n\nWhen this bit is set to 1, the core ignores the Packet Pending bit\n\nfor the purposes of stream selection and does not search for\n\nanother stream when it receives DP(PP=0) or ACK(NumP=0,\n\nPP=0). This can enhance the performance when the device\n\nsystem bus bandwidth is low or the host responds to the core's\n\nERDY transmission very quickly."]
    #[inline(always)]
    pub fn ignstrmpp(&self) -> IgnstrmppR {
        IgnstrmppR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Device Speed\n\nIndicates the speed at which the application requires the core to\n\nconnect, or the maximum speed the application can support.\n\nHowever, the actual bus speed is determined only after the chirp\n\nsequence is completed, and is based on the speed of the USB\n\nhost to which the core is connected."]
    #[inline(always)]
    #[must_use]
    pub fn devspd(&mut self) -> DevspdW<Usb3DcfgSpec> {
        DevspdW::new(self, 0)
    }
    #[doc = "Bits 3:9 - Device Address.\n\nThe application must perform the following:\n\n1. Program this field after every SetAddress request.\n\n2. Reset this field to zero after USB reset."]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DevaddrW<Usb3DcfgSpec> {
        DevaddrW::new(self, 3)
    }
    #[doc = "Bits 12:16 - Interrupt number\n\nIndicates interrupt/EventQ number on which non-endpoint-\n\nspecific device-related interrupts (see DEVT) are generated."]
    #[inline(always)]
    #[must_use]
    pub fn intrnum(&mut self) -> IntrnumW<Usb3DcfgSpec> {
        IntrnumW::new(self, 12)
    }
    #[doc = "Bits 17:21 - Number of Receive Buffers.\n\nThis bit indicates the number of receive buffers to be reported in\n\nthe ACK TP.\n\nThe DWC_usb3 controller uses this field if\n\nGRXTHRCFG.USBRxPktCntSel is set to 0. The application can\n\nprogram this value based on RxFIFO size, buffer sizes\n\nprogrammed in descriptors, and system latency.\n\nFor an OUT endpoint, this field controls the number of receive\n\nbuffers reported in the NumP field of the ACK TP transmitted by\n\nthe core.\n\nNote: This bit is used in host mode when Debug Capability is\n\nenabled."]
    #[inline(always)]
    #[must_use]
    pub fn nump(&mut self) -> NumpW<Usb3DcfgSpec> {
        NumpW::new(self, 17)
    }
    #[doc = "Bit 22 - LPM Capable\n\nThe application uses this bit to control the DWC_usb3 core LPM\n\ncapabilities. If the core operates as a non-LPM-capable device, it\n\ncannot respond to LPM transactions."]
    #[inline(always)]
    #[must_use]
    pub fn lpmcap(&mut self) -> LpmcapW<Usb3DcfgSpec> {
        LpmcapW::new(self, 22)
    }
    #[doc = "Bit 23 - IgnoreStreamPP\n\nThis bit only affects stream-capable bulk endpoints.\n\nWhen this bit is set to 0 and the controller receives a Data Packet\n\nwith the Packet Pending (PP) bit set to 0 for OUT endpoints, or it\n\nreceives an ACK with the NumP field set to 0 and PP set to 0 for\n\nIN endpoints, the core attempts to search for another stream\n\n(CStream) to initiate to the host. However, there are two\n\nsituations where this behavior is not optimal:\n\n1. When the host is setting PP=0 even though it has not finished\n\nthe stream, or\n\n2. When the endpoint on the device is configured with one\n\ntransfer resource and therefore does not have any other streams\n\nto initiate to the host.\n\nWhen this bit is set to 1, the core ignores the Packet Pending bit\n\nfor the purposes of stream selection and does not search for\n\nanother stream when it receives DP(PP=0) or ACK(NumP=0,\n\nPP=0). This can enhance the performance when the device\n\nsystem bus bandwidth is low or the host responds to the core's\n\nERDY transmission very quickly."]
    #[inline(always)]
    #[must_use]
    pub fn ignstrmpp(&mut self) -> IgnstrmppW<Usb3DcfgSpec> {
        IgnstrmppW::new(self, 23)
    }
}
#[doc = "Device Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_dcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_dcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3DcfgSpec;
impl crate::RegisterSpec for Usb3DcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_dcfg::R`](R) reader structure"]
impl crate::Readable for Usb3DcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_dcfg::W`](W) writer structure"]
impl crate::Writable for Usb3DcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_DCFG to value 0x0008_0004"]
impl crate::Resettable for Usb3DcfgSpec {
    const RESET_VALUE: u32 = 0x0008_0004;
}
