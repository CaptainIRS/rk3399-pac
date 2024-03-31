#[doc = "Register `DDR_PI_REG_106` reader"]
pub type R = crate::R<DdrPiReg106Spec>;
#[doc = "Register `DDR_PI_REG_106` writer"]
pub type W = crate::W<DdrPiReg106Spec>;
#[doc = "Field `PI_TVREF_LONG_F0` reader - Indicates delay from dfi_calvl_strobe to next CMD (more than one\n\nparam_calvl_vref_stepsize Vref change). The suffix '_f0' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTvrefLongF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_LONG_F0` writer - Indicates delay from dfi_calvl_strobe to next CMD (more than one\n\nparam_calvl_vref_stepsize Vref change). The suffix '_f0' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTvrefLongF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TDFI_CACSCA_F1` reader - Indicates DFI tcalvl_cs_ca timing parameter. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiCacscaF1R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CACSCA_F1` writer - Indicates DFI tcalvl_cs_ca timing parameter. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiCacscaF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TDFI_CASEL_F1` reader - Indicates DFI tcalvl_ca_sel timing parameter. The suffix '_f1' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCaselF1R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CASEL_F1` writer - Indicates DFI tcalvl_ca_sel timing parameter. The suffix '_f1' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCaselF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:9 - Indicates delay from dfi_calvl_strobe to next CMD (more than one\n\nparam_calvl_vref_stepsize Vref change). The suffix '_f0' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tvref_long_f0(&self) -> PiTvrefLongF0R {
        PiTvrefLongF0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:20 - Indicates DFI tcalvl_cs_ca timing parameter. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_cacsca_f1(&self) -> PiTdfiCacscaF1R {
        PiTdfiCacscaF1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Indicates DFI tcalvl_ca_sel timing parameter. The suffix '_f1' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_casel_f1(&self) -> PiTdfiCaselF1R {
        PiTdfiCaselF1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Indicates delay from dfi_calvl_strobe to next CMD (more than one\n\nparam_calvl_vref_stepsize Vref change). The suffix '_f0' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_long_f0(&mut self) -> PiTvrefLongF0W<DdrPiReg106Spec> {
        PiTvrefLongF0W::new(self, 0)
    }
    #[doc = "Bits 16:20 - Indicates DFI tcalvl_cs_ca timing parameter. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_cacsca_f1(&mut self) -> PiTdfiCacscaF1W<DdrPiReg106Spec> {
        PiTdfiCacscaF1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Indicates DFI tcalvl_ca_sel timing parameter. The suffix '_f1' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_casel_f1(&mut self) -> PiTdfiCaselF1W<DdrPiReg106Spec> {
        PiTdfiCaselF1W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 106\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_106::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_106::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg106Spec;
impl crate::RegisterSpec for DdrPiReg106Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_106::R`](R) reader structure"]
impl crate::Readable for DdrPiReg106Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_106::W`](W) writer structure"]
impl crate::Writable for DdrPiReg106Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_106 to value 0"]
impl crate::Resettable for DdrPiReg106Spec {
    const RESET_VALUE: u32 = 0;
}
