#[doc = "Register `SWREG38_VP9_GOLDENF_HOR_VIRSTRIDE` reader"]
pub type R = crate::R<Swreg38Vp9GoldenfHorVirstrideSpec>;
#[doc = "Register `SWREG38_VP9_GOLDENF_HOR_VIRSTRIDE` writer"]
pub type W = crate::W<Swreg38Vp9GoldenfHorVirstrideSpec>;
#[doc = "Field `SW_VP9_GOLDENFY_HOR_VIRSTRIDE` reader - sw_vp9_goldenfy_hor_virstirde\n\nvp9 golden frame y horizontal virstride"]
pub type SwVp9GoldenfyHorVirstrideR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_GOLDENFY_HOR_VIRSTRIDE` writer - sw_vp9_goldenfy_hor_virstirde\n\nvp9 golden frame y horizontal virstride"]
pub type SwVp9GoldenfyHorVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_VP9_GOLDENUV_HOR_VIRSTRIDE` reader - vp9 goldenuv horizontal virstride\n\nvp9 golden uv horizontal virstirde"]
pub type SwVp9GoldenuvHorVirstrideR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_GOLDENUV_HOR_VIRSTRIDE` writer - vp9 goldenuv horizontal virstride\n\nvp9 golden uv horizontal virstirde"]
pub type SwVp9GoldenuvHorVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - sw_vp9_goldenfy_hor_virstirde\n\nvp9 golden frame y horizontal virstride"]
    #[inline(always)]
    pub fn sw_vp9_goldenfy_hor_virstride(&self) -> SwVp9GoldenfyHorVirstrideR {
        SwVp9GoldenfyHorVirstrideR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - vp9 goldenuv horizontal virstride\n\nvp9 golden uv horizontal virstirde"]
    #[inline(always)]
    pub fn sw_vp9_goldenuv_hor_virstride(&self) -> SwVp9GoldenuvHorVirstrideR {
        SwVp9GoldenuvHorVirstrideR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - sw_vp9_goldenfy_hor_virstirde\n\nvp9 golden frame y horizontal virstride"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_goldenfy_hor_virstride(
        &mut self,
    ) -> SwVp9GoldenfyHorVirstrideW<Swreg38Vp9GoldenfHorVirstrideSpec> {
        SwVp9GoldenfyHorVirstrideW::new(self, 0)
    }
    #[doc = "Bits 16:24 - vp9 goldenuv horizontal virstride\n\nvp9 golden uv horizontal virstirde"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_goldenuv_hor_virstride(
        &mut self,
    ) -> SwVp9GoldenuvHorVirstrideW<Swreg38Vp9GoldenfHorVirstrideSpec> {
        SwVp9GoldenuvHorVirstrideW::new(self, 16)
    }
}
#[doc = "vp9 golden frame horizontal virstride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg38_vp9_goldenf_hor_virstride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg38_vp9_goldenf_hor_virstride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg38Vp9GoldenfHorVirstrideSpec;
impl crate::RegisterSpec for Swreg38Vp9GoldenfHorVirstrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg38_vp9_goldenf_hor_virstride::R`](R) reader structure"]
impl crate::Readable for Swreg38Vp9GoldenfHorVirstrideSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg38_vp9_goldenf_hor_virstride::W`](W) writer structure"]
impl crate::Writable for Swreg38Vp9GoldenfHorVirstrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG38_VP9_GOLDENF_HOR_VIRSTRIDE to value 0"]
impl crate::Resettable for Swreg38Vp9GoldenfHorVirstrideSpec {
    const RESET_VALUE: u32 = 0;
}
