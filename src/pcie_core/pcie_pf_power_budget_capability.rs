#[doc = "Register `PCIE_PF_POWER_BUDGET_CAPABILITY` reader"]
pub type R = crate::R<PciePfPowerBudgetCapabilitySpec>;
#[doc = "Field `SA` reader - System Allocated \\[SA\\]\n\nThis bit is set to indicate that the\n\ndevice power specified by this Power\n\nManagement Capability Structure is\n\nincluded in the system power\n\nbudget. When this bit set, the\n\nsoftware must exclude the device\n\npower reported by this Capability\n\nStructure from power calculations,\n\nwhen making power budgeting\n\ndecisions. This bit is set to 0 by\n\ndefault, but its setting can be\n\nmodified individually for each PF\n\nfrom the local management bus."]
pub type SaR = crate::BitReader;
#[doc = "Field `R4` reader - Reserved \\[R4\\]\n\nReserved"]
pub type R4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - System Allocated \\[SA\\]\n\nThis bit is set to indicate that the\n\ndevice power specified by this Power\n\nManagement Capability Structure is\n\nincluded in the system power\n\nbudget. When this bit set, the\n\nsoftware must exclude the device\n\npower reported by this Capability\n\nStructure from power calculations,\n\nwhen making power budgeting\n\ndecisions. This bit is set to 0 by\n\ndefault, but its setting can be\n\nmodified individually for each PF\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved \\[R4\\]\n\nReserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[doc = "Power Budget Capability Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_power_budget_capability::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfPowerBudgetCapabilitySpec;
impl crate::RegisterSpec for PciePfPowerBudgetCapabilitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_power_budget_capability::R`](R) reader structure"]
impl crate::Readable for PciePfPowerBudgetCapabilitySpec {}
#[doc = "`reset()` method sets PCIE_PF_POWER_BUDGET_CAPABILITY to value 0"]
impl crate::Resettable for PciePfPowerBudgetCapabilitySpec {
    const RESET_VALUE: u32 = 0;
}
