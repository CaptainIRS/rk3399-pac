#[doc = "Register `SWREG_16` reader"]
pub type R = crate::R<Swreg16Spec>;
#[doc = "Register `SWREG_16` writer"]
pub type W = crate::W<Swreg16Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT1` reader - jpeg chroma quantization 1\n\njpeg chroma quantization 1"]
pub type SwJpegChromaQuant1R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT1` writer - jpeg chroma quantization 1\n\njpeg chroma quantization 1"]
pub type SwJpegChromaQuant1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg chroma quantization 1\n\njpeg chroma quantization 1"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant1(&self) -> SwJpegChromaQuant1R {
        SwJpegChromaQuant1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg chroma quantization 1\n\njpeg chroma quantization 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant1(&mut self) -> SwJpegChromaQuant1W<Swreg16Spec> {
        SwJpegChromaQuant1W::new(self, 0)
    }
}
#[doc = "1st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg16Spec;
impl crate::RegisterSpec for Swreg16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_16::R`](R) reader structure"]
impl crate::Readable for Swreg16Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_16::W`](W) writer structure"]
impl crate::Writable for Swreg16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_16 to value 0"]
impl crate::Resettable for Swreg16Spec {
    const RESET_VALUE: u32 = 0;
}
