#[doc = "Register `TSADC_AUTO_PERIOD` reader"]
pub type R = crate::R<TsadcAutoPeriodSpec>;
#[doc = "Register `TSADC_AUTO_PERIOD` writer"]
pub type W = crate::W<TsadcAutoPeriodSpec>;
#[doc = "Field `AUTO_PERIOD` reader - when auto mode is enabled, this register controls the interleave\n\nbetween every two accessing of TSADC."]
pub type AutoPeriodR = crate::FieldReader<u32>;
#[doc = "Field `AUTO_PERIOD` writer - when auto mode is enabled, this register controls the interleave\n\nbetween every two accessing of TSADC."]
pub type AutoPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - when auto mode is enabled, this register controls the interleave\n\nbetween every two accessing of TSADC."]
    #[inline(always)]
    pub fn auto_period(&self) -> AutoPeriodR {
        AutoPeriodR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - when auto mode is enabled, this register controls the interleave\n\nbetween every two accessing of TSADC."]
    #[inline(always)]
    #[must_use]
    pub fn auto_period(&mut self) -> AutoPeriodW<TsadcAutoPeriodSpec> {
        AutoPeriodW::new(self, 0)
    }
}
#[doc = "TSADC auto access period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_auto_period::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_auto_period::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsadcAutoPeriodSpec;
impl crate::RegisterSpec for TsadcAutoPeriodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsadc_auto_period::R`](R) reader structure"]
impl crate::Readable for TsadcAutoPeriodSpec {}
#[doc = "`write(|w| ..)` method takes [`tsadc_auto_period::W`](W) writer structure"]
impl crate::Writable for TsadcAutoPeriodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSADC_AUTO_PERIOD to value 0x0001_0000"]
impl crate::Resettable for TsadcAutoPeriodSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
