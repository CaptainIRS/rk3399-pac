#[doc = "Register `DDR_DENALI_PHY_87` reader"]
pub type R = crate::R<DdrDenaliPhy87Spec>;
#[doc = "Register `DDR_DENALI_PHY_87` writer"]
pub type W = crate::W<DdrDenaliPhy87Spec>;
#[doc = "Field `PHY_MASTER_DELAY_STEP_0` reader - Incremental step size for master delay line locking algorithm for slice 0."]
pub type PhyMasterDelayStep0R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DELAY_STEP_0` writer - Incremental step size for master delay line locking algorithm for slice 0."]
pub type PhyMasterDelayStep0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_MASTER_DELAY_WAIT_0` reader - Wait cycles for master delay line locking algorithm for slice 0. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
pub type PhyMasterDelayWait0R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DELAY_WAIT_0` writer - Wait cycles for master delay line locking algorithm for slice 0. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
pub type PhyMasterDelayWait0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RPTR_UPDATE_0` reader - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 0."]
pub type PhyRptrUpdate0R = crate::FieldReader;
#[doc = "Field `PHY_RPTR_UPDATE_0` writer - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 0."]
pub type PhyRptrUpdate0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WRLVL_DLY_STEP_0` reader - DQS slave delay step size during write leveling for slice 0."]
pub type PhyWrlvlDlyStep0R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_DLY_STEP_0` writer - DQS slave delay step size during write leveling for slice 0."]
pub type PhyWrlvlDlyStep0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Incremental step size for master delay line locking algorithm for slice 0."]
    #[inline(always)]
    pub fn phy_master_delay_step_0(&self) -> PhyMasterDelayStep0R {
        PhyMasterDelayStep0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - Wait cycles for master delay line locking algorithm for slice 0. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
    #[inline(always)]
    pub fn phy_master_delay_wait_0(&self) -> PhyMasterDelayWait0R {
        PhyMasterDelayWait0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 0."]
    #[inline(always)]
    pub fn phy_rptr_update_0(&self) -> PhyRptrUpdate0R {
        PhyRptrUpdate0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DQS slave delay step size during write leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_dly_step_0(&self) -> PhyWrlvlDlyStep0R {
        PhyWrlvlDlyStep0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Incremental step size for master delay line locking algorithm for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_step_0(&mut self) -> PhyMasterDelayStep0W<DdrDenaliPhy87Spec> {
        PhyMasterDelayStep0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Wait cycles for master delay line locking algorithm for slice 0. Bits (3:0) are the cycle wait count after a calibration clock setting change. Bits (7:4) are the cycle wait count after a master delay setting change."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_wait_0(&mut self) -> PhyMasterDelayWait0W<DdrDenaliPhy87Spec> {
        PhyMasterDelayWait0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rptr_update_0(&mut self) -> PhyRptrUpdate0W<DdrDenaliPhy87Spec> {
        PhyRptrUpdate0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DQS slave delay step size during write leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_dly_step_0(&mut self) -> PhyWrlvlDlyStep0W<DdrDenaliPhy87Spec> {
        PhyWrlvlDlyStep0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_87::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_87::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy87Spec;
impl crate::RegisterSpec for DdrDenaliPhy87Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_87::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy87Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_87::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy87Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_87 to value 0"]
impl crate::Resettable for DdrDenaliPhy87Spec {
    const RESET_VALUE: u32 = 0;
}
