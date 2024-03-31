#[doc = "Register `DENALI_PHY_772` reader"]
pub type R = crate::R<DenaliPhy772Spec>;
#[doc = "Register `DENALI_PHY_772` writer"]
pub type W = crate::W<DenaliPhy772Spec>;
#[doc = "Field `PHY_ADR_LPBK_ERROR_COUNT_OBS_2` reader - Observation register containing total number of loopback error data for address slice 2."]
pub type PhyAdrLpbkErrorCountObs2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_SELECT_2` reader - Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 2."]
pub type PhyAdrMasterDlyLockObsSelect2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_SELECT_2` writer - Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 2."]
pub type PhyAdrMasterDlyLockObsSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - Observation register containing total number of loopback error data for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_lpbk_error_count_obs_2(&self) -> PhyAdrLpbkErrorCountObs2R {
        PhyAdrLpbkErrorCountObs2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_master_dly_lock_obs_select_2(&self) -> PhyAdrMasterDlyLockObsSelect2R {
        PhyAdrMasterDlyLockObsSelect2R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_dly_lock_obs_select_2(
        &mut self,
    ) -> PhyAdrMasterDlyLockObsSelect2W<DenaliPhy772Spec> {
        PhyAdrMasterDlyLockObsSelect2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_772::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_772::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy772Spec;
impl crate::RegisterSpec for DenaliPhy772Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_772::R`](R) reader structure"]
impl crate::Readable for DenaliPhy772Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_772::W`](W) writer structure"]
impl crate::Writable for DenaliPhy772Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_772 to value 0"]
impl crate::Resettable for DenaliPhy772Spec {
    const RESET_VALUE: u32 = 0;
}
