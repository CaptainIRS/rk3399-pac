#[doc = "Register `RAW_ENH_YUV_CNFG_1` reader"]
pub type R = crate::R<RawEnhYuvCnfg1Spec>;
#[doc = "Field `SIN_HUE` reader - the sin function value for hue adjustment\n\nsin function value range from -0.5 to 0.5 ,and this value is sin *\n\n128 ,and the high bit is sign bit"]
pub type SinHueR = crate::FieldReader;
#[doc = "Field `COS_HUE` reader - the cos function value for hue adjustment\n\nsin function value range from 0.866 to 1 ,and this value is cos *\n\n128 ,no sign bit"]
pub type CosHueR = crate::FieldReader;
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
#[doc = "Hue configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_enh_yuv_cnfg_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawEnhYuvCnfg1Spec;
impl crate::RegisterSpec for RawEnhYuvCnfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_enh_yuv_cnfg_1::R`](R) reader structure"]
impl crate::Readable for RawEnhYuvCnfg1Spec {}
#[doc = "`reset()` method sets RAW_ENH_YUV_CNFG_1 to value 0"]
impl crate::Resettable for RawEnhYuvCnfg1Spec {
    const RESET_VALUE: u32 = 0;
}
