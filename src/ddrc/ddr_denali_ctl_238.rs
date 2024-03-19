#[doc = "Register `DDR_DENALI_CTL_238` reader"]
pub type R = crate::R<DdrDenaliCtl238Spec>;
#[doc = "Register `DDR_DENALI_CTL_238` writer"]
pub type W = crate::W<DdrDenaliCtl238Spec>;
#[doc = "Field `RDLVL_ON_SREF_EXIT` reader - Enables automatic data eye training on a self-refresh exit. Set to 1 to enable."]
pub type RdlvlOnSrefExitR = crate::BitReader;
#[doc = "Field `RDLVL_ON_SREF_EXIT` writer - Enables automatic data eye training on a self-refresh exit. Set to 1 to enable."]
pub type RdlvlOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDLVL_GATE_PERIODIC` reader - Enables the use of the dfi_lvl_periodic signal during gate training. Set to 1 to enable."]
pub type RdlvlGatePeriodicR = crate::BitReader;
#[doc = "Field `RDLVL_GATE_PERIODIC` writer - Enables the use of the dfi_lvl_periodic signal during gate training. Set to 1 to enable."]
pub type RdlvlGatePeriodicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDLVL_GATE_ON_SREF_EXIT` reader - Enables automatic gate training on a self-refresh exit. Set to 1 to enable."]
pub type RdlvlGateOnSrefExitR = crate::BitReader;
#[doc = "Field `RDLVL_GATE_ON_SREF_EXIT` writer - Enables automatic gate training on a self-refresh exit. Set to 1 to enable."]
pub type RdlvlGateOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDLVL_AREF_EN` reader - Enables refreshes and other non- data commands to execute in the middle of data eye training. Set to 1 to enable."]
pub type RdlvlArefEnR = crate::BitReader;
#[doc = "Field `RDLVL_AREF_EN` writer - Enables refreshes and other non- data commands to execute in the middle of data eye training. Set to 1 to enable."]
pub type RdlvlArefEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables automatic data eye training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn rdlvl_on_sref_exit(&self) -> RdlvlOnSrefExitR {
        RdlvlOnSrefExitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the use of the dfi_lvl_periodic signal during gate training. Set to 1 to enable."]
    #[inline(always)]
    pub fn rdlvl_gate_periodic(&self) -> RdlvlGatePeriodicR {
        RdlvlGatePeriodicR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables automatic gate training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn rdlvl_gate_on_sref_exit(&self) -> RdlvlGateOnSrefExitR {
        RdlvlGateOnSrefExitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables refreshes and other non- data commands to execute in the middle of data eye training. Set to 1 to enable."]
    #[inline(always)]
    pub fn rdlvl_aref_en(&self) -> RdlvlArefEnR {
        RdlvlArefEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables automatic data eye training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_on_sref_exit(&mut self) -> RdlvlOnSrefExitW<DdrDenaliCtl238Spec> {
        RdlvlOnSrefExitW::new(self, 0)
    }
    #[doc = "Bit 8 - Enables the use of the dfi_lvl_periodic signal during gate training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_periodic(&mut self) -> RdlvlGatePeriodicW<DdrDenaliCtl238Spec> {
        RdlvlGatePeriodicW::new(self, 8)
    }
    #[doc = "Bit 16 - Enables automatic gate training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_on_sref_exit(&mut self) -> RdlvlGateOnSrefExitW<DdrDenaliCtl238Spec> {
        RdlvlGateOnSrefExitW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables refreshes and other non- data commands to execute in the middle of data eye training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_aref_en(&mut self) -> RdlvlArefEnW<DdrDenaliCtl238Spec> {
        RdlvlArefEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_238::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_238::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl238Spec;
impl crate::RegisterSpec for DdrDenaliCtl238Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_238::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl238Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_238::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl238Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_238 to value 0"]
impl crate::Resettable for DdrDenaliCtl238Spec {
    const RESET_VALUE: u32 = 0;
}
