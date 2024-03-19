#[doc = "Register `PCIE_PF_ARI_EXTENDED_CAPABILITY_HEADER` reader"]
pub type R = crate::R<PciePfAriExtendedCapabilityHeaderSpec>;
#[doc = "Field `PECID` reader - PCI Express Extended Capability ID \\[PECID\\]
This field is hardwired to the Capability ID assigned by PCI-SIG to the ARI Extended Capability (000E hex)."]
pub type PecidR = crate::FieldReader<u16>;
#[doc = "Field `ARICV` reader - Capability Version \\[ARICV\\]
Specifies the SIG-assigned value for the version of the capability structure. This field is set to 1 by default, but can be modified independently for each Function from the local management bus"]
pub type AricvR = crate::FieldReader;
#[doc = "Field `ARINCO` reader - Next Capability Offset \\[ARINCO\\]
Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub type ArincoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PCI Express Extended Capability ID \\[PECID\\]
This field is hardwired to the Capability ID assigned by PCI-SIG to the ARI Extended Capability (000E hex)."]
    #[inline(always)]
    pub fn pecid(&self) -> PecidR {
        PecidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Capability Version \\[ARICV\\]
Specifies the SIG-assigned value for the version of the capability structure. This field is set to 1 by default, but can be modified independently for each Function from the local management bus"]
    #[inline(always)]
    pub fn aricv(&self) -> AricvR {
        AricvR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Next Capability Offset \\[ARINCO\\]
Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub fn arinco(&self) -> ArincoR {
        ArincoR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "ARI Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_ari_extended_capability_header::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfAriExtendedCapabilityHeaderSpec;
impl crate::RegisterSpec for PciePfAriExtendedCapabilityHeaderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_ari_extended_capability_header::R`](R) reader structure"]
impl crate::Readable for PciePfAriExtendedCapabilityHeaderSpec {}
#[doc = "`reset()` method sets PCIE_PF_ARI_EXTENDED_CAPABILITY_HEADER to value 0x1601_000e"]
impl crate::Resettable for PciePfAriExtendedCapabilityHeaderSpec {
    const RESET_VALUE: u32 = 0x1601_000e;
}
