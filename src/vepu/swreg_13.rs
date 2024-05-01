#[doc = "Register `SWREG_13` reader"]
pub type R = crate::R<Swreg13Spec>;
#[doc = "Register `SWREG_13` writer"]
pub type W = crate::W<Swreg13Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT14` reader - jpeg luma quantization 14\n\njpeg luma quantization 14"]
pub type SwJpegLumaQuant14R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_LUMA_QUANT14` writer - jpeg luma quantization 14\n\njpeg luma quantization 14"]
pub type SwJpegLumaQuant14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg luma quantization 14\n\njpeg luma quantization 14"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant14(&self) -> SwJpegLumaQuant14R {
        SwJpegLumaQuant14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg luma quantization 14\n\njpeg luma quantization 14"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant14(&mut self) -> SwJpegLumaQuant14W<Swreg13Spec> {
        SwJpegLumaQuant14W::new(self, 0)
    }
}
#[doc = "14st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg13Spec;
impl crate::RegisterSpec for Swreg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_13::R`](R) reader structure"]
impl crate::Readable for Swreg13Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_13::W`](W) writer structure"]
impl crate::Writable for Swreg13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_13 to value 0"]
impl crate::Resettable for Swreg13Spec {
    const RESET_VALUE: u32 = 0;
}
