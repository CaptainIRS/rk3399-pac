#[doc = "Register `SWREG_27` reader"]
pub type R = crate::R<Swreg27Spec>;
#[doc = "Register `SWREG_27` writer"]
pub type W = crate::W<Swreg27Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT11` reader - jpeg chroma quantization 11\n\njpeg chroma quantization 11"]
pub type SwJpegChromaQuant11R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT11` writer - jpeg chroma quantization 11\n\njpeg chroma quantization 11"]
pub type SwJpegChromaQuant11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg chroma quantization 11\n\njpeg chroma quantization 11"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant11(&self) -> SwJpegChromaQuant11R {
        SwJpegChromaQuant11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg chroma quantization 11\n\njpeg chroma quantization 11"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant11(&mut self) -> SwJpegChromaQuant11W<Swreg27Spec> {
        SwJpegChromaQuant11W::new(self, 0)
    }
}
#[doc = "12st quantization for jpeg chroma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg27Spec;
impl crate::RegisterSpec for Swreg27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_27::R`](R) reader structure"]
impl crate::Readable for Swreg27Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_27::W`](W) writer structure"]
impl crate::Writable for Swreg27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_27 to value 0"]
impl crate::Resettable for Swreg27Spec {
    const RESET_VALUE: u32 = 0;
}
