#[doc = "Register `SWREG_106_REUSE` reader"]
pub type R = crate::R<Swreg106ReuseSpec>;
#[doc = "Register `SWREG_106_REUSE` writer"]
pub type W = crate::W<Swreg106ReuseSpec>;
#[doc = "Field `FRAME_NUM` reader - H.264 frame number"]
pub type FrameNumR = crate::FieldReader<u16>;
#[doc = "Field `FRAME_NUM` writer - H.264 frame number"]
pub type FrameNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INTRA_PRED_MODE` reader - intra prediction previous fpr 4x4 mode favor used in h264\n\nH.264 intra prediction previous 4x4 mode favor"]
pub type IntraPredModeR = crate::FieldReader;
#[doc = "Field `INTRA_PRED_MODE` writer - intra prediction previous fpr 4x4 mode favor used in h264\n\nH.264 intra prediction previous 4x4 mode favor"]
pub type IntraPredModeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PIC_PARA_ID` reader - H.264 picture parameter id set"]
pub type PicParaIdR = crate::FieldReader;
#[doc = "Field `PIC_PARA_ID` writer - H.264 picture parameter id set"]
pub type PicParaIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - H.264 frame number"]
    #[inline(always)]
    pub fn frame_num(&self) -> FrameNumR {
        FrameNumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - intra prediction previous fpr 4x4 mode favor used in h264\n\nH.264 intra prediction previous 4x4 mode favor"]
    #[inline(always)]
    pub fn intra_pred_mode(&self) -> IntraPredModeR {
        IntraPredModeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - H.264 picture parameter id set"]
    #[inline(always)]
    pub fn pic_para_id(&self) -> PicParaIdR {
        PicParaIdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - H.264 frame number"]
    #[inline(always)]
    #[must_use]
    pub fn frame_num(&mut self) -> FrameNumW<Swreg106ReuseSpec> {
        FrameNumW::new(self, 0)
    }
    #[doc = "Bits 16:23 - intra prediction previous fpr 4x4 mode favor used in h264\n\nH.264 intra prediction previous 4x4 mode favor"]
    #[inline(always)]
    #[must_use]
    pub fn intra_pred_mode(&mut self) -> IntraPredModeW<Swreg106ReuseSpec> {
        IntraPredModeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - H.264 picture parameter id set"]
    #[inline(always)]
    #[must_use]
    pub fn pic_para_id(&mut self) -> PicParaIdW<Swreg106ReuseSpec> {
        PicParaIdW::new(self, 24)
    }
}
#[doc = "encoder control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_106_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_106_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg106ReuseSpec;
impl crate::RegisterSpec for Swreg106ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_106_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg106ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_106_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg106ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_106_REUSE to value 0"]
impl crate::Resettable for Swreg106ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
