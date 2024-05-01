#[doc = "Register `SRC_TR_COLOR1` reader"]
pub type R = crate::R<SrcTrColor1Spec>;
#[doc = "Register `SRC_TR_COLOR1` writer"]
pub type W = crate::W<SrcTrColor1Spec>;
#[doc = "Field `SW_SRC_TRANS_RMAX` reader - source image transparency color R max value"]
pub type SwSrcTransRmaxR = crate::FieldReader;
#[doc = "Field `SW_SRC_TRANS_RMAX` writer - source image transparency color R max value"]
pub type SwSrcTransRmaxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_SRC_TRANS_GMAX` reader - source image transparency color G max value"]
pub type SwSrcTransGmaxR = crate::FieldReader;
#[doc = "Field `SW_SRC_TRANS_GMAX` writer - source image transparency color G max value"]
pub type SwSrcTransGmaxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_SRC_TRANS_BMAX` reader - source image transparency color B max value"]
pub type SwSrcTransBmaxR = crate::FieldReader;
#[doc = "Field `SW_SRC_TRANS_BMAX` writer - source image transparency color B max value"]
pub type SwSrcTransBmaxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_SRC_TRANS_AMAX` reader - source image transparency color A max value"]
pub type SwSrcTransAmaxR = crate::FieldReader;
#[doc = "Field `SW_SRC_TRANS_AMAX` writer - source image transparency color A max value"]
pub type SwSrcTransAmaxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - source image transparency color R max value"]
    #[inline(always)]
    pub fn sw_src_trans_rmax(&self) -> SwSrcTransRmaxR {
        SwSrcTransRmaxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - source image transparency color G max value"]
    #[inline(always)]
    pub fn sw_src_trans_gmax(&self) -> SwSrcTransGmaxR {
        SwSrcTransGmaxR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - source image transparency color B max value"]
    #[inline(always)]
    pub fn sw_src_trans_bmax(&self) -> SwSrcTransBmaxR {
        SwSrcTransBmaxR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - source image transparency color A max value"]
    #[inline(always)]
    pub fn sw_src_trans_amax(&self) -> SwSrcTransAmaxR {
        SwSrcTransAmaxR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - source image transparency color R max value"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_trans_rmax(&mut self) -> SwSrcTransRmaxW<SrcTrColor1Spec> {
        SwSrcTransRmaxW::new(self, 0)
    }
    #[doc = "Bits 8:15 - source image transparency color G max value"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_trans_gmax(&mut self) -> SwSrcTransGmaxW<SrcTrColor1Spec> {
        SwSrcTransGmaxW::new(self, 8)
    }
    #[doc = "Bits 16:23 - source image transparency color B max value"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_trans_bmax(&mut self) -> SwSrcTransBmaxW<SrcTrColor1Spec> {
        SwSrcTransBmaxW::new(self, 16)
    }
    #[doc = "Bits 24:31 - source image transparency color A max value"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_trans_amax(&mut self) -> SwSrcTransAmaxW<SrcTrColor1Spec> {
        SwSrcTransAmaxW::new(self, 24)
    }
}
#[doc = "Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_tr_color1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_tr_color1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcTrColor1Spec;
impl crate::RegisterSpec for SrcTrColor1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_tr_color1::R`](R) reader structure"]
impl crate::Readable for SrcTrColor1Spec {}
#[doc = "`write(|w| ..)` method takes [`src_tr_color1::W`](W) writer structure"]
impl crate::Writable for SrcTrColor1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_TR_COLOR1 to value 0"]
impl crate::Resettable for SrcTrColor1Spec {
    const RESET_VALUE: u32 = 0;
}
