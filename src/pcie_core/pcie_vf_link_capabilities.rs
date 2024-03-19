#[doc = "Register `PCIE_VF_LINK_CAPABILITIES` reader"]
pub type R = crate::R<PcieVfLinkCapabilitiesSpec>;
#[doc = "Field `MLS` reader - Maximum Link Speed \\[MLS\\]\n\nIndicates the maximum speed\n\nsupported by the link. (2.5 GT/s, 5\n\nGT/s per lane). This field is\n\nhardwired to 0001 (2.5GT/s) when\n\nthe strap input\n\nPCIE_GENERATION_SEL is set to 0,\n\nto 0010 (5GT/s) when the strap is\n\nset to 1."]
pub type MlsR = crate::FieldReader;
#[doc = "Field `MLW` reader - Maximum Link Width \\[MLW\\]\n\nIndicates the maximum number of\n\nlanes supported by the device. This\n\nfield is hardwired based on the\n\nsetting of the LANE_COUNT_IN strap\n\ninput."]
pub type MlwR = crate::FieldReader;
#[doc = "Field `ASPM` reader - Active State Power Management \\[ASPM\\]\n\nIndicates the level of ASPM support\n\nprovided by the device. This field\n\ncan be re-written independently for\n\neach Function from the local\n\nmanagement bus. When SRIS is\n\nenabled in local management\n\nregister bit, L0s capability is not\n\nsupported and is forced low."]
pub type AspmR = crate::FieldReader;
#[doc = "Field `L0SEL` reader - L0S Exit Latency \\[L0SEL\\]\n\nSpecifies the time required for the\n\ndevice to transition from L0S to L0.\n\nThis parameter is dependent on the\n\nPhysical Layer implementation. It is\n\nset by default to the value define in\n\nreg_defaults.h. It can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
pub type L0selR = crate::FieldReader;
#[doc = "Field `L1EL` reader - L1 Exit Latency \\[L1EL\\]\n\nSpecifies the exit latency from L1\n\nstate. This parameter is dependent\n\non the Physical Layer\n\nimplementation. It is set by default\n\nto the value define in\n\nreg_defaults.h. It can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
pub type L1elR = crate::FieldReader;
#[doc = "Field `CPM` reader - Clock Power Management \\[CPM\\]\n\nIndicates that the device supports\n\nremoval of referenc clocks. It is set\n\nby default to the value of the define\n\nin reg_defaults.h. It can be re-\n\nwritten independently for each\n\nfunction from the local management\n\nbus."]
pub type CpmR = crate::BitReader;
#[doc = "Field `SDERC` reader - Surprise Down Error Reporting Capability \\[SDERC\\]\n\nIndicates the capability of the device\n\nto report a Surprise Down error\n\ncondition. This bit is hardwired to 0,\n\nas this version of the core does not\n\nsupport the feature."]
pub type SdercR = crate::BitReader;
#[doc = "Field `DLLARC` reader - Data Link Layer Active Reporting Capability \\[DLLARC\\]\n\nSet to 1 if the device is capable of\n\nreporting that the DL Control and\n\nManagement State Machine has\n\nreached the DL Active state. This bit\n\nis hardwired to 0, as this version of\n\nthe core does not support the\n\nfeature."]
pub type DllarcR = crate::BitReader;
#[doc = "Field `LBNC` reader - Link Bandwidth Notification Capability \\[LBNC\\]\n\nA value of 1b indicates support for\n\nthe Link Bandwidth Notification\n\nstatus and interrupt mechanisms.\n\nReserved for Endpoint."]
pub type LbncR = crate::BitReader;
#[doc = "Field `AOC` reader - ASPM Optionality Compliance \\[AOC\\]\n\nSetting this bit indicates that the\n\ndevice supports the ASPM\n\nOptionality feature. It can be turned\n\noff by writing a 0 to this bit position\n\nthrough the local management bus."]
pub type AocR = crate::BitReader;
#[doc = "Field `R5` reader - Reserved \\[R5\\]\n\nReserved"]
pub type R5R = crate::BitReader;
#[doc = "Field `PN` reader - Port Number \\[PN\\]\n\nSpecifies the port number assigned\n\nto the PCI Express link connected to\n\nthis device."]
pub type PnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Maximum Link Speed \\[MLS\\]\n\nIndicates the maximum speed\n\nsupported by the link. (2.5 GT/s, 5\n\nGT/s per lane). This field is\n\nhardwired to 0001 (2.5GT/s) when\n\nthe strap input\n\nPCIE_GENERATION_SEL is set to 0,\n\nto 0010 (5GT/s) when the strap is\n\nset to 1."]
    #[inline(always)]
    pub fn mls(&self) -> MlsR {
        MlsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - Maximum Link Width \\[MLW\\]\n\nIndicates the maximum number of\n\nlanes supported by the device. This\n\nfield is hardwired based on the\n\nsetting of the LANE_COUNT_IN strap\n\ninput."]
    #[inline(always)]
    pub fn mlw(&self) -> MlwR {
        MlwR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:11 - Active State Power Management \\[ASPM\\]\n\nIndicates the level of ASPM support\n\nprovided by the device. This field\n\ncan be re-written independently for\n\neach Function from the local\n\nmanagement bus. When SRIS is\n\nenabled in local management\n\nregister bit, L0s capability is not\n\nsupported and is forced low."]
    #[inline(always)]
    pub fn aspm(&self) -> AspmR {
        AspmR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - L0S Exit Latency \\[L0SEL\\]\n\nSpecifies the time required for the\n\ndevice to transition from L0S to L0.\n\nThis parameter is dependent on the\n\nPhysical Layer implementation. It is\n\nset by default to the value define in\n\nreg_defaults.h. It can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn l0sel(&self) -> L0selR {
        L0selR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - L1 Exit Latency \\[L1EL\\]\n\nSpecifies the exit latency from L1\n\nstate. This parameter is dependent\n\non the Physical Layer\n\nimplementation. It is set by default\n\nto the value define in\n\nreg_defaults.h. It can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn l1el(&self) -> L1elR {
        L1elR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 18 - Clock Power Management \\[CPM\\]\n\nIndicates that the device supports\n\nremoval of referenc clocks. It is set\n\nby default to the value of the define\n\nin reg_defaults.h. It can be re-\n\nwritten independently for each\n\nfunction from the local management\n\nbus."]
    #[inline(always)]
    pub fn cpm(&self) -> CpmR {
        CpmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Surprise Down Error Reporting Capability \\[SDERC\\]\n\nIndicates the capability of the device\n\nto report a Surprise Down error\n\ncondition. This bit is hardwired to 0,\n\nas this version of the core does not\n\nsupport the feature."]
    #[inline(always)]
    pub fn sderc(&self) -> SdercR {
        SdercR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Link Layer Active Reporting Capability \\[DLLARC\\]\n\nSet to 1 if the device is capable of\n\nreporting that the DL Control and\n\nManagement State Machine has\n\nreached the DL Active state. This bit\n\nis hardwired to 0, as this version of\n\nthe core does not support the\n\nfeature."]
    #[inline(always)]
    pub fn dllarc(&self) -> DllarcR {
        DllarcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Link Bandwidth Notification Capability \\[LBNC\\]\n\nA value of 1b indicates support for\n\nthe Link Bandwidth Notification\n\nstatus and interrupt mechanisms.\n\nReserved for Endpoint."]
    #[inline(always)]
    pub fn lbnc(&self) -> LbncR {
        LbncR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ASPM Optionality Compliance \\[AOC\\]\n\nSetting this bit indicates that the\n\ndevice supports the ASPM\n\nOptionality feature. It can be turned\n\noff by writing a 0 to this bit position\n\nthrough the local management bus."]
    #[inline(always)]
    pub fn aoc(&self) -> AocR {
        AocR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved \\[R5\\]\n\nReserved"]
    #[inline(always)]
    pub fn r5(&self) -> R5R {
        R5R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Port Number \\[PN\\]\n\nSpecifies the port number assigned\n\nto the PCI Express link connected to\n\nthis device."]
    #[inline(always)]
    pub fn pn(&self) -> PnR {
        PnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Link Capabilities Register\n\nSpecifies the port number assigned\n\nto the PCI Express link connected to\n\nthis device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_link_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfLinkCapabilitiesSpec;
impl crate::RegisterSpec for PcieVfLinkCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_link_capabilities::R`](R) reader structure"]
impl crate::Readable for PcieVfLinkCapabilitiesSpec {}
#[doc = "`reset()` method sets PCIE_VF_LINK_CAPABILITIES to value 0x0041_ac42"]
impl crate::Resettable for PcieVfLinkCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x0041_ac42;
}
