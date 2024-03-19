#[doc = "Register `PCIE_VF_ARI_EXTENDED_CAPABILITY_HEADER` reader"]
pub type R = crate::R<PcieVfAriExtendedCapabilityHeaderSpec>;
#[doc = "Field `PCCID` reader - PCI Express Extended Capability ID \\[PCCID\\]\n\nThis field is hardwired to the\n\nCapability ID assigned by PCI-SIG to\n\nthe ARI Extended Capability (000E\n\nhex)."]
pub type PccidR = crate::FieldReader<u16>;
#[doc = "Field `CV` reader - Capability Version \\[CV\\]\n\nSpecifies the SIG-assigned value for\n\nthe version of the capability\n\nstructure. This field is taken from the\n\nsetting of the corresponding field in\n\nthe ARI Extended Capability Header\n\nRegister of PF 0."]
pub type CvR = crate::FieldReader;
#[doc = "Field `NCO` reader - Next Capability Offset \\[NCO\\]\n\nIndicates offset to the next PCI\n\nExpress capability structure. The\n\ndefault next pointer value is dynamic\n\nand is dependent on whether the\n\nstrap or LMI bits are set."]
pub type NcoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PCI Express Extended Capability ID \\[PCCID\\]\n\nThis field is hardwired to the\n\nCapability ID assigned by PCI-SIG to\n\nthe ARI Extended Capability (000E\n\nhex)."]
    #[inline(always)]
    pub fn pccid(&self) -> PccidR {
        PccidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Capability Version \\[CV\\]\n\nSpecifies the SIG-assigned value for\n\nthe version of the capability\n\nstructure. This field is taken from the\n\nsetting of the corresponding field in\n\nthe ARI Extended Capability Header\n\nRegister of PF 0."]
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
#[doc = "ARI Extended Capability Header Register\n\nIndicates offset to the next PCI\n\nExpress capability structure. The\n\ndefault next pointer value is dynamic\n\nand is dependent on whether the\n\nstrap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_ari_extended_capability_header::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfAriExtendedCapabilityHeaderSpec;
impl crate::RegisterSpec for PcieVfAriExtendedCapabilityHeaderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_ari_extended_capability_header::R`](R) reader structure"]
impl crate::Readable for PcieVfAriExtendedCapabilityHeaderSpec {}
#[doc = "`reset()` method sets PCIE_VF_ARI_EXTENDED_CAPABILITY_HEADER to value 0x2741_000e"]
impl crate::Resettable for PcieVfAriExtendedCapabilityHeaderSpec {
    const RESET_VALUE: u32 = 0x2741_000e;
}
