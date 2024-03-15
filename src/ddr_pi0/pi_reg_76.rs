#[doc = "Register `PI_REG_76` reader"]
pub type R = crate::R<PiReg76Spec>;
#[doc = "Register `PI_REG_76` writer"]
pub type W = crate::W<PiReg76Spec>;
#[doc = "Field `PI_RDLVL_GATE_PERIODIC` reader - Enables the use of the dfi_lvl_periodic signal during gate training. Set to 1 to enable."]
pub type PiRdlvlGatePeriodicR = crate::BitReader;
#[doc = "Field `PI_RDLVL_GATE_PERIODIC` writer - Enables the use of the dfi_lvl_periodic signal during gate training. Set to 1 to enable."]
pub type PiRdlvlGatePeriodicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_GATE_ON_SREF_EXIT` reader - Enables automatic gate training on a self-refresh exit. Set to 1 to enable."]
pub type PiRdlvlGateOnSrefExitR = crate::BitReader;
#[doc = "Field `PI_RDLVL_GATE_ON_SREF_EXIT` writer - Enables automatic gate training on a self-refresh exit. Set to 1 to enable."]
pub type PiRdlvlGateOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_ROTATE` reader - Enables rotational chip select for interval data eye training. Set to 1 for rotating CS."]
pub type PiRdlvlRotateR = crate::BitReader;
#[doc = "Field `PI_RDLVL_ROTATE` writer - Enables rotational chip select for interval data eye training. Set to 1 for rotating CS."]
pub type PiRdlvlRotateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the use of the dfi_lvl_periodic signal during gate training. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_periodic(&self) -> PiRdlvlGatePeriodicR {
        PiRdlvlGatePeriodicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables automatic gate training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_on_sref_exit(&self) -> PiRdlvlGateOnSrefExitR {
        PiRdlvlGateOnSrefExitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables rotational chip select for interval data eye training. Set to 1 for rotating CS."]
    #[inline(always)]
    pub fn pi_rdlvl_rotate(&self) -> PiRdlvlRotateR {
        PiRdlvlRotateR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the use of the dfi_lvl_periodic signal during gate training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_periodic(&mut self) -> PiRdlvlGatePeriodicW<PiReg76Spec> {
        PiRdlvlGatePeriodicW::new(self, 0)
    }
    #[doc = "Bit 8 - Enables automatic gate training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_on_sref_exit(&mut self) -> PiRdlvlGateOnSrefExitW<PiReg76Spec> {
        PiRdlvlGateOnSrefExitW::new(self, 8)
    }
    #[doc = "Bit 24 - Enables rotational chip select for interval data eye training. Set to 1 for rotating CS."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_rotate(&mut self) -> PiRdlvlRotateW<PiReg76Spec> {
        PiRdlvlRotateW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_76::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_76::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg76Spec;
impl crate::RegisterSpec for PiReg76Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_76::R`](R) reader structure"]
impl crate::Readable for PiReg76Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_76::W`](W) writer structure"]
impl crate::Writable for PiReg76Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_76 to value 0"]
impl crate::Resettable for PiReg76Spec {
    const RESET_VALUE: u32 = 0;
}
