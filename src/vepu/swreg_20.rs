#[doc = "Register `SWREG_20` reader"]
pub type R = crate::R<Swreg20Spec>;
#[doc = "Register `SWREG_20` writer"]
pub type W = crate::W<Swreg20Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT5` reader - jpeg chroma quantization 5\n\njpeg chroma quantization 5"]
pub type SwJpegChromaQuant5R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT5` writer - jpeg chroma quantization 5\n\njpeg chroma quantization 5"]
pub type SwJpegChromaQuant5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg chroma quantization 5\n\njpeg chroma quantization 5"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant5(&self) -> SwJpegChromaQuant5R {
        SwJpegChromaQuant5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg chroma quantization 5\n\njpeg chroma quantization 5"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant5(&mut self) -> SwJpegChromaQuant5W<Swreg20Spec> {
        SwJpegChromaQuant5W::new(self, 0)
    }
}
#[doc = "5st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg20Spec;
impl crate::RegisterSpec for Swreg20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_20::R`](R) reader structure"]
impl crate::Readable for Swreg20Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_20::W`](W) writer structure"]
impl crate::Writable for Swreg20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_20 to value 0"]
impl crate::Resettable for Swreg20Spec {
    const RESET_VALUE: u32 = 0;
}
