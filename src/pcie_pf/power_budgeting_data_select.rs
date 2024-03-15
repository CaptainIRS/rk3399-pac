#[doc = "Register `POWER_BUDGETING_DATA_SELECT` reader"]
pub type R = crate::R<PowerBudgetingDataSelectSpec>;
#[doc = "Register `POWER_BUDGETING_DATA_SELECT` writer"]
pub type W = crate::W<PowerBudgetingDataSelectSpec>;
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
    pub fn pbdn(&mut self) -> PbdnW<PowerBudgetingDataSelectSpec> {
        PbdnW::new(self, 0)
    }
}
#[doc = "Power Budgeting Data Select Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_budgeting_data_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_budgeting_data_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerBudgetingDataSelectSpec;
impl crate::RegisterSpec for PowerBudgetingDataSelectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_budgeting_data_select::R`](R) reader structure"]
impl crate::Readable for PowerBudgetingDataSelectSpec {}
#[doc = "`write(|w| ..)` method takes [`power_budgeting_data_select::W`](W) writer structure"]
impl crate::Writable for PowerBudgetingDataSelectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_BUDGETING_DATA_SELECT to value 0"]
impl crate::Resettable for PowerBudgetingDataSelectSpec {
    const RESET_VALUE: u32 = 0;
}
