#[doc = "Register `DDR_PI_REG_120` reader"]
pub type R = crate::R<DdrPiReg120Spec>;
#[doc = "Register `DDR_PI_REG_120` writer"]
pub type W = crate::W<DdrPiReg120Spec>;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_STOP_POINT` reader - Indicates write DQ training VREF stop value."]
pub type PiWdqlvlVrefInitialStopPointR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_STOP_POINT` writer - Indicates write DQ training VREF stop value."]
pub type PiWdqlvlVrefInitialStopPointW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_STEPSIZE` reader - Indicates write DQ training VREF step size."]
pub type PiWdqlvlVrefInitialStepsizeR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_STEPSIZE` writer - Indicates write DQ training VREF step size."]
pub type PiWdqlvlVrefInitialStepsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_WDQLVL_VERF_NORMAL_STEPSIZE` reader - Indicates write DQ training VREF step size."]
pub type PiWdqlvlVerfNormalStepsizeR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VERF_NORMAL_STEPSIZE` writer - Indicates write DQ training VREF step size."]
pub type PiWdqlvlVerfNormalStepsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_WDQLVL_VREF_DELTA` reader - Indicates the WDQ VREF adjustment for non-initial WDQ training."]
pub type PiWdqlvlVrefDeltaR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VREF_DELTA` writer - Indicates the WDQ VREF adjustment for non-initial WDQ training."]
pub type PiWdqlvlVrefDeltaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - Indicates write DQ training VREF stop value."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_initial_stop_point(&self) -> PiWdqlvlVrefInitialStopPointR {
        PiWdqlvlVrefInitialStopPointR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:12 - Indicates write DQ training VREF step size."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_initial_stepsize(&self) -> PiWdqlvlVrefInitialStepsizeR {
        PiWdqlvlVrefInitialStepsizeR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Indicates write DQ training VREF step size."]
    #[inline(always)]
    pub fn pi_wdqlvl_verf_normal_stepsize(&self) -> PiWdqlvlVerfNormalStepsizeR {
        PiWdqlvlVerfNormalStepsizeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the WDQ VREF adjustment for non-initial WDQ training."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_delta(&self) -> PiWdqlvlVrefDeltaR {
        PiWdqlvlVrefDeltaR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Indicates write DQ training VREF stop value."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_initial_stop_point(
        &mut self,
    ) -> PiWdqlvlVrefInitialStopPointW<DdrPiReg120Spec> {
        PiWdqlvlVrefInitialStopPointW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Indicates write DQ training VREF step size."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_initial_stepsize(
        &mut self,
    ) -> PiWdqlvlVrefInitialStepsizeW<DdrPiReg120Spec> {
        PiWdqlvlVrefInitialStepsizeW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Indicates write DQ training VREF step size."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_verf_normal_stepsize(
        &mut self,
    ) -> PiWdqlvlVerfNormalStepsizeW<DdrPiReg120Spec> {
        PiWdqlvlVerfNormalStepsizeW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Indicates the WDQ VREF adjustment for non-initial WDQ training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_delta(&mut self) -> PiWdqlvlVrefDeltaW<DdrPiReg120Spec> {
        PiWdqlvlVrefDeltaW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 120\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_120::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_120::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg120Spec;
impl crate::RegisterSpec for DdrPiReg120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_120::R`](R) reader structure"]
impl crate::Readable for DdrPiReg120Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_120::W`](W) writer structure"]
impl crate::Writable for DdrPiReg120Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_120 to value 0"]
impl crate::Resettable for DdrPiReg120Spec {
    const RESET_VALUE: u32 = 0;
}
