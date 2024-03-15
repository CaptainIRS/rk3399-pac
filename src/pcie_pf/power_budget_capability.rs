#[doc = "Register `POWER_BUDGET_CAPABILITY` reader"]
pub type R = crate::R<PowerBudgetCapabilitySpec>;
#[doc = "Field `SA` reader - System Allocated \\[SA\\]
This bit is set to indicate that the device power specified by this Power Management Capability Structure is included in the system power budget. When this bit set, the software must exclude the device power reported by this Capability Structure from power calculations, when making power budgeting decisions. This bit is set to 0 by default, but its setting can be modified individually for each PF from the local management bus."]
pub type SaR = crate::BitReader;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
Reserved"]
pub type R4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - System Allocated \\[SA\\]
This bit is set to indicate that the device power specified by this Power Management Capability Structure is included in the system power budget. When this bit set, the software must exclude the device power reported by this Capability Structure from power calculations, when making power budgeting decisions. This bit is set to 0 by default, but its setting can be modified individually for each PF from the local management bus."]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved \\[R4\\]
Reserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[doc = "Power Budget Capability Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_budget_capability::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerBudgetCapabilitySpec;
impl crate::RegisterSpec for PowerBudgetCapabilitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_budget_capability::R`](R) reader structure"]
impl crate::Readable for PowerBudgetCapabilitySpec {}
#[doc = "`reset()` method sets POWER_BUDGET_CAPABILITY to value 0"]
impl crate::Resettable for PowerBudgetCapabilitySpec {
    const RESET_VALUE: u32 = 0;
}
