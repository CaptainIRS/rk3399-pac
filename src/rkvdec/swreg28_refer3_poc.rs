#[doc = "Register `SWREG28_REFER3_POC` reader"]
pub type R = crate::R<Swreg28Refer3PocSpec>;
#[doc = "Register `SWREG28_REFER3_POC` writer"]
pub type W = crate::W<Swreg28Refer3PocSpec>;
#[doc = "Field `SW_REFER3_POC` reader - the poc of reference picture index 3\n\nthe poc of reference picture index 3"]
pub type SwRefer3PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER3_POC` writer - the poc of reference picture index 3\n\nthe poc of reference picture index 3"]
pub type SwRefer3PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 3\n\nthe poc of reference picture index 3"]
    #[inline(always)]
    pub fn sw_refer3_poc(&self) -> SwRefer3PocR {
        SwRefer3PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 3\n\nthe poc of reference picture index 3"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer3_poc(&mut self) -> SwRefer3PocW<Swreg28Refer3PocSpec> {
        SwRefer3PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg28_refer3_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg28_refer3_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg28Refer3PocSpec;
impl crate::RegisterSpec for Swreg28Refer3PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg28_refer3_poc::R`](R) reader structure"]
impl crate::Readable for Swreg28Refer3PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg28_refer3_poc::W`](W) writer structure"]
impl crate::Writable for Swreg28Refer3PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG28_REFER3_POC to value 0"]
impl crate::Resettable for Swreg28Refer3PocSpec {
    const RESET_VALUE: u32 = 0;
}
