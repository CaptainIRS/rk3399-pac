#[doc = "Register `SWREG109` reader"]
pub type R = crate::R<Swreg109Spec>;
#[doc = "Register `SWREG109` writer"]
pub type W = crate::W<Swreg109Spec>;
#[doc = "Field `H264_STRM_START_BIT` reader - the stream start word for decoder\n\nassosiates with sw_rlc_vlc_st_adr"]
pub type H264StrmStartBitR = crate::FieldReader;
#[doc = "Field `H264_STRM_START_BIT` writer - the stream start word for decoder\n\nassosiates with sw_rlc_vlc_st_adr"]
pub type H264StrmStartBitW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - the stream start word for decoder\n\nassosiates with sw_rlc_vlc_st_adr"]
    #[inline(always)]
    pub fn h264_strm_start_bit(&self) -> H264StrmStartBitR {
        H264StrmStartBitR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - the stream start word for decoder\n\nassosiates with sw_rlc_vlc_st_adr"]
    #[inline(always)]
    #[must_use]
    pub fn h264_strm_start_bit(&mut self) -> H264StrmStartBitW<Swreg109Spec> {
        H264StrmStartBitW::new(self, 0)
    }
}
#[doc = "the stream start word for decoder\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg109::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg109::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg109Spec;
impl crate::RegisterSpec for Swreg109Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg109::R`](R) reader structure"]
impl crate::Readable for Swreg109Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg109::W`](W) writer structure"]
impl crate::Writable for Swreg109Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG109 to value 0"]
impl crate::Resettable for Swreg109Spec {
    const RESET_VALUE: u32 = 0;
}
