#[doc = "Register `PI_REG_110` reader"]
pub type R = crate::R<PiReg110Spec>;
#[doc = "Register `PI_REG_110` writer"]
pub type W = crate::W<PiReg110Spec>;
#[doc = "Field `PI_CALVL_VREF_INITIAL_STEPSIZE` reader - Indicates the adjust step for the Vref(ca) training."]
pub type PiCalvlVrefInitialStepsizeR = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_INITIAL_STEPSIZE` writer - Indicates the adjust step for the Vref(ca) training."]
pub type PiCalvlVrefInitialStepsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_CALVL_VREF_NORMAL_STEPSIZE` reader - Indicates the adjust step for the Vref(ca) training."]
pub type PiCalvlVrefNormalStepsizeR = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_NORMAL_STEPSIZE` writer - Indicates the adjust step for the Vref(ca) training."]
pub type PiCalvlVrefNormalStepsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_CALVL_VREF_DELTA` reader - Indicates the CA VREF adjustment for non-initial CA training."]
pub type PiCalvlVrefDeltaR = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_DELTA` writer - Indicates the CA VREF adjustment for non-initial CA training."]
pub type PiCalvlVrefDeltaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TDFI_INIT_START_MIN` reader - Indicates the minimum number of DFI clocks before dfi_init_start can be driven after a previous command or training event."]
pub type PiTdfiInitStartMinR = crate::FieldReader;
#[doc = "Field `PI_TDFI_INIT_START_MIN` writer - Indicates the minimum number of DFI clocks before dfi_init_start can be driven after a previous command or training event."]
pub type PiTdfiInitStartMinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Indicates the adjust step for the Vref(ca) training."]
    #[inline(always)]
    pub fn pi_calvl_vref_initial_stepsize(&self) -> PiCalvlVrefInitialStepsizeR {
        PiCalvlVrefInitialStepsizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the adjust step for the Vref(ca) training."]
    #[inline(always)]
    pub fn pi_calvl_vref_normal_stepsize(&self) -> PiCalvlVrefNormalStepsizeR {
        PiCalvlVrefNormalStepsizeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the CA VREF adjustment for non-initial CA training."]
    #[inline(always)]
    pub fn pi_calvl_vref_delta(&self) -> PiCalvlVrefDeltaR {
        PiCalvlVrefDeltaR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Indicates the minimum number of DFI clocks before dfi_init_start can be driven after a previous command or training event."]
    #[inline(always)]
    pub fn pi_tdfi_init_start_min(&self) -> PiTdfiInitStartMinR {
        PiTdfiInitStartMinR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates the adjust step for the Vref(ca) training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_initial_stepsize(&mut self) -> PiCalvlVrefInitialStepsizeW<PiReg110Spec> {
        PiCalvlVrefInitialStepsizeW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Indicates the adjust step for the Vref(ca) training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_normal_stepsize(&mut self) -> PiCalvlVrefNormalStepsizeW<PiReg110Spec> {
        PiCalvlVrefNormalStepsizeW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Indicates the CA VREF adjustment for non-initial CA training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_delta(&mut self) -> PiCalvlVrefDeltaW<PiReg110Spec> {
        PiCalvlVrefDeltaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates the minimum number of DFI clocks before dfi_init_start can be driven after a previous command or training event."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_init_start_min(&mut self) -> PiTdfiInitStartMinW<PiReg110Spec> {
        PiTdfiInitStartMinW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 110\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_110::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_110::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg110Spec;
impl crate::RegisterSpec for PiReg110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_110::R`](R) reader structure"]
impl crate::Readable for PiReg110Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_110::W`](W) writer structure"]
impl crate::Writable for PiReg110Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_110 to value 0"]
impl crate::Resettable for PiReg110Spec {
    const RESET_VALUE: u32 = 0;
}
