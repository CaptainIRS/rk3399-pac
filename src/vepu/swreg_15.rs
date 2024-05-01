#[doc = "Register `SWREG_15` reader"]
pub type R = crate::R<Swreg15Spec>;
#[doc = "Register `SWREG_15` writer"]
pub type W = crate::W<Swreg15Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT16` reader - jpeg luma quantization 16\n\njpeg luma quantization 16"]
pub type SwJpegLumaQuant16R = crate::FieldReader;
#[doc = "Field `SW_JPEG_LUMA_QUANT16` writer - jpeg luma quantization 16\n\njpeg luma quantization 16"]
pub type SwJpegLumaQuant16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - jpeg luma quantization 16\n\njpeg luma quantization 16"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant16(&self) -> SwJpegLumaQuant16R {
        SwJpegLumaQuant16R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - jpeg luma quantization 16\n\njpeg luma quantization 16"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant16(&mut self) -> SwJpegLumaQuant16W<Swreg15Spec> {
        SwJpegLumaQuant16W::new(self, 0)
    }
}
#[doc = "16st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg15Spec;
impl crate::RegisterSpec for Swreg15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_15::R`](R) reader structure"]
impl crate::Readable for Swreg15Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_15::W`](W) writer structure"]
impl crate::Writable for Swreg15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_15 to value 0"]
impl crate::Resettable for Swreg15Spec {
    const RESET_VALUE: u32 = 0;
}
