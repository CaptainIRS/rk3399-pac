#[doc = "Register `AUTO_PERIOD_HT` reader"]
pub type R = crate::R<AutoPeriodHtSpec>;
#[doc = "Register `AUTO_PERIOD_HT` writer"]
pub type W = crate::W<AutoPeriodHtSpec>;
#[doc = "Field `AUTO_PERIOD` reader - This register controls the interleave between every two accessing\n\nof TSADC after the temperature is higher than COMP_SHUT or\n\nCOMP_INT"]
pub type AutoPeriodR = crate::FieldReader<u32>;
#[doc = "Field `AUTO_PERIOD` writer - This register controls the interleave between every two accessing\n\nof TSADC after the temperature is higher than COMP_SHUT or\n\nCOMP_INT"]
pub type AutoPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register controls the interleave between every two accessing\n\nof TSADC after the temperature is higher than COMP_SHUT or\n\nCOMP_INT"]
    #[inline(always)]
    pub fn auto_period(&self) -> AutoPeriodR {
        AutoPeriodR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register controls the interleave between every two accessing\n\nof TSADC after the temperature is higher than COMP_SHUT or\n\nCOMP_INT"]
    #[inline(always)]
    #[must_use]
    pub fn auto_period(&mut self) -> AutoPeriodW<AutoPeriodHtSpec> {
        AutoPeriodW::new(self, 0)
    }
}
#[doc = "TSADC auto access period when temperature is high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_period_ht::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_period_ht::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutoPeriodHtSpec;
impl crate::RegisterSpec for AutoPeriodHtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auto_period_ht::R`](R) reader structure"]
impl crate::Readable for AutoPeriodHtSpec {}
#[doc = "`write(|w| ..)` method takes [`auto_period_ht::W`](W) writer structure"]
impl crate::Writable for AutoPeriodHtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTO_PERIOD_HT to value 0x0001_0000"]
impl crate::Resettable for AutoPeriodHtSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
