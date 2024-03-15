#[doc = "Register `CCI500_PFMMON_CTRL` reader"]
pub type R = crate::R<Cci500PfmmonCtrlSpec>;
#[doc = "Register `CCI500_PFMMON_CTRL` writer"]
pub type W = crate::W<Cci500PfmmonCtrlSpec>;
#[doc = "Field `CEN` reader - Enable bit: 0b0 Disable all counters, including CCNT. 0b1 Enable all counters, including CCNT."]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - Enable bit: 0b0 Disable all counters, including CCNT. 0b1 Enable all counters, including CCNT."]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Performance counter reset: 0b0 No action. 0b1 Reset all performance counters to zero, not including CCNT."]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Performance counter reset: 0b0 No action. 0b1 Reset all performance counters to zero, not including CCNT."]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EX` reader - Enable export of the events to the event bus, EVNTBUS, for an external monitoring block to trace events: 0b0 Do not export EVNTBUS. 0b1 Export EVNTBUS."]
pub type ExR = crate::BitReader;
#[doc = "Field `EX` writer - Enable export of the events to the event bus, EVNTBUS, for an external monitoring block to trace events: 0b0 Do not export EVNTBUS. 0b1 Export EVNTBUS."]
pub type ExW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP` reader - Disables cycle counter, CCNT, if non-invasive debug is prohibited: 0b0 Count is not disabled when NIDEN input is LOW. 0b1 Count is disabled when NIDEN input is LOW."]
pub type DpR = crate::BitReader;
#[doc = "Field `DP` writer - Disables cycle counter, CCNT, if non-invasive debug is prohibited: 0b0 Count is not disabled when NIDEN input is LOW. 0b1 Count is disabled when NIDEN input is LOW."]
pub type DpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTER_NUM` reader - Specifies the number of counters implemented."]
pub type CounterNumR = crate::FieldReader;
#[doc = "Field `COUNTER_NUM` writer - Specifies the number of counters implemented."]
pub type CounterNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Enable bit: 0b0 Disable all counters, including CCNT. 0b1 Enable all counters, including CCNT."]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Performance counter reset: 0b0 No action. 0b1 Reset all performance counters to zero, not including CCNT."]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable export of the events to the event bus, EVNTBUS, for an external monitoring block to trace events: 0b0 Do not export EVNTBUS. 0b1 Export EVNTBUS."]
    #[inline(always)]
    pub fn ex(&self) -> ExR {
        ExR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disables cycle counter, CCNT, if non-invasive debug is prohibited: 0b0 Count is not disabled when NIDEN input is LOW. 0b1 Count is disabled when NIDEN input is LOW."]
    #[inline(always)]
    pub fn dp(&self) -> DpR {
        DpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 11:15 - Specifies the number of counters implemented."]
    #[inline(always)]
    pub fn counter_num(&self) -> CounterNumR {
        CounterNumR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit: 0b0 Disable all counters, including CCNT. 0b1 Enable all counters, including CCNT."]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CenW<Cci500PfmmonCtrlSpec> {
        CenW::new(self, 0)
    }
    #[doc = "Bit 1 - Performance counter reset: 0b0 No action. 0b1 Reset all performance counters to zero, not including CCNT."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<Cci500PfmmonCtrlSpec> {
        RstW::new(self, 1)
    }
    #[doc = "Bit 4 - Enable export of the events to the event bus, EVNTBUS, for an external monitoring block to trace events: 0b0 Do not export EVNTBUS. 0b1 Export EVNTBUS."]
    #[inline(always)]
    #[must_use]
    pub fn ex(&mut self) -> ExW<Cci500PfmmonCtrlSpec> {
        ExW::new(self, 4)
    }
    #[doc = "Bit 5 - Disables cycle counter, CCNT, if non-invasive debug is prohibited: 0b0 Count is not disabled when NIDEN input is LOW. 0b1 Count is disabled when NIDEN input is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn dp(&mut self) -> DpW<Cci500PfmmonCtrlSpec> {
        DpW::new(self, 5)
    }
    #[doc = "Bits 11:15 - Specifies the number of counters implemented."]
    #[inline(always)]
    #[must_use]
    pub fn counter_num(&mut self) -> CounterNumW<Cci500PfmmonCtrlSpec> {
        CounterNumW::new(self, 11)
    }
}
#[doc = "Performance Monitor Control Register (PMCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_pfmmon_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_pfmmon_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cci500PfmmonCtrlSpec;
impl crate::RegisterSpec for Cci500PfmmonCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci500_pfmmon_ctrl::R`](R) reader structure"]
impl crate::Readable for Cci500PfmmonCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cci500_pfmmon_ctrl::W`](W) writer structure"]
impl crate::Writable for Cci500PfmmonCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCI500_PFMMON_CTRL to value 0"]
impl crate::Resettable for Cci500PfmmonCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
