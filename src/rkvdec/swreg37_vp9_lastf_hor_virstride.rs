#[doc = "Register `SWREG37_VP9_LASTF_HOR_VIRSTRIDE` reader"]
pub type R = crate::R<Swreg37Vp9LastfHorVirstrideSpec>;
#[doc = "Register `SWREG37_VP9_LASTF_HOR_VIRSTRIDE` writer"]
pub type W = crate::W<Swreg37Vp9LastfHorVirstrideSpec>;
#[doc = "Field `SW_VP9_LASTFY_HOR_VIRSTRIDE` reader - vp9 last frame y horizontal virstride\n\nvp9 last frame y horizontal virstride"]
pub type SwVp9LastfyHorVirstrideR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_LASTFY_HOR_VIRSTRIDE` writer - vp9 last frame y horizontal virstride\n\nvp9 last frame y horizontal virstride"]
pub type SwVp9LastfyHorVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_VP9_LASTFUV_HOR_VIRSTRIDE` reader - sw_vp9_lastfuv_hor_virstride\n\nvp9 last frame uv horizontal virstride"]
pub type SwVp9LastfuvHorVirstrideR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_LASTFUV_HOR_VIRSTRIDE` writer - sw_vp9_lastfuv_hor_virstride\n\nvp9 last frame uv horizontal virstride"]
pub type SwVp9LastfuvHorVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - vp9 last frame y horizontal virstride\n\nvp9 last frame y horizontal virstride"]
    #[inline(always)]
    pub fn sw_vp9_lastfy_hor_virstride(&self) -> SwVp9LastfyHorVirstrideR {
        SwVp9LastfyHorVirstrideR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - sw_vp9_lastfuv_hor_virstride\n\nvp9 last frame uv horizontal virstride"]
    #[inline(always)]
    pub fn sw_vp9_lastfuv_hor_virstride(&self) -> SwVp9LastfuvHorVirstrideR {
        SwVp9LastfuvHorVirstrideR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - vp9 last frame y horizontal virstride\n\nvp9 last frame y horizontal virstride"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_lastfy_hor_virstride(
        &mut self,
    ) -> SwVp9LastfyHorVirstrideW<Swreg37Vp9LastfHorVirstrideSpec> {
        SwVp9LastfyHorVirstrideW::new(self, 0)
    }
    #[doc = "Bits 16:24 - sw_vp9_lastfuv_hor_virstride\n\nvp9 last frame uv horizontal virstride"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_lastfuv_hor_virstride(
        &mut self,
    ) -> SwVp9LastfuvHorVirstrideW<Swreg37Vp9LastfHorVirstrideSpec> {
        SwVp9LastfuvHorVirstrideW::new(self, 16)
    }
}
#[doc = "Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg37_vp9_lastf_hor_virstride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg37_vp9_lastf_hor_virstride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg37Vp9LastfHorVirstrideSpec;
impl crate::RegisterSpec for Swreg37Vp9LastfHorVirstrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg37_vp9_lastf_hor_virstride::R`](R) reader structure"]
impl crate::Readable for Swreg37Vp9LastfHorVirstrideSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg37_vp9_lastf_hor_virstride::W`](W) writer structure"]
impl crate::Writable for Swreg37Vp9LastfHorVirstrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG37_VP9_LASTF_HOR_VIRSTRIDE to value 0"]
impl crate::Resettable for Swreg37Vp9LastfHorVirstrideSpec {
    const RESET_VALUE: u32 = 0;
}
