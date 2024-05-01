#[doc = "Register `SWREG_17` reader"]
pub type R = crate::R<Swreg17Spec>;
#[doc = "Register `SWREG_17` writer"]
pub type W = crate::W<Swreg17Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT2` reader - jpeg chroma quantization 2\n\njpeg chroma quantization 2"]
pub type SwJpegChromaQuant2R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT2` writer - jpeg chroma quantization 2\n\njpeg chroma quantization 2"]
pub type SwJpegChromaQuant2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg chroma quantization 2\n\njpeg chroma quantization 2"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant2(&self) -> SwJpegChromaQuant2R {
        SwJpegChromaQuant2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg chroma quantization 2\n\njpeg chroma quantization 2"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant2(&mut self) -> SwJpegChromaQuant2W<Swreg17Spec> {
        SwJpegChromaQuant2W::new(self, 0)
    }
}
#[doc = "2st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg17Spec;
impl crate::RegisterSpec for Swreg17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_17::R`](R) reader structure"]
impl crate::Readable for Swreg17Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_17::W`](W) writer structure"]
impl crate::Writable for Swreg17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_17 to value 0"]
impl crate::Resettable for Swreg17Spec {
    const RESET_VALUE: u32 = 0;
}
