#[doc = "Register `SRC_TR_COLOR0` reader"]
pub type R = crate::R<SrcTrColor0Spec>;
#[doc = "Register `SRC_TR_COLOR0` writer"]
pub type W = crate::W<SrcTrColor0Spec>;
#[doc = "Field `SW_SRC_TRANS_RMIN` reader - source image transparency color R min value"]
pub type SwSrcTransRminR = crate::FieldReader;
#[doc = "Field `SW_SRC_TRANS_RMIN` writer - source image transparency color R min value"]
pub type SwSrcTransRminW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_SRC_TRANS_GMIN` reader - source image transparency color G min value"]
pub type SwSrcTransGminR = crate::FieldReader;
#[doc = "Field `SW_SRC_TRANS_GMIN` writer - source image transparency color G min value"]
pub type SwSrcTransGminW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_SRC_TRANS_BMIN` reader - source image transparency color B min value"]
pub type SwSrcTransBminR = crate::FieldReader;
#[doc = "Field `SW_SRC_TRANS_BMIN` writer - source image transparency color B min value"]
pub type SwSrcTransBminW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_SRC_TRANS_AMIN` reader - source image transparency color A min value"]
pub type SwSrcTransAminR = crate::FieldReader;
#[doc = "Field `SW_SRC_TRANS_AMIN` writer - source image transparency color A min value"]
pub type SwSrcTransAminW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - source image transparency color R min value"]
    #[inline(always)]
    pub fn sw_src_trans_rmin(&self) -> SwSrcTransRminR {
        SwSrcTransRminR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - source image transparency color G min value"]
    #[inline(always)]
    pub fn sw_src_trans_gmin(&self) -> SwSrcTransGminR {
        SwSrcTransGminR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - source image transparency color B min value"]
    #[inline(always)]
    pub fn sw_src_trans_bmin(&self) -> SwSrcTransBminR {
        SwSrcTransBminR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - source image transparency color A min value"]
    #[inline(always)]
    pub fn sw_src_trans_amin(&self) -> SwSrcTransAminR {
        SwSrcTransAminR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - source image transparency color R min value"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_trans_rmin(&mut self) -> SwSrcTransRminW<SrcTrColor0Spec> {
        SwSrcTransRminW::new(self, 0)
    }
    #[doc = "Bits 8:15 - source image transparency color G min value"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_trans_gmin(&mut self) -> SwSrcTransGminW<SrcTrColor0Spec> {
        SwSrcTransGminW::new(self, 8)
    }
    #[doc = "Bits 16:23 - source image transparency color B min value"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_trans_bmin(&mut self) -> SwSrcTransBminW<SrcTrColor0Spec> {
        SwSrcTransBminW::new(self, 16)
    }
    #[doc = "Bits 24:31 - source image transparency color A min value"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_trans_amin(&mut self) -> SwSrcTransAminW<SrcTrColor0Spec> {
        SwSrcTransAminW::new(self, 24)
    }
}
#[doc = "RGA source image transparency color min value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_tr_color0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_tr_color0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcTrColor0Spec;
impl crate::RegisterSpec for SrcTrColor0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_tr_color0::R`](R) reader structure"]
impl crate::Readable for SrcTrColor0Spec {}
#[doc = "`write(|w| ..)` method takes [`src_tr_color0::W`](W) writer structure"]
impl crate::Writable for SrcTrColor0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_TR_COLOR0 to value 0"]
impl crate::Resettable for SrcTrColor0Spec {
    const RESET_VALUE: u32 = 0;
}
