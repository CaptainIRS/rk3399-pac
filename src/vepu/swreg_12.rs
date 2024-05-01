#[doc = "Register `SWREG_12` reader"]
pub type R = crate::R<Swreg12Spec>;
#[doc = "Register `SWREG_12` writer"]
pub type W = crate::W<Swreg12Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT13` reader - jpeg luma quantization 13\n\njpeg luma quantization 13"]
pub type SwJpegLumaQuant13R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_LUMA_QUANT13` writer - jpeg luma quantization 13\n\njpeg luma quantization 13"]
pub type SwJpegLumaQuant13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg luma quantization 13\n\njpeg luma quantization 13"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant13(&self) -> SwJpegLumaQuant13R {
        SwJpegLumaQuant13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg luma quantization 13\n\njpeg luma quantization 13"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant13(&mut self) -> SwJpegLumaQuant13W<Swreg12Spec> {
        SwJpegLumaQuant13W::new(self, 0)
    }
}
#[doc = "13st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg12Spec;
impl crate::RegisterSpec for Swreg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_12::R`](R) reader structure"]
impl crate::Readable for Swreg12Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_12::W`](W) writer structure"]
impl crate::Writable for Swreg12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_12 to value 0"]
impl crate::Resettable for Swreg12Spec {
    const RESET_VALUE: u32 = 0;
}
