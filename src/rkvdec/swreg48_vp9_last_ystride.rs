#[doc = "Register `SWREG48_VP9_LAST_YSTRIDE` reader"]
pub type R = crate::R<Swreg48Vp9LastYstrideSpec>;
#[doc = "Register `SWREG48_VP9_LAST_YSTRIDE` writer"]
pub type W = crate::W<Swreg48Vp9LastYstrideSpec>;
#[doc = "Field `SW_VP9_LASTFY_VIRSTRIDE` reader - last frame y virstride\n\nvp9 last frame y stride"]
pub type SwVp9LastfyVirstrideR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9_LASTFY_VIRSTRIDE` writer - last frame y virstride\n\nvp9 last frame y stride"]
pub type SwVp9LastfyVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - last frame y virstride\n\nvp9 last frame y stride"]
    #[inline(always)]
    pub fn sw_vp9_lastfy_virstride(&self) -> SwVp9LastfyVirstrideR {
        SwVp9LastfyVirstrideR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - last frame y virstride\n\nvp9 last frame y stride"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_lastfy_virstride(&mut self) -> SwVp9LastfyVirstrideW<Swreg48Vp9LastYstrideSpec> {
        SwVp9LastfyVirstrideW::new(self, 0)
    }
}
#[doc = "last ref ystride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg48_vp9_last_ystride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg48_vp9_last_ystride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg48Vp9LastYstrideSpec;
impl crate::RegisterSpec for Swreg48Vp9LastYstrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg48_vp9_last_ystride::R`](R) reader structure"]
impl crate::Readable for Swreg48Vp9LastYstrideSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg48_vp9_last_ystride::W`](W) writer structure"]
impl crate::Writable for Swreg48Vp9LastYstrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG48_VP9_LAST_YSTRIDE to value 0"]
impl crate::Resettable for Swreg48Vp9LastYstrideSpec {
    const RESET_VALUE: u32 = 0;
}
