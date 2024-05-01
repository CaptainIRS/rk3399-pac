#[doc = "Register `SWREG34_REFER9_POC` reader"]
pub type R = crate::R<Swreg34Refer9PocSpec>;
#[doc = "Register `SWREG34_REFER9_POC` writer"]
pub type W = crate::W<Swreg34Refer9PocSpec>;
#[doc = "Field `SW_REFER9_POC` reader - the poc of reference picture index 9\n\nthe poc of reference picture index 9"]
pub type SwRefer9PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER9_POC` writer - the poc of reference picture index 9\n\nthe poc of reference picture index 9"]
pub type SwRefer9PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 9\n\nthe poc of reference picture index 9"]
    #[inline(always)]
    pub fn sw_refer9_poc(&self) -> SwRefer9PocR {
        SwRefer9PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 9\n\nthe poc of reference picture index 9"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer9_poc(&mut self) -> SwRefer9PocW<Swreg34Refer9PocSpec> {
        SwRefer9PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg34_refer9_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg34_refer9_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg34Refer9PocSpec;
impl crate::RegisterSpec for Swreg34Refer9PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg34_refer9_poc::R`](R) reader structure"]
impl crate::Readable for Swreg34Refer9PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg34_refer9_poc::W`](W) writer structure"]
impl crate::Writable for Swreg34Refer9PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG34_REFER9_POC to value 0"]
impl crate::Resettable for Swreg34Refer9PocSpec {
    const RESET_VALUE: u32 = 0;
}
