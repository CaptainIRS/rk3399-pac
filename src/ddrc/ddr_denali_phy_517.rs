#[doc = "Register `DDR_DENALI_PHY_517` reader"]
pub type R = crate::R<DdrDenaliPhy517Spec>;
#[doc = "Register `DDR_DENALI_PHY_517` writer"]
pub type W = crate::W<DdrDenaliPhy517Spec>;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_0` reader - Observation register for master delay results for address slice 0."]
pub type PhyAdrMasterDlyLockObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_SLAVE_LOOP_CNT_UPDATE_0` reader - Sets the frequency by which the slave delay encoded value holding registers are updated for address slice 0."]
pub type PhyAdrSlaveLoopCntUpdate0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SLAVE_LOOP_CNT_UPDATE_0` writer - Sets the frequency by which the slave delay encoded value holding registers are updated for address slice 0."]
pub type PhyAdrSlaveLoopCntUpdate0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_ADR_SLV_DLY_ENC_OBS_SELECT_0` reader - Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 0."]
pub type PhyAdrSlvDlyEncObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SLV_DLY_ENC_OBS_SELECT_0` writer - Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 0."]
pub type PhyAdrSlvDlyEncObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:9 - Observation register for master delay results for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_master_dly_lock_obs_0(&self) -> PhyAdrMasterDlyLockObs0R {
        PhyAdrMasterDlyLockObs0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18 - Sets the frequency by which the slave delay encoded value holding registers are updated for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_slave_loop_cnt_update_0(&self) -> PhyAdrSlaveLoopCntUpdate0R {
        PhyAdrSlaveLoopCntUpdate0R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_slv_dly_enc_obs_select_0(&self) -> PhyAdrSlvDlyEncObsSelect0R {
        PhyAdrSlvDlyEncObsSelect0R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - Sets the frequency by which the slave delay encoded value holding registers are updated for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_slave_loop_cnt_update_0(
        &mut self,
    ) -> PhyAdrSlaveLoopCntUpdate0W<DdrDenaliPhy517Spec> {
        PhyAdrSlaveLoopCntUpdate0W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_slv_dly_enc_obs_select_0(
        &mut self,
    ) -> PhyAdrSlvDlyEncObsSelect0W<DdrDenaliPhy517Spec> {
        PhyAdrSlvDlyEncObsSelect0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_517::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_517::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy517Spec;
impl crate::RegisterSpec for DdrDenaliPhy517Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_517::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy517Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_517::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy517Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_517 to value 0"]
impl crate::Resettable for DdrDenaliPhy517Spec {
    const RESET_VALUE: u32 = 0;
}
