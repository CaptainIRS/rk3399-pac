#[doc = "Register `PCIE_RC_PCI_EXPRESS_DEVICE_CAPABILITIES_2` reader"]
pub type R = crate::R<PcieRcPciExpressDeviceCapabilities2Spec>;
#[doc = "Field `CTR` reader - Completion Timeout Ranges \\[CTR\\]\n\nSpecifies the Completion Timeout\n\nvalues supported by the device. This\n\nfield is set by default to 0010 (10 ms\n\n- 250 ms), but can be modified from\n\nthe local management APB bus. The\n\nactual timeout values are in two\n\nprogrammable local management\n\nregisters, which allow the timeout\n\nsettings of the two sub-ranges\n\nwithin Range B to be programmed\n\nindependently. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type CtrR = crate::FieldReader;
#[doc = "Field `CTDS` reader - Completion Timeout Disable Supported \\[CTDS\\]\n\nA 1 in this field indicates that the\n\nassociated Function supports the\n\ncapability to turn off its Completion\n\ntimeout. This field can be written\n\nfrom the APB bus by setting \\[21\\]
bit\n\nhigh of the pcie_mgmt_APB_ADDR\n\nduring a local management register\n\nwrite"]
pub type CtdsR = crate::BitReader;
#[doc = "Field `AFS` reader - ARI Forwarding Supported \\[AFS\\]\n\nA 1 in this bit indicates that the\n\ndevice is able to forward TLPs with\n\nfunction number greater than 8. This\n\nfield can be written from the APB\n\nbus by setting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type AfsR = crate::BitReader;
#[doc = "Field `AOPRS` reader - Atomic OP routing supported \\[AOPRS\\]\n\nApplicable only to Switch Upstream\n\nPorts, Switch Downstream Ports, and\n\nRoot Ports; must be 0b for other\n\nFunction types. This bit must be set\n\nto 1b if the Port supports this\n\noptional capability. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type AoprsR = crate::BitReader;
#[doc = "Field `ACS32` reader - 32-Bit Atomic Op Completer Supported \\[ACS32\\]\n\nHardwired to 0."]
pub type Acs32R = crate::BitReader;
#[doc = "Field `ACS64` reader - 64-bit Atomic Op Completer Supported \\[ACS64\\]\n\nHardwired to 0."]
pub type Acs64R = crate::BitReader;
#[doc = "Field `ACS128` reader - 128-bit CAS Atomic Op Completer Supported \\[ACS128\\]\n\nHardwired to 0."]
pub type Acs128R = crate::BitReader;
#[doc = "Field `R14` reader - Reserved \\[R14\\]\n\nReserved"]
pub type R14R = crate::BitReader;
#[doc = "Field `LMS` reader - LTR mechanism supported \\[LMS\\]\n\nA value of 1b indicates support for\n\nthe optional Latency Tolerance\n\nReporting (LTR) mechanism. This\n\nfield can be written from the APB\n\nbus by setting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type LmsR = crate::BitReader;
#[doc = "Field `TPHC` reader - TPH Completer Supported \\[TPHC\\]\n\nThese bits, when set, indicate that\n\nthe Function is capable of serving as\n\na completer for requests with\n\nTransaction Processing Hints (TPH).\n\nIt can be turned off for all Physical\n\nFunctions by writing into PF 0.\n\nDefined Encodings are: 00b TPH and\n\nExtended TPH Completer not\n\nsupported. 01b TPH Completer\n\nsupported; Extended TPH Completer\n\nnot supported. 10b Reserved. 11b\n\nBoth TPH and Extended TPH\n\nCompleter supported."]
pub type TphcR = crate::BitReader;
#[doc = "Field `R15` reader - Reserved \\[R15\\]\n\nReserved"]
pub type R15R = crate::FieldReader;
#[doc = "Field `OBFF` reader - OBFF Supported \\[OBFF\\]\n\nA 1 in this bit position indicates that\n\nthe Function supports the Optimized\n\nBuffer Flush/Fill (OBFF) capability\n\nusing message signaling. This field\n\ncan be written from the APB bus by\n\nsetting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write"]
pub type ObffR = crate::FieldReader;
#[doc = "Field `EXFS` reader - Extended Format Field Supported \\[EXFS\\]\n\nIndicates that the Function supports\n\nthe 3-bit definition of the Fmt field in\n\nthe TLP header. This bit is hardwired\n\nto 1 for all Physical Functions."]
pub type ExfsR = crate::BitReader;
#[doc = "Field `EEPS` reader - End-End TLP Prefix Supported \\[EEPS\\]\n\nhard coded to zero."]
pub type EepsR = crate::BitReader;
#[doc = "Field `MEEP` reader - Max End- End TLP Prefixes \\[MEEP\\]\n\nhard coded to zero."]
pub type MeepR = crate::FieldReader;
#[doc = "Field `R16` reader - Reserved \\[R16\\]\n\nReserved"]
pub type R16R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Completion Timeout Ranges \\[CTR\\]\n\nSpecifies the Completion Timeout\n\nvalues supported by the device. This\n\nfield is set by default to 0010 (10 ms\n\n- 250 ms), but can be modified from\n\nthe local management APB bus. The\n\nactual timeout values are in two\n\nprogrammable local management\n\nregisters, which allow the timeout\n\nsettings of the two sub-ranges\n\nwithin Range B to be programmed\n\nindependently. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Completion Timeout Disable Supported \\[CTDS\\]\n\nA 1 in this field indicates that the\n\nassociated Function supports the\n\ncapability to turn off its Completion\n\ntimeout. This field can be written\n\nfrom the APB bus by setting \\[21\\]
bit\n\nhigh of the pcie_mgmt_APB_ADDR\n\nduring a local management register\n\nwrite"]
    #[inline(always)]
    pub fn ctds(&self) -> CtdsR {
        CtdsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ARI Forwarding Supported \\[AFS\\]\n\nA 1 in this bit indicates that the\n\ndevice is able to forward TLPs with\n\nfunction number greater than 8. This\n\nfield can be written from the APB\n\nbus by setting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn afs(&self) -> AfsR {
        AfsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Atomic OP routing supported \\[AOPRS\\]\n\nApplicable only to Switch Upstream\n\nPorts, Switch Downstream Ports, and\n\nRoot Ports; must be 0b for other\n\nFunction types. This bit must be set\n\nto 1b if the Port supports this\n\noptional capability. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn aoprs(&self) -> AoprsR {
        AoprsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 32-Bit Atomic Op Completer Supported \\[ACS32\\]\n\nHardwired to 0."]
    #[inline(always)]
    pub fn acs32(&self) -> Acs32R {
        Acs32R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 64-bit Atomic Op Completer Supported \\[ACS64\\]\n\nHardwired to 0."]
    #[inline(always)]
    pub fn acs64(&self) -> Acs64R {
        Acs64R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 128-bit CAS Atomic Op Completer Supported \\[ACS128\\]\n\nHardwired to 0."]
    #[inline(always)]
    pub fn acs128(&self) -> Acs128R {
        Acs128R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved \\[R14\\]\n\nReserved"]
    #[inline(always)]
    pub fn r14(&self) -> R14R {
        R14R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LTR mechanism supported \\[LMS\\]\n\nA value of 1b indicates support for\n\nthe optional Latency Tolerance\n\nReporting (LTR) mechanism. This\n\nfield can be written from the APB\n\nbus by setting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn lms(&self) -> LmsR {
        LmsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TPH Completer Supported \\[TPHC\\]\n\nThese bits, when set, indicate that\n\nthe Function is capable of serving as\n\na completer for requests with\n\nTransaction Processing Hints (TPH).\n\nIt can be turned off for all Physical\n\nFunctions by writing into PF 0.\n\nDefined Encodings are: 00b TPH and\n\nExtended TPH Completer not\n\nsupported. 01b TPH Completer\n\nsupported; Extended TPH Completer\n\nnot supported. 10b Reserved. 11b\n\nBoth TPH and Extended TPH\n\nCompleter supported."]
    #[inline(always)]
    pub fn tphc(&self) -> TphcR {
        TphcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:17 - Reserved \\[R15\\]\n\nReserved"]
    #[inline(always)]
    pub fn r15(&self) -> R15R {
        R15R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - OBFF Supported \\[OBFF\\]\n\nA 1 in this bit position indicates that\n\nthe Function supports the Optimized\n\nBuffer Flush/Fill (OBFF) capability\n\nusing message signaling. This field\n\ncan be written from the APB bus by\n\nsetting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write"]
    #[inline(always)]
    pub fn obff(&self) -> ObffR {
        ObffR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Extended Format Field Supported \\[EXFS\\]\n\nIndicates that the Function supports\n\nthe 3-bit definition of the Fmt field in\n\nthe TLP header. This bit is hardwired\n\nto 1 for all Physical Functions."]
    #[inline(always)]
    pub fn exfs(&self) -> ExfsR {
        ExfsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - End-End TLP Prefix Supported \\[EEPS\\]\n\nhard coded to zero."]
    #[inline(always)]
    pub fn eeps(&self) -> EepsR {
        EepsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Max End- End TLP Prefixes \\[MEEP\\]\n\nhard coded to zero."]
    #[inline(always)]
    pub fn meep(&self) -> MeepR {
        MeepR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Reserved \\[R16\\]\n\nReserved"]
    #[inline(always)]
    pub fn r16(&self) -> R16R {
        R16R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "PCI Express Device Capabilities 2 Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_pci_express_device_capabilities_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcPciExpressDeviceCapabilities2Spec;
impl crate::RegisterSpec for PcieRcPciExpressDeviceCapabilities2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_pci_express_device_capabilities_2::R`](R) reader structure"]
impl crate::Readable for PcieRcPciExpressDeviceCapabilities2Spec {}
#[doc = "`reset()` method sets PCIE_RC_PCI_EXPRESS_DEVICE_CAPABILITIES_2 to value 0x0004_1872"]
impl crate::Resettable for PcieRcPciExpressDeviceCapabilities2Spec {
    const RESET_VALUE: u32 = 0x0004_1872;
}
