#[doc = "Register `SWREG_21` reader"]
pub type R = crate::R<Swreg21Spec>;
#[doc = "Register `SWREG_21` writer"]
pub type W = crate::W<Swreg21Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT6` reader - jpeg chroma quantization 6\n\njpeg chroma quantization 6"]
pub type SwJpegChromaQuant6R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT6` writer - jpeg chroma quantization 6\n\njpeg chroma quantization 6"]
pub type SwJpegChromaQuant6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg chroma quantization 6\n\njpeg chroma quantization 6"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant6(&self) -> SwJpegChromaQuant6R {
        SwJpegChromaQuant6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg chroma quantization 6\n\njpeg chroma quantization 6"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant6(&mut self) -> SwJpegChromaQuant6W<Swreg21Spec> {
        SwJpegChromaQuant6W::new(self, 0)
    }
}
#[doc = "6st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg21Spec;
impl crate::RegisterSpec for Swreg21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_21::R`](R) reader structure"]
impl crate::Readable for Swreg21Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_21::W`](W) writer structure"]
impl crate::Writable for Swreg21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_21 to value 0"]
impl crate::Resettable for Swreg21Spec {
    const RESET_VALUE: u32 = 0;
}
