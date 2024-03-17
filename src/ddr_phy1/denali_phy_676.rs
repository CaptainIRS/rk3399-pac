#[doc = "Register `DENALI_PHY_676` reader"]
pub type R = crate::R<DenaliPhy676Spec>;
#[doc = "Register `DENALI_PHY_676` writer"]
pub type W = crate::W<DenaliPhy676Spec>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_START_1` reader - Start value for master delay line locking algorithm for address slice 1."]
pub type PhyAdrMasterDelayStart1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_START_1` writer - Start value for master delay line locking algorithm for address slice 1."]
pub type PhyAdrMasterDelayStart1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_STEP_1` reader - Incremental step size for master delay line locking algorithm for address slice 1."]
pub type PhyAdrMasterDelayStep1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DELAY_STEP_1` writer - Incremental step size for master delay line locking algorithm for address slice 1."]
pub type PhyAdrMasterDelayStep1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_WAIT_1` reader - Wait cycles for master delay line locking algorithm for address slice 1. Bits (3:0) is the cycle wait count after a calibration clock setting change. Bits (7:4) is the cycle wait count after a master delay setting change."]
pub type PhyAdrMasterDelayWait1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DELAY_WAIT_1` writer - Wait cycles for master delay line locking algorithm for address slice 1. Bits (3:0) is the cycle wait count after a calibration clock setting change. Bits (7:4) is the cycle wait count after a master delay setting change."]
pub type PhyAdrMasterDelayWait1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:9 - Start value for master delay line locking algorithm for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_master_delay_start_1(&self) -> PhyAdrMasterDelayStart1R {
        PhyAdrMasterDelayStart1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - Incremental step size for master delay line locking algorithm for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_master_delay_step_1(&self) -> PhyAdrMasterDelayStep1R {
        PhyAdrMasterDelayStep1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - Wait cycles for master delay line locking algorithm for address slice 1. Bits (3:0) is the cycle wait count after a calibration clock setting change. Bits (7:4) is the cycle wait count after a master delay setting change."]
    #[inline(always)]
    pub fn phy_adr_master_delay_wait_1(&self) -> PhyAdrMasterDelayWait1R {
        PhyAdrMasterDelayWait1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Start value for master delay line locking algorithm for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_start_1(&mut self) -> PhyAdrMasterDelayStart1W<DenaliPhy676Spec> {
        PhyAdrMasterDelayStart1W::new(self, 0)
    }
    #[doc = "Bits 16:21 - Incremental step size for master delay line locking algorithm for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_step_1(&mut self) -> PhyAdrMasterDelayStep1W<DenaliPhy676Spec> {
        PhyAdrMasterDelayStep1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Wait cycles for master delay line locking algorithm for address slice 1. Bits (3:0) is the cycle wait count after a calibration clock setting change. Bits (7:4) is the cycle wait count after a master delay setting change."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_wait_1(&mut self) -> PhyAdrMasterDelayWait1W<DenaliPhy676Spec> {
        PhyAdrMasterDelayWait1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_676::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_676::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy676Spec;
impl crate::RegisterSpec for DenaliPhy676Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_676::R`](R) reader structure"]
impl crate::Readable for DenaliPhy676Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_676::W`](W) writer structure"]
impl crate::Writable for DenaliPhy676Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_676 to value 0"]
impl crate::Resettable for DenaliPhy676Spec {
    const RESET_VALUE: u32 = 0;
}
