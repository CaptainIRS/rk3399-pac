#[doc = "Register `SWREG51_VP9_LASTREF_YUVSTRIDE` reader"]
pub type R = crate::R<Swreg51Vp9LastrefYuvstrideSpec>;
#[doc = "Register `SWREG51_VP9_LASTREF_YUVSTRIDE` writer"]
pub type W = crate::W<Swreg51Vp9LastrefYuvstrideSpec>;
#[doc = "Field `SW_VP9_LASTREF_YUV_VIRSTRIDE` reader - lastref frame yuv virstride\n\nvp9 lastref frame yuv vir stride"]
pub type SwVp9LastrefYuvVirstrideR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9_LASTREF_YUV_VIRSTRIDE` writer - lastref frame yuv virstride\n\nvp9 lastref frame yuv vir stride"]
pub type SwVp9LastrefYuvVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - lastref frame yuv virstride\n\nvp9 lastref frame yuv vir stride"]
    #[inline(always)]
    pub fn sw_vp9_lastref_yuv_virstride(&self) -> SwVp9LastrefYuvVirstrideR {
        SwVp9LastrefYuvVirstrideR::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - lastref frame yuv virstride\n\nvp9 lastref frame yuv vir stride"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_lastref_yuv_virstride(
        &mut self,
    ) -> SwVp9LastrefYuvVirstrideW<Swreg51Vp9LastrefYuvstrideSpec> {
        SwVp9LastrefYuvVirstrideW::new(self, 0)
    }
}
#[doc = "vp9 lastref yuv stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg51_vp9_lastref_yuvstride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg51_vp9_lastref_yuvstride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg51Vp9LastrefYuvstrideSpec;
impl crate::RegisterSpec for Swreg51Vp9LastrefYuvstrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg51_vp9_lastref_yuvstride::R`](R) reader structure"]
impl crate::Readable for Swreg51Vp9LastrefYuvstrideSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg51_vp9_lastref_yuvstride::W`](W) writer structure"]
impl crate::Writable for Swreg51Vp9LastrefYuvstrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG51_VP9_LASTREF_YUVSTRIDE to value 0"]
impl crate::Resettable for Swreg51Vp9LastrefYuvstrideSpec {
    const RESET_VALUE: u32 = 0;
}
