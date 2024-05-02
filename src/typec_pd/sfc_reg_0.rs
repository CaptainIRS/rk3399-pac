#[doc = "Register `SFC_REG_0` reader"]
pub type R = crate::R<SfcReg0Spec>;
#[doc = "Register `SFC_REG_0` writer"]
pub type W = crate::W<SfcReg0Spec>;
#[doc = "Field `CFG_STALL` reader - 0b \n\n- \n\nXtensa \n\nstall \n\ndeasserted \n\n1b \n\n- \n\nXtensa stall asserted \n\nXtensa - RO, APB -\n\nRW"]
pub type CfgStallR = crate::BitReader;
#[doc = "Field `CFG_STALL` writer - 0b \n\n- \n\nXtensa \n\nstall \n\ndeasserted \n\n1b \n\n- \n\nXtensa stall asserted \n\nXtensa - RO, APB -\n\nRW"]
pub type CfgStallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_SLEEP` reader - 0b - Xtensa is in sleep mode \n\n1b - Xtensa is not in sleep mode"]
pub type CfgSleepR = crate::BitReader;
#[doc = "Field `PHY_RESET` reader - 0b - TCPC PD PHY layer reset \n\ndeasserted 1b - TCPC PD PHY \n\nlayer reset asserted"]
pub type PhyResetR = crate::BitReader;
#[doc = "Field `PHY_RESET` writer - 0b - TCPC PD PHY layer reset \n\ndeasserted 1b - TCPC PD PHY \n\nlayer reset asserted"]
pub type PhyResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_STRAP` reader - Value \n\nof \n\nStrap \n\npins: 00b - do not \n\nadvertise 01b \n\n- \n\nadvertise as DFP \n\n10b \n\n- advertise \n\nas UFP 11b - start \n\nas DRP"]
pub type CfgStrapR = crate::FieldReader;
#[doc = "Field `ALERT_ALERT` reader - State of alert_n pin. \n\nDefault value is 1(inactive)."]
pub type AlertAlertR = crate::BitReader;
#[doc = "Field `ALERT_ALERT` writer - State of alert_n pin. \n\nDefault value is 1(inactive)."]
pub type AlertAlertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERT_DP_HPD` reader - State \n\nof \n\nDisplay \n\nPort \n\nHPD \n\n(source_hpd)."]
pub type AlertDpHpdR = crate::BitReader;
#[doc = "Field `ALERT_DP_HPD` writer - State \n\nof \n\nDisplay \n\nPort \n\nHPD \n\n(source_hpd)."]
pub type AlertDpHpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERT_APB_BUSY` reader - Last APB Slave transfer status. \n\n0b - processed. Ready for next transfer. \n\n1b - transfer pending. Next transfer will \n\nbe postponed. Note: Read from the first \n\n4 SFR is never postponed."]
pub type AlertApbBusyR = crate::BitReader;
#[doc = "Field `ALERT_INIT` reader - Controller \n\ninitialization \n\nstatus: \n\n0b – TCPC initialized \n\n1b \n\n- \n\nTCPC \n\nbeing \n\ninitialized \n\n- \n\nonly \n\nSFRs\\[0..3\\]
are readable. No SFR shall be \n\nwritten during initialization."]
pub type AlertInitR = crate::BitReader;
#[doc = "Field `ALERT_CMN_RESET_N` reader - Value \n\nof \n\nthis \n\nbit \n\nis \n\npresents \n\ncmn_reset_n output."]
pub type AlertCmnResetNR = crate::BitReader;
#[doc = "Field `ALERT_CMN_RESET_N` writer - Value \n\nof \n\nthis \n\nbit \n\nis \n\npresents \n\ncmn_reset_n output."]
pub type AlertCmnResetNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERT_CMN_READY` reader - When asserted TCPC PHY is ready"]
pub type AlertCmnReadyR = crate::BitReader;
#[doc = "Field `PRESET_N_M` reader - Value \n\nof \n\nthis \n\nbit \n\nis \n\npresents \n\npreset_n_m output."]
pub type PresetNMR = crate::BitReader;
#[doc = "Field `PRESET_N_M` writer - Value \n\nof \n\nthis \n\nbit \n\nis \n\npresents \n\npreset_n_m output."]
pub type PresetNMW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER` reader - Start APB transfer. Writing one to this \n\nregister \n\ninitiates \n\nAPB \n\ntransfer. \n\nThis \n\nregister is cleared by HW APB transfer \n\nwas finished (pready_m assert). \n\nType of this bit is RWAC - Read Write Auto \n\nClear."]
pub type TransferR = crate::BitReader;
#[doc = "Field `TRANSFER` writer - Start APB transfer. Writing one to this \n\nregister \n\ninitiates \n\nAPB \n\ntransfer. \n\nThis \n\nregister is cleared by HW APB transfer \n\nwas finished (pready_m assert). \n\nType of this bit is RWAC - Read Write Auto \n\nClear."]
pub type TransferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRITE_M` reader - Value \n\nof \n\nthis \n\nbit \n\nis \n\npresents \n\npenable_moutput."]
pub type PwriteMR = crate::BitReader;
#[doc = "Field `PWRITE_M` writer - Value \n\nof \n\nthis \n\nbit \n\nis \n\npresents \n\npenable_moutput."]
pub type PwriteMW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 0b \n\n- \n\nXtensa \n\nstall \n\ndeasserted \n\n1b \n\n- \n\nXtensa stall asserted \n\nXtensa - RO, APB -\n\nRW"]
    #[inline(always)]
    pub fn cfg_stall(&self) -> CfgStallR {
        CfgStallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0b - Xtensa is in sleep mode \n\n1b - Xtensa is not in sleep mode"]
    #[inline(always)]
    pub fn cfg_sleep(&self) -> CfgSleepR {
        CfgSleepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0b - TCPC PD PHY layer reset \n\ndeasserted 1b - TCPC PD PHY \n\nlayer reset asserted"]
    #[inline(always)]
    pub fn phy_reset(&self) -> PhyResetR {
        PhyResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Value \n\nof \n\nStrap \n\npins: 00b - do not \n\nadvertise 01b \n\n- \n\nadvertise as DFP \n\n10b \n\n- advertise \n\nas UFP 11b - start \n\nas DRP"]
    #[inline(always)]
    pub fn cfg_strap(&self) -> CfgStrapR {
        CfgStrapR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - State of alert_n pin. \n\nDefault value is 1(inactive)."]
    #[inline(always)]
    pub fn alert_alert(&self) -> AlertAlertR {
        AlertAlertR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - State \n\nof \n\nDisplay \n\nPort \n\nHPD \n\n(source_hpd)."]
    #[inline(always)]
    pub fn alert_dp_hpd(&self) -> AlertDpHpdR {
        AlertDpHpdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Last APB Slave transfer status. \n\n0b - processed. Ready for next transfer. \n\n1b - transfer pending. Next transfer will \n\nbe postponed. Note: Read from the first \n\n4 SFR is never postponed."]
    #[inline(always)]
    pub fn alert_apb_busy(&self) -> AlertApbBusyR {
        AlertApbBusyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Controller \n\ninitialization \n\nstatus: \n\n0b – TCPC initialized \n\n1b \n\n- \n\nTCPC \n\nbeing \n\ninitialized \n\n- \n\nonly \n\nSFRs\\[0..3\\]
are readable. No SFR shall be \n\nwritten during initialization."]
    #[inline(always)]
    pub fn alert_init(&self) -> AlertInitR {
        AlertInitR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Value \n\nof \n\nthis \n\nbit \n\nis \n\npresents \n\ncmn_reset_n output."]
    #[inline(always)]
    pub fn alert_cmn_reset_n(&self) -> AlertCmnResetNR {
        AlertCmnResetNR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When asserted TCPC PHY is ready"]
    #[inline(always)]
    pub fn alert_cmn_ready(&self) -> AlertCmnReadyR {
        AlertCmnReadyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Value \n\nof \n\nthis \n\nbit \n\nis \n\npresents \n\npreset_n_m output."]
    #[inline(always)]
    pub fn preset_n_m(&self) -> PresetNMR {
        PresetNMR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Start APB transfer. Writing one to this \n\nregister \n\ninitiates \n\nAPB \n\ntransfer. \n\nThis \n\nregister is cleared by HW APB transfer \n\nwas finished (pready_m assert). \n\nType of this bit is RWAC - Read Write Auto \n\nClear."]
    #[inline(always)]
    pub fn transfer(&self) -> TransferR {
        TransferR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Value \n\nof \n\nthis \n\nbit \n\nis \n\npresents \n\npenable_moutput."]
    #[inline(always)]
    pub fn pwrite_m(&self) -> PwriteMR {
        PwriteMR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 0b \n\n- \n\nXtensa \n\nstall \n\ndeasserted \n\n1b \n\n- \n\nXtensa stall asserted \n\nXtensa - RO, APB -\n\nRW"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_stall(&mut self) -> CfgStallW<SfcReg0Spec> {
        CfgStallW::new(self, 1)
    }
    #[doc = "Bit 3 - 0b - TCPC PD PHY layer reset \n\ndeasserted 1b - TCPC PD PHY \n\nlayer reset asserted"]
    #[inline(always)]
    #[must_use]
    pub fn phy_reset(&mut self) -> PhyResetW<SfcReg0Spec> {
        PhyResetW::new(self, 3)
    }
    #[doc = "Bit 8 - State of alert_n pin. \n\nDefault value is 1(inactive)."]
    #[inline(always)]
    #[must_use]
    pub fn alert_alert(&mut self) -> AlertAlertW<SfcReg0Spec> {
        AlertAlertW::new(self, 8)
    }
    #[doc = "Bit 9 - State \n\nof \n\nDisplay \n\nPort \n\nHPD \n\n(source_hpd)."]
    #[inline(always)]
    #[must_use]
    pub fn alert_dp_hpd(&mut self) -> AlertDpHpdW<SfcReg0Spec> {
        AlertDpHpdW::new(self, 9)
    }
    #[doc = "Bit 14 - Value \n\nof \n\nthis \n\nbit \n\nis \n\npresents \n\ncmn_reset_n output."]
    #[inline(always)]
    #[must_use]
    pub fn alert_cmn_reset_n(&mut self) -> AlertCmnResetNW<SfcReg0Spec> {
        AlertCmnResetNW::new(self, 14)
    }
    #[doc = "Bit 24 - Value \n\nof \n\nthis \n\nbit \n\nis \n\npresents \n\npreset_n_m output."]
    #[inline(always)]
    #[must_use]
    pub fn preset_n_m(&mut self) -> PresetNMW<SfcReg0Spec> {
        PresetNMW::new(self, 24)
    }
    #[doc = "Bit 25 - Start APB transfer. Writing one to this \n\nregister \n\ninitiates \n\nAPB \n\ntransfer. \n\nThis \n\nregister is cleared by HW APB transfer \n\nwas finished (pready_m assert). \n\nType of this bit is RWAC - Read Write Auto \n\nClear."]
    #[inline(always)]
    #[must_use]
    pub fn transfer(&mut self) -> TransferW<SfcReg0Spec> {
        TransferW::new(self, 25)
    }
    #[doc = "Bit 27 - Value \n\nof \n\nthis \n\nbit \n\nis \n\npresents \n\npenable_moutput."]
    #[inline(always)]
    #[must_use]
    pub fn pwrite_m(&mut self) -> PwriteMW<SfcReg0Spec> {
        PwriteMW::new(self, 27)
    }
}
#[doc = "SFC Reg0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfc_reg_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfc_reg_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfcReg0Spec;
impl crate::RegisterSpec for SfcReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfc_reg_0::R`](R) reader structure"]
impl crate::Readable for SfcReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfc_reg_0::W`](W) writer structure"]
impl crate::Writable for SfcReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SFC_REG_0 to value 0x0101"]
impl crate::Resettable for SfcReg0Spec {
    const RESET_VALUE: u32 = 0x0101;
}
