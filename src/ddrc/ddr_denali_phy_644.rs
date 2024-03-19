#[doc = "Register `DDR_DENALI_PHY_644` reader"]
pub type R = crate::R<DdrDenaliPhy644Spec>;
#[doc = "Register `DDR_DENALI_PHY_644` writer"]
pub type W = crate::W<DdrDenaliPhy644Spec>;
#[doc = "Field `PHY_ADR_LPBK_ERROR_COUNT_OBS_1` reader - Observation register containing total number of loopback error data for address slice 1. READ-ONLY"]
pub type PhyAdrLpbkErrorCountObs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_SELECT_1` reader - Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 1."]
pub type PhyAdrMasterDlyLockObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_SELECT_1` writer - Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 1."]
pub type PhyAdrMasterDlyLockObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - Observation register containing total number of loopback error data for address slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_lpbk_error_count_obs_1(&self) -> PhyAdrLpbkErrorCountObs1R {
        PhyAdrLpbkErrorCountObs1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_master_dly_lock_obs_select_1(&self) -> PhyAdrMasterDlyLockObsSelect1R {
        PhyAdrMasterDlyLockObsSelect1R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_dly_lock_obs_select_1(
        &mut self,
    ) -> PhyAdrMasterDlyLockObsSelect1W<DdrDenaliPhy644Spec> {
        PhyAdrMasterDlyLockObsSelect1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_644::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_644::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy644Spec;
impl crate::RegisterSpec for DdrDenaliPhy644Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_644::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy644Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_644::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy644Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_644 to value 0"]
impl crate::Resettable for DdrDenaliPhy644Spec {
    const RESET_VALUE: u32 = 0;
}
