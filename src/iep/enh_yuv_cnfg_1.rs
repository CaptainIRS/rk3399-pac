#[doc = "Register `ENH_YUV_CNFG_1` reader"]
pub type R = crate::R<EnhYuvCnfg1Spec>;
#[doc = "Register `ENH_YUV_CNFG_1` writer"]
pub type W = crate::W<EnhYuvCnfg1Spec>;
#[doc = "Field `SIN_HUE` reader - the sin function value for hue adjustment\n\nsin function value range from -0.5 to 0.5 ,and this value is sin *\n\n128 ,and the high bit is sign bit"]
pub type SinHueR = crate::FieldReader;
#[doc = "Field `SIN_HUE` writer - the sin function value for hue adjustment\n\nsin function value range from -0.5 to 0.5 ,and this value is sin *\n\n128 ,and the high bit is sign bit"]
pub type SinHueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COS_HUE` reader - the cos function value for hue adjustment\n\nsin function value range from 0.866 to 1 ,and this value is cos *\n\n128 ,no sign bit"]
pub type CosHueR = crate::FieldReader;
#[doc = "Field `COS_HUE` writer - the cos function value for hue adjustment\n\nsin function value range from 0.866 to 1 ,and this value is cos *\n\n128 ,no sign bit"]
pub type CosHueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - the sin function value for hue adjustment\n\nsin function value range from -0.5 to 0.5 ,and this value is sin *\n\n128 ,and the high bit is sign bit"]
    #[inline(always)]
    pub fn sin_hue(&self) -> SinHueR {
        SinHueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - the cos function value for hue adjustment\n\nsin function value range from 0.866 to 1 ,and this value is cos *\n\n128 ,no sign bit"]
    #[inline(always)]
    pub fn cos_hue(&self) -> CosHueR {
        CosHueR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - the sin function value for hue adjustment\n\nsin function value range from -0.5 to 0.5 ,and this value is sin *\n\n128 ,and the high bit is sign bit"]
    #[inline(always)]
    #[must_use]
    pub fn sin_hue(&mut self) -> SinHueW<EnhYuvCnfg1Spec> {
        SinHueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - the cos function value for hue adjustment\n\nsin function value range from 0.866 to 1 ,and this value is cos *\n\n128 ,no sign bit"]
    #[inline(always)]
    #[must_use]
    pub fn cos_hue(&mut self) -> CosHueW<EnhYuvCnfg1Spec> {
        CosHueW::new(self, 8)
    }
}
#[doc = "Hue configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_yuv_cnfg_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_yuv_cnfg_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnhYuvCnfg1Spec;
impl crate::RegisterSpec for EnhYuvCnfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enh_yuv_cnfg_1::R`](R) reader structure"]
impl crate::Readable for EnhYuvCnfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`enh_yuv_cnfg_1::W`](W) writer structure"]
impl crate::Writable for EnhYuvCnfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENH_YUV_CNFG_1 to value 0"]
impl crate::Resettable for EnhYuvCnfg1Spec {
    const RESET_VALUE: u32 = 0;
}
