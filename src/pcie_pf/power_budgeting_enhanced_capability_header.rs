#[doc = "Register `POWER_BUDGETING_ENHANCED_CAPABILITY_HEADER` reader"]
pub type R = crate::R<PowerBudgetingEnhancedCapabilityHeaderSpec>;
#[doc = "Field `PECID` reader - PCI Express Extended Capability ID \\[PECID\\]
This field is hardwired to the Capability ID assigned by PCI SIG to the PCI Express Power Budgeting Capability (0004 hex)."]
pub type PecidR = crate::FieldReader<u16>;
#[doc = "Field `PCV` reader - Capability Version \\[PCV\\]
Specifies the SIG assigned value for the version of the capability structure. This field is set by default to 1, but can be modified from the local management bus by writing into Function 0 from the local management bus."]
pub type PcvR = crate::FieldReader;
#[doc = "Field `PBNCO` reader - Next Capability Offset \\[PBNCO\\]
Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub type PbncoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PCI Express Extended Capability ID \\[PECID\\]
This field is hardwired to the Capability ID assigned by PCI SIG to the PCI Express Power Budgeting Capability (0004 hex)."]
    #[inline(always)]
    pub fn pecid(&self) -> PecidR {
        PecidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Capability Version \\[PCV\\]
Specifies the SIG assigned value for the version of the capability structure. This field is set by default to 1, but can be modified from the local management bus by writing into Function 0 from the local management bus."]
    #[inline(always)]
    pub fn pcv(&self) -> PcvR {
        PcvR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Next Capability Offset \\[PBNCO\\]
Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub fn pbnco(&self) -> PbncoR {
        PbncoR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "Power Budgeting Enhanced Capability Header Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_budgeting_enhanced_capability_header::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerBudgetingEnhancedCapabilityHeaderSpec;
impl crate::RegisterSpec for PowerBudgetingEnhancedCapabilityHeaderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_budgeting_enhanced_capability_header::R`](R) reader structure"]
impl crate::Readable for PowerBudgetingEnhancedCapabilityHeaderSpec {}
#[doc = "`reset()` method sets POWER_BUDGETING_ENHANCED_CAPABILITY_HEADER to value 0x1b81_0004"]
impl crate::Resettable for PowerBudgetingEnhancedCapabilityHeaderSpec {
    const RESET_VALUE: u32 = 0x1b81_0004;
}
