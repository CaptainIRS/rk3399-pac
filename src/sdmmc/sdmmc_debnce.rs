#[doc = "Register `SDMMC_DEBNCE` reader"]
pub type R = crate::R<SdmmcDebnceSpec>;
#[doc = "Register `SDMMC_DEBNCE` writer"]
pub type W = crate::W<SdmmcDebnceSpec>;
#[doc = "Field `DEBOUNCE_COUNT` reader - Number of host clocks (clk) used by debounce filter logic; typical debounce time is 5-25 ms."]
pub type DebounceCountR = crate::FieldReader<u32>;
#[doc = "Field `DEBOUNCE_COUNT` writer - Number of host clocks (clk) used by debounce filter logic; typical debounce time is 5-25 ms."]
pub type DebounceCountW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Number of host clocks (clk) used by debounce filter logic; typical debounce time is 5-25 ms."]
    #[inline(always)]
    pub fn debounce_count(&self) -> DebounceCountR {
        DebounceCountR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Number of host clocks (clk) used by debounce filter logic; typical debounce time is 5-25 ms."]
    #[inline(always)]
    #[must_use]
    pub fn debounce_count(&mut self) -> DebounceCountW<SdmmcDebnceSpec> {
        DebounceCountW::new(self, 0)
    }
}
#[doc = "Card detect debounce register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_debnce::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_debnce::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcDebnceSpec;
impl crate::RegisterSpec for SdmmcDebnceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_debnce::R`](R) reader structure"]
impl crate::Readable for SdmmcDebnceSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_debnce::W`](W) writer structure"]
impl crate::Writable for SdmmcDebnceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_DEBNCE to value 0x00ff_ffff"]
impl crate::Resettable for SdmmcDebnceSpec {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
