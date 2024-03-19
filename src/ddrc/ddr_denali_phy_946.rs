#[doc = "Register `DDR_DENALI_PHY_946` reader"]
pub type R = crate::R<DdrDenaliPhy946Spec>;
#[doc = "Register `DDR_DENALI_PHY_946` writer"]
pub type W = crate::W<DdrDenaliPhy946Spec>;
#[doc = "Field `PHY_CAL_INTERVAL_COUNT_0` reader - Pad calibration interval counter compare value for block 0."]
pub type PhyCalIntervalCount0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_CAL_INTERVAL_COUNT_0` writer - Pad calibration interval counter compare value for block 0."]
pub type PhyCalIntervalCount0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pad calibration interval counter compare value for block 0."]
    #[inline(always)]
    pub fn phy_cal_interval_count_0(&self) -> PhyCalIntervalCount0R {
        PhyCalIntervalCount0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pad calibration interval counter compare value for block 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_interval_count_0(&mut self) -> PhyCalIntervalCount0W<DdrDenaliPhy946Spec> {
        PhyCalIntervalCount0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_946::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_946::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy946Spec;
impl crate::RegisterSpec for DdrDenaliPhy946Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_946::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy946Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_946::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy946Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_946 to value 0"]
impl crate::Resettable for DdrDenaliPhy946Spec {
    const RESET_VALUE: u32 = 0;
}
