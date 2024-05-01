#[doc = "Register `SWREG_23` reader"]
pub type R = crate::R<Swreg23Spec>;
#[doc = "Register `SWREG_23` writer"]
pub type W = crate::W<Swreg23Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT8` reader - jpeg chroma quantization 8\n\njpeg chroma quantization 8"]
pub type SwJpegChromaQuant8R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT8` writer - jpeg chroma quantization 8\n\njpeg chroma quantization 8"]
pub type SwJpegChromaQuant8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg chroma quantization 8\n\njpeg chroma quantization 8"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant8(&self) -> SwJpegChromaQuant8R {
        SwJpegChromaQuant8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg chroma quantization 8\n\njpeg chroma quantization 8"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant8(&mut self) -> SwJpegChromaQuant8W<Swreg23Spec> {
        SwJpegChromaQuant8W::new(self, 0)
    }
}
#[doc = "8st quantization for jpeg chroma table/part 3 for qp round\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg23Spec;
impl crate::RegisterSpec for Swreg23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_23::R`](R) reader structure"]
impl crate::Readable for Swreg23Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_23::W`](W) writer structure"]
impl crate::Writable for Swreg23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_23 to value 0"]
impl crate::Resettable for Swreg23Spec {
    const RESET_VALUE: u32 = 0;
}
