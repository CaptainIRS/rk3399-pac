#[doc = "Register `PCIE_RC_PCI_EXPRESS_DEVICE_CAPABILITIES_2` reader"]
pub type R = crate::R<PcieRcPciExpressDeviceCapabilities2Spec>;
#[doc = "Field `CTR` reader - Completion Timeout Ranges \\[CTR\\]
Specifies the Completion Timeout values supported by the device. This field is set by default to 0010 (10 ms - 250 ms), but can be modified from the local management APB bus. The actual timeout values are in two programmable local management registers, which allow the timeout settings of the two sub-ranges within Range B to be programmed independently. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type CtrR = crate::FieldReader;
#[doc = "Field `CTDS` reader - Completion Timeout Disable Supported \\[CTDS\\]
A 1 in this field indicates that the associated Function supports the capability to turn off its Completion timeout. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write"]
pub type CtdsR = crate::BitReader;
#[doc = "Field `AFS` reader - ARI Forwarding Supported \\[AFS\\]
A 1 in this bit indicates that the device is able to forward TLPs with function number greater than 8. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type AfsR = crate::BitReader;
#[doc = "Field `AOPRS` reader - Atomic OP routing supported \\[AOPRS\\]
Applicable only to Switch Upstream Ports, Switch Downstream Ports, and Root Ports; must be 0b for other Function types. This bit must be set to 1b if the Port supports this optional capability. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type AoprsR = crate::BitReader;
#[doc = "Field `ACS32` reader - 32-Bit Atomic Op Completer Supported \\[ACS32\\]
Hardwired to 0."]
pub type Acs32R = crate::BitReader;
#[doc = "Field `ACS64` reader - 64-bit Atomic Op Completer Supported \\[ACS64\\]
Hardwired to 0."]
pub type Acs64R = crate::BitReader;
#[doc = "Field `ACS128` reader - 128-bit CAS Atomic Op Completer Supported \\[ACS128\\]
Hardwired to 0."]
pub type Acs128R = crate::BitReader;
#[doc = "Field `R14` reader - Reserved \\[R14\\]
Reserved"]
pub type R14R = crate::BitReader;
#[doc = "Field `LMS` reader - LTR mechanism supported \\[LMS\\]
A value of 1b indicates support for the optional Latency Tolerance Reporting (LTR) mechanism. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type LmsR = crate::BitReader;
#[doc = "Field `TPHC` reader - TPH Completer Supported \\[TPHC\\]
These bits, when set, indicate that the Function is capable of serving as a completer for requests with Transaction Processing Hints (TPH). It can be turned off for all Physical Functions by writing into PF 0. Defined Encodings are: 00b TPH and Extended TPH Completer not supported. 01b TPH Completer supported; Extended TPH Completer not supported. 10b Reserved. 11b Both TPH and Extended TPH Completer supported."]
pub type TphcR = crate::BitReader;
#[doc = "Field `R15` reader - Reserved \\[R15\\]
Reserved"]
pub type R15R = crate::FieldReader;
#[doc = "Field `OBFF` reader - OBFF Supported \\[OBFF\\]
A 1 in this bit position indicates that the Function supports the Optimized Buffer Flush/Fill (OBFF) capability using message signaling. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write"]
pub type ObffR = crate::FieldReader;
#[doc = "Field `EXFS` reader - Extended Format Field Supported \\[EXFS\\]
Indicates that the Function supports the 3-bit definition of the Fmt field in the TLP header. This bit is hardwired to 1 for all Physical Functions."]
pub type ExfsR = crate::BitReader;
#[doc = "Field `EEPS` reader - End-End TLP Prefix Supported \\[EEPS\\]
hard coded to zero."]
pub type EepsR = crate::BitReader;
#[doc = "Field `MEEP` reader - Max End- End TLP Prefixes \\[MEEP\\]
hard coded to zero."]
pub type MeepR = crate::FieldReader;
#[doc = "Field `R16` reader - Reserved \\[R16\\]
Reserved"]
pub type R16R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Completion Timeout Ranges \\[CTR\\]
Specifies the Completion Timeout values supported by the device. This field is set by default to 0010 (10 ms - 250 ms), but can be modified from the local management APB bus. The actual timeout values are in two programmable local management registers, which allow the timeout settings of the two sub-ranges within Range B to be programmed independently. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Completion Timeout Disable Supported \\[CTDS\\]
A 1 in this field indicates that the associated Function supports the capability to turn off its Completion timeout. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write"]
    #[inline(always)]
    pub fn ctds(&self) -> CtdsR {
        CtdsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ARI Forwarding Supported \\[AFS\\]
A 1 in this bit indicates that the device is able to forward TLPs with function number greater than 8. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn afs(&self) -> AfsR {
        AfsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Atomic OP routing supported \\[AOPRS\\]
Applicable only to Switch Upstream Ports, Switch Downstream Ports, and Root Ports; must be 0b for other Function types. This bit must be set to 1b if the Port supports this optional capability. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn aoprs(&self) -> AoprsR {
        AoprsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 32-Bit Atomic Op Completer Supported \\[ACS32\\]
Hardwired to 0."]
    #[inline(always)]
    pub fn acs32(&self) -> Acs32R {
        Acs32R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 64-bit Atomic Op Completer Supported \\[ACS64\\]
Hardwired to 0."]
    #[inline(always)]
    pub fn acs64(&self) -> Acs64R {
        Acs64R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 128-bit CAS Atomic Op Completer Supported \\[ACS128\\]
Hardwired to 0."]
    #[inline(always)]
    pub fn acs128(&self) -> Acs128R {
        Acs128R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved \\[R14\\]
Reserved"]
    #[inline(always)]
    pub fn r14(&self) -> R14R {
        R14R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LTR mechanism supported \\[LMS\\]
A value of 1b indicates support for the optional Latency Tolerance Reporting (LTR) mechanism. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn lms(&self) -> LmsR {
        LmsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TPH Completer Supported \\[TPHC\\]
These bits, when set, indicate that the Function is capable of serving as a completer for requests with Transaction Processing Hints (TPH). It can be turned off for all Physical Functions by writing into PF 0. Defined Encodings are: 00b TPH and Extended TPH Completer not supported. 01b TPH Completer supported; Extended TPH Completer not supported. 10b Reserved. 11b Both TPH and Extended TPH Completer supported."]
    #[inline(always)]
    pub fn tphc(&self) -> TphcR {
        TphcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:17 - Reserved \\[R15\\]
Reserved"]
    #[inline(always)]
    pub fn r15(&self) -> R15R {
        R15R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - OBFF Supported \\[OBFF\\]
A 1 in this bit position indicates that the Function supports the Optimized Buffer Flush/Fill (OBFF) capability using message signaling. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write"]
    #[inline(always)]
    pub fn obff(&self) -> ObffR {
        ObffR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Extended Format Field Supported \\[EXFS\\]
Indicates that the Function supports the 3-bit definition of the Fmt field in the TLP header. This bit is hardwired to 1 for all Physical Functions."]
    #[inline(always)]
    pub fn exfs(&self) -> ExfsR {
        ExfsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - End-End TLP Prefix Supported \\[EEPS\\]
hard coded to zero."]
    #[inline(always)]
    pub fn eeps(&self) -> EepsR {
        EepsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Max End- End TLP Prefixes \\[MEEP\\]
hard coded to zero."]
    #[inline(always)]
    pub fn meep(&self) -> MeepR {
        MeepR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Reserved \\[R16\\]
Reserved"]
    #[inline(always)]
    pub fn r16(&self) -> R16R {
        R16R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "PCI Express Device Capabilities 2 Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_pci_express_device_capabilities_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
