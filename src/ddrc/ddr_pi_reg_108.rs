#[doc = "Register `DDR_PI_REG_108` reader"]
pub type R = crate::R<DdrPiReg108Spec>;
#[doc = "Register `DDR_PI_REG_108` writer"]
pub type W = crate::W<DdrPiReg108Spec>;
#[doc = "Field `PI_TDFI_CACSCA_F2` reader - Indicates DFI tcalvl_cs_ca timing parameter. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiCacscaF2R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CACSCA_F2` writer - Indicates DFI tcalvl_cs_ca timing parameter. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiCacscaF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TDFI_CASEL_F2` reader - Indicates DFI tcalvl_ca_sel timing parameter. The suffix '_f2' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCaselF2R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CASEL_F2` writer - Indicates DFI tcalvl_ca_sel timing parameter. The suffix '_f2' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCaselF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TVREF_SHORT_F2` reader - Indicates delay from dfi_calvl_strobe to next CMD (only\n\nparam_calvl_vref_stepsize change of the VREF). The suffix '_f2' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiTvrefShortF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_SHORT_F2` writer - Indicates delay from dfi_calvl_strobe to next CMD (only\n\nparam_calvl_vref_stepsize change of the VREF). The suffix '_f2' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiTvrefShortF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:4 - Indicates DFI tcalvl_cs_ca timing parameter. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_cacsca_f2(&self) -> PiTdfiCacscaF2R {
        PiTdfiCacscaF2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Indicates DFI tcalvl_ca_sel timing parameter. The suffix '_f2' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_casel_f2(&self) -> PiTdfiCaselF2R {
        PiTdfiCaselF2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:25 - Indicates delay from dfi_calvl_strobe to next CMD (only\n\nparam_calvl_vref_stepsize change of the VREF). The suffix '_f2' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tvref_short_f2(&self) -> PiTvrefShortF2R {
        PiTvrefShortF2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Indicates DFI tcalvl_cs_ca timing parameter. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_cacsca_f2(&mut self) -> PiTdfiCacscaF2W<DdrPiReg108Spec> {
        PiTdfiCacscaF2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Indicates DFI tcalvl_ca_sel timing parameter. The suffix '_f2' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_casel_f2(&mut self) -> PiTdfiCaselF2W<DdrPiReg108Spec> {
        PiTdfiCaselF2W::new(self, 8)
    }
    #[doc = "Bits 16:25 - Indicates delay from dfi_calvl_strobe to next CMD (only\n\nparam_calvl_vref_stepsize change of the VREF). The suffix '_f2' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_short_f2(&mut self) -> PiTvrefShortF2W<DdrPiReg108Spec> {
        PiTvrefShortF2W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_108::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_108::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg108Spec;
impl crate::RegisterSpec for DdrPiReg108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_108::R`](R) reader structure"]
impl crate::Readable for DdrPiReg108Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_108::W`](W) writer structure"]
impl crate::Writable for DdrPiReg108Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_108 to value 0"]
impl crate::Resettable for DdrPiReg108Spec {
    const RESET_VALUE: u32 = 0;
}
