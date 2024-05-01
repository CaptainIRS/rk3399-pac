#[doc = "Register `SWREG38_REFER13_POC` reader"]
pub type R = crate::R<Swreg38Refer13PocSpec>;
#[doc = "Register `SWREG38_REFER13_POC` writer"]
pub type W = crate::W<Swreg38Refer13PocSpec>;
#[doc = "Field `SW_REFER13_POC` reader - the poc of reference picture index 13\n\nthe poc of reference picture index 13"]
pub type SwRefer13PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER13_POC` writer - the poc of reference picture index 13\n\nthe poc of reference picture index 13"]
pub type SwRefer13PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 13\n\nthe poc of reference picture index 13"]
    #[inline(always)]
    pub fn sw_refer13_poc(&self) -> SwRefer13PocR {
        SwRefer13PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 13\n\nthe poc of reference picture index 13"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer13_poc(&mut self) -> SwRefer13PocW<Swreg38Refer13PocSpec> {
        SwRefer13PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg38_refer13_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg38_refer13_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg38Refer13PocSpec;
impl crate::RegisterSpec for Swreg38Refer13PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg38_refer13_poc::R`](R) reader structure"]
impl crate::Readable for Swreg38Refer13PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg38_refer13_poc::W`](W) writer structure"]
impl crate::Writable for Swreg38Refer13PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG38_REFER13_POC to value 0"]
impl crate::Resettable for Swreg38Refer13PocSpec {
    const RESET_VALUE: u32 = 0;
}
