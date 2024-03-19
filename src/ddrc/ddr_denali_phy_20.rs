#[doc = "Register `DDR_DENALI_PHY_20` reader"]
pub type R = crate::R<DdrDenaliPhy20Spec>;
#[doc = "Register `DDR_DENALI_PHY_20` writer"]
pub type W = crate::W<DdrDenaliPhy20Spec>;
#[doc = "Field `PHY_SLAVE_LOOP_CNT_UPDATE_0` reader - Sets the frequency by which the slave delay encoded value holding registers are updated for slice 0."]
pub type PhySlaveLoopCntUpdate0R = crate::FieldReader;
#[doc = "Field `PHY_SLAVE_LOOP_CNT_UPDATE_0` writer - Sets the frequency by which the slave delay encoded value holding registers are updated for slice 0."]
pub type PhySlaveLoopCntUpdate0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_SW_FIFO_PTR_RST_DISABLE_0` reader - Disables automatic reset of the read entry FIFO pointers for slice 0. Set to 1 to disable automatic resets."]
pub type PhySwFifoPtrRstDisable0R = crate::BitReader;
#[doc = "Field `PHY_SW_FIFO_PTR_RST_DISABLE_0` writer - Disables automatic reset of the read entry FIFO pointers for slice 0. Set to 1 to disable automatic resets."]
pub type PhySwFifoPtrRstDisable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_SELECT_0` reader - Select value to map the internal master delay observation registers to the accessible master delay observation register for slice 0."]
pub type PhyMasterDlyLockObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_SELECT_0` writer - Select value to map the internal master delay observation registers to the accessible master delay observation register for slice 0."]
pub type PhyMasterDlyLockObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_RDDQ_ENC_OBS_SELECT_0` reader - Select value to map the internal read DQ slave delay encoded settings to the accessible read DQ encoded slave delay observation register for slice 0."]
pub type PhyRddqEncObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_RDDQ_ENC_OBS_SELECT_0` writer - Select value to map the internal read DQ slave delay encoded settings to the accessible read DQ encoded slave delay observation register for slice 0."]
pub type PhyRddqEncObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Sets the frequency by which the slave delay encoded value holding registers are updated for slice 0."]
    #[inline(always)]
    pub fn phy_slave_loop_cnt_update_0(&self) -> PhySlaveLoopCntUpdate0R {
        PhySlaveLoopCntUpdate0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Disables automatic reset of the read entry FIFO pointers for slice 0. Set to 1 to disable automatic resets."]
    #[inline(always)]
    pub fn phy_sw_fifo_ptr_rst_disable_0(&self) -> PhySwFifoPtrRstDisable0R {
        PhySwFifoPtrRstDisable0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Select value to map the internal master delay observation registers to the accessible master delay observation register for slice 0."]
    #[inline(always)]
    pub fn phy_master_dly_lock_obs_select_0(&self) -> PhyMasterDlyLockObsSelect0R {
        PhyMasterDlyLockObsSelect0R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Select value to map the internal read DQ slave delay encoded settings to the accessible read DQ encoded slave delay observation register for slice 0."]
    #[inline(always)]
    pub fn phy_rddq_enc_obs_select_0(&self) -> PhyRddqEncObsSelect0R {
        PhyRddqEncObsSelect0R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the frequency by which the slave delay encoded value holding registers are updated for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_slave_loop_cnt_update_0(&mut self) -> PhySlaveLoopCntUpdate0W<DdrDenaliPhy20Spec> {
        PhySlaveLoopCntUpdate0W::new(self, 0)
    }
    #[doc = "Bit 8 - Disables automatic reset of the read entry FIFO pointers for slice 0. Set to 1 to disable automatic resets."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_fifo_ptr_rst_disable_0(
        &mut self,
    ) -> PhySwFifoPtrRstDisable0W<DdrDenaliPhy20Spec> {
        PhySwFifoPtrRstDisable0W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Select value to map the internal master delay observation registers to the accessible master delay observation register for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_dly_lock_obs_select_0(
        &mut self,
    ) -> PhyMasterDlyLockObsSelect0W<DdrDenaliPhy20Spec> {
        PhyMasterDlyLockObsSelect0W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Select value to map the internal read DQ slave delay encoded settings to the accessible read DQ encoded slave delay observation register for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq_enc_obs_select_0(&mut self) -> PhyRddqEncObsSelect0W<DdrDenaliPhy20Spec> {
        PhyRddqEncObsSelect0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy20Spec;
impl crate::RegisterSpec for DdrDenaliPhy20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_20::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy20Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_20::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_20 to value 0"]
impl crate::Resettable for DdrDenaliPhy20Spec {
    const RESET_VALUE: u32 = 0;
}
