#[doc = "Register `DENALI_CTL_68` reader"]
pub type R = crate::R<DenaliCtl68Spec>;
#[doc = "Register `DENALI_CTL_68` writer"]
pub type W = crate::W<DenaliCtl68Spec>;
#[doc = "Field `TCSCKEH_F2` reader - DRAM TCSCKEH value for frequency copy 2 in cycles."]
pub type TcsckehF2R = crate::FieldReader;
#[doc = "Field `TCSCKEH_F2` writer - DRAM TCSCKEH value for frequency copy 2 in cycles."]
pub type TcsckehF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCMDCKE_F2` reader - DRAM TCMDCKE value for frequency copy 2 in cycles."]
pub type TcmdckeF2R = crate::FieldReader;
#[doc = "Field `TCMDCKE_F2` writer - DRAM TCMDCKE value for frequency copy 2 in cycles."]
pub type TcmdckeF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PWRUP_SREFRESH_EXIT` reader - Allow powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
pub type PwrupSrefreshExitR = crate::BitReader;
#[doc = "Field `PWRUP_SREFRESH_EXIT` writer - Allow powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
pub type PwrupSrefreshExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SREFRESH_EXIT_NO_REFRESH` reader - Disables the automatic refresh request associated with self-refresh exit. Set to 1 to disable."]
pub type SrefreshExitNoRefreshR = crate::BitReader;
#[doc = "Field `SREFRESH_EXIT_NO_REFRESH` writer - Disables the automatic refresh request associated with self-refresh exit. Set to 1 to disable."]
pub type SrefreshExitNoRefreshW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - DRAM TCSCKEH value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tcsckeh_f2(&self) -> TcsckehF2R {
        TcsckehF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DRAM TCMDCKE value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tcmdcke_f2(&self) -> TcmdckeF2R {
        TcmdckeF2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Allow powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pwrup_srefresh_exit(&self) -> PwrupSrefreshExitR {
        PwrupSrefreshExitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Disables the automatic refresh request associated with self-refresh exit. Set to 1 to disable."]
    #[inline(always)]
    pub fn srefresh_exit_no_refresh(&self) -> SrefreshExitNoRefreshR {
        SrefreshExitNoRefreshR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRAM TCSCKEH value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcsckeh_f2(&mut self) -> TcsckehF2W<DenaliCtl68Spec> {
        TcsckehF2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DRAM TCMDCKE value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcmdcke_f2(&mut self) -> TcmdckeF2W<DenaliCtl68Spec> {
        TcmdckeF2W::new(self, 8)
    }
    #[doc = "Bit 16 - Allow powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pwrup_srefresh_exit(&mut self) -> PwrupSrefreshExitW<DenaliCtl68Spec> {
        PwrupSrefreshExitW::new(self, 16)
    }
    #[doc = "Bit 24 - Disables the automatic refresh request associated with self-refresh exit. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn srefresh_exit_no_refresh(&mut self) -> SrefreshExitNoRefreshW<DenaliCtl68Spec> {
        SrefreshExitNoRefreshW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_68::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_68::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl68Spec;
impl crate::RegisterSpec for DenaliCtl68Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_68::R`](R) reader structure"]
impl crate::Readable for DenaliCtl68Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_68::W`](W) writer structure"]
impl crate::Writable for DenaliCtl68Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_68 to value 0"]
impl crate::Resettable for DenaliCtl68Spec {
    const RESET_VALUE: u32 = 0;
}
