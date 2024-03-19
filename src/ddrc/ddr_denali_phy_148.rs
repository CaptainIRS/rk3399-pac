#[doc = "Register `DDR_DENALI_PHY_148` reader"]
pub type R = crate::R<DdrDenaliPhy148Spec>;
#[doc = "Register `DDR_DENALI_PHY_148` writer"]
pub type W = crate::W<DdrDenaliPhy148Spec>;
#[doc = "Field `PHY_SLAVE_LOOP_CNT_UPDATE_1` reader - Sets the frequency by which the slave delay encoded value holding registers are updated for slice 1."]
pub type PhySlaveLoopCntUpdate1R = crate::FieldReader;
#[doc = "Field `PHY_SLAVE_LOOP_CNT_UPDATE_1` writer - Sets the frequency by which the slave delay encoded value holding registers are updated for slice 1."]
pub type PhySlaveLoopCntUpdate1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_SW_FIFO_PTR_RST_DISABLE_1` reader - Disables automatic reset of the read entry FIFO pointers for slice 1. Set to 1 to disable automatic resets."]
pub type PhySwFifoPtrRstDisable1R = crate::BitReader;
#[doc = "Field `PHY_SW_FIFO_PTR_RST_DISABLE_1` writer - Disables automatic reset of the read entry FIFO pointers for slice 1. Set to 1 to disable automatic resets."]
pub type PhySwFifoPtrRstDisable1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_SELECT_1` reader - Select value to map the internal master delay observation registers to the accessible master delay observation register for slice 1."]
pub type PhyMasterDlyLockObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_SELECT_1` writer - Select value to map the internal master delay observation registers to the accessible master delay observation register for slice 1."]
pub type PhyMasterDlyLockObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_RDDQ_ENC_OBS_SELECT_1` reader - Select value to map the internal read DQ slave delay encoded settings to the accessible read DQ encoded slave delay observation register for slice 1."]
pub type PhyRddqEncObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_RDDQ_ENC_OBS_SELECT_1` writer - Select value to map the internal read DQ slave delay encoded settings to the accessible read DQ encoded slave delay observation register for slice 1."]
pub type PhyRddqEncObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Sets the frequency by which the slave delay encoded value holding registers are updated for slice 1."]
    #[inline(always)]
    pub fn phy_slave_loop_cnt_update_1(&self) -> PhySlaveLoopCntUpdate1R {
        PhySlaveLoopCntUpdate1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Disables automatic reset of the read entry FIFO pointers for slice 1. Set to 1 to disable automatic resets."]
    #[inline(always)]
    pub fn phy_sw_fifo_ptr_rst_disable_1(&self) -> PhySwFifoPtrRstDisable1R {
        PhySwFifoPtrRstDisable1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Select value to map the internal master delay observation registers to the accessible master delay observation register for slice 1."]
    #[inline(always)]
    pub fn phy_master_dly_lock_obs_select_1(&self) -> PhyMasterDlyLockObsSelect1R {
        PhyMasterDlyLockObsSelect1R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Select value to map the internal read DQ slave delay encoded settings to the accessible read DQ encoded slave delay observation register for slice 1."]
    #[inline(always)]
    pub fn phy_rddq_enc_obs_select_1(&self) -> PhyRddqEncObsSelect1R {
        PhyRddqEncObsSelect1R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the frequency by which the slave delay encoded value holding registers are updated for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_slave_loop_cnt_update_1(&mut self) -> PhySlaveLoopCntUpdate1W<DdrDenaliPhy148Spec> {
        PhySlaveLoopCntUpdate1W::new(self, 0)
    }
    #[doc = "Bit 8 - Disables automatic reset of the read entry FIFO pointers for slice 1. Set to 1 to disable automatic resets."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_fifo_ptr_rst_disable_1(
        &mut self,
    ) -> PhySwFifoPtrRstDisable1W<DdrDenaliPhy148Spec> {
        PhySwFifoPtrRstDisable1W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Select value to map the internal master delay observation registers to the accessible master delay observation register for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_dly_lock_obs_select_1(
        &mut self,
    ) -> PhyMasterDlyLockObsSelect1W<DdrDenaliPhy148Spec> {
        PhyMasterDlyLockObsSelect1W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Select value to map the internal read DQ slave delay encoded settings to the accessible read DQ encoded slave delay observation register for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq_enc_obs_select_1(&mut self) -> PhyRddqEncObsSelect1W<DdrDenaliPhy148Spec> {
        PhyRddqEncObsSelect1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_148::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_148::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy148Spec;
impl crate::RegisterSpec for DdrDenaliPhy148Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_148::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy148Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_148::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy148Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_148 to value 0"]
impl crate::Resettable for DdrDenaliPhy148Spec {
    const RESET_VALUE: u32 = 0;
}
