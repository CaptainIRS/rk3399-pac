#[doc = "Register `DDR_PI_REG_109` reader"]
pub type R = crate::R<DdrPiReg109Spec>;
#[doc = "Register `DDR_PI_REG_109` writer"]
pub type W = crate::W<DdrPiReg109Spec>;
#[doc = "Field `PI_TVREF_LONG_F2` reader - Indicates delay from dfi_calvl_strobe to next CMD (more than one param_calvl_vref_stepsize Vref change). The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTvrefLongF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_LONG_F2` writer - Indicates delay from dfi_calvl_strobe to next CMD (more than one param_calvl_vref_stepsize Vref change). The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTvrefLongF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_CALVL_VREF_INITIAL_START_POINT` reader - Indicates the start point of VREF for the Vref (ca) training vrefca_range, vref_ca_setting\\[5:0\\]."]
pub type PiCalvlVrefInitialStartPointR = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_INITIAL_START_POINT` writer - Indicates the start point of VREF for the Vref (ca) training vrefca_range, vref_ca_setting\\[5:0\\]."]
pub type PiCalvlVrefInitialStartPointW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_CALVL_VREF_INITIAL_STOP_POINT` reader - Indicates the end point of VREF for the Vref(ca) training vrefca_range, vref_ca_setting\\[5:0\\]"]
pub type PiCalvlVrefInitialStopPointR = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_INITIAL_STOP_POINT` writer - Indicates the end point of VREF for the Vref(ca) training vrefca_range, vref_ca_setting\\[5:0\\]"]
pub type PiCalvlVrefInitialStopPointW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:9 - Indicates delay from dfi_calvl_strobe to next CMD (more than one param_calvl_vref_stepsize Vref change). The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tvref_long_f2(&self) -> PiTvrefLongF2R {
        PiTvrefLongF2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:22 - Indicates the start point of VREF for the Vref (ca) training vrefca_range, vref_ca_setting\\[5:0\\]."]
    #[inline(always)]
    pub fn pi_calvl_vref_initial_start_point(&self) -> PiCalvlVrefInitialStartPointR {
        PiCalvlVrefInitialStartPointR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Indicates the end point of VREF for the Vref(ca) training vrefca_range, vref_ca_setting\\[5:0\\]"]
    #[inline(always)]
    pub fn pi_calvl_vref_initial_stop_point(&self) -> PiCalvlVrefInitialStopPointR {
        PiCalvlVrefInitialStopPointR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Indicates delay from dfi_calvl_strobe to next CMD (more than one param_calvl_vref_stepsize Vref change). The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_long_f2(&mut self) -> PiTvrefLongF2W<DdrPiReg109Spec> {
        PiTvrefLongF2W::new(self, 0)
    }
    #[doc = "Bits 16:22 - Indicates the start point of VREF for the Vref (ca) training vrefca_range, vref_ca_setting\\[5:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_initial_start_point(
        &mut self,
    ) -> PiCalvlVrefInitialStartPointW<DdrPiReg109Spec> {
        PiCalvlVrefInitialStartPointW::new(self, 16)
    }
    #[doc = "Bits 24:30 - Indicates the end point of VREF for the Vref(ca) training vrefca_range, vref_ca_setting\\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_initial_stop_point(
        &mut self,
    ) -> PiCalvlVrefInitialStopPointW<DdrPiReg109Spec> {
        PiCalvlVrefInitialStopPointW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 109\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_109::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_109::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg109Spec;
impl crate::RegisterSpec for DdrPiReg109Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_109::R`](R) reader structure"]
impl crate::Readable for DdrPiReg109Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_109::W`](W) writer structure"]
impl crate::Writable for DdrPiReg109Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_109 to value 0"]
impl crate::Resettable for DdrPiReg109Spec {
    const RESET_VALUE: u32 = 0;
}
