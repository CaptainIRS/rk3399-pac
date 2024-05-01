#[doc = "Register `SWREG_11` reader"]
pub type R = crate::R<Swreg11Spec>;
#[doc = "Register `SWREG_11` writer"]
pub type W = crate::W<Swreg11Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT12` reader - jpeg luma quantization 12\n\njpeg luma quantization 12"]
pub type SwJpegLumaQuant12R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_LUMA_QUANT12` writer - jpeg luma quantization 12\n\njpeg luma quantization 12"]
pub type SwJpegLumaQuant12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg luma quantization 12\n\njpeg luma quantization 12"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant12(&self) -> SwJpegLumaQuant12R {
        SwJpegLumaQuant12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg luma quantization 12\n\njpeg luma quantization 12"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant12(&mut self) -> SwJpegLumaQuant12W<Swreg11Spec> {
        SwJpegLumaQuant12W::new(self, 0)
    }
}
#[doc = "12st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg11Spec;
impl crate::RegisterSpec for Swreg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_11::R`](R) reader structure"]
impl crate::Readable for Swreg11Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_11::W`](W) writer structure"]
impl crate::Writable for Swreg11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_11 to value 0"]
impl crate::Resettable for Swreg11Spec {
    const RESET_VALUE: u32 = 0;
}
