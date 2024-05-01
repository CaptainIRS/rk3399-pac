#[doc = "Register `SWREG36_REFER11_POC` reader"]
pub type R = crate::R<Swreg36Refer11PocSpec>;
#[doc = "Register `SWREG36_REFER11_POC` writer"]
pub type W = crate::W<Swreg36Refer11PocSpec>;
#[doc = "Field `SW_REFER11_POC` reader - the poc of reference picture index 11\n\nthe poc of reference picture index 11"]
pub type SwRefer11PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER11_POC` writer - the poc of reference picture index 11\n\nthe poc of reference picture index 11"]
pub type SwRefer11PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 11\n\nthe poc of reference picture index 11"]
    #[inline(always)]
    pub fn sw_refer11_poc(&self) -> SwRefer11PocR {
        SwRefer11PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 11\n\nthe poc of reference picture index 11"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer11_poc(&mut self) -> SwRefer11PocW<Swreg36Refer11PocSpec> {
        SwRefer11PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg36_refer11_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg36_refer11_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg36Refer11PocSpec;
impl crate::RegisterSpec for Swreg36Refer11PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg36_refer11_poc::R`](R) reader structure"]
impl crate::Readable for Swreg36Refer11PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg36_refer11_poc::W`](W) writer structure"]
impl crate::Writable for Swreg36Refer11PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG36_REFER11_POC to value 0"]
impl crate::Resettable for Swreg36Refer11PocSpec {
    const RESET_VALUE: u32 = 0;
}
