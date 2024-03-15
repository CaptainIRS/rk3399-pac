#[doc = "Register `PCI_EXPRESS_DEVICE_CAPABILITIES_2` reader"]
pub type R = crate::R<PciExpressDeviceCapabilities2Spec>;
#[doc = "Field `CTR` reader - Completion Timeout Ranges \\[CTR\\]
Specifies the Completion Timeout values supported by the device. This field is set by default to 0010 (10 ms - 250 ms). The actual timeout values are in two programmable local management registers, which allow the timeout settings of the two sub-ranges within Range B to be programmed independently."]
pub type CtrR = crate::FieldReader;
#[doc = "Field `CTDS` reader - Completion Timeout Disable Supported \\[CTDS\\]
A 1 in this field indicates that the associated Function supports the capability to turn off its Completion timeout. This bit is set to 1 by default, but can be re-written independently for each Function from the local management bus."]
pub type CtdsR = crate::BitReader;
#[doc = "Field `AFS` reader - ARI forwarding support \\[AFS\\]
ARI forwarding supported."]
pub type AfsR = crate::BitReader;
#[doc = "Field `OPRS` reader - OP routing supported \\[OPRS\\]
Atomic OP routing supported."]
pub type OprsR = crate::BitReader;
#[doc = "Field `BAOCS32` reader - 32-Bit Atomic Op Completer Supported \\[BAOCS32\\]
Hardwired to 0."]
pub type Baocs32R = crate::BitReader;
#[doc = "Field `BAOCS64` reader - 64-Bit Atomic Op Completer Supported \\[BAOCS64\\]
Hardwired to 0."]
pub type Baocs64R = crate::BitReader;
#[doc = "Field `BAOCS128` reader - 128-Bit CAS Atomic Op Completer Supported \\[BAOCS128\\]
Hardwired to 0."]
pub type Baocs128R = crate::BitReader;
#[doc = "Field `R12` reader - Reserved \\[R12\\]
Reserved"]
pub type R12R = crate::BitReader;
#[doc = "Field `LMS` reader - LTR Mechanism Supported \\[LMS\\]
A 1 in this bit position indicates that the Function supports the Latency Tolerance Reporting (LTR) Capability. This bit is set to 1 by default, but can be turned off for all Physical Functions by writing into PF 0."]
pub type LmsR = crate::BitReader;
#[doc = "Field `TCS` reader - TPH Completer Supported \\[TCS\\]
These bits, when set, indicate that the Function is capable of serving as a completer for requests with Transaction Processing Hints (TPH). It can be turned off for all Physical Functions by writing into PF 0. Defined Encodings are: 00b TPH and Extended TPH Completer not supported. 01b TPH Completer supported; Extended TPH Completer not supported. 10b Reserved. 11b Both TPH and Extended TPH Completer supported."]
pub type TcsR = crate::FieldReader;
#[doc = "Field `R13` reader - Reserved \\[R13\\]
Reserved"]
pub type R13R = crate::FieldReader;
#[doc = "Field `OPFFS` reader - OBFF Supported \\[OPFFS\\]
A 1 in this bit position indicates that the Function supports the Optimized Buffer Flush/Fill (OBFF) capability using message signaling."]
pub type OpffsR = crate::FieldReader;
#[doc = "Field `EXFS` reader - Extended Format Field Supported \\[EXFS\\]
Indicates that the Function supports the 3-bit definition of the Fmt field in the TLP header. This bit is hardwired to 1 for all Physical Functions."]
pub type ExfsR = crate::BitReader;
#[doc = "Field `EEPS` reader - End-End TLP Prefix Supported \\[EEPS\\]
Indicates whether the Function supports End-End TLP Prefixes. A 1 in this field indicates that the Function supports receiving TLPs containing End- End TLP Prefixes."]
pub type EepsR = crate::BitReader;
#[doc = "Field `MEEP` reader - Max End- End TLP Prefixes \\[MEEP\\]
Indicates the maximum number of End-End TLP Prefixes supported by the Function. The supported values are: 01b 1 End-End TLP Prefix 10b 2 End- End TLP Prefixes"]
pub type MeepR = crate::FieldReader;
#[doc = "Field `R14` reader - Reserved \\[R14\\]
Reserved"]
pub type R14R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Completion Timeout Ranges \\[CTR\\]
Specifies the Completion Timeout values supported by the device. This field is set by default to 0010 (10 ms - 250 ms). The actual timeout values are in two programmable local management registers, which allow the timeout settings of the two sub-ranges within Range B to be programmed independently."]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Completion Timeout Disable Supported \\[CTDS\\]
A 1 in this field indicates that the associated Function supports the capability to turn off its Completion timeout. This bit is set to 1 by default, but can be re-written independently for each Function from the local management bus."]
    #[inline(always)]
    pub fn ctds(&self) -> CtdsR {
        CtdsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ARI forwarding support \\[AFS\\]
ARI forwarding supported."]
    #[inline(always)]
    pub fn afs(&self) -> AfsR {
        AfsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OP routing supported \\[OPRS\\]
Atomic OP routing supported."]
    #[inline(always)]
    pub fn oprs(&self) -> OprsR {
        OprsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 32-Bit Atomic Op Completer Supported \\[BAOCS32\\]
Hardwired to 0."]
    #[inline(always)]
    pub fn baocs32(&self) -> Baocs32R {
        Baocs32R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 64-Bit Atomic Op Completer Supported \\[BAOCS64\\]
Hardwired to 0."]
    #[inline(always)]
    pub fn baocs64(&self) -> Baocs64R {
        Baocs64R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 128-Bit CAS Atomic Op Completer Supported \\[BAOCS128\\]
Hardwired to 0."]
    #[inline(always)]
    pub fn baocs128(&self) -> Baocs128R {
        Baocs128R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved \\[R12\\]
Reserved"]
    #[inline(always)]
    pub fn r12(&self) -> R12R {
        R12R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LTR Mechanism Supported \\[LMS\\]
A 1 in this bit position indicates that the Function supports the Latency Tolerance Reporting (LTR) Capability. This bit is set to 1 by default, but can be turned off for all Physical Functions by writing into PF 0."]
    #[inline(always)]
    pub fn lms(&self) -> LmsR {
        LmsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - TPH Completer Supported \\[TCS\\]
These bits, when set, indicate that the Function is capable of serving as a completer for requests with Transaction Processing Hints (TPH). It can be turned off for all Physical Functions by writing into PF 0. Defined Encodings are: 00b TPH and Extended TPH Completer not supported. 01b TPH Completer supported; Extended TPH Completer not supported. 10b Reserved. 11b Both TPH and Extended TPH Completer supported."]
    #[inline(always)]
    pub fn tcs(&self) -> TcsR {
        TcsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17 - Reserved \\[R13\\]
Reserved"]
    #[inline(always)]
    pub fn r13(&self) -> R13R {
        R13R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - OBFF Supported \\[OPFFS\\]
A 1 in this bit position indicates that the Function supports the Optimized Buffer Flush/Fill (OBFF) capability using message signaling."]
    #[inline(always)]
    pub fn opffs(&self) -> OpffsR {
        OpffsR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Extended Format Field Supported \\[EXFS\\]
Indicates that the Function supports the 3-bit definition of the Fmt field in the TLP header. This bit is hardwired to 1 for all Physical Functions."]
    #[inline(always)]
    pub fn exfs(&self) -> ExfsR {
        ExfsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - End-End TLP Prefix Supported \\[EEPS\\]
Indicates whether the Function supports End-End TLP Prefixes. A 1 in this field indicates that the Function supports receiving TLPs containing End- End TLP Prefixes."]
    #[inline(always)]
    pub fn eeps(&self) -> EepsR {
        EepsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Max End- End TLP Prefixes \\[MEEP\\]
Indicates the maximum number of End-End TLP Prefixes supported by the Function. The supported values are: 01b 1 End-End TLP Prefix 10b 2 End- End TLP Prefixes"]
    #[inline(always)]
    pub fn meep(&self) -> MeepR {
        MeepR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Reserved \\[R14\\]
Reserved"]
    #[inline(always)]
    pub fn r14(&self) -> R14R {
        R14R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "PCI Express Device Capabilities Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pci_express_device_capabilities_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciExpressDeviceCapabilities2Spec;
impl crate::RegisterSpec for PciExpressDeviceCapabilities2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pci_express_device_capabilities_2::R`](R) reader structure"]
impl crate::Readable for PciExpressDeviceCapabilities2Spec {}
#[doc = "`reset()` method sets PCI_EXPRESS_DEVICE_CAPABILITIES_2 to value 0x0004_1812"]
impl crate::Resettable for PciExpressDeviceCapabilities2Spec {
    const RESET_VALUE: u32 = 0x0004_1812;
}
