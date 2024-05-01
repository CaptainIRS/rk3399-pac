#[doc = "Register `SWREG_6` reader"]
pub type R = crate::R<Swreg6Spec>;
#[doc = "Register `SWREG_6` writer"]
pub type W = crate::W<Swreg6Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT7` reader - jpeg luma quantization 7\n\njpeg luma quantization 7"]
pub type SwJpegLumaQuant7R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_LUMA_QUANT7` writer - jpeg luma quantization 7\n\njpeg luma quantization 7"]
pub type SwJpegLumaQuant7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg luma quantization 7\n\njpeg luma quantization 7"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant7(&self) -> SwJpegLumaQuant7R {
        SwJpegLumaQuant7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg luma quantization 7\n\njpeg luma quantization 7"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant7(&mut self) -> SwJpegLumaQuant7W<Swreg6Spec> {
        SwJpegLumaQuant7W::new(self, 0)
    }
}
#[doc = "7st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg6Spec;
impl crate::RegisterSpec for Swreg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_6::R`](R) reader structure"]
impl crate::Readable for Swreg6Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_6::W`](W) writer structure"]
impl crate::Writable for Swreg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_6 to value 0"]
impl crate::Resettable for Swreg6Spec {
    const RESET_VALUE: u32 = 0;
}
