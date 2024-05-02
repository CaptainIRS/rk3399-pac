#[doc = "Register `AFM_VAR_SHIFT` reader"]
pub type R = crate::R<AfmVarShiftSpec>;
#[doc = "Register `AFM_VAR_SHIFT` writer"]
pub type W = crate::W<AfmVarShiftSpec>;
#[doc = "Field `afm_var_shift` reader - variable shift for AF measurement\n\nThe afm_var_shift defines the number of bits for the\n\nshift operation at the end of the calculation chain. The shift\n\noperation is used to avoid an AF measurement sum\n\noverflow."]
pub type AfmVarShiftR = crate::FieldReader;
#[doc = "Field `afm_var_shift` writer - variable shift for AF measurement\n\nThe afm_var_shift defines the number of bits for the\n\nshift operation at the end of the calculation chain. The shift\n\noperation is used to avoid an AF measurement sum\n\noverflow."]
pub type AfmVarShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `lum_var_shift` reader - variable shift for luminance summation\n\nThe lum_var_shift defines the number of bits for the\n\nshift operation of the value of the current pixel before\n\nsummation. The shift operation is used to avoid a\n\nluminance sum overflow."]
pub type LumVarShiftR = crate::FieldReader;
#[doc = "Field `lum_var_shift` writer - variable shift for luminance summation\n\nThe lum_var_shift defines the number of bits for the\n\nshift operation of the value of the current pixel before\n\nsummation. The shift operation is used to avoid a\n\nluminance sum overflow."]
pub type LumVarShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - variable shift for AF measurement\n\nThe afm_var_shift defines the number of bits for the\n\nshift operation at the end of the calculation chain. The shift\n\noperation is used to avoid an AF measurement sum\n\noverflow."]
    #[inline(always)]
    pub fn afm_var_shift(&self) -> AfmVarShiftR {
        AfmVarShiftR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - variable shift for luminance summation\n\nThe lum_var_shift defines the number of bits for the\n\nshift operation of the value of the current pixel before\n\nsummation. The shift operation is used to avoid a\n\nluminance sum overflow."]
    #[inline(always)]
    pub fn lum_var_shift(&self) -> LumVarShiftR {
        LumVarShiftR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - variable shift for AF measurement\n\nThe afm_var_shift defines the number of bits for the\n\nshift operation at the end of the calculation chain. The shift\n\noperation is used to avoid an AF measurement sum\n\noverflow."]
    #[inline(always)]
    #[must_use]
    pub fn afm_var_shift(&mut self) -> AfmVarShiftW<AfmVarShiftSpec> {
        AfmVarShiftW::new(self, 0)
    }
    #[doc = "Bits 16:18 - variable shift for luminance summation\n\nThe lum_var_shift defines the number of bits for the\n\nshift operation of the value of the current pixel before\n\nsummation. The shift operation is used to avoid a\n\nluminance sum overflow."]
    #[inline(always)]
    #[must_use]
    pub fn lum_var_shift(&mut self) -> LumVarShiftW<AfmVarShiftSpec> {
        LumVarShiftW::new(self, 16)
    }
}
#[doc = "Variable shift register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_var_shift::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_var_shift::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmVarShiftSpec;
impl crate::RegisterSpec for AfmVarShiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_var_shift::R`](R) reader structure"]
impl crate::Readable for AfmVarShiftSpec {}
#[doc = "`write(|w| ..)` method takes [`afm_var_shift::W`](W) writer structure"]
impl crate::Writable for AfmVarShiftSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFM_VAR_SHIFT to value 0"]
impl crate::Resettable for AfmVarShiftSpec {
    const RESET_VALUE: u32 = 0;
}
