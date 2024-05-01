#[doc = "Register `SWREG_29` reader"]
pub type R = crate::R<Swreg29Spec>;
#[doc = "Register `SWREG_29` writer"]
pub type W = crate::W<Swreg29Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT14` reader - jpeg chroma quantization 14\n\njpeg chroma quantization 14"]
pub type SwJpegChromaQuant14R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT14` writer - jpeg chroma quantization 14\n\njpeg chroma quantization 14"]
pub type SwJpegChromaQuant14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg chroma quantization 14\n\njpeg chroma quantization 14"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant14(&self) -> SwJpegChromaQuant14R {
        SwJpegChromaQuant14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg chroma quantization 14\n\njpeg chroma quantization 14"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant14(&mut self) -> SwJpegChromaQuant14W<Swreg29Spec> {
        SwJpegChromaQuant14W::new(self, 0)
    }
}
#[doc = "14st quantization for jpeg chroma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg29Spec;
impl crate::RegisterSpec for Swreg29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_29::R`](R) reader structure"]
impl crate::Readable for Swreg29Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_29::W`](W) writer structure"]
impl crate::Writable for Swreg29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_29 to value 0"]
impl crate::Resettable for Swreg29Spec {
    const RESET_VALUE: u32 = 0;
}
