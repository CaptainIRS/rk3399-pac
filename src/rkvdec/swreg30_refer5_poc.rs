#[doc = "Register `SWREG30_REFER5_POC` reader"]
pub type R = crate::R<Swreg30Refer5PocSpec>;
#[doc = "Register `SWREG30_REFER5_POC` writer"]
pub type W = crate::W<Swreg30Refer5PocSpec>;
#[doc = "Field `SW_REFER5_POC` reader - the poc of reference picture index 5\n\nthe poc of reference picture index 5"]
pub type SwRefer5PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER5_POC` writer - the poc of reference picture index 5\n\nthe poc of reference picture index 5"]
pub type SwRefer5PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 5\n\nthe poc of reference picture index 5"]
    #[inline(always)]
    pub fn sw_refer5_poc(&self) -> SwRefer5PocR {
        SwRefer5PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 5\n\nthe poc of reference picture index 5"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer5_poc(&mut self) -> SwRefer5PocW<Swreg30Refer5PocSpec> {
        SwRefer5PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg30_refer5_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg30_refer5_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg30Refer5PocSpec;
impl crate::RegisterSpec for Swreg30Refer5PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg30_refer5_poc::R`](R) reader structure"]
impl crate::Readable for Swreg30Refer5PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg30_refer5_poc::W`](W) writer structure"]
impl crate::Writable for Swreg30Refer5PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG30_REFER5_POC to value 0"]
impl crate::Resettable for Swreg30Refer5PocSpec {
    const RESET_VALUE: u32 = 0;
}
