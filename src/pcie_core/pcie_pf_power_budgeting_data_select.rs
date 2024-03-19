#[doc = "Register `PCIE_PF_POWER_BUDGETING_DATA_SELECT` reader"]
pub type R = crate::R<PciePfPowerBudgetingDataSelectSpec>;
#[doc = "Register `PCIE_PF_POWER_BUDGETING_DATA_SELECT` writer"]
pub type W = crate::W<PciePfPowerBudgetingDataSelectSpec>;
#[doc = "Field `PBDN` reader - Power Budgeting Data Index \\[PBDN\\]
This field selects the power budgeting data read from the Power Budgeting Data Register. Its settings are: 00: Selects power budgeting data for power state D0 MAX for the associated PF. 01: Selects power budgeting data for power state D0 SUSTAINED for the associated PF. 10: Selects power budgeting data for power state D3hot for the associated PF. 11: Selects power budgeting data for power state D1 for the associated PF. Others: Not a valid setting. A read from the Power Budgeting Data Register returns all zeroes."]
pub type PbdnR = crate::FieldReader;
#[doc = "Field `PBDN` writer - Power Budgeting Data Index \\[PBDN\\]
This field selects the power budgeting data read from the Power Budgeting Data Register. Its settings are: 00: Selects power budgeting data for power state D0 MAX for the associated PF. 01: Selects power budgeting data for power state D0 SUSTAINED for the associated PF. 10: Selects power budgeting data for power state D3hot for the associated PF. 11: Selects power budgeting data for power state D1 for the associated PF. Others: Not a valid setting. A read from the Power Budgeting Data Register returns all zeroes."]
pub type PbdnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
(no description)"]
pub type R0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Power Budgeting Data Index \\[PBDN\\]
This field selects the power budgeting data read from the Power Budgeting Data Register. Its settings are: 00: Selects power budgeting data for power state D0 MAX for the associated PF. 01: Selects power budgeting data for power state D0 SUSTAINED for the associated PF. 10: Selects power budgeting data for power state D3hot for the associated PF. 11: Selects power budgeting data for power state D1 for the associated PF. Others: Not a valid setting. A read from the Power Budgeting Data Register returns all zeroes."]
    #[inline(always)]
    pub fn pbdn(&self) -> PbdnR {
        PbdnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved \\[R0\\]
(no description)"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - Power Budgeting Data Index \\[PBDN\\]
This field selects the power budgeting data read from the Power Budgeting Data Register. Its settings are: 00: Selects power budgeting data for power state D0 MAX for the associated PF. 01: Selects power budgeting data for power state D0 SUSTAINED for the associated PF. 10: Selects power budgeting data for power state D3hot for the associated PF. 11: Selects power budgeting data for power state D1 for the associated PF. Others: Not a valid setting. A read from the Power Budgeting Data Register returns all zeroes."]
    #[inline(always)]
    #[must_use]
    pub fn pbdn(&mut self) -> PbdnW<PciePfPowerBudgetingDataSelectSpec> {
        PbdnW::new(self, 0)
    }
}
#[doc = "Power Budgeting Data Select Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_power_budgeting_data_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_power_budgeting_data_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfPowerBudgetingDataSelectSpec;
impl crate::RegisterSpec for PciePfPowerBudgetingDataSelectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_power_budgeting_data_select::R`](R) reader structure"]
impl crate::Readable for PciePfPowerBudgetingDataSelectSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_power_budgeting_data_select::W`](W) writer structure"]
impl crate::Writable for PciePfPowerBudgetingDataSelectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_POWER_BUDGETING_DATA_SELECT to value 0"]
impl crate::Resettable for PciePfPowerBudgetingDataSelectSpec {
    const RESET_VALUE: u32 = 0;
}
