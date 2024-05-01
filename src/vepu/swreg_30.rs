#[doc = "Register `SWREG_30` reader"]
pub type R = crate::R<Swreg30Spec>;
#[doc = "Register `SWREG_30` writer"]
pub type W = crate::W<Swreg30Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT15` reader - jpeg chroma quantization 15\n\njpeg chroma quantization 15"]
pub type SwJpegChromaQuant15R = crate::FieldReader<u16>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT15` writer - jpeg chroma quantization 15\n\njpeg chroma quantization 15"]
pub type SwJpegChromaQuant15W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - jpeg chroma quantization 15\n\njpeg chroma quantization 15"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant15(&self) -> SwJpegChromaQuant15R {
        SwJpegChromaQuant15R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - jpeg chroma quantization 15\n\njpeg chroma quantization 15"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant15(&mut self) -> SwJpegChromaQuant15W<Swreg30Spec> {
        SwJpegChromaQuant15W::new(self, 0)
    }
}
#[doc = "15st quantization for jpeg chroma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg30Spec;
impl crate::RegisterSpec for Swreg30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_30::R`](R) reader structure"]
impl crate::Readable for Swreg30Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_30::W`](W) writer structure"]
impl crate::Writable for Swreg30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_30 to value 0"]
impl crate::Resettable for Swreg30Spec {
    const RESET_VALUE: u32 = 0;
}
