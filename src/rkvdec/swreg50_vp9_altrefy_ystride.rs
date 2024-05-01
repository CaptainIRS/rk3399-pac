#[doc = "Register `SWREG50_VP9_ALTREFY_YSTRIDE` reader"]
pub type R = crate::R<Swreg50Vp9AltrefyYstrideSpec>;
#[doc = "Register `SWREG50_VP9_ALTREFY_YSTRIDE` writer"]
pub type W = crate::W<Swreg50Vp9AltrefyYstrideSpec>;
#[doc = "Field `SW_VP9_ALTREFY_VIRSTRIDE` reader - altref frame y virstride\n\nvp9 altref frame y stride"]
pub type SwVp9AltrefyVirstrideR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9_ALTREFY_VIRSTRIDE` writer - altref frame y virstride\n\nvp9 altref frame y stride"]
pub type SwVp9AltrefyVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - altref frame y virstride\n\nvp9 altref frame y stride"]
    #[inline(always)]
    pub fn sw_vp9_altrefy_virstride(&self) -> SwVp9AltrefyVirstrideR {
        SwVp9AltrefyVirstrideR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - altref frame y virstride\n\nvp9 altref frame y stride"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_altrefy_virstride(
        &mut self,
    ) -> SwVp9AltrefyVirstrideW<Swreg50Vp9AltrefyYstrideSpec> {
        SwVp9AltrefyVirstrideW::new(self, 0)
    }
}
#[doc = "altref ref ystride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg50_vp9_altrefy_ystride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg50_vp9_altrefy_ystride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg50Vp9AltrefyYstrideSpec;
impl crate::RegisterSpec for Swreg50Vp9AltrefyYstrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg50_vp9_altrefy_ystride::R`](R) reader structure"]
impl crate::Readable for Swreg50Vp9AltrefyYstrideSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg50_vp9_altrefy_ystride::W`](W) writer structure"]
impl crate::Writable for Swreg50Vp9AltrefyYstrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG50_VP9_ALTREFY_YSTRIDE to value 0"]
impl crate::Resettable for Swreg50Vp9AltrefyYstrideSpec {
    const RESET_VALUE: u32 = 0;
}
