#[doc = "Register `SWREG14_VP9COUNT_BASE` reader"]
pub type R = crate::R<Swreg14Vp9countBaseSpec>;
#[doc = "Register `SWREG14_VP9COUNT_BASE` writer"]
pub type W = crate::W<Swreg14Vp9countBaseSpec>;
#[doc = "Field `SW_VP9COUNT_BASE` reader - vp9 count base addr\n\nvp9 count base addr"]
pub type SwVp9countBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9COUNT_BASE` writer - vp9 count base addr\n\nvp9 count base addr"]
pub type SwVp9countBaseW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - vp9 count base addr\n\nvp9 count base addr"]
    #[inline(always)]
    pub fn sw_vp9count_base(&self) -> SwVp9countBaseR {
        SwVp9countBaseR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - vp9 count base addr\n\nvp9 count base addr"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9count_base(&mut self) -> SwVp9countBaseW<Swreg14Vp9countBaseSpec> {
        SwVp9countBaseW::new(self, 3)
    }
}
#[doc = "vp9 count base addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg14_vp9count_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg14_vp9count_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg14Vp9countBaseSpec;
impl crate::RegisterSpec for Swreg14Vp9countBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg14_vp9count_base::R`](R) reader structure"]
impl crate::Readable for Swreg14Vp9countBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg14_vp9count_base::W`](W) writer structure"]
impl crate::Writable for Swreg14Vp9countBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG14_VP9COUNT_BASE to value 0"]
impl crate::Resettable for Swreg14Vp9countBaseSpec {
    const RESET_VALUE: u32 = 0;
}
