#[doc = "Register `SWREG_83` reader"]
pub type R = crate::R<Swreg83Spec>;
#[doc = "Register `SWREG_83` writer"]
pub type W = crate::W<Swreg83Spec>;
#[doc = "Field `SECOND_ROI_TMB` reader - qp=qp - roi1_Delta_Qp\n\n(outside area)"]
pub type SecondRoiTmbR = crate::FieldReader;
#[doc = "Field `SECOND_ROI_TMB` writer - qp=qp - roi1_Delta_Qp\n\n(outside area)"]
pub type SecondRoiTmbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SECOND_ROI_LMB` reader - qp=qp + roi1_Delta_Qp\n\n(inside area)"]
pub type SecondRoiLmbR = crate::FieldReader;
#[doc = "Field `SECOND_ROI_LMB` writer - qp=qp + roi1_Delta_Qp\n\n(inside area)"]
pub type SecondRoiLmbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SECOND_ROI_BMB` reader - (outside area)"]
pub type SecondRoiBmbR = crate::FieldReader;
#[doc = "Field `SECOND_ROI_BMB` writer - (outside area)"]
pub type SecondRoiBmbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SECOND_ROI_RMB` reader - (inside area)"]
pub type SecondRoiRmbR = crate::FieldReader;
#[doc = "Field `SECOND_ROI_RMB` writer - (inside area)"]
pub type SecondRoiRmbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - qp=qp - roi1_Delta_Qp\n\n(outside area)"]
    #[inline(always)]
    pub fn second_roi_tmb(&self) -> SecondRoiTmbR {
        SecondRoiTmbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - qp=qp + roi1_Delta_Qp\n\n(inside area)"]
    #[inline(always)]
    pub fn second_roi_lmb(&self) -> SecondRoiLmbR {
        SecondRoiLmbR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - (outside area)"]
    #[inline(always)]
    pub fn second_roi_bmb(&self) -> SecondRoiBmbR {
        SecondRoiBmbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - (inside area)"]
    #[inline(always)]
    pub fn second_roi_rmb(&self) -> SecondRoiRmbR {
        SecondRoiRmbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - qp=qp - roi1_Delta_Qp\n\n(outside area)"]
    #[inline(always)]
    #[must_use]
    pub fn second_roi_tmb(&mut self) -> SecondRoiTmbW<Swreg83Spec> {
        SecondRoiTmbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - qp=qp + roi1_Delta_Qp\n\n(inside area)"]
    #[inline(always)]
    #[must_use]
    pub fn second_roi_lmb(&mut self) -> SecondRoiLmbW<Swreg83Spec> {
        SecondRoiLmbW::new(self, 8)
    }
    #[doc = "Bits 16:23 - (outside area)"]
    #[inline(always)]
    #[must_use]
    pub fn second_roi_bmb(&mut self) -> SecondRoiBmbW<Swreg83Spec> {
        SecondRoiBmbW::new(self, 16)
    }
    #[doc = "Bits 24:31 - (inside area)"]
    #[inline(always)]
    #[must_use]
    pub fn second_roi_rmb(&mut self) -> SecondRoiRmbW<Swreg83Spec> {
        SecondRoiRmbW::new(self, 24)
    }
}
#[doc = "the second of ROI area register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_83::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_83::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg83Spec;
impl crate::RegisterSpec for Swreg83Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_83::R`](R) reader structure"]
impl crate::Readable for Swreg83Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_83::W`](W) writer structure"]
impl crate::Writable for Swreg83Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_83 to value 0"]
impl crate::Resettable for Swreg83Spec {
    const RESET_VALUE: u32 = 0;
}
