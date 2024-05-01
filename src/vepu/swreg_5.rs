#[doc = "Register `SWREG_5` reader"]
pub type R = crate::R<Swreg5Spec>;
#[doc = "Register `SWREG_5` writer"]
pub type W = crate::W<Swreg5Spec>;
#[doc = "Field `SW_JPEG_LUMA_QUANT6` reader - jpeg luma quantization 6\n\njpeg luma quantization 6"]
pub type SwJpegLumaQuant6R = crate::FieldReader<u32>;
#[doc = "Field `SW_JPEG_LUMA_QUANT6` writer - jpeg luma quantization 6\n\njpeg luma quantization 6"]
pub type SwJpegLumaQuant6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - jpeg luma quantization 6\n\njpeg luma quantization 6"]
    #[inline(always)]
    pub fn sw_jpeg_luma_quant6(&self) -> SwJpegLumaQuant6R {
        SwJpegLumaQuant6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - jpeg luma quantization 6\n\njpeg luma quantization 6"]
    #[inline(always)]
    #[must_use]
    pub fn sw_jpeg_luma_quant6(&mut self) -> SwJpegLumaQuant6W<Swreg5Spec> {
        SwJpegLumaQuant6W::new(self, 0)
    }
}
#[doc = "6st quantization for jpeg lumin table/part 1 for qp round\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg5Spec;
impl crate::RegisterSpec for Swreg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_5::R`](R) reader structure"]
impl crate::Readable for Swreg5Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_5::W`](W) writer structure"]
impl crate::Writable for Swreg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_5 to value 0"]
impl crate::Resettable for Swreg5Spec {
    const RESET_VALUE: u32 = 0;
}
