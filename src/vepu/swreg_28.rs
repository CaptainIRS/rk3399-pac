#[doc = "Register `SWREG_28` reader"]
pub type R = crate::R<Swreg28Spec>;
#[doc = "Register `SWREG_28` writer"]
pub type W = crate::W<Swreg28Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT13` reader - jpeg chroma quantization 13\n\njpeg chroma quantization 13"]
pub type SwJpegChromaQuant13R = crate::FieldReader<u16>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT13` writer - jpeg chroma quantization 13\n\njpeg chroma quantization 13"]
pub type SwJpegChromaQuant13W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - jpeg chroma quantization 13\n\njpeg chroma quantization 13"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant13(&self) -> SwJpegChromaQuant13R {
        SwJpegChromaQuant13R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - jpeg chroma quantization 13\n\njpeg chroma quantization 13"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant13(&mut self) -> SwJpegChromaQuant13W<Swreg28Spec> {
        SwJpegChromaQuant13W::new(self, 0)
    }
}
#[doc = "13st quantization for jpeg chroma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg28Spec;
impl crate::RegisterSpec for Swreg28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_28::R`](R) reader structure"]
impl crate::Readable for Swreg28Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_28::W`](W) writer structure"]
impl crate::Writable for Swreg28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_28 to value 0"]
impl crate::Resettable for Swreg28Spec {
    const RESET_VALUE: u32 = 0;
}
