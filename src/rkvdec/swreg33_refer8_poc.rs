#[doc = "Register `SWREG33_REFER8_POC` reader"]
pub type R = crate::R<Swreg33Refer8PocSpec>;
#[doc = "Register `SWREG33_REFER8_POC` writer"]
pub type W = crate::W<Swreg33Refer8PocSpec>;
#[doc = "Field `SW_REFER8_POC` reader - the poc of reference picture index 8\n\nthe poc of reference picture index 8"]
pub type SwRefer8PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER8_POC` writer - the poc of reference picture index 8\n\nthe poc of reference picture index 8"]
pub type SwRefer8PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 8\n\nthe poc of reference picture index 8"]
    #[inline(always)]
    pub fn sw_refer8_poc(&self) -> SwRefer8PocR {
        SwRefer8PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 8\n\nthe poc of reference picture index 8"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer8_poc(&mut self) -> SwRefer8PocW<Swreg33Refer8PocSpec> {
        SwRefer8PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg33_refer8_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg33_refer8_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg33Refer8PocSpec;
impl crate::RegisterSpec for Swreg33Refer8PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg33_refer8_poc::R`](R) reader structure"]
impl crate::Readable for Swreg33Refer8PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg33_refer8_poc::W`](W) writer structure"]
impl crate::Writable for Swreg33Refer8PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG33_REFER8_POC to value 0"]
impl crate::Resettable for Swreg33Refer8PocSpec {
    const RESET_VALUE: u32 = 0;
}
