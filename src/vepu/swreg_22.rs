#[doc = "Register `SWREG_22` reader"]
pub type R = crate::R<Swreg22Spec>;
#[doc = "Register `SWREG_22` writer"]
pub type W = crate::W<Swreg22Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT7` reader - jpeg chroma quantization 7\n\njpeg chroma quantization 7"]
pub type SwJpegChromaQuant7R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT7` writer - jpeg chroma quantization 7\n\njpeg chroma quantization 7"]
pub type SwJpegChromaQuant7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg chroma quantization 7\n\njpeg chroma quantization 7"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant7(&self) -> SwJpegChromaQuant7R {
        SwJpegChromaQuant7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg chroma quantization 7\n\njpeg chroma quantization 7"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant7(&mut self) -> SwJpegChromaQuant7W<Swreg22Spec> {
        SwJpegChromaQuant7W::new(self, 0)
    }
}
#[doc = "7st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg22Spec;
impl crate::RegisterSpec for Swreg22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_22::R`](R) reader structure"]
impl crate::Readable for Swreg22Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_22::W`](W) writer structure"]
impl crate::Writable for Swreg22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_22 to value 0"]
impl crate::Resettable for Swreg22Spec {
    const RESET_VALUE: u32 = 0;
}
