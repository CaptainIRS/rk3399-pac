#[doc = "Register `SWREG_31` reader"]
pub type R = crate::R<Swreg31Spec>;
#[doc = "Register `SWREG_31` writer"]
pub type W = crate::W<Swreg31Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT16` reader - jpeg chroma quantization 16\n\njpeg chroma quantization 16"]
pub type SwJpegChromaQuant16R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT16` writer - jpeg chroma quantization 16\n\njpeg chroma quantization 16"]
pub type SwJpegChromaQuant16W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg chroma quantization 16\n\njpeg chroma quantization 16"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant16(&self) -> SwJpegChromaQuant16R {
        SwJpegChromaQuant16R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg chroma quantization 16\n\njpeg chroma quantization 16"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant16(&mut self) -> SwJpegChromaQuant16W<Swreg31Spec> {
        SwJpegChromaQuant16W::new(self, 0)
    }
}
#[doc = "16st quantization for jpeg chroma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg31Spec;
impl crate::RegisterSpec for Swreg31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_31::R`](R) reader structure"]
impl crate::Readable for Swreg31Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_31::W`](W) writer structure"]
impl crate::Writable for Swreg31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_31 to value 0"]
impl crate::Resettable for Swreg31Spec {
    const RESET_VALUE: u32 = 0;
}
