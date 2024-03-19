#[doc = "Register `DDR_DENALI_PHY_516` reader"]
pub type R = crate::R<DdrDenaliPhy516Spec>;
#[doc = "Register `DDR_DENALI_PHY_516` writer"]
pub type W = crate::W<DdrDenaliPhy516Spec>;
#[doc = "Field `PHY_ADR_LPBK_ERROR_COUNT_OBS_0` reader - Observation register containing total number of loopback error data for address slice 0."]
pub type PhyAdrLpbkErrorCountObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_SELECT_0` reader - Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 0."]
pub type PhyAdrMasterDlyLockObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_SELECT_0` writer - Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 0."]
pub type PhyAdrMasterDlyLockObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - Observation register containing total number of loopback error data for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_lpbk_error_count_obs_0(&self) -> PhyAdrLpbkErrorCountObs0R {
        PhyAdrLpbkErrorCountObs0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_master_dly_lock_obs_select_0(&self) -> PhyAdrMasterDlyLockObsSelect0R {
        PhyAdrMasterDlyLockObsSelect0R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_dly_lock_obs_select_0(
        &mut self,
    ) -> PhyAdrMasterDlyLockObsSelect0W<DdrDenaliPhy516Spec> {
        PhyAdrMasterDlyLockObsSelect0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_516::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_516::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy516Spec;
impl crate::RegisterSpec for DdrDenaliPhy516Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_516::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy516Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_516::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy516Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_516 to value 0"]
impl crate::Resettable for DdrDenaliPhy516Spec {
    const RESET_VALUE: u32 = 0;
}
