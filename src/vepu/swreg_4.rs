#[doc = "Register `SWREG_4` reader"]
pub type R = crate::R<Swreg4Spec>;
#[doc = "Register `SWREG_4` writer"]
pub type W = crate::W<Swreg4Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT5` reader - jpeg luma quantization 5\n\njpeg luma quantization 5"]
pub type SwJpegLumaQuant5R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_LUMA_QUANT5` writer - jpeg luma quantization 5\n\njpeg luma quantization 5"]
pub type SwJpegLumaQuant5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg luma quantization 5\n\njpeg luma quantization 5"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant5(&self) -> SwJpegLumaQuant5R {
        SwJpegLumaQuant5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg luma quantization 5\n\njpeg luma quantization 5"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant5(&mut self) -> SwJpegLumaQuant5W<Swreg4Spec> {
        SwJpegLumaQuant5W::new(self, 0)
    }
}
#[doc = "5st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg4Spec;
impl crate::RegisterSpec for Swreg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_4::R`](R) reader structure"]
impl crate::Readable for Swreg4Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_4::W`](W) writer structure"]
impl crate::Writable for Swreg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_4 to value 0"]
impl crate::Resettable for Swreg4Spec {
    const RESET_VALUE: u32 = 0;
}
