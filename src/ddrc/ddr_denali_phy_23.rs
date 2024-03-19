#[doc = "Register `DDR_DENALI_PHY_23` reader"]
pub type R = crate::R<DdrDenaliPhy23Spec>;
#[doc = "Register `DDR_DENALI_PHY_23` writer"]
pub type W = crate::W<DdrDenaliPhy23Spec>;
#[doc = "Field `PHY_GTLVL_CAPTURE_CNT_0` reader - Number of samples to take at each DQS slave delay setting during gate training for slice 0."]
pub type PhyGtlvlCaptureCnt0R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_CAPTURE_CNT_0` writer - Number of samples to take at each DQS slave delay setting during gate training for slice 0."]
pub type PhyGtlvlCaptureCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_GTLVL_UPDT_WAIT_CNT_0` reader - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 0. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlUpdtWaitCnt0R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_UPDT_WAIT_CNT_0` writer - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 0. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlUpdtWaitCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_RDLVL_CAPTURE_CNT_0` reader - Number of samples to take at each DQS slave delay setting during read leveling for slice 0."]
pub type PhyRdlvlCaptureCnt0R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_CAPTURE_CNT_0` writer - Number of samples to take at each DQS slave delay setting during read leveling for slice 0."]
pub type PhyRdlvlCaptureCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_RDLVL_UPDT_WAIT_CNT_0` reader - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 0."]
pub type PhyRdlvlUpdtWaitCnt0R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_UPDT_WAIT_CNT_0` writer - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 0."]
pub type PhyRdlvlUpdtWaitCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Number of samples to take at each DQS slave delay setting during gate training for slice 0."]
    #[inline(always)]
    pub fn phy_gtlvl_capture_cnt_0(&self) -> PhyGtlvlCaptureCnt0R {
        PhyGtlvlCaptureCnt0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 0. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    pub fn phy_gtlvl_updt_wait_cnt_0(&self) -> PhyGtlvlUpdtWaitCnt0R {
        PhyGtlvlUpdtWaitCnt0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during read leveling for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_capture_cnt_0(&self) -> PhyRdlvlCaptureCnt0R {
        PhyRdlvlCaptureCnt0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_updt_wait_cnt_0(&self) -> PhyRdlvlUpdtWaitCnt0R {
        PhyRdlvlUpdtWaitCnt0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of samples to take at each DQS slave delay setting during gate training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_capture_cnt_0(&mut self) -> PhyGtlvlCaptureCnt0W<DdrDenaliPhy23Spec> {
        PhyGtlvlCaptureCnt0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 0. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_updt_wait_cnt_0(&mut self) -> PhyGtlvlUpdtWaitCnt0W<DdrDenaliPhy23Spec> {
        PhyGtlvlUpdtWaitCnt0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during read leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_capture_cnt_0(&mut self) -> PhyRdlvlCaptureCnt0W<DdrDenaliPhy23Spec> {
        PhyRdlvlCaptureCnt0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_updt_wait_cnt_0(&mut self) -> PhyRdlvlUpdtWaitCnt0W<DdrDenaliPhy23Spec> {
        PhyRdlvlUpdtWaitCnt0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy23Spec;
impl crate::RegisterSpec for DdrDenaliPhy23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_23::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy23Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_23::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_23 to value 0"]
impl crate::Resettable for DdrDenaliPhy23Spec {
    const RESET_VALUE: u32 = 0;
}
