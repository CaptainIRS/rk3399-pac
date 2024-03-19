#[doc = "Register `PCIE_PF_PCI_EXPRESS_DEVICE_CAPABILITIES_2` reader"]
pub type R = crate::R<PciePfPciExpressDeviceCapabilities2Spec>;
#[doc = "Field `CTR` reader - Completion Timeout Ranges \\[CTR\\]\n\nSpecifies the Completion Timeout\n\nvalues supported by the device. This\n\nfield is set by default to 0010 (10\n\nms - 250 ms). The actual timeout\n\nvalues are in two programmable\n\nlocal management registers, which\n\nallow the timeout settings of the two\n\nsub-ranges within Range B to be\n\nprogrammed independently."]
pub type CtrR = crate::FieldReader;
#[doc = "Field `CTDS` reader - Completion Timeout Disable Supported \\[CTDS\\]\n\nA 1 in this field indicates that the\n\nassociated Function supports the\n\ncapability to turn off its Completion\n\ntimeout. This bit is set to 1 by\n\ndefault, but can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
pub type CtdsR = crate::BitReader;
#[doc = "Field `AFS` reader - ARI forwarding support \\[AFS\\]\n\nARI forwarding supported."]
pub type AfsR = crate::BitReader;
#[doc = "Field `OPRS` reader - OP routing supported \\[OPRS\\]\n\nAtomic OP routing supported."]
pub type OprsR = crate::BitReader;
#[doc = "Field `BAOCS32` reader - 32-Bit Atomic Op Completer Supported \\[BAOCS32\\]\n\nHardwired to 0."]
pub type Baocs32R = crate::BitReader;
#[doc = "Field `BAOCS64` reader - 64-Bit Atomic Op Completer Supported \\[BAOCS64\\]\n\nHardwired to 0."]
pub type Baocs64R = crate::BitReader;
#[doc = "Field `BAOCS128` reader - 128-Bit CAS Atomic Op Completer Supported \\[BAOCS128\\]\n\nHardwired to 0."]
pub type Baocs128R = crate::BitReader;
#[doc = "Field `R12` reader - Reserved \\[R12\\]\n\nReserved"]
pub type R12R = crate::BitReader;
#[doc = "Field `LMS` reader - LTR Mechanism Supported \\[LMS\\]\n\nA 1 in this bit position indicates that\n\nthe Function supports the Latency\n\nTolerance Reporting (LTR) Capability.\n\nThis bit is set to 1 by default, but\n\ncan be turned off for all Physical\n\nFunctions by writing into PF 0."]
pub type LmsR = crate::BitReader;
#[doc = "Field `TCS` reader - TPH Completer Supported \\[TCS\\]\n\nThese bits, when set, indicate that\n\nthe Function is capable of serving as\n\na completer for requests with\n\nTransaction Processing Hints (TPH).\n\nIt can be turned off for all Physical\n\nFunctions by writing into PF 0.\n\nDefined Encodings are: 00b TPH and\n\nExtended TPH Completer not\n\nsupported. 01b TPH Completer\n\nsupported; Extended TPH Completer\n\nnot supported. 10b Reserved. 11b\n\nBoth TPH and Extended TPH\n\nCompleter supported."]
pub type TcsR = crate::FieldReader;
#[doc = "Field `R13` reader - Reserved \\[R13\\]\n\nReserved"]
pub type R13R = crate::FieldReader;
#[doc = "Field `OPFFS` reader - OBFF Supported \\[OPFFS\\]\n\nA 1 in this bit position indicates that\n\nthe Function supports the Optimized\n\nBuffer Flush/Fill (OBFF) capability\n\nusing message signaling."]
pub type OpffsR = crate::FieldReader;
#[doc = "Field `EXFS` reader - Extended Format Field Supported \\[EXFS\\]\n\nIndicates that the Function supports\n\nthe 3-bit definition of the Fmt field\n\nin the TLP header. This bit is\n\nhardwired to 1 for all Physical\n\nFunctions."]
pub type ExfsR = crate::BitReader;
#[doc = "Field `EEPS` reader - End-End TLP Prefix Supported \\[EEPS\\]\n\nIndicates whether the Function\n\nsupports End-End TLP Prefixes. A 1\n\nin this field indicates that the\n\nFunction supports receiving TLPs\n\ncontaining End- End TLP Prefixes."]
pub type EepsR = crate::BitReader;
#[doc = "Field `MEEP` reader - Max End- End TLP Prefixes \\[MEEP\\]\n\nIndicates the maximum number of\n\nEnd-End TLP Prefixes supported by\n\nthe Function. The supported values\n\nare: 01b 1 End-End TLP Prefix 10b 2\n\nEnd- End TLP Prefixes"]
pub type MeepR = crate::FieldReader;
#[doc = "Field `R14` reader - Reserved \\[R14\\]\n\nReserved"]
pub type R14R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Completion Timeout Ranges \\[CTR\\]\n\nSpecifies the Completion Timeout\n\nvalues supported by the device. This\n\nfield is set by default to 0010 (10\n\nms - 250 ms). The actual timeout\n\nvalues are in two programmable\n\nlocal management registers, which\n\nallow the timeout settings of the two\n\nsub-ranges within Range B to be\n\nprogrammed independently."]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Completion Timeout Disable Supported \\[CTDS\\]\n\nA 1 in this field indicates that the\n\nassociated Function supports the\n\ncapability to turn off its Completion\n\ntimeout. This bit is set to 1 by\n\ndefault, but can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn ctds(&self) -> CtdsR {
        CtdsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ARI forwarding support \\[AFS\\]\n\nARI forwarding supported."]
    #[inline(always)]
    pub fn afs(&self) -> AfsR {
        AfsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OP routing supported \\[OPRS\\]\n\nAtomic OP routing supported."]
    #[inline(always)]
    pub fn oprs(&self) -> OprsR {
        OprsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 32-Bit Atomic Op Completer Supported \\[BAOCS32\\]\n\nHardwired to 0."]
    #[inline(always)]
    pub fn baocs32(&self) -> Baocs32R {
        Baocs32R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 64-Bit Atomic Op Completer Supported \\[BAOCS64\\]\n\nHardwired to 0."]
    #[inline(always)]
    pub fn baocs64(&self) -> Baocs64R {
        Baocs64R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 128-Bit CAS Atomic Op Completer Supported \\[BAOCS128\\]\n\nHardwired to 0."]
    #[inline(always)]
    pub fn baocs128(&self) -> Baocs128R {
        Baocs128R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved \\[R12\\]\n\nReserved"]
    #[inline(always)]
    pub fn r12(&self) -> R12R {
        R12R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LTR Mechanism Supported \\[LMS\\]\n\nA 1 in this bit position indicates that\n\nthe Function supports the Latency\n\nTolerance Reporting (LTR) Capability.\n\nThis bit is set to 1 by default, but\n\ncan be turned off for all Physical\n\nFunctions by writing into PF 0."]
    #[inline(always)]
    pub fn lms(&self) -> LmsR {
        LmsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - TPH Completer Supported \\[TCS\\]\n\nThese bits, when set, indicate that\n\nthe Function is capable of serving as\n\na completer for requests with\n\nTransaction Processing Hints (TPH).\n\nIt can be turned off for all Physical\n\nFunctions by writing into PF 0.\n\nDefined Encodings are: 00b TPH and\n\nExtended TPH Completer not\n\nsupported. 01b TPH Completer\n\nsupported; Extended TPH Completer\n\nnot supported. 10b Reserved. 11b\n\nBoth TPH and Extended TPH\n\nCompleter supported."]
    #[inline(always)]
    pub fn tcs(&self) -> TcsR {
        TcsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17 - Reserved \\[R13\\]\n\nReserved"]
    #[inline(always)]
    pub fn r13(&self) -> R13R {
        R13R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - OBFF Supported \\[OPFFS\\]\n\nA 1 in this bit position indicates that\n\nthe Function supports the Optimized\n\nBuffer Flush/Fill (OBFF) capability\n\nusing message signaling."]
    #[inline(always)]
    pub fn opffs(&self) -> OpffsR {
        OpffsR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Extended Format Field Supported \\[EXFS\\]\n\nIndicates that the Function supports\n\nthe 3-bit definition of the Fmt field\n\nin the TLP header. This bit is\n\nhardwired to 1 for all Physical\n\nFunctions."]
    #[inline(always)]
    pub fn exfs(&self) -> ExfsR {
        ExfsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - End-End TLP Prefix Supported \\[EEPS\\]\n\nIndicates whether the Function\n\nsupports End-End TLP Prefixes. A 1\n\nin this field indicates that the\n\nFunction supports receiving TLPs\n\ncontaining End- End TLP Prefixes."]
    #[inline(always)]
    pub fn eeps(&self) -> EepsR {
        EepsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Max End- End TLP Prefixes \\[MEEP\\]\n\nIndicates the maximum number of\n\nEnd-End TLP Prefixes supported by\n\nthe Function. The supported values\n\nare: 01b 1 End-End TLP Prefix 10b 2\n\nEnd- End TLP Prefixes"]
    #[inline(always)]
    pub fn meep(&self) -> MeepR {
        MeepR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Reserved \\[R14\\]\n\nReserved"]
    #[inline(always)]
    pub fn r14(&self) -> R14R {
        R14R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "PCI Express Device Capabilities Register 2\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_pci_express_device_capabilities_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfPciExpressDeviceCapabilities2Spec;
impl crate::RegisterSpec for PciePfPciExpressDeviceCapabilities2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_pci_express_device_capabilities_2::R`](R) reader structure"]
impl crate::Readable for PciePfPciExpressDeviceCapabilities2Spec {}
#[doc = "`reset()` method sets PCIE_PF_PCI_EXPRESS_DEVICE_CAPABILITIES_2 to value 0x0004_1812"]
impl crate::Resettable for PciePfPciExpressDeviceCapabilities2Spec {
    const RESET_VALUE: u32 = 0x0004_1812;
}
