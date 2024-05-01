#[doc = "Register `SWREG_10` reader"]
pub type R = crate::R<Swreg10Spec>;
#[doc = "Register `SWREG_10` writer"]
pub type W = crate::W<Swreg10Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT11` reader - jpeg luma quantization 11\n\njpeg luma quantization 11"]
pub type SwJpegLumaQuant11R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_LUMA_QUANT11` writer - jpeg luma quantization 11\n\njpeg luma quantization 11"]
pub type SwJpegLumaQuant11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg luma quantization 11\n\njpeg luma quantization 11"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant11(&self) -> SwJpegLumaQuant11R {
        SwJpegLumaQuant11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg luma quantization 11\n\njpeg luma quantization 11"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant11(&mut self) -> SwJpegLumaQuant11W<Swreg10Spec> {
        SwJpegLumaQuant11W::new(self, 0)
    }
}
#[doc = "11st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg10Spec;
impl crate::RegisterSpec for Swreg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_10::R`](R) reader structure"]
impl crate::Readable for Swreg10Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_10::W`](W) writer structure"]
impl crate::Writable for Swreg10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_10 to value 0"]
impl crate::Resettable for Swreg10Spec {
    const RESET_VALUE: u32 = 0;
}
