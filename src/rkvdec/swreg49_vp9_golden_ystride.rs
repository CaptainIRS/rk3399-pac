#[doc = "Register `SWREG49_VP9_GOLDEN_YSTRIDE` reader"]
pub type R = crate::R<Swreg49Vp9GoldenYstrideSpec>;
#[doc = "Register `SWREG49_VP9_GOLDEN_YSTRIDE` writer"]
pub type W = crate::W<Swreg49Vp9GoldenYstrideSpec>;
#[doc = "Field `SW_VP9_GOLDENY_VIRSTRIDE` reader - golden frame y virstride\n\nvp9 golden frame y stride"]
pub type SwVp9GoldenyVirstrideR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9_GOLDENY_VIRSTRIDE` writer - golden frame y virstride\n\nvp9 golden frame y stride"]
pub type SwVp9GoldenyVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - golden frame y virstride\n\nvp9 golden frame y stride"]
    #[inline(always)]
    pub fn sw_vp9_goldeny_virstride(&self) -> SwVp9GoldenyVirstrideR {
        SwVp9GoldenyVirstrideR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - golden frame y virstride\n\nvp9 golden frame y stride"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_goldeny_virstride(
        &mut self,
    ) -> SwVp9GoldenyVirstrideW<Swreg49Vp9GoldenYstrideSpec> {
        SwVp9GoldenyVirstrideW::new(self, 0)
    }
}
#[doc = "golden ref ystride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg49_vp9_golden_ystride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg49_vp9_golden_ystride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg49Vp9GoldenYstrideSpec;
impl crate::RegisterSpec for Swreg49Vp9GoldenYstrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg49_vp9_golden_ystride::R`](R) reader structure"]
impl crate::Readable for Swreg49Vp9GoldenYstrideSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg49_vp9_golden_ystride::W`](W) writer structure"]
impl crate::Writable for Swreg49Vp9GoldenYstrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG49_VP9_GOLDEN_YSTRIDE to value 0"]
impl crate::Resettable for Swreg49Vp9GoldenYstrideSpec {
    const RESET_VALUE: u32 = 0;
}
