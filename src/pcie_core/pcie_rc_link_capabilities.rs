#[doc = "Register `PCIE_RC_LINK_CAPABILITIES` reader"]
pub type R = crate::R<PcieRcLinkCapabilitiesSpec>;
#[doc = "Field `MLS` reader - Max Link Speed \\[MLS\\]\n\nIndicates the speeds supported by\n\nthe link (2.5 GT/s, 5 GT/s per lane).\n\nThis field is hardwired to 0001\n\n(2.5GT/s) when the strap input\n\nPCIE_GENERATION_SEL is set to 0,\n\nto 0010 (5GT/s) when the strap is\n\nset to 1."]
pub type MlsR = crate::FieldReader;
#[doc = "Field `MLW` reader - Max Link Width \\[MLW\\]\n\nIndicates the maximum number of\n\nlanes supported by the device. This\n\nfield is hardwired based on the\n\nsetting of the LANE_COUNT_IN strap\n\ninput."]
pub type MlwR = crate::FieldReader;
#[doc = "Field `ASPM` reader - Active State Power Management \\[ASPM\\]\n\nIndicates the level of ASPM support\n\nprovided by the device. This field\n\ncan be written from the APB bus by\n\nsetting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type AspmR = crate::FieldReader;
#[doc = "Field `L0EL` reader - L0S Exit Latency \\[L0EL\\]\n\nSpecifies the time required for the\n\ndevice to transition from L0S to L0.\n\nThis parameter is dependent on the\n\nPhysical Layer implementation. This\n\nfield can be written from the APB\n\nbus by setting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type L0elR = crate::FieldReader;
#[doc = "Field `L1EL` reader - L1 Exit Latency \\[L1EL\\]\n\nSpecifies the exit latency from L1\n\nstate. This parameter is dependent\n\non the Physical Layer\n\nimplementation. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type L1elR = crate::FieldReader;
#[doc = "Field `CPM` reader - Clock Power Management \\[CPM\\]\n\nIndicates that the device supports\n\nremoval of reference clocks. Not\n\nsupported in this version of the core.\n\nHardwired to 0."]
pub type CpmR = crate::BitReader;
#[doc = "Field `SERC` reader - Surprise Down Error Reporting Capability \\[SERC\\]\n\nIndicates the capability of the device\n\nto report a Surprise Down error\n\ncondition. This bit is hardwired to 0,\n\nas this version of the core does not\n\nsupport the feature."]
pub type SercR = crate::BitReader;
#[doc = "Field `DARC` reader - Data Link Layer Active Reporting Capability \\[DARC\\]\n\nSet to 1 if the device is capable of\n\nreporting that the DL Control and\n\nManagement State Machine has\n\nreached the DL_Active state. This bit\n\nis hardwired to 0, as this version of\n\nthe core does not support the\n\nfeature."]
pub type DarcR = crate::BitReader;
#[doc = "Field `LBNC` reader - Link Bandwidth Notification Capability \\[LBNC\\]\n\nA value of 1b indicates support for\n\nthe Link Bandwidth Notification\n\nstatus and interrupt mechanisms.\n\nThis field can be written from the\n\nAPB bus by setting \\[21\\]
bit high of\n\nthe pcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type LbncR = crate::BitReader;
#[doc = "Field `ASPMOC` reader - ASPM Optionality Compliance \\[ASPMOC\\]\n\nA 1 in this position indicates the\n\ndevice supports the ASPM\n\nOptionality feature. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type AspmocR = crate::BitReader;
#[doc = "Field `R9` reader - Reserved \\[R9\\]\n\nReserved"]
pub type R9R = crate::BitReader;
#[doc = "Field `PN` reader - Port Number \\[PN\\]\n\nSpecifies the port number assigned\n\nto the PCI Express link connected to\n\nthis device. This field can be written\n\nfrom the APB bus by setting \\[21\\]
bit\n\nhigh of the pcie_mgmt_APB_ADDR\n\nduring a local management register\n\nwrite."]
pub type PnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Max Link Speed \\[MLS\\]\n\nIndicates the speeds supported by\n\nthe link (2.5 GT/s, 5 GT/s per lane).\n\nThis field is hardwired to 0001\n\n(2.5GT/s) when the strap input\n\nPCIE_GENERATION_SEL is set to 0,\n\nto 0010 (5GT/s) when the strap is\n\nset to 1."]
    #[inline(always)]
    pub fn mls(&self) -> MlsR {
        MlsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - Max Link Width \\[MLW\\]\n\nIndicates the maximum number of\n\nlanes supported by the device. This\n\nfield is hardwired based on the\n\nsetting of the LANE_COUNT_IN strap\n\ninput."]
    #[inline(always)]
    pub fn mlw(&self) -> MlwR {
        MlwR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:11 - Active State Power Management \\[ASPM\\]\n\nIndicates the level of ASPM support\n\nprovided by the device. This field\n\ncan be written from the APB bus by\n\nsetting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn aspm(&self) -> AspmR {
        AspmR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - L0S Exit Latency \\[L0EL\\]\n\nSpecifies the time required for the\n\ndevice to transition from L0S to L0.\n\nThis parameter is dependent on the\n\nPhysical Layer implementation. This\n\nfield can be written from the APB\n\nbus by setting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn l0el(&self) -> L0elR {
        L0elR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - L1 Exit Latency \\[L1EL\\]\n\nSpecifies the exit latency from L1\n\nstate. This parameter is dependent\n\non the Physical Layer\n\nimplementation. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn l1el(&self) -> L1elR {
        L1elR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 18 - Clock Power Management \\[CPM\\]\n\nIndicates that the device supports\n\nremoval of reference clocks. Not\n\nsupported in this version of the core.\n\nHardwired to 0."]
    #[inline(always)]
    pub fn cpm(&self) -> CpmR {
        CpmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Surprise Down Error Reporting Capability \\[SERC\\]\n\nIndicates the capability of the device\n\nto report a Surprise Down error\n\ncondition. This bit is hardwired to 0,\n\nas this version of the core does not\n\nsupport the feature."]
    #[inline(always)]
    pub fn serc(&self) -> SercR {
        SercR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Link Layer Active Reporting Capability \\[DARC\\]\n\nSet to 1 if the device is capable of\n\nreporting that the DL Control and\n\nManagement State Machine has\n\nreached the DL_Active state. This bit\n\nis hardwired to 0, as this version of\n\nthe core does not support the\n\nfeature."]
    #[inline(always)]
    pub fn darc(&self) -> DarcR {
        DarcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Link Bandwidth Notification Capability \\[LBNC\\]\n\nA value of 1b indicates support for\n\nthe Link Bandwidth Notification\n\nstatus and interrupt mechanisms.\n\nThis field can be written from the\n\nAPB bus by setting \\[21\\]
bit high of\n\nthe pcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn lbnc(&self) -> LbncR {
        LbncR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ASPM Optionality Compliance \\[ASPMOC\\]\n\nA 1 in this position indicates the\n\ndevice supports the ASPM\n\nOptionality feature. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn aspmoc(&self) -> AspmocR {
        AspmocR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved \\[R9\\]\n\nReserved"]
    #[inline(always)]
    pub fn r9(&self) -> R9R {
        R9R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Port Number \\[PN\\]\n\nSpecifies the port number assigned\n\nto the PCI Express link connected to\n\nthis device. This field can be written\n\nfrom the APB bus by setting \\[21\\]
bit\n\nhigh of the pcie_mgmt_APB_ADDR\n\nduring a local management register\n\nwrite."]
    #[inline(always)]
    pub fn pn(&self) -> PnR {
        PnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Link Capabilities Register\n\nSpecifies the port number assigned\n\nto the PCI Express link connected to\n\nthis device. This field can be written\n\nfrom the APB bus by setting \\[21\\]
bit\n\nhigh of the pcie_mgmt_APB_ADDR\n\nduring a local management register\n\nwrite.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_link_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
