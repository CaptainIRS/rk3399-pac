#[doc = "Register `SWREG_55` reader"]
pub type R = crate::R<Swreg55Spec>;
#[doc = "Register `SWREG_55` writer"]
pub type W = crate::W<Swreg55Spec>;
#[doc = "Field `QP_ADJST` reader - QP adjustment for mad\n\nsigned register;\n\nrange from -8 to 7"]
pub type QpAdjstR = crate::FieldReader;
#[doc = "Field `QP_ADJST` writer - QP adjustment for mad\n\nsigned register;\n\nrange from -8 to 7"]
pub type QpAdjstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ROI_DLT_QP2` reader - 2st for delta qp for roi\n\n2st for delta qp for roi"]
pub type RoiDltQp2R = crate::FieldReader;
#[doc = "Field `ROI_DLT_QP2` writer - 2st for delta qp for roi\n\n2st for delta qp for roi"]
pub type RoiDltQp2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ROI_DLT_QP1` reader - 1st for delta qp for roi\n\n1st for delta qp for roi"]
pub type RoiDltQp1R = crate::FieldReader;
#[doc = "Field `ROI_DLT_QP1` writer - 1st for delta qp for roi\n\n1st for delta qp for roi"]
pub type RoiDltQp1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - QP adjustment for mad\n\nsigned register;\n\nrange from -8 to 7"]
    #[inline(always)]
    pub fn qp_adjst(&self) -> QpAdjstR {
        QpAdjstR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 2st for delta qp for roi\n\n2st for delta qp for roi"]
    #[inline(always)]
    pub fn roi_dlt_qp2(&self) -> RoiDltQp2R {
        RoiDltQp2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 1st for delta qp for roi\n\n1st for delta qp for roi"]
    #[inline(always)]
    pub fn roi_dlt_qp1(&self) -> RoiDltQp1R {
        RoiDltQp1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - QP adjustment for mad\n\nsigned register;\n\nrange from -8 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn qp_adjst(&mut self) -> QpAdjstW<Swreg55Spec> {
        QpAdjstW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 2st for delta qp for roi\n\n2st for delta qp for roi"]
    #[inline(always)]
    #[must_use]
    pub fn roi_dlt_qp2(&mut self) -> RoiDltQp2W<Swreg55Spec> {
        RoiDltQp2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - 1st for delta qp for roi\n\n1st for delta qp for roi"]
    #[inline(always)]
    #[must_use]
    pub fn roi_dlt_qp1(&mut self) -> RoiDltQp1W<Swreg55Spec> {
        RoiDltQp1W::new(self, 12)
    }
}
#[doc = "qp related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_55::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_55::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg55Spec;
impl crate::RegisterSpec for Swreg55Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_55::R`](R) reader structure"]
impl crate::Readable for Swreg55Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_55::W`](W) writer structure"]
impl crate::Writable for Swreg55Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_55 to value 0"]
impl crate::Resettable for Swreg55Spec {
    const RESET_VALUE: u32 = 0;
}
