#[doc = "Register `FLASH_PREDIV` reader"]
pub type R = crate::R<FlashPredivSpec>;
#[doc = "Register `FLASH_PREDIV` writer"]
pub type W = crate::W<FlashPredivSpec>;
#[doc = "Field `fl_pre_div` reader - pre-divider for flush/preflash counter\n\n"]
pub type FlPreDivR = crate::FieldReader<u16>;
#[doc = "Field `fl_pre_div` writer - pre-divider for flush/preflash counter\n\n"]
pub type FlPreDivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - pre-divider for flush/preflash counter\n\n"]
    #[inline(always)]
    pub fn fl_pre_div(&self) -> FlPreDivR {
        FlPreDivR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - pre-divider for flush/preflash counter\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn fl_pre_div(&mut self) -> FlPreDivW<FlashPredivSpec> {
        FlPreDivW::new(self, 0)
    }
}
#[doc = "Flash Counter Pre-Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_prediv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_prediv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashPredivSpec;
impl crate::RegisterSpec for FlashPredivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_prediv::R`](R) reader structure"]
impl crate::Readable for FlashPredivSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_prediv::W`](W) writer structure"]
impl crate::Writable for FlashPredivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_PREDIV to value 0"]
impl crate::Resettable for FlashPredivSpec {
    const RESET_VALUE: u32 = 0;
}
