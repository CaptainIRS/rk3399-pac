#[doc = "Register `ENH_YUV_CNFG_0` reader"]
pub type R = crate::R<EnhYuvCnfg0Spec>;
#[doc = "Register `ENH_YUV_CNFG_0` writer"]
pub type W = crate::W<EnhYuvCnfg0Spec>;
#[doc = "Field `BRIGHTNESS` reader - YUV brightness adjustment\n\nrange from -32 to 31\n\n000000:0;\n\n000001:1;\n\n......\n\n011111:31;\n\n100000:-32;\n\n100001:-31;\n\n......\n\n111110:-2;\n\n111111:-1;"]
pub type BrightnessR = crate::FieldReader;
#[doc = "Field `BRIGHTNESS` writer - YUV brightness adjustment\n\nrange from -32 to 31\n\n000000:0;\n\n000001:1;\n\n......\n\n011111:31;\n\n100000:-32;\n\n100001:-31;\n\n......\n\n111110:-2;\n\n111111:-1;"]
pub type BrightnessW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CONTRAST` reader - YUV contrast adjustment\n\ncontrast value range from 0 to 1.992, and this value is\n\ncontrast*128."]
pub type ContrastR = crate::FieldReader;
#[doc = "Field `CONTRAST` writer - YUV contrast adjustment\n\ncontrast value range from 0 to 1.992, and this value is\n\ncontrast*128."]
pub type ContrastW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAT_CON` reader - YUV saturation and contrast adjustment\n\nsaturation * contrast range from 0 to 1.992*1.992, and this value\n\nis saturation* contrast * 128"]
pub type SatConR = crate::FieldReader<u16>;
#[doc = "Field `SAT_CON` writer - YUV saturation and contrast adjustment\n\nsaturation * contrast range from 0 to 1.992*1.992, and this value\n\nis saturation* contrast * 128"]
pub type SatConW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
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
impl W {
    #[doc = "Bits 0:5 - YUV brightness adjustment\n\nrange from -32 to 31\n\n000000:0;\n\n000001:1;\n\n......\n\n011111:31;\n\n100000:-32;\n\n100001:-31;\n\n......\n\n111110:-2;\n\n111111:-1;"]
    #[inline(always)]
    #[must_use]
    pub fn brightness(&mut self) -> BrightnessW<EnhYuvCnfg0Spec> {
        BrightnessW::new(self, 0)
    }
    #[doc = "Bits 8:15 - YUV contrast adjustment\n\ncontrast value range from 0 to 1.992, and this value is\n\ncontrast*128."]
    #[inline(always)]
    #[must_use]
    pub fn contrast(&mut self) -> ContrastW<EnhYuvCnfg0Spec> {
        ContrastW::new(self, 8)
    }
    #[doc = "Bits 16:24 - YUV saturation and contrast adjustment\n\nsaturation * contrast range from 0 to 1.992*1.992, and this value\n\nis saturation* contrast * 128"]
    #[inline(always)]
    #[must_use]
    pub fn sat_con(&mut self) -> SatConW<EnhYuvCnfg0Spec> {
        SatConW::new(self, 16)
    }
}
#[doc = "brightness,contrast,saturation adjustment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_yuv_cnfg_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_yuv_cnfg_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnhYuvCnfg0Spec;
impl crate::RegisterSpec for EnhYuvCnfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enh_yuv_cnfg_0::R`](R) reader structure"]
impl crate::Readable for EnhYuvCnfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`enh_yuv_cnfg_0::W`](W) writer structure"]
impl crate::Writable for EnhYuvCnfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENH_YUV_CNFG_0 to value 0"]
impl crate::Resettable for EnhYuvCnfg0Spec {
    const RESET_VALUE: u32 = 0;
}
