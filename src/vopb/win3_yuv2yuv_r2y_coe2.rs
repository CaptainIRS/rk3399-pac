#[doc = "Register `WIN3_YUV2YUV_R2Y_COE2` reader"]
pub type R = crate::R<Win3Yuv2yuvR2yCoe2Spec>;
#[doc = "Register `WIN3_YUV2YUV_R2Y_COE2` writer"]
pub type W = crate::W<Win3Yuv2yuvR2yCoe2Spec>;
#[doc = "Field `CSC_COE11` reader - coefficient of 3x4 matrix"]
pub type CscCoe11R = crate::FieldReader<u16>;
#[doc = "Field `CSC_COE11` writer - coefficient of 3x4 matrix"]
pub type CscCoe11W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CSC_COE12` reader - coefficient of 3x4 matrix"]
pub type CscCoe12R = crate::FieldReader<u16>;
#[doc = "Field `CSC_COE12` writer - coefficient of 3x4 matrix"]
pub type CscCoe12W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - coefficient of 3x4 matrix"]
    #[inline(always)]
    pub fn csc_coe11(&self) -> CscCoe11R {
        CscCoe11R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - coefficient of 3x4 matrix"]
    #[inline(always)]
    pub fn csc_coe12(&self) -> CscCoe12R {
        CscCoe12R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - coefficient of 3x4 matrix"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coe11(&mut self) -> CscCoe11W<Win3Yuv2yuvR2yCoe2Spec> {
        CscCoe11W::new(self, 0)
    }
    #[doc = "Bits 16:31 - coefficient of 3x4 matrix"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coe12(&mut self) -> CscCoe12W<Win3Yuv2yuvR2yCoe2Spec> {
        CscCoe12W::new(self, 16)
    }
}
#[doc = "WIN3 yuv2yuv r2y cofficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_yuv2yuv_r2y_coe2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_yuv2yuv_r2y_coe2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3Yuv2yuvR2yCoe2Spec;
impl crate::RegisterSpec for Win3Yuv2yuvR2yCoe2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_yuv2yuv_r2y_coe2::R`](R) reader structure"]
impl crate::Readable for Win3Yuv2yuvR2yCoe2Spec {}
#[doc = "`write(|w| ..)` method takes [`win3_yuv2yuv_r2y_coe2::W`](W) writer structure"]
impl crate::Writable for Win3Yuv2yuvR2yCoe2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_YUV2YUV_R2Y_COE2 to value 0"]
impl crate::Resettable for Win3Yuv2yuvR2yCoe2Spec {
    const RESET_VALUE: u32 = 0;
}
