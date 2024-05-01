#[doc = "Register `SWREG_3` reader"]
pub type R = crate::R<Swreg3Spec>;
#[doc = "Register `SWREG_3` writer"]
pub type W = crate::W<Swreg3Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT4` reader - jpeg luma quantization 4\n\njpeg luma quantization 4"]
pub type SwJpegLumaQuant4R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_LUMA_QUANT4` writer - jpeg luma quantization 4\n\njpeg luma quantization 4"]
pub type SwJpegLumaQuant4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg luma quantization 4\n\njpeg luma quantization 4"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant4(&self) -> SwJpegLumaQuant4R {
        SwJpegLumaQuant4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg luma quantization 4\n\njpeg luma quantization 4"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant4(&mut self) -> SwJpegLumaQuant4W<Swreg3Spec> {
        SwJpegLumaQuant4W::new(self, 0)
    }
}
#[doc = "4st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg3Spec;
impl crate::RegisterSpec for Swreg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_3::R`](R) reader structure"]
impl crate::Readable for Swreg3Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_3::W`](W) writer structure"]
impl crate::Writable for Swreg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_3 to value 0"]
impl crate::Resettable for Swreg3Spec {
    const RESET_VALUE: u32 = 0;
}
