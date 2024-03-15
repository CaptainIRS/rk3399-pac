#[doc = "Register `DENALI_PHY_407` reader"]
pub type R = crate::R<DenaliPhy407Spec>;
#[doc = "Register `DENALI_PHY_407` writer"]
pub type W = crate::W<DenaliPhy407Spec>;
#[doc = "Field `PHY_GTLVL_CAPTURE_CNT_3` reader - Number of samples to take at each DQS slave delay setting during gate training for slice 3."]
pub type PhyGtlvlCaptureCnt3R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_CAPTURE_CNT_3` writer - Number of samples to take at each DQS slave delay setting during gate training for slice 3."]
pub type PhyGtlvlCaptureCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_GTLVL_UPDT_WAIT_CNT_3` reader - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 3. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlUpdtWaitCnt3R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_UPDT_WAIT_CNT_3` writer - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 3. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlUpdtWaitCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_RDLVL_CAPTURE_CNT_3` reader - Number of samples to take at each DQS slave delay setting during read leveling for slice 3."]
pub type PhyRdlvlCaptureCnt3R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_CAPTURE_CNT_3` writer - Number of samples to take at each DQS slave delay setting during read leveling for slice 3."]
pub type PhyRdlvlCaptureCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_RDLVL_UPDT_WAIT_CNT_3` reader - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 3."]
pub type PhyRdlvlUpdtWaitCnt3R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_UPDT_WAIT_CNT_3` writer - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 3."]
pub type PhyRdlvlUpdtWaitCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Number of samples to take at each DQS slave delay setting during gate training for slice 3."]
    #[inline(always)]
    pub fn phy_gtlvl_capture_cnt_3(&self) -> PhyGtlvlCaptureCnt3R {
        PhyGtlvlCaptureCnt3R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 3. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    pub fn phy_gtlvl_updt_wait_cnt_3(&self) -> PhyGtlvlUpdtWaitCnt3R {
        PhyGtlvlUpdtWaitCnt3R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during read leveling for slice 3."]
    #[inline(always)]
    pub fn phy_rdlvl_capture_cnt_3(&self) -> PhyRdlvlCaptureCnt3R {
        PhyRdlvlCaptureCnt3R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 3."]
    #[inline(always)]
    pub fn phy_rdlvl_updt_wait_cnt_3(&self) -> PhyRdlvlUpdtWaitCnt3R {
        PhyRdlvlUpdtWaitCnt3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of samples to take at each DQS slave delay setting during gate training for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_capture_cnt_3(&mut self) -> PhyGtlvlCaptureCnt3W<DenaliPhy407Spec> {
        PhyGtlvlCaptureCnt3W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 3. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_updt_wait_cnt_3(&mut self) -> PhyGtlvlUpdtWaitCnt3W<DenaliPhy407Spec> {
        PhyGtlvlUpdtWaitCnt3W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during read leveling for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_capture_cnt_3(&mut self) -> PhyRdlvlCaptureCnt3W<DenaliPhy407Spec> {
        PhyRdlvlCaptureCnt3W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_updt_wait_cnt_3(&mut self) -> PhyRdlvlUpdtWaitCnt3W<DenaliPhy407Spec> {
        PhyRdlvlUpdtWaitCnt3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_407::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_407::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy407Spec;
impl crate::RegisterSpec for DenaliPhy407Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_407::R`](R) reader structure"]
impl crate::Readable for DenaliPhy407Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_407::W`](W) writer structure"]
impl crate::Writable for DenaliPhy407Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_407 to value 0"]
impl crate::Resettable for DenaliPhy407Spec {
    const RESET_VALUE: u32 = 0;
}
