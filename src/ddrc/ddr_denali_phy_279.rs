#[doc = "Register `DDR_DENALI_PHY_279` reader"]
pub type R = crate::R<DdrDenaliPhy279Spec>;
#[doc = "Register `DDR_DENALI_PHY_279` writer"]
pub type W = crate::W<DdrDenaliPhy279Spec>;
#[doc = "Field `PHY_GTLVL_CAPTURE_CNT_2` reader - Number of samples to take at each DQS slave delay setting during gate training for slice 2."]
pub type PhyGtlvlCaptureCnt2R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_CAPTURE_CNT_2` writer - Number of samples to take at each DQS slave delay setting during gate training for slice 2."]
pub type PhyGtlvlCaptureCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_GTLVL_UPDT_WAIT_CNT_2` reader - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 2. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlUpdtWaitCnt2R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_UPDT_WAIT_CNT_2` writer - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 2. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlUpdtWaitCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_RDLVL_CAPTURE_CNT_2` reader - Number of samples to take at each DQS slave delay setting during read leveling for slice 2."]
pub type PhyRdlvlCaptureCnt2R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_CAPTURE_CNT_2` writer - Number of samples to take at each DQS slave delay setting during read leveling for slice 2."]
pub type PhyRdlvlCaptureCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_RDLVL_UPDT_WAIT_CNT_2` reader - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 2."]
pub type PhyRdlvlUpdtWaitCnt2R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_UPDT_WAIT_CNT_2` writer - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 2."]
pub type PhyRdlvlUpdtWaitCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Number of samples to take at each DQS slave delay setting during gate training for slice 2."]
    #[inline(always)]
    pub fn phy_gtlvl_capture_cnt_2(&self) -> PhyGtlvlCaptureCnt2R {
        PhyGtlvlCaptureCnt2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 2. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    pub fn phy_gtlvl_updt_wait_cnt_2(&self) -> PhyGtlvlUpdtWaitCnt2R {
        PhyGtlvlUpdtWaitCnt2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during read leveling for slice 2."]
    #[inline(always)]
    pub fn phy_rdlvl_capture_cnt_2(&self) -> PhyRdlvlCaptureCnt2R {
        PhyRdlvlCaptureCnt2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 2."]
    #[inline(always)]
    pub fn phy_rdlvl_updt_wait_cnt_2(&self) -> PhyRdlvlUpdtWaitCnt2R {
        PhyRdlvlUpdtWaitCnt2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of samples to take at each DQS slave delay setting during gate training for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_capture_cnt_2(&mut self) -> PhyGtlvlCaptureCnt2W<DdrDenaliPhy279Spec> {
        PhyGtlvlCaptureCnt2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 2. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_updt_wait_cnt_2(&mut self) -> PhyGtlvlUpdtWaitCnt2W<DdrDenaliPhy279Spec> {
        PhyGtlvlUpdtWaitCnt2W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during read leveling for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_capture_cnt_2(&mut self) -> PhyRdlvlCaptureCnt2W<DdrDenaliPhy279Spec> {
        PhyRdlvlCaptureCnt2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_updt_wait_cnt_2(&mut self) -> PhyRdlvlUpdtWaitCnt2W<DdrDenaliPhy279Spec> {
        PhyRdlvlUpdtWaitCnt2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_279::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_279::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy279Spec;
impl crate::RegisterSpec for DdrDenaliPhy279Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_279::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy279Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_279::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy279Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_279 to value 0"]
impl crate::Resettable for DdrDenaliPhy279Spec {
    const RESET_VALUE: u32 = 0;
}
