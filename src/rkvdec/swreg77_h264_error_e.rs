#[doc = "Register `SWREG77_H264_ERROR_E` reader"]
pub type R = crate::R<Swreg77H264ErrorESpec>;
#[doc = "Register `SWREG77_H264_ERROR_E` writer"]
pub type W = crate::W<Swreg77H264ErrorESpec>;
#[doc = "Field `SW_H264_ERROR_EN_HIGHBITS` reader - h264 error enable high bits\n\nh264 error enable bits"]
pub type SwH264ErrorEnHighbitsR = crate::FieldReader<u32>;
#[doc = "Field `SW_H264_ERROR_EN_HIGHBITS` writer - h264 error enable high bits\n\nh264 error enable bits"]
pub type SwH264ErrorEnHighbitsW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - h264 error enable high bits\n\nh264 error enable bits"]
    #[inline(always)]
    pub fn sw_h264_error_en_highbits(&self) -> SwH264ErrorEnHighbitsR {
        SwH264ErrorEnHighbitsR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - h264 error enable high bits\n\nh264 error enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn sw_h264_error_en_highbits(&mut self) -> SwH264ErrorEnHighbitsW<Swreg77H264ErrorESpec> {
        SwH264ErrorEnHighbitsW::new(self, 0)
    }
}
#[doc = "h264 error enable high bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg77_h264_error_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg77_h264_error_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg77H264ErrorESpec;
impl crate::RegisterSpec for Swreg77H264ErrorESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg77_h264_error_e::R`](R) reader structure"]
impl crate::Readable for Swreg77H264ErrorESpec {}
#[doc = "`write(|w| ..)` method takes [`swreg77_h264_error_e::W`](W) writer structure"]
impl crate::Writable for Swreg77H264ErrorESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG77_H264_ERROR_E to value 0"]
impl crate::Resettable for Swreg77H264ErrorESpec {
    const RESET_VALUE: u32 = 0;
}
