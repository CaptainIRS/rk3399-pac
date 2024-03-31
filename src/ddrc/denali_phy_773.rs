#[doc = "Register `DENALI_PHY_773` reader"]
pub type R = crate::R<DenaliPhy773Spec>;
#[doc = "Register `DENALI_PHY_773` writer"]
pub type W = crate::W<DenaliPhy773Spec>;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_2` reader - Observation register for master delay results for address slice 2."]
pub type PhyAdrMasterDlyLockObs2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_SLAVE_LOOP_CNT_UPDATE_2` reader - Sets the frequency by which the slave delay encoded value holding registers are updated for address slice 2."]
pub type PhyAdrSlaveLoopCntUpdate2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SLAVE_LOOP_CNT_UPDATE_2` writer - Sets the frequency by which the slave delay encoded value holding registers are updated for address slice 2."]
pub type PhyAdrSlaveLoopCntUpdate2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_ADR_SLV_DLY_ENC_OBS_SELECT_2` reader - Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 2."]
pub type PhyAdrSlvDlyEncObsSelect2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SLV_DLY_ENC_OBS_SELECT_2` writer - Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 2."]
pub type PhyAdrSlvDlyEncObsSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:9 - Observation register for master delay results for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_master_dly_lock_obs_2(&self) -> PhyAdrMasterDlyLockObs2R {
        PhyAdrMasterDlyLockObs2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18 - Sets the frequency by which the slave delay encoded value holding registers are updated for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_slave_loop_cnt_update_2(&self) -> PhyAdrSlaveLoopCntUpdate2R {
        PhyAdrSlaveLoopCntUpdate2R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_slv_dly_enc_obs_select_2(&self) -> PhyAdrSlvDlyEncObsSelect2R {
        PhyAdrSlvDlyEncObsSelect2R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - Sets the frequency by which the slave delay encoded value holding registers are updated for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_slave_loop_cnt_update_2(
        &mut self,
    ) -> PhyAdrSlaveLoopCntUpdate2W<DenaliPhy773Spec> {
        PhyAdrSlaveLoopCntUpdate2W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Select value to map the addr bits delay observation registers to the accessible delay observation register for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_slv_dly_enc_obs_select_2(
        &mut self,
    ) -> PhyAdrSlvDlyEncObsSelect2W<DenaliPhy773Spec> {
        PhyAdrSlvDlyEncObsSelect2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_773::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_773::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy773Spec;
impl crate::RegisterSpec for DenaliPhy773Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_773::R`](R) reader structure"]
impl crate::Readable for DenaliPhy773Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_773::W`](W) writer structure"]
impl crate::Writable for DenaliPhy773Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_773 to value 0"]
impl crate::Resettable for DenaliPhy773Spec {
    const RESET_VALUE: u32 = 0;
}
