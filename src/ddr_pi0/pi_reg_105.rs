#[doc = "Register `PI_REG_105` reader"]
pub type R = crate::R<PiReg105Spec>;
#[doc = "Register `PI_REG_105` writer"]
pub type W = crate::W<PiReg105Spec>;
#[doc = "Field `PI_TDFI_CACSCA_F0` reader - Indicates DFI tcalvl_cs_ca timing parameter. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCacscaF0R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CACSCA_F0` writer - Indicates DFI tcalvl_cs_ca timing parameter. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCacscaF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TDFI_CASEL_F0` reader - Indicates DFI tcalvl_ca_sel timing parameter. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCaselF0R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CASEL_F0` writer - Indicates DFI tcalvl_ca_sel timing parameter. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCaselF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TVREF_SHORT_F0` reader - Indicates delay from dfi_calvl_strobe to next CMD (only param_calvl_vref_stepsize change of the VREF). The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTvrefShortF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_SHORT_F0` writer - Indicates delay from dfi_calvl_strobe to next CMD (only param_calvl_vref_stepsize change of the VREF). The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTvrefShortF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:4 - Indicates DFI tcalvl_cs_ca timing parameter. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_cacsca_f0(&self) -> PiTdfiCacscaF0R {
        PiTdfiCacscaF0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Indicates DFI tcalvl_ca_sel timing parameter. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_casel_f0(&self) -> PiTdfiCaselF0R {
        PiTdfiCaselF0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:25 - Indicates delay from dfi_calvl_strobe to next CMD (only param_calvl_vref_stepsize change of the VREF). The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tvref_short_f0(&self) -> PiTvrefShortF0R {
        PiTvrefShortF0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Indicates DFI tcalvl_cs_ca timing parameter. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_cacsca_f0(&mut self) -> PiTdfiCacscaF0W<PiReg105Spec> {
        PiTdfiCacscaF0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Indicates DFI tcalvl_ca_sel timing parameter. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_casel_f0(&mut self) -> PiTdfiCaselF0W<PiReg105Spec> {
        PiTdfiCaselF0W::new(self, 8)
    }
    #[doc = "Bits 16:25 - Indicates delay from dfi_calvl_strobe to next CMD (only param_calvl_vref_stepsize change of the VREF). The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_short_f0(&mut self) -> PiTvrefShortF0W<PiReg105Spec> {
        PiTvrefShortF0W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 105\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_105::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_105::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg105Spec;
impl crate::RegisterSpec for PiReg105Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_105::R`](R) reader structure"]
impl crate::Readable for PiReg105Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_105::W`](W) writer structure"]
impl crate::Writable for PiReg105Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_105 to value 0"]
impl crate::Resettable for PiReg105Spec {
    const RESET_VALUE: u32 = 0;
}
