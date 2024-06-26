#[doc = "Register `StatPeriod` reader"]
pub type R = crate::R<StatPeriodSpec>;
#[doc = "Register `StatPeriod` writer"]
pub type W = crate::W<StatPeriodSpec>;
#[doc = "Field `STATPERIOD` reader - Register StatPeriod is a 5-bit register that sets a period, within a\n\nrange of 2 cycles to 2 gigacycles, during which statistics are\n\ncollected before being dumped automatically. Setting the register\n\nimplicitly enables automatic mode operation for statistics\n\ncollection. The period is calculated with the formula: N_Cycle =\n\n2**StatPeriodWhen register StatPeriod is set to its default value 0,\n\nautomatic dump mode is disabled, and register StatGo is activated\n\nfor manual mode operation. Note: When parameter\n\nstatisticsCollection is set to False, StatPeriod is reserved."]
pub type StatperiodR = crate::FieldReader;
#[doc = "Field `STATPERIOD` writer - Register StatPeriod is a 5-bit register that sets a period, within a\n\nrange of 2 cycles to 2 gigacycles, during which statistics are\n\ncollected before being dumped automatically. Setting the register\n\nimplicitly enables automatic mode operation for statistics\n\ncollection. The period is calculated with the formula: N_Cycle =\n\n2**StatPeriodWhen register StatPeriod is set to its default value 0,\n\nautomatic dump mode is disabled, and register StatGo is activated\n\nfor manual mode operation. Note: When parameter\n\nstatisticsCollection is set to False, StatPeriod is reserved."]
pub type StatperiodW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Register StatPeriod is a 5-bit register that sets a period, within a\n\nrange of 2 cycles to 2 gigacycles, during which statistics are\n\ncollected before being dumped automatically. Setting the register\n\nimplicitly enables automatic mode operation for statistics\n\ncollection. The period is calculated with the formula: N_Cycle =\n\n2**StatPeriodWhen register StatPeriod is set to its default value 0,\n\nautomatic dump mode is disabled, and register StatGo is activated\n\nfor manual mode operation. Note: When parameter\n\nstatisticsCollection is set to False, StatPeriod is reserved."]
    #[inline(always)]
    pub fn statperiod(&self) -> StatperiodR {
        StatperiodR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Register StatPeriod is a 5-bit register that sets a period, within a\n\nrange of 2 cycles to 2 gigacycles, during which statistics are\n\ncollected before being dumped automatically. Setting the register\n\nimplicitly enables automatic mode operation for statistics\n\ncollection. The period is calculated with the formula: N_Cycle =\n\n2**StatPeriodWhen register StatPeriod is set to its default value 0,\n\nautomatic dump mode is disabled, and register StatGo is activated\n\nfor manual mode operation. Note: When parameter\n\nstatisticsCollection is set to False, StatPeriod is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn statperiod(&mut self) -> StatperiodW<StatPeriodSpec> {
        StatperiodW::new(self, 0)
    }
}
#[doc = "Statistics Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat_period::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat_period::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatPeriodSpec;
impl crate::RegisterSpec for StatPeriodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_period::R`](R) reader structure"]
impl crate::Readable for StatPeriodSpec {}
#[doc = "`write(|w| ..)` method takes [`stat_period::W`](W) writer structure"]
impl crate::Writable for StatPeriodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets StatPeriod to value 0"]
impl crate::Resettable for StatPeriodSpec {
    const RESET_VALUE: u32 = 0;
}
