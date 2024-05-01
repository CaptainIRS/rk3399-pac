#[doc = "Register `SWREG_18` reader"]
pub type R = crate::R<Swreg18Spec>;
#[doc = "Register `SWREG_18` writer"]
pub type W = crate::W<Swreg18Spec>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT3` reader - jpeg chroma quantization 3\n\njpeg chroma quantization 3"]
pub type SwJpegChromaQuant3R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_CHROMA_QUANT3` writer - jpeg chroma quantization 3\n\njpeg chroma quantization 3"]
pub type SwJpegChromaQuant3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg chroma quantization 3\n\njpeg chroma quantization 3"]
    #[inline(always)]
    pub fn sw_jpeg_chroma_quant3(&self) -> SwJpegChromaQuant3R {
        SwJpegChromaQuant3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg chroma quantization 3\n\njpeg chroma quantization 3"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_chroma_quant3(&mut self) -> SwJpegChromaQuant3W<Swreg18Spec> {
        SwJpegChromaQuant3W::new(self, 0)
    }
}
#[doc = "3st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg18Spec;
impl crate::RegisterSpec for Swreg18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_18::R`](R) reader structure"]
impl crate::Readable for Swreg18Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_18::W`](W) writer structure"]
impl crate::Writable for Swreg18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_18 to value 0"]
impl crate::Resettable for Swreg18Spec {
    const RESET_VALUE: u32 = 0;
}
