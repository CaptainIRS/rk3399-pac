#[doc = "Register `SWREG_19` reader"]
pub type R = crate::R<Swreg19Spec>;
#[doc = "Register `SWREG_19` writer"]
pub type W = crate::W<Swreg19Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT4` reader - jpeg chroma quantization 4\n\njpeg chroma quantization 4"]
pub type SwJpegChromaQuant4R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT4` writer - jpeg chroma quantization 4\n\njpeg chroma quantization 4"]
pub type SwJpegChromaQuant4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg chroma quantization 4\n\njpeg chroma quantization 4"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant4(&self) -> SwJpegChromaQuant4R {
        SwJpegChromaQuant4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg chroma quantization 4\n\njpeg chroma quantization 4"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant4(&mut self) -> SwJpegChromaQuant4W<Swreg19Spec> {
        SwJpegChromaQuant4W::new(self, 0)
    }
}
#[doc = "4st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg19Spec;
impl crate::RegisterSpec for Swreg19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_19::R`](R) reader structure"]
impl crate::Readable for Swreg19Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_19::W`](W) writer structure"]
impl crate::Writable for Swreg19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_19 to value 0"]
impl crate::Resettable for Swreg19Spec {
    const RESET_VALUE: u32 = 0;
}
