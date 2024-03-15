#[doc = "Register `LINK_CAPABILITIES` reader"]
pub type R = crate::R<LinkCapabilitiesSpec>;
#[doc = "Field `MLS` reader - Maximum Link Speed \\[MLS\\]
Indicates the maximum speed supported by the link. (2.5 GT/s, 5 GT/s per lane). This field is hardwired to 0001 (2.5GT/s) when the strap input PCIE_GENERATION_SEL is set to 0, to 0010 (5 GT/s) when the strap is set to 1."]
pub type MlsR = crate::FieldReader;
#[doc = "Field `MLW` reader - Maximum Link Width \\[MLW\\]
Indicates the maximum number of lanes supported by the device. This field is hardwired based on the setting of the LANE_COUNT_IN strap input."]
pub type MlwR = crate::FieldReader;
#[doc = "Field `ASPM` reader - Active State Power Management \\[ASPM\\]
Indicates the level of ASPM support provided by the device. This field can be re-written independently for each Function from the local management bus. When SRIS is enabled in local management register bit, L0s capability is not supported and is forced low."]
pub type AspmR = crate::FieldReader;
#[doc = "Field `L0SEL` reader - L0S Exit Latency \\[L0SEL\\]
Specifies the time required for the device to transition from L0S to L0. This parameter is dependent on the Physical Layer implementation. It is set by default to the value define in reg_defaults.h. It can be re-written independently for each Function from the local management bus."]
pub type L0selR = crate::FieldReader;
#[doc = "Field `L1EL` reader - L1 Exit Latency \\[L1EL\\]
Specifies the exit latency from L1 state. This parameter is dependent on the Physical Layer implementation. It is set by default to the value define in reg_defaults.h. It can be re-written independently for each Function from the local management bus."]
pub type L1elR = crate::FieldReader;
#[doc = "Field `CPM` reader - Clock Power Management \\[CPM\\]
Indicates that the device supports removal of referenc clocks. It is set by default to the value of the define in reg_defaults.h. It can be re- written independently for each function from the local management bus."]
pub type CpmR = crate::BitReader;
#[doc = "Field `SDERC` reader - Surprise Down Error Reporting Capability \\[SDERC\\]
Indicates the capability of the device to report a Surprise Down error condition. This bit is hardwired to 0, as this version of the core does not support the feature."]
pub type SdercR = crate::BitReader;
#[doc = "Field `DLLARC` reader - Data Link Layer Active Reporting Capability \\[DLLARC\\]
Set to 1 if the device is capable of reporting that the DL Control and Management State Machine has reached the DL_Active state. This bit is hardwired to 0, as this version of the core does not support the feature."]
pub type DllarcR = crate::BitReader;
#[doc = "Field `LBNC` reader - Link Bandwidth Notification Capability \\[LBNC\\]
A value of 1b indicates support for the Link Bandwidth Notification status and interrupt mechanisms. Reserved for Endpoint."]
pub type LbncR = crate::BitReader;
#[doc = "Field `AOC` reader - ASPM Optionality Compliance \\[AOC\\]
Setting this bit indicates that the device supports the ASPM Optionality feature. It can be turned off by writing a 0 to this bit position through the local management bus."]
pub type AocR = crate::BitReader;
#[doc = "Field `R5` reader - Reserved \\[R5\\]
Reserved"]
pub type R5R = crate::BitReader;
#[doc = "Field `PN` reader - Port Number \\[PN\\]
Specifies the port number assigned to the PCI Express link connected to this device."]
pub type PnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Maximum Link Speed \\[MLS\\]
Indicates the maximum speed supported by the link. (2.5 GT/s, 5 GT/s per lane). This field is hardwired to 0001 (2.5GT/s) when the strap input PCIE_GENERATION_SEL is set to 0, to 0010 (5 GT/s) when the strap is set to 1."]
    #[inline(always)]
    pub fn mls(&self) -> MlsR {
        MlsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - Maximum Link Width \\[MLW\\]
Indicates the maximum number of lanes supported by the device. This field is hardwired based on the setting of the LANE_COUNT_IN strap input."]
    #[inline(always)]
    pub fn mlw(&self) -> MlwR {
        MlwR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:11 - Active State Power Management \\[ASPM\\]
Indicates the level of ASPM support provided by the device. This field can be re-written independently for each Function from the local management bus. When SRIS is enabled in local management register bit, L0s capability is not supported and is forced low."]
    #[inline(always)]
    pub fn aspm(&self) -> AspmR {
        AspmR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - L0S Exit Latency \\[L0SEL\\]
Specifies the time required for the device to transition from L0S to L0. This parameter is dependent on the Physical Layer implementation. It is set by default to the value define in reg_defaults.h. It can be re-written independently for each Function from the local management bus."]
    #[inline(always)]
    pub fn l0sel(&self) -> L0selR {
        L0selR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - L1 Exit Latency \\[L1EL\\]
Specifies the exit latency from L1 state. This parameter is dependent on the Physical Layer implementation. It is set by default to the value define in reg_defaults.h. It can be re-written independently for each Function from the local management bus."]
    #[inline(always)]
    pub fn l1el(&self) -> L1elR {
        L1elR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 18 - Clock Power Management \\[CPM\\]
Indicates that the device supports removal of referenc clocks. It is set by default to the value of the define in reg_defaults.h. It can be re- written independently for each function from the local management bus."]
    #[inline(always)]
    pub fn cpm(&self) -> CpmR {
        CpmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Surprise Down Error Reporting Capability \\[SDERC\\]
Indicates the capability of the device to report a Surprise Down error condition. This bit is hardwired to 0, as this version of the core does not support the feature."]
    #[inline(always)]
    pub fn sderc(&self) -> SdercR {
        SdercR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Link Layer Active Reporting Capability \\[DLLARC\\]
Set to 1 if the device is capable of reporting that the DL Control and Management State Machine has reached the DL_Active state. This bit is hardwired to 0, as this version of the core does not support the feature."]
    #[inline(always)]
    pub fn dllarc(&self) -> DllarcR {
        DllarcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Link Bandwidth Notification Capability \\[LBNC\\]
A value of 1b indicates support for the Link Bandwidth Notification status and interrupt mechanisms. Reserved for Endpoint."]
    #[inline(always)]
    pub fn lbnc(&self) -> LbncR {
        LbncR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ASPM Optionality Compliance \\[AOC\\]
Setting this bit indicates that the device supports the ASPM Optionality feature. It can be turned off by writing a 0 to this bit position through the local management bus."]
    #[inline(always)]
    pub fn aoc(&self) -> AocR {
        AocR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved \\[R5\\]
Reserved"]
    #[inline(always)]
    pub fn r5(&self) -> R5R {
        R5R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Port Number \\[PN\\]
Specifies the port number assigned to the PCI Express link connected to this device."]
    #[inline(always)]
    pub fn pn(&self) -> PnR {
        PnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinkCapabilitiesSpec;
impl crate::RegisterSpec for LinkCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link_capabilities::R`](R) reader structure"]
impl crate::Readable for LinkCapabilitiesSpec {}
#[doc = "`reset()` method sets LINK_CAPABILITIES to value 0x0041_ac42"]
impl crate::Resettable for LinkCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x0041_ac42;
}
