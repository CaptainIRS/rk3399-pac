#[doc = "Register `SWREG4` reader"]
pub type R = crate::R<Swreg4Spec>;
#[doc = "Register `SWREG4` writer"]
pub type W = crate::W<Swreg4Spec>;
#[doc = "Field `SW_SCL_FCT_W` reader - scaling factor of width\n\nvalue = (output_width-1)/(input_width-1)"]
pub type SwSclFctWR = crate::FieldReader<u32>;
#[doc = "Field `SW_SCL_FCT_W` writer - scaling factor of width\n\nvalue = (output_width-1)/(input_width-1)"]
pub type SwSclFctWW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SW_SCL_MODE_HRZ` reader - to sellect scaling mode for Horizontal\n\n0 = no scl\n\n1 = up scl\n\n2 = down scl"]
pub type SwSclModeHrzR = crate::FieldReader;
#[doc = "Field `SW_SCL_MODE_HRZ` writer - to sellect scaling mode for Horizontal\n\n0 = no scl\n\n1 = up scl\n\n2 = down scl"]
pub type SwSclModeHrzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_SCL_MODE_VRT` reader - 0 = no scl\n\n1 = up scl\n\n2 = down scl"]
pub type SwSclModeVrtR = crate::FieldReader;
#[doc = "Field `SW_SCL_MODE_VRT` writer - 0 = no scl\n\n1 = up scl\n\n2 = down scl"]
pub type SwSclModeVrtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:17 - scaling factor of width\n\nvalue = (output_width-1)/(input_width-1)"]
    #[inline(always)]
    pub fn sw_scl_fct_w(&self) -> SwSclFctWR {
        SwSclFctWR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 20:21 - to sellect scaling mode for Horizontal\n\n0 = no scl\n\n1 = up scl\n\n2 = down scl"]
    #[inline(always)]
    pub fn sw_scl_mode_hrz(&self) -> SwSclModeHrzR {
        SwSclModeHrzR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - 0 = no scl\n\n1 = up scl\n\n2 = down scl"]
    #[inline(always)]
    pub fn sw_scl_mode_vrt(&self) -> SwSclModeVrtR {
        SwSclModeVrtR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - scaling factor of width\n\nvalue = (output_width-1)/(input_width-1)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scl_fct_w(&mut self) -> SwSclFctWW<Swreg4Spec> {
        SwSclFctWW::new(self, 0)
    }
    #[doc = "Bits 20:21 - to sellect scaling mode for Horizontal\n\n0 = no scl\n\n1 = up scl\n\n2 = down scl"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scl_mode_hrz(&mut self) -> SwSclModeHrzW<Swreg4Spec> {
        SwSclModeHrzW::new(self, 20)
    }
    #[doc = "Bits 22:23 - 0 = no scl\n\n1 = up scl\n\n2 = down scl"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scl_mode_vrt(&mut self) -> SwSclModeVrtW<Swreg4Spec> {
        SwSclModeVrtW::new(self, 22)
    }
}
#[doc = "scl ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg4Spec;
impl crate::RegisterSpec for Swreg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg4::R`](R) reader structure"]
impl crate::Readable for Swreg4Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg4::W`](W) writer structure"]
impl crate::Writable for Swreg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG4 to value 0"]
impl crate::Resettable for Swreg4Spec {
    const RESET_VALUE: u32 = 0;
}
