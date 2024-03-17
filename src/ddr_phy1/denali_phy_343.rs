#[doc = "Register `DENALI_PHY_343` reader"]
pub type R = crate::R<DenaliPhy343Spec>;
#[doc = "Register `DENALI_PHY_343` writer"]
pub type W = crate::W<DenaliPhy343Spec>;
#[doc = "Field `PHY_MASTER_DELAY_STEP_2` reader - Incremental step size for master delay line locking algorithm for slice 2."]
pub type PhyMasterDelayStep2R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DELAY_STEP_2` writer - Incremental step size for master delay line locking algorithm for slice 2."]
pub type PhyMasterDelayStep2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_MASTER_DELAY_WAIT_2` reader - Wait cycles for master delay line locking algorithm for slice 2. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
pub type PhyMasterDelayWait2R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DELAY_WAIT_2` writer - Wait cycles for master delay line locking algorithm for slice 2. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
pub type PhyMasterDelayWait2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RPTR_UPDATE_2` reader - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 2."]
pub type PhyRptrUpdate2R = crate::FieldReader;
#[doc = "Field `PHY_RPTR_UPDATE_2` writer - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 2."]
pub type PhyRptrUpdate2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WRLVL_DLY_STEP_2` reader - DQS slave delay step size during write leveling for slice 2."]
pub type PhyWrlvlDlyStep2R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_DLY_STEP_2` writer - DQS slave delay step size during write leveling for slice 2."]
pub type PhyWrlvlDlyStep2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Incremental step size for master delay line locking algorithm for slice 2."]
    #[inline(always)]
    pub fn phy_master_delay_step_2(&self) -> PhyMasterDelayStep2R {
        PhyMasterDelayStep2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - Wait cycles for master delay line locking algorithm for slice 2. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
    #[inline(always)]
    pub fn phy_master_delay_wait_2(&self) -> PhyMasterDelayWait2R {
        PhyMasterDelayWait2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 2."]
    #[inline(always)]
    pub fn phy_rptr_update_2(&self) -> PhyRptrUpdate2R {
        PhyRptrUpdate2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DQS slave delay step size during write leveling for slice 2."]
    #[inline(always)]
    pub fn phy_wrlvl_dly_step_2(&self) -> PhyWrlvlDlyStep2R {
        PhyWrlvlDlyStep2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Incremental step size for master delay line locking algorithm for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_step_2(&mut self) -> PhyMasterDelayStep2W<DenaliPhy343Spec> {
        PhyMasterDelayStep2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Wait cycles for master delay line locking algorithm for slice 2. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_wait_2(&mut self) -> PhyMasterDelayWait2W<DenaliPhy343Spec> {
        PhyMasterDelayWait2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rptr_update_2(&mut self) -> PhyRptrUpdate2W<DenaliPhy343Spec> {
        PhyRptrUpdate2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DQS slave delay step size during write leveling for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_dly_step_2(&mut self) -> PhyWrlvlDlyStep2W<DenaliPhy343Spec> {
        PhyWrlvlDlyStep2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_343::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_343::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy343Spec;
impl crate::RegisterSpec for DenaliPhy343Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_343::R`](R) reader structure"]
impl crate::Readable for DenaliPhy343Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_343::W`](W) writer structure"]
impl crate::Writable for DenaliPhy343Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_343 to value 0"]
impl crate::Resettable for DenaliPhy343Spec {
    const RESET_VALUE: u32 = 0;
}
