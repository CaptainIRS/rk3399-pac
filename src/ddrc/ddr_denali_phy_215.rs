#[doc = "Register `DDR_DENALI_PHY_215` reader"]
pub type R = crate::R<DdrDenaliPhy215Spec>;
#[doc = "Register `DDR_DENALI_PHY_215` writer"]
pub type W = crate::W<DdrDenaliPhy215Spec>;
#[doc = "Field `PHY_MASTER_DELAY_STEP_1` reader - Incremental step size for master delay line locking algorithm for slice 1."]
pub type PhyMasterDelayStep1R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DELAY_STEP_1` writer - Incremental step size for master delay line locking algorithm for slice 1."]
pub type PhyMasterDelayStep1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_MASTER_DELAY_WAIT_1` reader - Wait cycles for master delay line locking algorithm for slice 1. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
pub type PhyMasterDelayWait1R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DELAY_WAIT_1` writer - Wait cycles for master delay line locking algorithm for slice 1. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
pub type PhyMasterDelayWait1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RPTR_UPDATE_1` reader - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 1."]
pub type PhyRptrUpdate1R = crate::FieldReader;
#[doc = "Field `PHY_RPTR_UPDATE_1` writer - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 1."]
pub type PhyRptrUpdate1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WRLVL_DLY_STEP_1` reader - DQS slave delay step size during write leveling for slice 1."]
pub type PhyWrlvlDlyStep1R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_DLY_STEP_1` writer - DQS slave delay step size during write leveling for slice 1."]
pub type PhyWrlvlDlyStep1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Incremental step size for master delay line locking algorithm for slice 1."]
    #[inline(always)]
    pub fn phy_master_delay_step_1(&self) -> PhyMasterDelayStep1R {
        PhyMasterDelayStep1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - Wait cycles for master delay line locking algorithm for slice 1. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
    #[inline(always)]
    pub fn phy_master_delay_wait_1(&self) -> PhyMasterDelayWait1R {
        PhyMasterDelayWait1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 1."]
    #[inline(always)]
    pub fn phy_rptr_update_1(&self) -> PhyRptrUpdate1R {
        PhyRptrUpdate1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DQS slave delay step size during write leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_dly_step_1(&self) -> PhyWrlvlDlyStep1R {
        PhyWrlvlDlyStep1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Incremental step size for master delay line locking algorithm for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_step_1(&mut self) -> PhyMasterDelayStep1W<DdrDenaliPhy215Spec> {
        PhyMasterDelayStep1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Wait cycles for master delay line locking algorithm for slice 1. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_wait_1(&mut self) -> PhyMasterDelayWait1W<DdrDenaliPhy215Spec> {
        PhyMasterDelayWait1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rptr_update_1(&mut self) -> PhyRptrUpdate1W<DdrDenaliPhy215Spec> {
        PhyRptrUpdate1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DQS slave delay step size during write leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_dly_step_1(&mut self) -> PhyWrlvlDlyStep1W<DdrDenaliPhy215Spec> {
        PhyWrlvlDlyStep1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_215::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_215::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy215Spec;
impl crate::RegisterSpec for DdrDenaliPhy215Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_215::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy215Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_215::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy215Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_215 to value 0"]
impl crate::Resettable for DdrDenaliPhy215Spec {
    const RESET_VALUE: u32 = 0;
}
