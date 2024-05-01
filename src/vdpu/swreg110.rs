#[doc = "Register `SWREG110` reader"]
pub type R = crate::R<Swreg110Spec>;
#[doc = "Register `SWREG110` writer"]
pub type W = crate::W<Swreg110Spec>;
#[doc = "Field `H264_PIC_MB_W` reader - Picture width in macroblocks\n\nvalue = ((pixel width + 15) /16)"]
pub type H264PicMbWR = crate::FieldReader<u16>;
#[doc = "Field `H264_PIC_MB_W` writer - Picture width in macroblocks\n\nvalue = ((pixel width + 15) /16)"]
pub type H264PicMbWW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `H264_PIC_MB_H` reader - Picture height in macroblocks\n\nvalue =((pixel height+15)/16).\n\nused for frame or single field size being decoded"]
pub type H264PicMbHR = crate::FieldReader;
#[doc = "Field `H264_PIC_MB_H` writer - Picture height in macroblocks\n\nvalue =((pixel height+15)/16).\n\nused for frame or single field size being decoded"]
pub type H264PicMbHW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `H264_FLT_OFFSET_CB_QP` reader - filter offset of cb qp"]
pub type H264FltOffsetCbQpR = crate::FieldReader;
#[doc = "Field `H264_FLT_OFFSET_CB_QP` writer - filter offset of cb qp"]
pub type H264FltOffsetCbQpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_FLT_OFFSET_CR_QP` reader - filter offset of cr qp"]
pub type H264FltOffsetCrQpR = crate::FieldReader;
#[doc = "Field `H264_FLT_OFFSET_CR_QP` writer - filter offset of cr qp"]
pub type H264FltOffsetCrQpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:8 - Picture width in macroblocks\n\nvalue = ((pixel width + 15) /16)"]
    #[inline(always)]
    pub fn h264_pic_mb_w(&self) -> H264PicMbWR {
        H264PicMbWR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:16 - Picture height in macroblocks\n\nvalue =((pixel height+15)/16).\n\nused for frame or single field size being decoded"]
    #[inline(always)]
    pub fn h264_pic_mb_h(&self) -> H264PicMbHR {
        H264PicMbHR::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 17:21 - filter offset of cb qp"]
    #[inline(always)]
    pub fn h264_flt_offset_cb_qp(&self) -> H264FltOffsetCbQpR {
        H264FltOffsetCbQpR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - filter offset of cr qp"]
    #[inline(always)]
    pub fn h264_flt_offset_cr_qp(&self) -> H264FltOffsetCrQpR {
        H264FltOffsetCrQpR::new(((self.bits >> 22) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Picture width in macroblocks\n\nvalue = ((pixel width + 15) /16)"]
    #[inline(always)]
    #[must_use]
    pub fn h264_pic_mb_w(&mut self) -> H264PicMbWW<Swreg110Spec> {
        H264PicMbWW::new(self, 0)
    }
    #[doc = "Bits 9:16 - Picture height in macroblocks\n\nvalue =((pixel height+15)/16).\n\nused for frame or single field size being decoded"]
    #[inline(always)]
    #[must_use]
    pub fn h264_pic_mb_h(&mut self) -> H264PicMbHW<Swreg110Spec> {
        H264PicMbHW::new(self, 9)
    }
    #[doc = "Bits 17:21 - filter offset of cb qp"]
    #[inline(always)]
    #[must_use]
    pub fn h264_flt_offset_cb_qp(&mut self) -> H264FltOffsetCbQpW<Swreg110Spec> {
        H264FltOffsetCbQpW::new(self, 17)
    }
    #[doc = "Bits 22:26 - filter offset of cr qp"]
    #[inline(always)]
    #[must_use]
    pub fn h264_flt_offset_cr_qp(&mut self) -> H264FltOffsetCrQpW<Swreg110Spec> {
        H264FltOffsetCrQpW::new(self, 22)
    }
}
#[doc = "h264 pic mb size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg110::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg110::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg110Spec;
impl crate::RegisterSpec for Swreg110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg110::R`](R) reader structure"]
impl crate::Readable for Swreg110Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg110::W`](W) writer structure"]
impl crate::Writable for Swreg110Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG110 to value 0"]
impl crate::Resettable for Swreg110Spec {
    const RESET_VALUE: u32 = 0;
}
