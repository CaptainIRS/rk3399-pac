#[doc = "Register `SWREG_7` reader"]
pub type R = crate::R<Swreg7Spec>;
#[doc = "Register `SWREG_7` writer"]
pub type W = crate::W<Swreg7Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT8` reader - jpeg luma quantization 8\n\njpeg luma quantization 8"]
pub type SwJpegLumaQuant8R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_LUMA_QUANT8` writer - jpeg luma quantization 8\n\njpeg luma quantization 8"]
pub type SwJpegLumaQuant8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg luma quantization 8\n\njpeg luma quantization 8"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant8(&self) -> SwJpegLumaQuant8R {
        SwJpegLumaQuant8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg luma quantization 8\n\njpeg luma quantization 8"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant8(&mut self) -> SwJpegLumaQuant8W<Swreg7Spec> {
        SwJpegLumaQuant8W::new(self, 0)
    }
}
#[doc = "8st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg7Spec;
impl crate::RegisterSpec for Swreg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_7::R`](R) reader structure"]
impl crate::Readable for Swreg7Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_7::W`](W) writer structure"]
impl crate::Writable for Swreg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_7 to value 0"]
impl crate::Resettable for Swreg7Spec {
    const RESET_VALUE: u32 = 0;
}
