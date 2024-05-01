#[doc = "Register `RAW_ENH_YUV_CNFG_0` reader"]
pub type R = crate::R<RawEnhYuvCnfg0Spec>;
#[doc = "Field `BRIGHTNESS` reader - YUV brightness adjustment\n\nrange from -32 to 31\n\n000000:0;\n\n000001:1;\n\n......\n\n011111:31;\n\n100000:-32;\n\n100001:-31;\n\n......\n\n111110:-2;\n\n111111:-1;"]
pub type BrightnessR = crate::FieldReader;
#[doc = "Field `CONTRAST` reader - YUV contrast adjustment\n\ncontrast value range from 0 to 1.992, and this value is\n\ncontrast*128."]
pub type ContrastR = crate::FieldReader;
#[doc = "Field `SAT_CON` reader - YUV saturation and contrast adjustment\n\nsaturation * contrast range from 0 to 1.992*1.992, and this value\n\nis saturation* contrast * 128"]
pub type SatConR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:5 - YUV brightness adjustment\n\nrange from -32 to 31\n\n000000:0;\n\n000001:1;\n\n......\n\n011111:31;\n\n100000:-32;\n\n100001:-31;\n\n......\n\n111110:-2;\n\n111111:-1;"]
    #[inline(always)]
    pub fn brightness(&self) -> BrightnessR {
        BrightnessR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - YUV contrast adjustment\n\ncontrast value range from 0 to 1.992, and this value is\n\ncontrast*128."]
    #[inline(always)]
    pub fn contrast(&self) -> ContrastR {
        ContrastR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - YUV saturation and contrast adjustment\n\nsaturation * contrast range from 0 to 1.992*1.992, and this value\n\nis saturation* contrast * 128"]
    #[inline(always)]
    pub fn sat_con(&self) -> SatConR {
        SatConR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
#[doc = "brightness,contrast,saturation adjustment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_enh_yuv_cnfg_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawEnhYuvCnfg0Spec;
impl crate::RegisterSpec for RawEnhYuvCnfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_enh_yuv_cnfg_0::R`](R) reader structure"]
impl crate::Readable for RawEnhYuvCnfg0Spec {}
#[doc = "`reset()` method sets RAW_ENH_YUV_CNFG_0 to value 0"]
impl crate::Resettable for RawEnhYuvCnfg0Spec {
    const RESET_VALUE: u32 = 0;
}
