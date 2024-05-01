#[doc = "Register `SWREG39_REFER14_POC` reader"]
pub type R = crate::R<Swreg39Refer14PocSpec>;
#[doc = "Register `SWREG39_REFER14_POC` writer"]
pub type W = crate::W<Swreg39Refer14PocSpec>;
#[doc = "Field `SW_REFER14_POC` reader - the poc of reference picture index 14\n\nthe poc of reference picture index 14"]
pub type SwRefer14PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER14_POC` writer - the poc of reference picture index 14\n\nthe poc of reference picture index 14"]
pub type SwRefer14PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 14\n\nthe poc of reference picture index 14"]
    #[inline(always)]
    pub fn sw_refer14_poc(&self) -> SwRefer14PocR {
        SwRefer14PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 14\n\nthe poc of reference picture index 14"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer14_poc(&mut self) -> SwRefer14PocW<Swreg39Refer14PocSpec> {
        SwRefer14PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg39_refer14_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg39_refer14_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg39Refer14PocSpec;
impl crate::RegisterSpec for Swreg39Refer14PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg39_refer14_poc::R`](R) reader structure"]
impl crate::Readable for Swreg39Refer14PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg39_refer14_poc::W`](W) writer structure"]
impl crate::Writable for Swreg39Refer14PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG39_REFER14_POC to value 0"]
impl crate::Resettable for Swreg39Refer14PocSpec {
    const RESET_VALUE: u32 = 0;
}
