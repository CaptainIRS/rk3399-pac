#[doc = "Register `SWREG52_VP9_REFCOLMV_BASE` reader"]
pub type R = crate::R<Swreg52Vp9RefcolmvBaseSpec>;
#[doc = "Register `SWREG52_VP9_REFCOLMV_BASE` writer"]
pub type W = crate::W<Swreg52Vp9RefcolmvBaseSpec>;
#[doc = "Field `SW_VP9_REFCOLMV_BASE` reader - vp9 ref colmv base\n\nvp9 ref colmv base addr"]
pub type SwVp9RefcolmvBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9_REFCOLMV_BASE` writer - vp9 ref colmv base\n\nvp9 ref colmv base addr"]
pub type SwVp9RefcolmvBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - vp9 ref colmv base\n\nvp9 ref colmv base addr"]
    #[inline(always)]
    pub fn sw_vp9_refcolmv_base(&self) -> SwVp9RefcolmvBaseR {
        SwVp9RefcolmvBaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - vp9 ref colmv base\n\nvp9 ref colmv base addr"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_refcolmv_base(&mut self) -> SwVp9RefcolmvBaseW<Swreg52Vp9RefcolmvBaseSpec> {
        SwVp9RefcolmvBaseW::new(self, 4)
    }
}
#[doc = "vp9 colmv ref base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg52_vp9_refcolmv_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg52_vp9_refcolmv_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg52Vp9RefcolmvBaseSpec;
impl crate::RegisterSpec for Swreg52Vp9RefcolmvBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg52_vp9_refcolmv_base::R`](R) reader structure"]
impl crate::Readable for Swreg52Vp9RefcolmvBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg52_vp9_refcolmv_base::W`](W) writer structure"]
impl crate::Writable for Swreg52Vp9RefcolmvBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG52_VP9_REFCOLMV_BASE to value 0"]
impl crate::Resettable for Swreg52Vp9RefcolmvBaseSpec {
    const RESET_VALUE: u32 = 0;
}
