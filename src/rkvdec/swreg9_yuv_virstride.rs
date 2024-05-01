#[doc = "Register `SWREG9_YUV_VIRSTRIDE` reader"]
pub type R = crate::R<Swreg9YuvVirstrideSpec>;
#[doc = "Register `SWREG9_YUV_VIRSTRIDE` writer"]
pub type W = crate::W<Swreg9YuvVirstrideSpec>;
#[doc = "Field `SW_YUV_VIRSTRIDE` reader - the ouput picture yuv virtual stride\n\nthe output picture yuv virtual stride (the unit is 128bit)\n\nthe max : (4096x1.5 +128) x 2304 x1.5 = 0x14ac000\n\nwe can know the sw_mvout_base = sw_decout_base +\n\n(sw_yuv_virstride &lt;&lt;4)\n\nfor yuv422: 4096x2304x2x1.25= 0x1680000"]
pub type SwYuvVirstrideR = crate::FieldReader<u32>;
#[doc = "Field `SW_YUV_VIRSTRIDE` writer - the ouput picture yuv virtual stride\n\nthe output picture yuv virtual stride (the unit is 128bit)\n\nthe max : (4096x1.5 +128) x 2304 x1.5 = 0x14ac000\n\nwe can know the sw_mvout_base = sw_decout_base +\n\n(sw_yuv_virstride &lt;&lt;4)\n\nfor yuv422: 4096x2304x2x1.25= 0x1680000"]
pub type SwYuvVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - the ouput picture yuv virtual stride\n\nthe output picture yuv virtual stride (the unit is 128bit)\n\nthe max : (4096x1.5 +128) x 2304 x1.5 = 0x14ac000\n\nwe can know the sw_mvout_base = sw_decout_base +\n\n(sw_yuv_virstride &lt;&lt;4)\n\nfor yuv422: 4096x2304x2x1.25= 0x1680000"]
    #[inline(always)]
    pub fn sw_yuv_virstride(&self) -> SwYuvVirstrideR {
        SwYuvVirstrideR::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - the ouput picture yuv virtual stride\n\nthe output picture yuv virtual stride (the unit is 128bit)\n\nthe max : (4096x1.5 +128) x 2304 x1.5 = 0x14ac000\n\nwe can know the sw_mvout_base = sw_decout_base +\n\n(sw_yuv_virstride &lt;&lt;4)\n\nfor yuv422: 4096x2304x2x1.25= 0x1680000"]
    #[inline(always)]
    #[must_use]
    pub fn sw_yuv_virstride(&mut self) -> SwYuvVirstrideW<Swreg9YuvVirstrideSpec> {
        SwYuvVirstrideW::new(self, 0)
    }
}
#[doc = "the ouput picture yuv virtual stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg9_yuv_virstride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg9_yuv_virstride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg9YuvVirstrideSpec;
impl crate::RegisterSpec for Swreg9YuvVirstrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg9_yuv_virstride::R`](R) reader structure"]
impl crate::Readable for Swreg9YuvVirstrideSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg9_yuv_virstride::W`](W) writer structure"]
impl crate::Writable for Swreg9YuvVirstrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG9_YUV_VIRSTRIDE to value 0"]
impl crate::Resettable for Swreg9YuvVirstrideSpec {
    const RESET_VALUE: u32 = 0;
}
