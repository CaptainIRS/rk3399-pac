#[doc = "Register `SWREG_9` reader"]
pub type R = crate::R<Swreg9Spec>;
#[doc = "Register `SWREG_9` writer"]
pub type W = crate::W<Swreg9Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT10` reader - jpeg luma quantization 10\n\njpeg luma quantization 10"]
pub type SwJpegLumaQuant10R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_LUMA_QUANT10` writer - jpeg luma quantization 10\n\njpeg luma quantization 10"]
pub type SwJpegLumaQuant10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg luma quantization 10\n\njpeg luma quantization 10"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant10(&self) -> SwJpegLumaQuant10R {
        SwJpegLumaQuant10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg luma quantization 10\n\njpeg luma quantization 10"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant10(&mut self) -> SwJpegLumaQuant10W<Swreg9Spec> {
        SwJpegLumaQuant10W::new(self, 0)
    }
}
#[doc = "10st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg9Spec;
impl crate::RegisterSpec for Swreg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_9::R`](R) reader structure"]
impl crate::Readable for Swreg9Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_9::W`](W) writer structure"]
impl crate::Writable for Swreg9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_9 to value 0"]
impl crate::Resettable for Swreg9Spec {
    const RESET_VALUE: u32 = 0;
}
