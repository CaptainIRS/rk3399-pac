#[doc = "Register `SWREG29_REFER4_POC` reader"]
pub type R = crate::R<Swreg29Refer4PocSpec>;
#[doc = "Register `SWREG29_REFER4_POC` writer"]
pub type W = crate::W<Swreg29Refer4PocSpec>;
#[doc = "Field `SW_REFER4_POC` reader - the poc of reference picture index 4\n\nthe poc of reference picture index 4"]
pub type SwRefer4PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER4_POC` writer - the poc of reference picture index 4\n\nthe poc of reference picture index 4"]
pub type SwRefer4PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 4\n\nthe poc of reference picture index 4"]
    #[inline(always)]
    pub fn sw_refer4_poc(&self) -> SwRefer4PocR {
        SwRefer4PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 4\n\nthe poc of reference picture index 4"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer4_poc(&mut self) -> SwRefer4PocW<Swreg29Refer4PocSpec> {
        SwRefer4PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg29_refer4_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg29_refer4_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg29Refer4PocSpec;
impl crate::RegisterSpec for Swreg29Refer4PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg29_refer4_poc::R`](R) reader structure"]
impl crate::Readable for Swreg29Refer4PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg29_refer4_poc::W`](W) writer structure"]
impl crate::Writable for Swreg29Refer4PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG29_REFER4_POC to value 0"]
impl crate::Resettable for Swreg29Refer4PocSpec {
    const RESET_VALUE: u32 = 0;
}
