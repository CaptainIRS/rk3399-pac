#[doc = "Register `SWREG37_REFER12_POC` reader"]
pub type R = crate::R<Swreg37Refer12PocSpec>;
#[doc = "Register `SWREG37_REFER12_POC` writer"]
pub type W = crate::W<Swreg37Refer12PocSpec>;
#[doc = "Field `SW_REFER12_POC` reader - the poc of reference picture index 12\n\nthe poc of reference picture index 12"]
pub type SwRefer12PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER12_POC` writer - the poc of reference picture index 12\n\nthe poc of reference picture index 12"]
pub type SwRefer12PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 12\n\nthe poc of reference picture index 12"]
    #[inline(always)]
    pub fn sw_refer12_poc(&self) -> SwRefer12PocR {
        SwRefer12PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 12\n\nthe poc of reference picture index 12"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer12_poc(&mut self) -> SwRefer12PocW<Swreg37Refer12PocSpec> {
        SwRefer12PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg37_refer12_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg37_refer12_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg37Refer12PocSpec;
impl crate::RegisterSpec for Swreg37Refer12PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg37_refer12_poc::R`](R) reader structure"]
impl crate::Readable for Swreg37Refer12PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg37_refer12_poc::W`](W) writer structure"]
impl crate::Writable for Swreg37Refer12PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG37_REFER12_POC to value 0"]
impl crate::Resettable for Swreg37Refer12PocSpec {
    const RESET_VALUE: u32 = 0;
}
