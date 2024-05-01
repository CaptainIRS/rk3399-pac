#[doc = "Register `SWREG39_VP9_ALTREFF_HOR_VIRSTRIDE` reader"]
pub type R = crate::R<Swreg39Vp9AltreffHorVirstrideSpec>;
#[doc = "Register `SWREG39_VP9_ALTREFF_HOR_VIRSTRIDE` writer"]
pub type W = crate::W<Swreg39Vp9AltreffHorVirstrideSpec>;
#[doc = "Field `SW_VP9_ALTREFFY_HOR_VIRSTRIDE` reader - sw_vp9_altreffy_hor_virstirde\n\nvp9 altref frame y horizontal virstride"]
pub type SwVp9AltreffyHorVirstrideR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_ALTREFFY_HOR_VIRSTRIDE` writer - sw_vp9_altreffy_hor_virstirde\n\nvp9 altref frame y horizontal virstride"]
pub type SwVp9AltreffyHorVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_VP9_ALTREFFUV_HOR_VIRSTRIDE` reader - vp9 altreffuv horizontal virstride\n\nvp9 altreff uv horizontal virstirde"]
pub type SwVp9AltreffuvHorVirstrideR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_ALTREFFUV_HOR_VIRSTRIDE` writer - vp9 altreffuv horizontal virstride\n\nvp9 altreff uv horizontal virstirde"]
pub type SwVp9AltreffuvHorVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - sw_vp9_altreffy_hor_virstirde\n\nvp9 altref frame y horizontal virstride"]
    #[inline(always)]
    pub fn sw_vp9_altreffy_hor_virstride(&self) -> SwVp9AltreffyHorVirstrideR {
        SwVp9AltreffyHorVirstrideR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - vp9 altreffuv horizontal virstride\n\nvp9 altreff uv horizontal virstirde"]
    #[inline(always)]
    pub fn sw_vp9_altreffuv_hor_virstride(&self) -> SwVp9AltreffuvHorVirstrideR {
        SwVp9AltreffuvHorVirstrideR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - sw_vp9_altreffy_hor_virstirde\n\nvp9 altref frame y horizontal virstride"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_altreffy_hor_virstride(
        &mut self,
    ) -> SwVp9AltreffyHorVirstrideW<Swreg39Vp9AltreffHorVirstrideSpec> {
        SwVp9AltreffyHorVirstrideW::new(self, 0)
    }
    #[doc = "Bits 16:24 - vp9 altreffuv horizontal virstride\n\nvp9 altreff uv horizontal virstirde"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_altreffuv_hor_virstride(
        &mut self,
    ) -> SwVp9AltreffuvHorVirstrideW<Swreg39Vp9AltreffHorVirstrideSpec> {
        SwVp9AltreffuvHorVirstrideW::new(self, 16)
    }
}
#[doc = "vp9 altref frame horizontal virstride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg39_vp9_altreff_hor_virstride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg39_vp9_altreff_hor_virstride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg39Vp9AltreffHorVirstrideSpec;
impl crate::RegisterSpec for Swreg39Vp9AltreffHorVirstrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg39_vp9_altreff_hor_virstride::R`](R) reader structure"]
impl crate::Readable for Swreg39Vp9AltreffHorVirstrideSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg39_vp9_altreff_hor_virstride::W`](W) writer structure"]
impl crate::Writable for Swreg39Vp9AltreffHorVirstrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG39_VP9_ALTREFF_HOR_VIRSTRIDE to value 0"]
impl crate::Resettable for Swreg39Vp9AltreffHorVirstrideSpec {
    const RESET_VALUE: u32 = 0;
}
