#[doc = "Register `PCIE_PF_POWER_BUDGETING_ENHANCED_CAPABILITY_HEADER` reader"]
pub type R = crate::R<PciePfPowerBudgetingEnhancedCapabilityHeaderSpec>;
#[doc = "Field `PECID` reader - PCI Express Extended Capability ID \\[PECID\\]\n\nThis field is hardwired to the\n\nCapability ID assigned by PCI SIG to\n\nthe PCI Express Power Budgeting\n\nCapability (0004 hex)."]
pub type PecidR = crate::FieldReader<u16>;
#[doc = "Field `PCV` reader - Capability Version \\[PCV\\]\n\nSpecifies the SIG assigned value for\n\nthe version of the capability\n\nstructure. This field is set by default\n\nto 1, but can be modified from the\n\nlocal management bus by writing\n\ninto Function 0 from the local\n\nmanagement bus."]
pub type PcvR = crate::FieldReader;
#[doc = "Field `PBNCO` reader - Next Capability Offset \\[PBNCO\\]\n\nIndicates offset to the next PCI\n\nExpress capability structure. The\n\ndefault next pointer value is dynamic\n\nand is dependent on whether the\n\nstrap or LMI bits are set."]
pub type PbncoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PCI Express Extended Capability ID \\[PECID\\]\n\nThis field is hardwired to the\n\nCapability ID assigned by PCI SIG to\n\nthe PCI Express Power Budgeting\n\nCapability (0004 hex)."]
    #[inline(always)]
    pub fn pecid(&self) -> PecidR {
        PecidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Capability Version \\[PCV\\]\n\nSpecifies the SIG assigned value for\n\nthe version of the capability\n\nstructure. This field is set by default\n\nto 1, but can be modified from the\n\nlocal management bus by writing\n\ninto Function 0 from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn pcv(&self) -> PcvR {
        PcvR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Next Capability Offset \\[PBNCO\\]\n\nIndicates offset to the next PCI\n\nExpress capability structure. The\n\ndefault next pointer value is dynamic\n\nand is dependent on whether the\n\nstrap or LMI bits are set."]
    #[inline(always)]
    pub fn pbnco(&self) -> PbncoR {
        PbncoR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "Power Budgeting Enhanced Capability Header\n\nIndicates offset to the next PCI\n\nExpress capability structure. The\n\ndefault next pointer value is dynamic\n\nand is dependent on whether the\n\nstrap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_power_budgeting_enhanced_capability_header::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfPowerBudgetingEnhancedCapabilityHeaderSpec;
impl crate::RegisterSpec for PciePfPowerBudgetingEnhancedCapabilityHeaderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_power_budgeting_enhanced_capability_header::R`](R) reader structure"]
impl crate::Readable for PciePfPowerBudgetingEnhancedCapabilityHeaderSpec {}
#[doc = "`reset()` method sets PCIE_PF_POWER_BUDGETING_ENHANCED_CAPABILITY_HEADER to value 0x1b81_0004"]
impl crate::Resettable for PciePfPowerBudgetingEnhancedCapabilityHeaderSpec {
    const RESET_VALUE: u32 = 0x1b81_0004;
}
