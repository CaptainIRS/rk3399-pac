#[doc = "Register `SWREG32_REFER7_POC` reader"]
pub type R = crate::R<Swreg32Refer7PocSpec>;
#[doc = "Register `SWREG32_REFER7_POC` writer"]
pub type W = crate::W<Swreg32Refer7PocSpec>;
#[doc = "Field `SW_REFER7_POC` reader - the poc of reference picture index 7\n\nthe poc of reference picture index 7"]
pub type SwRefer7PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER7_POC` writer - the poc of reference picture index 7\n\nthe poc of reference picture index 7"]
pub type SwRefer7PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 7\n\nthe poc of reference picture index 7"]
    #[inline(always)]
    pub fn sw_refer7_poc(&self) -> SwRefer7PocR {
        SwRefer7PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 7\n\nthe poc of reference picture index 7"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer7_poc(&mut self) -> SwRefer7PocW<Swreg32Refer7PocSpec> {
        SwRefer7PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg32_refer7_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg32_refer7_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg32Refer7PocSpec;
impl crate::RegisterSpec for Swreg32Refer7PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg32_refer7_poc::R`](R) reader structure"]
impl crate::Readable for Swreg32Refer7PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg32_refer7_poc::W`](W) writer structure"]
impl crate::Writable for Swreg32Refer7PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG32_REFER7_POC to value 0"]
impl crate::Resettable for Swreg32Refer7PocSpec {
    const RESET_VALUE: u32 = 0;
}
