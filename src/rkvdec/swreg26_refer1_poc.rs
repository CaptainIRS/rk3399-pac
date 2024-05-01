#[doc = "Register `SWREG26_REFER1_POC` reader"]
pub type R = crate::R<Swreg26Refer1PocSpec>;
#[doc = "Register `SWREG26_REFER1_POC` writer"]
pub type W = crate::W<Swreg26Refer1PocSpec>;
#[doc = "Field `SW_REFER1_POC` reader - the poc of reference picture index 1\n\nthe poc of reference picture index 1"]
pub type SwRefer1PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER1_POC` writer - the poc of reference picture index 1\n\nthe poc of reference picture index 1"]
pub type SwRefer1PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 1\n\nthe poc of reference picture index 1"]
    #[inline(always)]
    pub fn sw_refer1_poc(&self) -> SwRefer1PocR {
        SwRefer1PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 1\n\nthe poc of reference picture index 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer1_poc(&mut self) -> SwRefer1PocW<Swreg26Refer1PocSpec> {
        SwRefer1PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg26_refer1_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg26_refer1_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg26Refer1PocSpec;
impl crate::RegisterSpec for Swreg26Refer1PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg26_refer1_poc::R`](R) reader structure"]
impl crate::Readable for Swreg26Refer1PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg26_refer1_poc::W`](W) writer structure"]
impl crate::Writable for Swreg26Refer1PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG26_REFER1_POC to value 0"]
impl crate::Resettable for Swreg26Refer1PocSpec {
    const RESET_VALUE: u32 = 0;
}
