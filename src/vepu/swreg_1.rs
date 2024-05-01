#[doc = "Register `SWREG_1` reader"]
pub type R = crate::R<Swreg1Spec>;
#[doc = "Register `SWREG_1` writer"]
pub type W = crate::W<Swreg1Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT2` reader - jpeg luma quantization 2\n\njpeg luma quantization 2"]
pub type SwJpegLumaQuant2R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_LUMA_QUANT2` writer - jpeg luma quantization 2\n\njpeg luma quantization 2"]
pub type SwJpegLumaQuant2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg luma quantization 2\n\njpeg luma quantization 2"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant2(&self) -> SwJpegLumaQuant2R {
        SwJpegLumaQuant2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg luma quantization 2\n\njpeg luma quantization 2"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant2(&mut self) -> SwJpegLumaQuant2W<Swreg1Spec> {
        SwJpegLumaQuant2W::new(self, 0)
    }
}
#[doc = "2st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg1Spec;
impl crate::RegisterSpec for Swreg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_1::R`](R) reader structure"]
impl crate::Readable for Swreg1Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_1::W`](W) writer structure"]
impl crate::Writable for Swreg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_1 to value 0"]
impl crate::Resettable for Swreg1Spec {
    const RESET_VALUE: u32 = 0;
}
