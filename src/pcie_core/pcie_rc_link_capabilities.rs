#[doc = "Register `PCIE_RC_LINK_CAPABILITIES` reader"]
pub type R = crate::R<PcieRcLinkCapabilitiesSpec>;
#[doc = "Field `MLS` reader - Max Link Speed \\[MLS\\]
Indicates the speeds supported by the link (2.5 GT/s, 5 GT/s per lane). This field is hardwired to 0001 (2.5GT/s) when the strap input PCIE_GENERATION_SEL is set to 0, to 0010 (5GT/s) when the strap is set to 1."]
pub type MlsR = crate::FieldReader;
#[doc = "Field `MLW` reader - Max Link Width \\[MLW\\]
Indicates the maximum number of lanes supported by the device. This field is hardwired based on the setting of the LANE_COUNT_IN strap input."]
pub type MlwR = crate::FieldReader;
#[doc = "Field `ASPM` reader - Active State Power Management \\[ASPM\\]
Indicates the level of ASPM support provided by the device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type AspmR = crate::FieldReader;
#[doc = "Field `L0EL` reader - L0S Exit Latency \\[L0EL\\]
Specifies the time required for the device to transition from L0S to L0. This parameter is dependent on the Physical Layer implementation. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type L0elR = crate::FieldReader;
#[doc = "Field `L1EL` reader - L1 Exit Latency \\[L1EL\\]
Specifies the exit latency from L1 state. This parameter is dependent on the Physical Layer implementation. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type L1elR = crate::FieldReader;
#[doc = "Field `CPM` reader - Clock Power Management \\[CPM\\]
Indicates that the device supports removal of reference clocks. Not supported in this version of the core. Hardwired to 0."]
pub type CpmR = crate::BitReader;
#[doc = "Field `SERC` reader - Surprise Down Error Reporting Capability \\[SERC\\]
Indicates the capability of the device to report a Surprise Down error condition. This bit is hardwired to 0, as this version of the core does not support the feature."]
pub type SercR = crate::BitReader;
#[doc = "Field `DARC` reader - Data Link Layer Active Reporting Capability \\[DARC\\]
Set to 1 if the device is capable of reporting that the DL Control and Management State Machine has reached the DL_Active state. This bit is hardwired to 0, as this version of the core does not support the feature."]
pub type DarcR = crate::BitReader;
#[doc = "Field `LBNC` reader - Link Bandwidth Notification Capability \\[LBNC\\]
A value of 1b indicates support for the Link Bandwidth Notification status and interrupt mechanisms. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type LbncR = crate::BitReader;
#[doc = "Field `ASPMOC` reader - ASPM Optionality Compliance \\[ASPMOC\\]
A 1 in this position indicates the device supports the ASPM Optionality feature. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type AspmocR = crate::BitReader;
#[doc = "Field `R9` reader - Reserved \\[R9\\]
Reserved"]
pub type R9R = crate::BitReader;
#[doc = "Field `PN` reader - Port Number \\[PN\\]
Specifies the port number assigned to the PCI Express link connected to this device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type PnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Max Link Speed \\[MLS\\]
Indicates the speeds supported by the link (2.5 GT/s, 5 GT/s per lane). This field is hardwired to 0001 (2.5GT/s) when the strap input PCIE_GENERATION_SEL is set to 0, to 0010 (5GT/s) when the strap is set to 1."]
    #[inline(always)]
    pub fn mls(&self) -> MlsR {
        MlsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - Max Link Width \\[MLW\\]
Indicates the maximum number of lanes supported by the device. This field is hardwired based on the setting of the LANE_COUNT_IN strap input."]
    #[inline(always)]
    pub fn mlw(&self) -> MlwR {
        MlwR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:11 - Active State Power Management \\[ASPM\\]
Indicates the level of ASPM support provided by the device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn aspm(&self) -> AspmR {
        AspmR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - L0S Exit Latency \\[L0EL\\]
Specifies the time required for the device to transition from L0S to L0. This parameter is dependent on the Physical Layer implementation. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn l0el(&self) -> L0elR {
        L0elR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - L1 Exit Latency \\[L1EL\\]
Specifies the exit latency from L1 state. This parameter is dependent on the Physical Layer implementation. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn l1el(&self) -> L1elR {
        L1elR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 18 - Clock Power Management \\[CPM\\]
Indicates that the device supports removal of reference clocks. Not supported in this version of the core. Hardwired to 0."]
    #[inline(always)]
    pub fn cpm(&self) -> CpmR {
        CpmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Surprise Down Error Reporting Capability \\[SERC\\]
Indicates the capability of the device to report a Surprise Down error condition. This bit is hardwired to 0, as this version of the core does not support the feature."]
    #[inline(always)]
    pub fn serc(&self) -> SercR {
        SercR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Link Layer Active Reporting Capability \\[DARC\\]
Set to 1 if the device is capable of reporting that the DL Control and Management State Machine has reached the DL_Active state. This bit is hardwired to 0, as this version of the core does not support the feature."]
    #[inline(always)]
    pub fn darc(&self) -> DarcR {
        DarcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Link Bandwidth Notification Capability \\[LBNC\\]
A value of 1b indicates support for the Link Bandwidth Notification status and interrupt mechanisms. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn lbnc(&self) -> LbncR {
        LbncR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ASPM Optionality Compliance \\[ASPMOC\\]
A 1 in this position indicates the device supports the ASPM Optionality feature. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn aspmoc(&self) -> AspmocR {
        AspmocR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved \\[R9\\]
Reserved"]
    #[inline(always)]
    pub fn r9(&self) -> R9R {
        R9R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Port Number \\[PN\\]
Specifies the port number assigned to the PCI Express link connected to this device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn pn(&self) -> PnR {
        PnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_link_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcLinkCapabilitiesSpec;
impl crate::RegisterSpec for PcieRcLinkCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_link_capabilities::R`](R) reader structure"]
impl crate::Readable for PcieRcLinkCapabilitiesSpec {}
#[doc = "`reset()` method sets PCIE_RC_LINK_CAPABILITIES to value 0x0061_ac42"]
impl crate::Resettable for PcieRcLinkCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x0061_ac42;
}
