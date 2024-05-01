#[doc = "Register `SWREG_82` reader"]
pub type R = crate::R<Swreg82Spec>;
#[doc = "Register `SWREG_82` writer"]
pub type W = crate::W<Swreg82Spec>;
#[doc = "Field `FIRST_ROI_RMB` reader - the right mb column for first roi area\n\nqp=qp - roi1_Delta_Qp\n\n(outside area)"]
pub type FirstRoiRmbR = crate::FieldReader;
#[doc = "Field `FIRST_ROI_RMB` writer - the right mb column for first roi area\n\nqp=qp - roi1_Delta_Qp\n\n(outside area)"]
pub type FirstRoiRmbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FIRST_ROI_LMB` reader - the left mb column for first roi area\n\nqp=qp + roi1_Delta_Qp\n\n(inside area)"]
pub type FirstRoiLmbR = crate::FieldReader;
#[doc = "Field `FIRST_ROI_LMB` writer - the left mb column for first roi area\n\nqp=qp + roi1_Delta_Qp\n\n(inside area)"]
pub type FirstRoiLmbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FIRST_ROI_BMB` reader - the bottom mb column for first roi area\n\n(outside area)"]
pub type FirstRoiBmbR = crate::FieldReader;
#[doc = "Field `FIRST_ROI_BMB` writer - the bottom mb column for first roi area\n\n(outside area)"]
pub type FirstRoiBmbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FIRST_ROI_TMB` reader - the top mb column for first roi area\n\n(inside area)"]
pub type FirstRoiTmbR = crate::FieldReader;
#[doc = "Field `FIRST_ROI_TMB` writer - the top mb column for first roi area\n\n(inside area)"]
pub type FirstRoiTmbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - the right mb column for first roi area\n\nqp=qp - roi1_Delta_Qp\n\n(outside area)"]
    #[inline(always)]
    pub fn first_roi_rmb(&self) -> FirstRoiRmbR {
        FirstRoiRmbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - the left mb column for first roi area\n\nqp=qp + roi1_Delta_Qp\n\n(inside area)"]
    #[inline(always)]
    pub fn first_roi_lmb(&self) -> FirstRoiLmbR {
        FirstRoiLmbR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - the bottom mb column for first roi area\n\n(outside area)"]
    #[inline(always)]
    pub fn first_roi_bmb(&self) -> FirstRoiBmbR {
        FirstRoiBmbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - the top mb column for first roi area\n\n(inside area)"]
    #[inline(always)]
    pub fn first_roi_tmb(&self) -> FirstRoiTmbR {
        FirstRoiTmbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - the right mb column for first roi area\n\nqp=qp - roi1_Delta_Qp\n\n(outside area)"]
    #[inline(always)]
    #[must_use]
    pub fn first_roi_rmb(&mut self) -> FirstRoiRmbW<Swreg82Spec> {
        FirstRoiRmbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - the left mb column for first roi area\n\nqp=qp + roi1_Delta_Qp\n\n(inside area)"]
    #[inline(always)]
    #[must_use]
    pub fn first_roi_lmb(&mut self) -> FirstRoiLmbW<Swreg82Spec> {
        FirstRoiLmbW::new(self, 8)
    }
    #[doc = "Bits 16:23 - the bottom mb column for first roi area\n\n(outside area)"]
    #[inline(always)]
    #[must_use]
    pub fn first_roi_bmb(&mut self) -> FirstRoiBmbW<Swreg82Spec> {
        FirstRoiBmbW::new(self, 16)
    }
    #[doc = "Bits 24:31 - the top mb column for first roi area\n\n(inside area)"]
    #[inline(always)]
    #[must_use]
    pub fn first_roi_tmb(&mut self) -> FirstRoiTmbW<Swreg82Spec> {
        FirstRoiTmbW::new(self, 24)
    }
}
#[doc = "the first of ROI area register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_82::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_82::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg82Spec;
impl crate::RegisterSpec for Swreg82Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_82::R`](R) reader structure"]
impl crate::Readable for Swreg82Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_82::W`](W) writer structure"]
impl crate::Writable for Swreg82Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_82 to value 0"]
impl crate::Resettable for Swreg82Spec {
    const RESET_VALUE: u32 = 0;
}
