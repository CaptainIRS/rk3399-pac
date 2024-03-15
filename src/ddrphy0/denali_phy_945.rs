#[doc = "Register `DENALI_PHY_945` writer"]
pub type W = crate::W<DenaliPhy945Spec>;
#[doc = "Field `PHY_CAL_CLEAR_0` writer - Clear the pad calibration state machine and results for block 0. Set to 1 to trigger. WRITE-ONLY"]
pub type PhyCalClear0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CAL_START_0` writer - Manual start for the pad calibration state machine for block 0. Set to 1 to trigger. WRITE-ONLY"]
pub type PhyCalStart0W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear the pad calibration state machine and results for block 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_clear_0(&mut self) -> PhyCalClear0W<DenaliPhy945Spec> {
        PhyCalClear0W::new(self, 0)
    }
    #[doc = "Bit 8 - Manual start for the pad calibration state machine for block 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_start_0(&mut self) -> PhyCalStart0W<DenaliPhy945Spec> {
        PhyCalStart0W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_945::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy945Spec;
impl crate::RegisterSpec for DenaliPhy945Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`denali_phy_945::W`](W) writer structure"]
impl crate::Writable for DenaliPhy945Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_945 to value 0"]
impl crate::Resettable for DenaliPhy945Spec {
    const RESET_VALUE: u32 = 0;
}
