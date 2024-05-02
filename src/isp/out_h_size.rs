#[doc = "Register `OUT_H_SIZE` reader"]
pub type R = crate::R<OutHSizeSpec>;
#[doc = "Register `OUT_H_SIZE` writer"]
pub type W = crate::W<OutHSizeSpec>;
#[doc = "Field `ISP_OUT_H_SIZE` reader - horizontal picture size in pixel if ISP_MODE is set to\n\n001-(ITU-R BT.656 YUV),\n\n010-( ITU-R BT.601 YUV),\n\n011-( ITU-R BT.601 Bayer RGB),\n\n101-( ITU-R BT.656 Bayer RGB)\n\nonly even numbers are accepted, because complete\n\nquadruples of YUYV(YCbYCr) are needed for the 422\n\noutput. (if an odd size is programmed the value will be\n\ntruncated to even size)\n\n"]
pub type IspOutHSizeR = crate::FieldReader<u16>;
#[doc = "Field `ISP_OUT_H_SIZE` writer - horizontal picture size in pixel if ISP_MODE is set to\n\n001-(ITU-R BT.656 YUV),\n\n010-( ITU-R BT.601 YUV),\n\n011-( ITU-R BT.601 Bayer RGB),\n\n101-( ITU-R BT.656 Bayer RGB)\n\nonly even numbers are accepted, because complete\n\nquadruples of YUYV(YCbYCr) are needed for the 422\n\noutput. (if an odd size is programmed the value will be\n\ntruncated to even size)\n\n"]
pub type IspOutHSizeW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - horizontal picture size in pixel if ISP_MODE is set to\n\n001-(ITU-R BT.656 YUV),\n\n010-( ITU-R BT.601 YUV),\n\n011-( ITU-R BT.601 Bayer RGB),\n\n101-( ITU-R BT.656 Bayer RGB)\n\nonly even numbers are accepted, because complete\n\nquadruples of YUYV(YCbYCr) are needed for the 422\n\noutput. (if an odd size is programmed the value will be\n\ntruncated to even size)\n\n"]
    #[inline(always)]
    pub fn isp_out_h_size(&self) -> IspOutHSizeR {
        IspOutHSizeR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - horizontal picture size in pixel if ISP_MODE is set to\n\n001-(ITU-R BT.656 YUV),\n\n010-( ITU-R BT.601 YUV),\n\n011-( ITU-R BT.601 Bayer RGB),\n\n101-( ITU-R BT.656 Bayer RGB)\n\nonly even numbers are accepted, because complete\n\nquadruples of YUYV(YCbYCr) are needed for the 422\n\noutput. (if an odd size is programmed the value will be\n\ntruncated to even size)\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn isp_out_h_size(&mut self) -> IspOutHSizeW<OutHSizeSpec> {
        IspOutHSizeW::new(self, 0)
    }
}
#[doc = "Output horizontal picture size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_h_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_h_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutHSizeSpec;
impl crate::RegisterSpec for OutHSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_h_size::R`](R) reader structure"]
impl crate::Readable for OutHSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`out_h_size::W`](W) writer structure"]
impl crate::Writable for OutHSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_H_SIZE to value 0x1000"]
impl crate::Resettable for OutHSizeSpec {
    const RESET_VALUE: u32 = 0x1000;
}
