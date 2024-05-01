#[doc = "Register `SWREG27_REFER2_POC` reader"]
pub type R = crate::R<Swreg27Refer2PocSpec>;
#[doc = "Register `SWREG27_REFER2_POC` writer"]
pub type W = crate::W<Swreg27Refer2PocSpec>;
#[doc = "Field `SW_REFER2_POC` reader - the poc of reference picture index 2\n\nthe poc of reference picture index 2"]
pub type SwRefer2PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER2_POC` writer - the poc of reference picture index 2\n\nthe poc of reference picture index 2"]
pub type SwRefer2PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 2\n\nthe poc of reference picture index 2"]
    #[inline(always)]
    pub fn sw_refer2_poc(&self) -> SwRefer2PocR {
        SwRefer2PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 2\n\nthe poc of reference picture index 2"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer2_poc(&mut self) -> SwRefer2PocW<Swreg27Refer2PocSpec> {
        SwRefer2PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg27_refer2_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg27_refer2_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg27Refer2PocSpec;
impl crate::RegisterSpec for Swreg27Refer2PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg27_refer2_poc::R`](R) reader structure"]
impl crate::Readable for Swreg27Refer2PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg27_refer2_poc::W`](W) writer structure"]
impl crate::Writable for Swreg27Refer2PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG27_REFER2_POC to value 0"]
impl crate::Resettable for Swreg27Refer2PocSpec {
    const RESET_VALUE: u32 = 0;
}
