#[doc = "Register `WIN1_YUV2YUV_Y2R_COE3` reader"]
pub type R = crate::R<Win1Yuv2yuvY2rCoe3Spec>;
#[doc = "Register `WIN1_YUV2YUV_Y2R_COE3` writer"]
pub type W = crate::W<Win1Yuv2yuvY2rCoe3Spec>;
#[doc = "Field `CSC_COE20` reader - coefficient of 3x4 matrix"]
pub type CscCoe20R = crate::FieldReader<u16>;
#[doc = "Field `CSC_COE20` writer - coefficient of 3x4 matrix"]
pub type CscCoe20W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CSC_COE21` reader - coefficient of 3x4 matrix"]
pub type CscCoe21R = crate::FieldReader<u16>;
#[doc = "Field `CSC_COE21` writer - coefficient of 3x4 matrix"]
pub type CscCoe21W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - coefficient of 3x4 matrix"]
    #[inline(always)]
    pub fn csc_coe20(&self) -> CscCoe20R {
        CscCoe20R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - coefficient of 3x4 matrix"]
    #[inline(always)]
    pub fn csc_coe21(&self) -> CscCoe21R {
        CscCoe21R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - coefficient of 3x4 matrix"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coe20(&mut self) -> CscCoe20W<Win1Yuv2yuvY2rCoe3Spec> {
        CscCoe20W::new(self, 0)
    }
    #[doc = "Bits 16:31 - coefficient of 3x4 matrix"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coe21(&mut self) -> CscCoe21W<Win1Yuv2yuvY2rCoe3Spec> {
        CscCoe21W::new(self, 16)
    }
}
#[doc = "WIN1 yuv2yuv y2r cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_yuv2yuv_y2r_coe3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_yuv2yuv_y2r_coe3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1Yuv2yuvY2rCoe3Spec;
impl crate::RegisterSpec for Win1Yuv2yuvY2rCoe3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_yuv2yuv_y2r_coe3::R`](R) reader structure"]
impl crate::Readable for Win1Yuv2yuvY2rCoe3Spec {}
#[doc = "`write(|w| ..)` method takes [`win1_yuv2yuv_y2r_coe3::W`](W) writer structure"]
impl crate::Writable for Win1Yuv2yuvY2rCoe3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_YUV2YUV_Y2R_COE3 to value 0"]
impl crate::Resettable for Win1Yuv2yuvY2rCoe3Spec {
    const RESET_VALUE: u32 = 0;
}
