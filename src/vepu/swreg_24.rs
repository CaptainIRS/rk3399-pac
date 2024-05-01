#[doc = "Register `SWREG_24` reader"]
pub type R = crate::R<Swreg24Spec>;
#[doc = "Register `SWREG_24` writer"]
pub type W = crate::W<Swreg24Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT9` reader - jpeg chroma quantization 9\n\njpeg chroma quantization 9"]
pub type SwJpegChromaQuant9R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT9` writer - jpeg chroma quantization 9\n\njpeg chroma quantization 9"]
pub type SwJpegChromaQuant9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg chroma quantization 9\n\njpeg chroma quantization 9"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant9(&self) -> SwJpegChromaQuant9R {
        SwJpegChromaQuant9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg chroma quantization 9\n\njpeg chroma quantization 9"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant9(&mut self) -> SwJpegChromaQuant9W<Swreg24Spec> {
        SwJpegChromaQuant9W::new(self, 0)
    }
}
#[doc = "9st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg24Spec;
impl crate::RegisterSpec for Swreg24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_24::R`](R) reader structure"]
impl crate::Readable for Swreg24Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_24::W`](W) writer structure"]
impl crate::Writable for Swreg24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_24 to value 0"]
impl crate::Resettable for Swreg24Spec {
    const RESET_VALUE: u32 = 0;
}
