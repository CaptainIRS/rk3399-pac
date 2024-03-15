#[doc = "Register `DENALI_PHY_471` reader"]
pub type R = crate::R<DenaliPhy471Spec>;
#[doc = "Register `DENALI_PHY_471` writer"]
pub type W = crate::W<DenaliPhy471Spec>;
#[doc = "Field `PHY_MASTER_DELAY_STEP_3` reader - Incremental step size for master delay line locking algorithm for slice 3."]
pub type PhyMasterDelayStep3R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DELAY_STEP_3` writer - Incremental step size for master delay line locking algorithm for slice 3."]
pub type PhyMasterDelayStep3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_MASTER_DELAY_WAIT_3` reader - Wait cycles for master delay line locking algorithm for slice 3. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
pub type PhyMasterDelayWait3R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DELAY_WAIT_3` writer - Wait cycles for master delay line locking algorithm for slice 3. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
pub type PhyMasterDelayWait3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RPTR_UPDATE_3` reader - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 3."]
pub type PhyRptrUpdate3R = crate::FieldReader;
#[doc = "Field `PHY_RPTR_UPDATE_3` writer - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 3."]
pub type PhyRptrUpdate3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WRLVL_DLY_STEP_3` reader - DQS slave delay step size during write leveling for slice 3."]
pub type PhyWrlvlDlyStep3R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_DLY_STEP_3` writer - DQS slave delay step size during write leveling for slice 3."]
pub type PhyWrlvlDlyStep3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Incremental step size for master delay line locking algorithm for slice 3."]
    #[inline(always)]
    pub fn phy_master_delay_step_3(&self) -> PhyMasterDelayStep3R {
        PhyMasterDelayStep3R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - Wait cycles for master delay line locking algorithm for slice 3. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
    #[inline(always)]
    pub fn phy_master_delay_wait_3(&self) -> PhyMasterDelayWait3R {
        PhyMasterDelayWait3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 3."]
    #[inline(always)]
    pub fn phy_rptr_update_3(&self) -> PhyRptrUpdate3R {
        PhyRptrUpdate3R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DQS slave delay step size during write leveling for slice 3."]
    #[inline(always)]
    pub fn phy_wrlvl_dly_step_3(&self) -> PhyWrlvlDlyStep3R {
        PhyWrlvlDlyStep3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Incremental step size for master delay line locking algorithm for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_step_3(&mut self) -> PhyMasterDelayStep3W<DenaliPhy471Spec> {
        PhyMasterDelayStep3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Wait cycles for master delay line locking algorithm for slice 3. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_wait_3(&mut self) -> PhyMasterDelayWait3W<DenaliPhy471Spec> {
        PhyMasterDelayWait3W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rptr_update_3(&mut self) -> PhyRptrUpdate3W<DenaliPhy471Spec> {
        PhyRptrUpdate3W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DQS slave delay step size during write leveling for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_dly_step_3(&mut self) -> PhyWrlvlDlyStep3W<DenaliPhy471Spec> {
        PhyWrlvlDlyStep3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_471::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_471::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy471Spec;
impl crate::RegisterSpec for DenaliPhy471Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_471::R`](R) reader structure"]
impl crate::Readable for DenaliPhy471Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_471::W`](W) writer structure"]
impl crate::Writable for DenaliPhy471Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_471 to value 0"]
impl crate::Resettable for DenaliPhy471Spec {
    const RESET_VALUE: u32 = 0;
}
