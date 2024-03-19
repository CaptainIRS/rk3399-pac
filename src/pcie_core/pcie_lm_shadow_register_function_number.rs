#[doc = "Register `PCIE_LM_SHADOW_REGISTER_FUNCTION_NUMBER` reader"]
pub type R = crate::R<PcieLmShadowRegisterFunctionNumberSpec>;
#[doc = "Register `PCIE_LM_SHADOW_REGISTER_FUNCTION_NUMBER` writer"]
pub type W = crate::W<PcieLmShadowRegisterFunctionNumberSpec>;
#[doc = "Field `SHDW_FUNC_NUM` reader - Shadow register target function number \\[SHDW_FUNC_NUM\\]
The value here will be the target function number when f/w sets any bit in the shadow error register."]
pub type ShdwFuncNumR = crate::FieldReader;
#[doc = "Field `SHDW_FUNC_NUM` writer - Shadow register target function number \\[SHDW_FUNC_NUM\\]
The value here will be the target function number when f/w sets any bit in the shadow error register."]
pub type ShdwFuncNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Shadow register target function number \\[SHDW_FUNC_NUM\\]
The value here will be the target function number when f/w sets any bit in the shadow error register."]
    #[inline(always)]
    pub fn shdw_func_num(&self) -> ShdwFuncNumR {
        ShdwFuncNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shadow register target function number \\[SHDW_FUNC_NUM\\]
The value here will be the target function number when f/w sets any bit in the shadow error register."]
    #[inline(always)]
    #[must_use]
    pub fn shdw_func_num(&mut self) -> ShdwFuncNumW<PcieLmShadowRegisterFunctionNumberSpec> {
        ShdwFuncNumW::new(self, 0)
    }
}
#[doc = "Shadow register function number. Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_shadow_register_function_number::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_shadow_register_function_number::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmShadowRegisterFunctionNumberSpec;
impl crate::RegisterSpec for PcieLmShadowRegisterFunctionNumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_shadow_register_function_number::R`](R) reader structure"]
impl crate::Readable for PcieLmShadowRegisterFunctionNumberSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_shadow_register_function_number::W`](W) writer structure"]
impl crate::Writable for PcieLmShadowRegisterFunctionNumberSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_SHADOW_REGISTER_FUNCTION_NUMBER to value 0"]
impl crate::Resettable for PcieLmShadowRegisterFunctionNumberSpec {
    const RESET_VALUE: u32 = 0;
}
