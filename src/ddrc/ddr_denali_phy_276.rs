#[doc = "Register `DDR_DENALI_PHY_276` reader"]
pub type R = crate::R<DdrDenaliPhy276Spec>;
#[doc = "Register `DDR_DENALI_PHY_276` writer"]
pub type W = crate::W<DdrDenaliPhy276Spec>;
#[doc = "Field `PHY_SLAVE_LOOP_CNT_UPDATE_2` reader - Sets the frequency by which the slave delay encoded value holding registers are updated for slice 2."]
pub type PhySlaveLoopCntUpdate2R = crate::FieldReader;
#[doc = "Field `PHY_SLAVE_LOOP_CNT_UPDATE_2` writer - Sets the frequency by which the slave delay encoded value holding registers are updated for slice 2."]
pub type PhySlaveLoopCntUpdate2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_SW_FIFO_PTR_RST_DISABLE_2` reader - Disables automatic reset of the read entry FIFO pointers for slice 2. Set to 1 to disable automatic resets."]
pub type PhySwFifoPtrRstDisable2R = crate::BitReader;
#[doc = "Field `PHY_SW_FIFO_PTR_RST_DISABLE_2` writer - Disables automatic reset of the read entry FIFO pointers for slice 2. Set to 1 to disable automatic resets."]
pub type PhySwFifoPtrRstDisable2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_SELECT_2` reader - Select value to map the internal master delay observation registers to the accessible master delay observation register for slice 2."]
pub type PhyMasterDlyLockObsSelect2R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_SELECT_2` writer - Select value to map the internal master delay observation registers to the accessible master delay observation register for slice 2."]
pub type PhyMasterDlyLockObsSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_RDDQ_ENC_OBS_SELECT_2` reader - Select value to map the internal read DQ slave delay encoded settings to the accessible read DQ encoded slave delay observation register for slice 2."]
pub type PhyRddqEncObsSelect2R = crate::FieldReader;
#[doc = "Field `PHY_RDDQ_ENC_OBS_SELECT_2` writer - Select value to map the internal read DQ slave delay encoded settings to the accessible read DQ encoded slave delay observation register for slice 2."]
pub type PhyRddqEncObsSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Sets the frequency by which the slave delay encoded value holding registers are updated for slice 2."]
    #[inline(always)]
    pub fn phy_slave_loop_cnt_update_2(&self) -> PhySlaveLoopCntUpdate2R {
        PhySlaveLoopCntUpdate2R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Disables automatic reset of the read entry FIFO pointers for slice 2. Set to 1 to disable automatic resets."]
    #[inline(always)]
    pub fn phy_sw_fifo_ptr_rst_disable_2(&self) -> PhySwFifoPtrRstDisable2R {
        PhySwFifoPtrRstDisable2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Select value to map the internal master delay observation registers to the accessible master delay observation register for slice 2."]
    #[inline(always)]
    pub fn phy_master_dly_lock_obs_select_2(&self) -> PhyMasterDlyLockObsSelect2R {
        PhyMasterDlyLockObsSelect2R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Select value to map the internal read DQ slave delay encoded settings to the accessible read DQ encoded slave delay observation register for slice 2."]
    #[inline(always)]
    pub fn phy_rddq_enc_obs_select_2(&self) -> PhyRddqEncObsSelect2R {
        PhyRddqEncObsSelect2R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the frequency by which the slave delay encoded value holding registers are updated for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_slave_loop_cnt_update_2(&mut self) -> PhySlaveLoopCntUpdate2W<DdrDenaliPhy276Spec> {
        PhySlaveLoopCntUpdate2W::new(self, 0)
    }
    #[doc = "Bit 8 - Disables automatic reset of the read entry FIFO pointers for slice 2. Set to 1 to disable automatic resets."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_fifo_ptr_rst_disable_2(
        &mut self,
    ) -> PhySwFifoPtrRstDisable2W<DdrDenaliPhy276Spec> {
        PhySwFifoPtrRstDisable2W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Select value to map the internal master delay observation registers to the accessible master delay observation register for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_dly_lock_obs_select_2(
        &mut self,
    ) -> PhyMasterDlyLockObsSelect2W<DdrDenaliPhy276Spec> {
        PhyMasterDlyLockObsSelect2W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Select value to map the internal read DQ slave delay encoded settings to the accessible read DQ encoded slave delay observation register for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq_enc_obs_select_2(&mut self) -> PhyRddqEncObsSelect2W<DdrDenaliPhy276Spec> {
        PhyRddqEncObsSelect2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_276::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_276::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy276Spec;
impl crate::RegisterSpec for DdrDenaliPhy276Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_276::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy276Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_276::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy276Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_276 to value 0"]
impl crate::Resettable for DdrDenaliPhy276Spec {
    const RESET_VALUE: u32 = 0;
}
