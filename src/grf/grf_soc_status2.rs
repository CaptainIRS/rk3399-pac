#[doc = "Register `GRF_SOC_STATUS2` reader"]
pub type R = crate::R<GrfSocStatus2Spec>;
#[doc = "Register `GRF_SOC_STATUS2` writer"]
pub type W = crate::W<GrfSocStatus2Spec>;
#[doc = "Field `M0_PERILP_WAKEUP` reader - status bit of m0_perilp_wakeup"]
pub type M0PerilpWakeupR = crate::BitReader;
#[doc = "Field `M0_PERILP_WAKEUP` writer - status bit of m0_perilp_wakeup"]
pub type M0PerilpWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M0_PERILP_SLEEPING` reader - status bit of m0_perilp_sleeping"]
pub type M0PerilpSleepingR = crate::BitReader;
#[doc = "Field `M0_PERILP_SLEEPING` writer - status bit of m0_perilp_sleeping"]
pub type M0PerilpSleepingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M0_PERILP_SLEEPDEEP` reader - status bit of m0_perilp_sleeping"]
pub type M0PerilpSleepdeepR = crate::BitReader;
#[doc = "Field `M0_PERILP_SLEEPDEEP` writer - status bit of m0_perilp_sleeping"]
pub type M0PerilpSleepdeepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M0_PERILP_CORE_LOCKUP` reader - status bit of m0_perilp_core_lockup"]
pub type M0PerilpCoreLockupR = crate::BitReader;
#[doc = "Field `M0_PERILP_CORE_LOCKUP` writer - status bit of m0_perilp_core_lockup"]
pub type M0PerilpCoreLockupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M0_PERILP_HALTED` reader - status bit of m0_perilp_halted"]
pub type M0PerilpHaltedR = crate::BitReader;
#[doc = "Field `M0_PERILP_HALTED` writer - status bit of m0_perilp_halted"]
pub type M0PerilpHaltedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M0_PERILP_DBGRESTARTED` reader - status bit of m0_perilp_dbgrestarted"]
pub type M0PerilpDbgrestartedR = crate::BitReader;
#[doc = "Field `M0_PERILP_DBGRESTARTED` writer - status bit of m0_perilp_dbgrestarted"]
pub type M0PerilpDbgrestartedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEV_M0_PERILP` reader - status bit of xev_m0_perilp"]
pub type TxevM0PerilpR = crate::BitReader;
#[doc = "Field `TXEV_M0_PERILP` writer - status bit of xev_m0_perilp"]
pub type TxevM0PerilpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAGTOP_ST` reader - status bit of jtagtop_st"]
pub type JtagtopStR = crate::BitReader;
#[doc = "Field `JTAGTOP_ST` writer - status bit of jtagtop_st"]
pub type JtagtopStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAGNSW_ST` reader - status bit of jtagnsw_st"]
pub type JtagnswStR = crate::BitReader;
#[doc = "Field `JTAGNSW_ST` writer - status bit of jtagnsw_st"]
pub type JtagnswStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M0_PERILP_SYSRESETREQ` reader - m0_perilp_sysresetreq"]
pub type M0PerilpSysresetreqR = crate::BitReader;
#[doc = "Field `M0_PERILP_SYSRESETREQ` writer - m0_perilp_sysresetreq"]
pub type M0PerilpSysresetreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M0_PERILP_CDBGPWRUPREQ` reader - m0_perilp_cdbgpwrupreq status bit"]
pub type M0PerilpCdbgpwrupreqR = crate::BitReader;
#[doc = "Field `M0_PERILP_CDBGPWRUPREQ` writer - m0_perilp_cdbgpwrupreq status bit"]
pub type M0PerilpCdbgpwrupreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - status bit of m0_perilp_wakeup"]
    #[inline(always)]
    pub fn m0_perilp_wakeup(&self) -> M0PerilpWakeupR {
        M0PerilpWakeupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - status bit of m0_perilp_sleeping"]
    #[inline(always)]
    pub fn m0_perilp_sleeping(&self) -> M0PerilpSleepingR {
        M0PerilpSleepingR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - status bit of m0_perilp_sleeping"]
    #[inline(always)]
    pub fn m0_perilp_sleepdeep(&self) -> M0PerilpSleepdeepR {
        M0PerilpSleepdeepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - status bit of m0_perilp_core_lockup"]
    #[inline(always)]
    pub fn m0_perilp_core_lockup(&self) -> M0PerilpCoreLockupR {
        M0PerilpCoreLockupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - status bit of m0_perilp_halted"]
    #[inline(always)]
    pub fn m0_perilp_halted(&self) -> M0PerilpHaltedR {
        M0PerilpHaltedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - status bit of m0_perilp_dbgrestarted"]
    #[inline(always)]
    pub fn m0_perilp_dbgrestarted(&self) -> M0PerilpDbgrestartedR {
        M0PerilpDbgrestartedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - status bit of xev_m0_perilp"]
    #[inline(always)]
    pub fn txev_m0_perilp(&self) -> TxevM0PerilpR {
        TxevM0PerilpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - status bit of jtagtop_st"]
    #[inline(always)]
    pub fn jtagtop_st(&self) -> JtagtopStR {
        JtagtopStR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - status bit of jtagnsw_st"]
    #[inline(always)]
    pub fn jtagnsw_st(&self) -> JtagnswStR {
        JtagnswStR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - m0_perilp_sysresetreq"]
    #[inline(always)]
    pub fn m0_perilp_sysresetreq(&self) -> M0PerilpSysresetreqR {
        M0PerilpSysresetreqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - m0_perilp_cdbgpwrupreq status bit"]
    #[inline(always)]
    pub fn m0_perilp_cdbgpwrupreq(&self) -> M0PerilpCdbgpwrupreqR {
        M0PerilpCdbgpwrupreqR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - status bit of m0_perilp_wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn m0_perilp_wakeup(&mut self) -> M0PerilpWakeupW<GrfSocStatus2Spec> {
        M0PerilpWakeupW::new(self, 0)
    }
    #[doc = "Bit 1 - status bit of m0_perilp_sleeping"]
    #[inline(always)]
    #[must_use]
    pub fn m0_perilp_sleeping(&mut self) -> M0PerilpSleepingW<GrfSocStatus2Spec> {
        M0PerilpSleepingW::new(self, 1)
    }
    #[doc = "Bit 2 - status bit of m0_perilp_sleeping"]
    #[inline(always)]
    #[must_use]
    pub fn m0_perilp_sleepdeep(&mut self) -> M0PerilpSleepdeepW<GrfSocStatus2Spec> {
        M0PerilpSleepdeepW::new(self, 2)
    }
    #[doc = "Bit 3 - status bit of m0_perilp_core_lockup"]
    #[inline(always)]
    #[must_use]
    pub fn m0_perilp_core_lockup(&mut self) -> M0PerilpCoreLockupW<GrfSocStatus2Spec> {
        M0PerilpCoreLockupW::new(self, 3)
    }
    #[doc = "Bit 4 - status bit of m0_perilp_halted"]
    #[inline(always)]
    #[must_use]
    pub fn m0_perilp_halted(&mut self) -> M0PerilpHaltedW<GrfSocStatus2Spec> {
        M0PerilpHaltedW::new(self, 4)
    }
    #[doc = "Bit 5 - status bit of m0_perilp_dbgrestarted"]
    #[inline(always)]
    #[must_use]
    pub fn m0_perilp_dbgrestarted(&mut self) -> M0PerilpDbgrestartedW<GrfSocStatus2Spec> {
        M0PerilpDbgrestartedW::new(self, 5)
    }
    #[doc = "Bit 6 - status bit of xev_m0_perilp"]
    #[inline(always)]
    #[must_use]
    pub fn txev_m0_perilp(&mut self) -> TxevM0PerilpW<GrfSocStatus2Spec> {
        TxevM0PerilpW::new(self, 6)
    }
    #[doc = "Bit 7 - status bit of jtagtop_st"]
    #[inline(always)]
    #[must_use]
    pub fn jtagtop_st(&mut self) -> JtagtopStW<GrfSocStatus2Spec> {
        JtagtopStW::new(self, 7)
    }
    #[doc = "Bit 8 - status bit of jtagnsw_st"]
    #[inline(always)]
    #[must_use]
    pub fn jtagnsw_st(&mut self) -> JtagnswStW<GrfSocStatus2Spec> {
        JtagnswStW::new(self, 8)
    }
    #[doc = "Bit 9 - m0_perilp_sysresetreq"]
    #[inline(always)]
    #[must_use]
    pub fn m0_perilp_sysresetreq(&mut self) -> M0PerilpSysresetreqW<GrfSocStatus2Spec> {
        M0PerilpSysresetreqW::new(self, 9)
    }
    #[doc = "Bit 10 - m0_perilp_cdbgpwrupreq status bit"]
    #[inline(always)]
    #[must_use]
    pub fn m0_perilp_cdbgpwrupreq(&mut self) -> M0PerilpCdbgpwrupreqW<GrfSocStatus2Spec> {
        M0PerilpCdbgpwrupreqW::new(self, 10)
    }
}
#[doc = "SOC status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_status2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_status2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocStatus2Spec;
impl crate::RegisterSpec for GrfSocStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_status2::R`](R) reader structure"]
impl crate::Readable for GrfSocStatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_status2::W`](W) writer structure"]
impl crate::Writable for GrfSocStatus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_STATUS2 to value 0"]
impl crate::Resettable for GrfSocStatus2Spec {
    const RESET_VALUE: u32 = 0;
}
