#[doc = "Register `DENALI_PHY_804` reader"]
pub type R = crate::R<DenaliPhy804Spec>;
#[doc = "Register `DENALI_PHY_804` writer"]
pub type W = crate::W<DenaliPhy804Spec>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_START_2` reader - Start value for master delay line locking algorithm for address slice 2."]
pub type PhyAdrMasterDelayStart2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_START_2` writer - Start value for master delay line locking algorithm for address slice 2."]
pub type PhyAdrMasterDelayStart2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_STEP_2` reader - Incremental step size for master delay line locking algorithm for address slice 2."]
pub type PhyAdrMasterDelayStep2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DELAY_STEP_2` writer - Incremental step size for master delay line locking algorithm for address slice 2."]
pub type PhyAdrMasterDelayStep2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_WAIT_2` reader - Wait cycles for master delay line locking algorithm for address slice 2. Bits (3:0) is the cycle wait count after a calibration clock setting change. Bits (7:4) is the cycle wait count after a master delay setting change."]
pub type PhyAdrMasterDelayWait2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DELAY_WAIT_2` writer - Wait cycles for master delay line locking algorithm for address slice 2. Bits (3:0) is the cycle wait count after a calibration clock setting change. Bits (7:4) is the cycle wait count after a master delay setting change."]
pub type PhyAdrMasterDelayWait2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:9 - Start value for master delay line locking algorithm for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_master_delay_start_2(&self) -> PhyAdrMasterDelayStart2R {
        PhyAdrMasterDelayStart2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - Incremental step size for master delay line locking algorithm for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_master_delay_step_2(&self) -> PhyAdrMasterDelayStep2R {
        PhyAdrMasterDelayStep2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - Wait cycles for master delay line locking algorithm for address slice 2. Bits (3:0) is the cycle wait count after a calibration clock setting change. Bits (7:4) is the cycle wait count after a master delay setting change."]
    #[inline(always)]
    pub fn phy_adr_master_delay_wait_2(&self) -> PhyAdrMasterDelayWait2R {
        PhyAdrMasterDelayWait2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Start value for master delay line locking algorithm for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_start_2(&mut self) -> PhyAdrMasterDelayStart2W<DenaliPhy804Spec> {
        PhyAdrMasterDelayStart2W::new(self, 0)
    }
    #[doc = "Bits 16:21 - Incremental step size for master delay line locking algorithm for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_step_2(&mut self) -> PhyAdrMasterDelayStep2W<DenaliPhy804Spec> {
        PhyAdrMasterDelayStep2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Wait cycles for master delay line locking algorithm for address slice 2. Bits (3:0) is the cycle wait count after a calibration clock setting change. Bits (7:4) is the cycle wait count after a master delay setting change."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_wait_2(&mut self) -> PhyAdrMasterDelayWait2W<DenaliPhy804Spec> {
        PhyAdrMasterDelayWait2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_804::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_804::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy804Spec;
impl crate::RegisterSpec for DenaliPhy804Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_804::R`](R) reader structure"]
impl crate::Readable for DenaliPhy804Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_804::W`](W) writer structure"]
impl crate::Writable for DenaliPhy804Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_804 to value 0"]
impl crate::Resettable for DenaliPhy804Spec {
    const RESET_VALUE: u32 = 0;
}
