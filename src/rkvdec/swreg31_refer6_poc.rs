#[doc = "Register `SWREG31_REFER6_POC` reader"]
pub type R = crate::R<Swreg31Refer6PocSpec>;
#[doc = "Register `SWREG31_REFER6_POC` writer"]
pub type W = crate::W<Swreg31Refer6PocSpec>;
#[doc = "Field `SW_REFER6_POC` reader - the poc of reference picture index 6\n\nthe poc of reference picture index 6"]
pub type SwRefer6PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER6_POC` writer - the poc of reference picture index 6\n\nthe poc of reference picture index 6"]
pub type SwRefer6PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 6\n\nthe poc of reference picture index 6"]
    #[inline(always)]
    pub fn sw_refer6_poc(&self) -> SwRefer6PocR {
        SwRefer6PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 6\n\nthe poc of reference picture index 6"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer6_poc(&mut self) -> SwRefer6PocW<Swreg31Refer6PocSpec> {
        SwRefer6PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg31_refer6_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg31_refer6_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg31Refer6PocSpec;
impl crate::RegisterSpec for Swreg31Refer6PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg31_refer6_poc::R`](R) reader structure"]
impl crate::Readable for Swreg31Refer6PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg31_refer6_poc::W`](W) writer structure"]
impl crate::Writable for Swreg31Refer6PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG31_REFER6_POC to value 0"]
impl crate::Resettable for Swreg31Refer6PocSpec {
    const RESET_VALUE: u32 = 0;
}
