#[doc = "Register `DDR_PI_REG_107` reader"]
pub type R = crate::R<DdrPiReg107Spec>;
#[doc = "Register `DDR_PI_REG_107` writer"]
pub type W = crate::W<DdrPiReg107Spec>;
#[doc = "Field `PI_TVREF_SHORT_F1` reader - Indicates delay from dfi_calvl_strobe to next CMD (only param_calvl_vref_stepsize change of the VREF). The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTvrefShortF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_SHORT_F1` writer - Indicates delay from dfi_calvl_strobe to next CMD (only param_calvl_vref_stepsize change of the VREF). The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTvrefShortF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TVREF_LONG_F1` reader - Indicates delay from dfi_calvl_strobe to next CMD (more than one param_calvl_vref_stepsize Vref change). The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTvrefLongF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_LONG_F1` writer - Indicates delay from dfi_calvl_strobe to next CMD (more than one param_calvl_vref_stepsize Vref change). The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTvrefLongF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Indicates delay from dfi_calvl_strobe to next CMD (only param_calvl_vref_stepsize change of the VREF). The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tvref_short_f1(&self) -> PiTvrefShortF1R {
        PiTvrefShortF1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Indicates delay from dfi_calvl_strobe to next CMD (more than one param_calvl_vref_stepsize Vref change). The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tvref_long_f1(&self) -> PiTvrefLongF1R {
        PiTvrefLongF1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Indicates delay from dfi_calvl_strobe to next CMD (only param_calvl_vref_stepsize change of the VREF). The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_short_f1(&mut self) -> PiTvrefShortF1W<DdrPiReg107Spec> {
        PiTvrefShortF1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Indicates delay from dfi_calvl_strobe to next CMD (more than one param_calvl_vref_stepsize Vref change). The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_long_f1(&mut self) -> PiTvrefLongF1W<DdrPiReg107Spec> {
        PiTvrefLongF1W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 107\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_107::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_107::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg107Spec;
impl crate::RegisterSpec for DdrPiReg107Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_107::R`](R) reader structure"]
impl crate::Readable for DdrPiReg107Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_107::W`](W) writer structure"]
impl crate::Writable for DdrPiReg107Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_107 to value 0"]
impl crate::Resettable for DdrPiReg107Spec {
    const RESET_VALUE: u32 = 0;
}
