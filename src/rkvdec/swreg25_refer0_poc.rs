#[doc = "Register `SWREG25_REFER0_POC` reader"]
pub type R = crate::R<Swreg25Refer0PocSpec>;
#[doc = "Register `SWREG25_REFER0_POC` writer"]
pub type W = crate::W<Swreg25Refer0PocSpec>;
#[doc = "Field `SW_REFER0_POC` reader - the poc of reference picture index 0\n\nthe poc of reference picture index 0"]
pub type SwRefer0PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER0_POC` writer - the poc of reference picture index 0\n\nthe poc of reference picture index 0"]
pub type SwRefer0PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 0\n\nthe poc of reference picture index 0"]
    #[inline(always)]
    pub fn sw_refer0_poc(&self) -> SwRefer0PocR {
        SwRefer0PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 0\n\nthe poc of reference picture index 0"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer0_poc(&mut self) -> SwRefer0PocW<Swreg25Refer0PocSpec> {
        SwRefer0PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg25_refer0_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg25_refer0_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg25Refer0PocSpec;
impl crate::RegisterSpec for Swreg25Refer0PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg25_refer0_poc::R`](R) reader structure"]
impl crate::Readable for Swreg25Refer0PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg25_refer0_poc::W`](W) writer structure"]
impl crate::Writable for Swreg25Refer0PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG25_REFER0_POC to value 0"]
impl crate::Resettable for Swreg25Refer0PocSpec {
    const RESET_VALUE: u32 = 0;
}
