#[doc = "Register `SWREG_100_REUSE` reader"]
pub type R = crate::R<Swreg100ReuseSpec>;
#[doc = "Register `SWREG_100_REUSE` writer"]
pub type W = crate::W<Swreg100ReuseSpec>;
#[doc = "Field `H264_CHKPT_DISTANCE` reader - checkpoint distance for macro block\n\ncheckpoint distance for macro block"]
pub type H264ChkptDistanceR = crate::FieldReader<u16>;
#[doc = "Field `H264_CHKPT_DISTANCE` writer - checkpoint distance for macro block\n\ncheckpoint distance for macro block"]
pub type H264ChkptDistanceW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `H264_MIN_QP` reader - Minimum QP\n\nrange:0~51"]
pub type H264MinQpR = crate::FieldReader;
#[doc = "Field `H264_MIN_QP` writer - Minimum QP\n\nrange:0~51"]
pub type H264MinQpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `H264_MAX_QP` reader - H.264 Minimum QP\n\nrange : 0~51"]
pub type H264MaxQpR = crate::FieldReader;
#[doc = "Field `H264_MAX_QP` writer - H.264 Minimum QP\n\nrange : 0~51"]
pub type H264MaxQpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `H264_INIT_LUMA_QP` reader - Initial luma qp used in h264\n\nrange: 0~51"]
pub type H264InitLumaQpR = crate::FieldReader;
#[doc = "Field `H264_INIT_LUMA_QP` writer - Initial luma qp used in h264\n\nrange: 0~51"]
pub type H264InitLumaQpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:12 - checkpoint distance for macro block\n\ncheckpoint distance for macro block"]
    #[inline(always)]
    pub fn h264_chkpt_distance(&self) -> H264ChkptDistanceR {
        H264ChkptDistanceR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 14:19 - Minimum QP\n\nrange:0~51"]
    #[inline(always)]
    pub fn h264_min_qp(&self) -> H264MinQpR {
        H264MinQpR::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - H.264 Minimum QP\n\nrange : 0~51"]
    #[inline(always)]
    pub fn h264_max_qp(&self) -> H264MaxQpR {
        H264MaxQpR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - Initial luma qp used in h264\n\nrange: 0~51"]
    #[inline(always)]
    pub fn h264_init_luma_qp(&self) -> H264InitLumaQpR {
        H264InitLumaQpR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - checkpoint distance for macro block\n\ncheckpoint distance for macro block"]
    #[inline(always)]
    #[must_use]
    pub fn h264_chkpt_distance(&mut self) -> H264ChkptDistanceW<Swreg100ReuseSpec> {
        H264ChkptDistanceW::new(self, 0)
    }
    #[doc = "Bits 14:19 - Minimum QP\n\nrange:0~51"]
    #[inline(always)]
    #[must_use]
    pub fn h264_min_qp(&mut self) -> H264MinQpW<Swreg100ReuseSpec> {
        H264MinQpW::new(self, 14)
    }
    #[doc = "Bits 20:25 - H.264 Minimum QP\n\nrange : 0~51"]
    #[inline(always)]
    #[must_use]
    pub fn h264_max_qp(&mut self) -> H264MaxQpW<Swreg100ReuseSpec> {
        H264MaxQpW::new(self, 20)
    }
    #[doc = "Bits 26:31 - Initial luma qp used in h264\n\nrange: 0~51"]
    #[inline(always)]
    #[must_use]
    pub fn h264_init_luma_qp(&mut self) -> H264InitLumaQpW<Swreg100ReuseSpec> {
        H264InitLumaQpW::new(self, 26)
    }
}
#[doc = "QP register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_100_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_100_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg100ReuseSpec;
impl crate::RegisterSpec for Swreg100ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_100_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg100ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_100_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg100ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_100_REUSE to value 0"]
impl crate::Resettable for Swreg100ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
