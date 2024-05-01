#[doc = "Register `SWREG_14` reader"]
pub type R = crate::R<Swreg14Spec>;
#[doc = "Register `SWREG_14` writer"]
pub type W = crate::W<Swreg14Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT15` reader - jpeg luma quantization 15\n\njpeg luma quantization 15"]
pub type SwJpegLumaQuant15R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_LUMA_QUANT15` writer - jpeg luma quantization 15\n\njpeg luma quantization 15"]
pub type SwJpegLumaQuant15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg luma quantization 15\n\njpeg luma quantization 15"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant15(&self) -> SwJpegLumaQuant15R {
        SwJpegLumaQuant15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg luma quantization 15\n\njpeg luma quantization 15"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant15(&mut self) -> SwJpegLumaQuant15W<Swreg14Spec> {
        SwJpegLumaQuant15W::new(self, 0)
    }
}
#[doc = "15st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg14Spec;
impl crate::RegisterSpec for Swreg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_14::R`](R) reader structure"]
impl crate::Readable for Swreg14Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_14::W`](W) writer structure"]
impl crate::Writable for Swreg14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_14 to value 0"]
impl crate::Resettable for Swreg14Spec {
    const RESET_VALUE: u32 = 0;
}
