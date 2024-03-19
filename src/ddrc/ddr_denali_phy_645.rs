#[doc = "Register `DDR_DENALI_PHY_645` reader"]
pub type R = crate::R<DdrDenaliPhy645Spec>;
#[doc = "Register `DDR_DENALI_PHY_645` writer"]
pub type W = crate::W<DdrDenaliPhy645Spec>;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_1` reader - Observation register for master delay results for address slice 1. READ-ONLY"]
pub type PhyAdrMasterDlyLockObs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_SLAVE_LOOP_CNT_UPDATE_1` reader - Sets the frequency by which the slave delay encoded value holding registers are updated for address slice 1."]
pub type PhyAdrSlaveLoopCntUpdate1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SLAVE_LOOP_CNT_UPDATE_1` writer - Sets the frequency by which the slave delay encoded value holding registers are updated for address slice 1."]
pub type PhyAdrSlaveLoopCntUpdate1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_ADR_SLV_DLY_ENC_OBS_SELECT_1` reader - Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 1."]
pub type PhyAdrSlvDlyEncObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SLV_DLY_ENC_OBS_SELECT_1` writer - Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 1."]
pub type PhyAdrSlvDlyEncObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:9 - Observation register for master delay results for address slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_master_dly_lock_obs_1(&self) -> PhyAdrMasterDlyLockObs1R {
        PhyAdrMasterDlyLockObs1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18 - Sets the frequency by which the slave delay encoded value holding registers are updated for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_slave_loop_cnt_update_1(&self) -> PhyAdrSlaveLoopCntUpdate1R {
        PhyAdrSlaveLoopCntUpdate1R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_slv_dly_enc_obs_select_1(&self) -> PhyAdrSlvDlyEncObsSelect1R {
        PhyAdrSlvDlyEncObsSelect1R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - Sets the frequency by which the slave delay encoded value holding registers are updated for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_slave_loop_cnt_update_1(
        &mut self,
    ) -> PhyAdrSlaveLoopCntUpdate1W<DdrDenaliPhy645Spec> {
        PhyAdrSlaveLoopCntUpdate1W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_slv_dly_enc_obs_select_1(
        &mut self,
    ) -> PhyAdrSlvDlyEncObsSelect1W<DdrDenaliPhy645Spec> {
        PhyAdrSlvDlyEncObsSelect1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_645::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_645::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy645Spec;
impl crate::RegisterSpec for DdrDenaliPhy645Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_645::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy645Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_645::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy645Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_645 to value 0"]
impl crate::Resettable for DdrDenaliPhy645Spec {
    const RESET_VALUE: u32 = 0;
}
