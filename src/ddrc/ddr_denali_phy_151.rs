#[doc = "Register `DDR_DENALI_PHY_151` reader"]
pub type R = crate::R<DdrDenaliPhy151Spec>;
#[doc = "Register `DDR_DENALI_PHY_151` writer"]
pub type W = crate::W<DdrDenaliPhy151Spec>;
#[doc = "Field `PHY_GTLVL_CAPTURE_CNT_1` reader - Number of samples to take at each DQS slave delay setting during gate training for slice 1."]
pub type PhyGtlvlCaptureCnt1R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_CAPTURE_CNT_1` writer - Number of samples to take at each DQS slave delay setting during gate training for slice 1."]
pub type PhyGtlvlCaptureCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_GTLVL_UPDT_WAIT_CNT_1` reader - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 1. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlUpdtWaitCnt1R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_UPDT_WAIT_CNT_1` writer - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 1. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlUpdtWaitCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_RDLVL_CAPTURE_CNT_1` reader - Number of samples to take at each DQS slave delay setting during read leveling for slice 1."]
pub type PhyRdlvlCaptureCnt1R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_CAPTURE_CNT_1` writer - Number of samples to take at each DQS slave delay setting during read leveling for slice 1."]
pub type PhyRdlvlCaptureCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_RDLVL_UPDT_WAIT_CNT_1` reader - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 1."]
pub type PhyRdlvlUpdtWaitCnt1R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_UPDT_WAIT_CNT_1` writer - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 1."]
pub type PhyRdlvlUpdtWaitCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Number of samples to take at each DQS slave delay setting during gate training for slice 1."]
    #[inline(always)]
    pub fn phy_gtlvl_capture_cnt_1(&self) -> PhyGtlvlCaptureCnt1R {
        PhyGtlvlCaptureCnt1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 1. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    pub fn phy_gtlvl_updt_wait_cnt_1(&self) -> PhyGtlvlUpdtWaitCnt1R {
        PhyGtlvlUpdtWaitCnt1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during read leveling for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_capture_cnt_1(&self) -> PhyRdlvlCaptureCnt1R {
        PhyRdlvlCaptureCnt1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_updt_wait_cnt_1(&self) -> PhyRdlvlUpdtWaitCnt1R {
        PhyRdlvlUpdtWaitCnt1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of samples to take at each DQS slave delay setting during gate training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_capture_cnt_1(&mut self) -> PhyGtlvlCaptureCnt1W<DdrDenaliPhy151Spec> {
        PhyGtlvlCaptureCnt1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 1. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_updt_wait_cnt_1(&mut self) -> PhyGtlvlUpdtWaitCnt1W<DdrDenaliPhy151Spec> {
        PhyGtlvlUpdtWaitCnt1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during read leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_capture_cnt_1(&mut self) -> PhyRdlvlCaptureCnt1W<DdrDenaliPhy151Spec> {
        PhyRdlvlCaptureCnt1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_updt_wait_cnt_1(&mut self) -> PhyRdlvlUpdtWaitCnt1W<DdrDenaliPhy151Spec> {
        PhyRdlvlUpdtWaitCnt1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_151::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_151::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy151Spec;
impl crate::RegisterSpec for DdrDenaliPhy151Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_151::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy151Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_151::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy151Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_151 to value 0"]
impl crate::Resettable for DdrDenaliPhy151Spec {
    const RESET_VALUE: u32 = 0;
}
