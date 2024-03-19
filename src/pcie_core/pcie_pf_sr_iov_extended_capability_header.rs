#[doc = "Register `PCIE_PF_SR_IOV_EXTENDED_CAPABILITY_HEADER` reader"]
pub type R = crate::R<PciePfSrIovExtendedCapabilityHeaderSpec>;
#[doc = "Field `PECID` reader - PCI Express Extended Capability ID \\[PECID\\]\n\nThis field is hardwired to the\n\nCapability ID assigned by PCI-SIG to\n\nthe SR-IOV Extended Capability\n\nStructure (0010 hex)."]
pub type PecidR = crate::FieldReader<u16>;
#[doc = "Field `CV` reader - Capability Version \\[CV\\]\n\nSpecifies the SIG-assigned value for\n\nthe version of the capability\n\nstructure. This field is set by default\n\nto 1, but can be modified\n\nindependently for each Function\n\nfrom the local management bus."]
pub type CvR = crate::FieldReader;
#[doc = "Field `NCO` reader - Next Capability Offset \\[NCO\\]\n\nIndicates offset to the next PCI\n\nExpress capability structure. The\n\ndefault next pointer value is dynamic\n\nand is dependent on whether the\n\nstrap or LMI bits are set."]
pub type NcoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PCI Express Extended Capability ID \\[PECID\\]\n\nThis field is hardwired to the\n\nCapability ID assigned by PCI-SIG to\n\nthe SR-IOV Extended Capability\n\nStructure (0010 hex)."]
    #[inline(always)]
    pub fn pecid(&self) -> PecidR {
        PecidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Capability Version \\[CV\\]\n\nSpecifies the SIG-assigned value for\n\nthe version of the capability\n\nstructure. This field is set by default\n\nto 1, but can be modified\n\nindependently for each Function\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn cv(&self) -> CvR {
        CvR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Next Capability Offset \\[NCO\\]\n\nIndicates offset to the next PCI\n\nExpress capability structure. The\n\ndefault next pointer value is dynamic\n\nand is dependent on whether the\n\nstrap or LMI bits are set."]
    #[inline(always)]
    pub fn nco(&self) -> NcoR {
        NcoR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "SR-IOV Extended Capability Header Register\n\nIndicates offset to the next PCI\n\nExpress capability structure. The\n\ndefault next pointer value is dynamic\n\nand is dependent on whether the\n\nstrap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_sr_iov_extended_capability_header::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfSrIovExtendedCapabilityHeaderSpec;
impl crate::RegisterSpec for PciePfSrIovExtendedCapabilityHeaderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_sr_iov_extended_capability_header::R`](R) reader structure"]
impl crate::Readable for PciePfSrIovExtendedCapabilityHeaderSpec {}
#[doc = "`reset()` method sets PCIE_PF_SR_IOV_EXTENDED_CAPABILITY_HEADER to value 0x2741_0010"]
impl crate::Resettable for PciePfSrIovExtendedCapabilityHeaderSpec {
    const RESET_VALUE: u32 = 0x2741_0010;
}
