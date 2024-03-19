#[doc = "Register `PCIE_PF_PCI_EXPRESS_DEVICE_CAPABILITIES` reader"]
pub type R = crate::R<PciePfPciExpressDeviceCapabilitiesSpec>;
#[doc = "Field `MPS` reader - Max Payload Size \\[MPS\\]\n\nSpecifies maximum payload size\n\nsupported by the device."]
pub type MpsR = crate::FieldReader;
#[doc = "Field `PFS` reader - Phantom Functions Supported \\[PFS\\]\n\nThis field is used to extend the tag\n\nfield by combining unused Function\n\nbits with the tag bits. This field is\n\nhardwired to 00 to disable this\n\nfeature."]
pub type PfsR = crate::FieldReader;
#[doc = "Field `ETFS` reader - Extended Tag Field Supported \\[ETFS\\]\n\nExtended Tag Field Not Supported.\n\nHard coded to 0."]
pub type EtfsR = crate::BitReader;
#[doc = "Field `AL0SL` reader - Acceptable L0S Latency \\[AL0SL\\]\n\nSpecifies acceptable latency that the\n\nEndpoint can tolerate while\n\ntransitioning from L0S to L0. It is set\n\nby default to the value define in\n\nreg_defaults.h. It can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
pub type Al0slR = crate::FieldReader;
#[doc = "Field `AL1SL` reader - Acceptable L1 Latency \\[AL1SL\\]\n\nSpecifies acceptable latency that the\n\nEndpoint can tolerate while\n\ntransitioning from L1 to L0. It is set\n\nby default to the value define in\n\nreg_defaults.h. It can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
pub type Al1slR = crate::FieldReader;
#[doc = "Field `R1` reader - Reserved \\[R1\\]\n\nReserved"]
pub type R1R = crate::FieldReader;
#[doc = "Field `RBER` reader - Role- Based Error Reporting \\[RBER\\]\n\nEnables role-based error reporting.\n\nIt is hardwired to 1.It can be re-\n\nwritten independently for each\n\nFunction from the local management\n\nbus."]
pub type RberR = crate::BitReader;
#[doc = "Field `R2` reader - Reserved \\[R2\\]\n\nReserved"]
pub type R2R = crate::FieldReader;
#[doc = "Field `CSPLV` reader - Captured Slot Power Limit Value \\[CSPLV\\]\n\nSpecifies upper limit on power\n\nsupplied by slot. It is set by default\n\nto the value define in reg_defaults.h.\n\nIt can be re-written independently\n\nfor each Function from the local\n\nmanagement bus."]
pub type CsplvR = crate::FieldReader;
#[doc = "Field `CPLS` reader - Captured Power Limit Scale \\[CPLS\\]\n\nSpecifies the scale used by Slot\n\nPower Limit Value. It is set by\n\ndefault to the value define in\n\nreg_defaults.h. It can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
pub type CplsR = crate::FieldReader;
#[doc = "Field `FC` reader - FLR Capable \\[FC\\]\n\nSet when device has Function-Level\n\nReset capability. It is set by default\n\nto 1. It can be re- written\n\nindependently for each Function\n\nfrom the local management bus."]
pub type FcR = crate::BitReader;
#[doc = "Field `R3` reader - Reserved \\[R3\\]\n\nReserved"]
pub type R3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Max Payload Size \\[MPS\\]\n\nSpecifies maximum payload size\n\nsupported by the device."]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Phantom Functions Supported \\[PFS\\]\n\nThis field is used to extend the tag\n\nfield by combining unused Function\n\nbits with the tag bits. This field is\n\nhardwired to 00 to disable this\n\nfeature."]
    #[inline(always)]
    pub fn pfs(&self) -> PfsR {
        PfsR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Extended Tag Field Supported \\[ETFS\\]\n\nExtended Tag Field Not Supported.\n\nHard coded to 0."]
    #[inline(always)]
    pub fn etfs(&self) -> EtfsR {
        EtfsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Acceptable L0S Latency \\[AL0SL\\]\n\nSpecifies acceptable latency that the\n\nEndpoint can tolerate while\n\ntransitioning from L0S to L0. It is set\n\nby default to the value define in\n\nreg_defaults.h. It can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn al0sl(&self) -> Al0slR {
        Al0slR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Acceptable L1 Latency \\[AL1SL\\]\n\nSpecifies acceptable latency that the\n\nEndpoint can tolerate while\n\ntransitioning from L1 to L0. It is set\n\nby default to the value define in\n\nreg_defaults.h. It can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn al1sl(&self) -> Al1slR {
        Al1slR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Reserved \\[R1\\]\n\nReserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Role- Based Error Reporting \\[RBER\\]\n\nEnables role-based error reporting.\n\nIt is hardwired to 1.It can be re-\n\nwritten independently for each\n\nFunction from the local management\n\nbus."]
    #[inline(always)]
    pub fn rber(&self) -> RberR {
        RberR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Reserved \\[R2\\]\n\nReserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:25 - Captured Slot Power Limit Value \\[CSPLV\\]\n\nSpecifies upper limit on power\n\nsupplied by slot. It is set by default\n\nto the value define in reg_defaults.h.\n\nIt can be re-written independently\n\nfor each Function from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn csplv(&self) -> CsplvR {
        CsplvR::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 26:27 - Captured Power Limit Scale \\[CPLS\\]\n\nSpecifies the scale used by Slot\n\nPower Limit Value. It is set by\n\ndefault to the value define in\n\nreg_defaults.h. It can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn cpls(&self) -> CplsR {
        CplsR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - FLR Capable \\[FC\\]\n\nSet when device has Function-Level\n\nReset capability. It is set by default\n\nto 1. It can be re- written\n\nindependently for each Function\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn fc(&self) -> FcR {
        FcR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Reserved \\[R3\\]\n\nReserved"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "PCI Express Device Capabilities Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_pci_express_device_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfPciExpressDeviceCapabilitiesSpec;
impl crate::RegisterSpec for PciePfPciExpressDeviceCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_pci_express_device_capabilities::R`](R) reader structure"]
impl crate::Readable for PciePfPciExpressDeviceCapabilitiesSpec {}
#[doc = "`reset()` method sets PCIE_PF_PCI_EXPRESS_DEVICE_CAPABILITIES to value 0x1000_8101"]
impl crate::Resettable for PciePfPciExpressDeviceCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x1000_8101;
}
