#[doc = "Register `SWREG5_STREAM_RLC_LEN` reader"]
pub type R = crate::R<Swreg5StreamRlcLenSpec>;
#[doc = "Register `SWREG5_STREAM_RLC_LEN` writer"]
pub type W = crate::W<Swreg5StreamRlcLenSpec>;
#[doc = "Field `SW_STREAM_LEN` reader - amount of stream (unit is 8bit) in the input buffer\n\namount of stream 8bits in the input buffer\n\nthe max of sw_stream_len : 4096x2304x1.5x1.5 = 0x1440000\n\n128bits unit: 0x1440000/16 = 0x144000\n\nit is count from 0\n\n2013.10.15 change to 23bit for zty's suggestion\n\n2013.10.28, amount of stream data bytes in input buffer.\n\nit is count from 1, change to 27bits"]
pub type SwStreamLenR = crate::FieldReader<u32>;
#[doc = "Field `SW_STREAM_LEN` writer - amount of stream (unit is 8bit) in the input buffer\n\namount of stream 8bits in the input buffer\n\nthe max of sw_stream_len : 4096x2304x1.5x1.5 = 0x1440000\n\n128bits unit: 0x1440000/16 = 0x144000\n\nit is count from 0\n\n2013.10.15 change to 23bit for zty's suggestion\n\n2013.10.28, amount of stream data bytes in input buffer.\n\nit is count from 1, change to 27bits"]
pub type SwStreamLenW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:26 - amount of stream (unit is 8bit) in the input buffer\n\namount of stream 8bits in the input buffer\n\nthe max of sw_stream_len : 4096x2304x1.5x1.5 = 0x1440000\n\n128bits unit: 0x1440000/16 = 0x144000\n\nit is count from 0\n\n2013.10.15 change to 23bit for zty's suggestion\n\n2013.10.28, amount of stream data bytes in input buffer.\n\nit is count from 1, change to 27bits"]
    #[inline(always)]
    pub fn sw_stream_len(&self) -> SwStreamLenR {
        SwStreamLenR::new(self.bits & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:26 - amount of stream (unit is 8bit) in the input buffer\n\namount of stream 8bits in the input buffer\n\nthe max of sw_stream_len : 4096x2304x1.5x1.5 = 0x1440000\n\n128bits unit: 0x1440000/16 = 0x144000\n\nit is count from 0\n\n2013.10.15 change to 23bit for zty's suggestion\n\n2013.10.28, amount of stream data bytes in input buffer.\n\nit is count from 1, change to 27bits"]
    #[inline(always)]
    #[must_use]
    pub fn sw_stream_len(&mut self) -> SwStreamLenW<Swreg5StreamRlcLenSpec> {
        SwStreamLenW::new(self, 0)
    }
}
#[doc = "amount of stream bytes or rlc data byte in the input buffer or the\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg5_stream_rlc_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg5_stream_rlc_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg5StreamRlcLenSpec;
impl crate::RegisterSpec for Swreg5StreamRlcLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg5_stream_rlc_len::R`](R) reader structure"]
impl crate::Readable for Swreg5StreamRlcLenSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg5_stream_rlc_len::W`](W) writer structure"]
impl crate::Writable for Swreg5StreamRlcLenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG5_STREAM_RLC_LEN to value 0"]
impl crate::Resettable for Swreg5StreamRlcLenSpec {
    const RESET_VALUE: u32 = 0;
}
