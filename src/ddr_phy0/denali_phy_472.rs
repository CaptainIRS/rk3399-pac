#[doc = "Register `DENALI_PHY_472` reader"]
pub type R = crate::R<DenaliPhy472Spec>;
#[doc = "Register `DENALI_PHY_472` writer"]
pub type W = crate::W<DenaliPhy472Spec>;
#[doc = "Field `PHY_WRLVL_RESP_WAIT_CNT_3` reader - Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 3."]
pub type PhyWrlvlRespWaitCnt3R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_RESP_WAIT_CNT_3` writer - Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 3."]
pub type PhyWrlvlRespWaitCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_GTLVL_DLY_STEP_3` reader - DQS slave delay step size during gate training for slice 3."]
pub type PhyGtlvlDlyStep3R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_DLY_STEP_3` writer - DQS slave delay step size during gate training for slice 3."]
pub type PhyGtlvlDlyStep3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_GTLVL_RESP_WAIT_CNT_3` reader - Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 3. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlRespWaitCnt3R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_RESP_WAIT_CNT_3` writer - Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 3. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlRespWaitCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 3."]
    #[inline(always)]
    pub fn phy_wrlvl_resp_wait_cnt_3(&self) -> PhyWrlvlRespWaitCnt3R {
        PhyWrlvlRespWaitCnt3R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - DQS slave delay step size during gate training for slice 3."]
    #[inline(always)]
    pub fn phy_gtlvl_dly_step_3(&self) -> PhyGtlvlDlyStep3R {
        PhyGtlvlDlyStep3R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 3. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    pub fn phy_gtlvl_resp_wait_cnt_3(&self) -> PhyGtlvlRespWaitCnt3R {
        PhyGtlvlRespWaitCnt3R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_resp_wait_cnt_3(&mut self) -> PhyWrlvlRespWaitCnt3W<DenaliPhy472Spec> {
        PhyWrlvlRespWaitCnt3W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQS slave delay step size during gate training for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_dly_step_3(&mut self) -> PhyGtlvlDlyStep3W<DenaliPhy472Spec> {
        PhyGtlvlDlyStep3W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 3. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_resp_wait_cnt_3(&mut self) -> PhyGtlvlRespWaitCnt3W<DenaliPhy472Spec> {
        PhyGtlvlRespWaitCnt3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_472::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_472::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy472Spec;
impl crate::RegisterSpec for DenaliPhy472Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_472::R`](R) reader structure"]
impl crate::Readable for DenaliPhy472Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_472::W`](W) writer structure"]
impl crate::Writable for DenaliPhy472Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_472 to value 0"]
impl crate::Resettable for DenaliPhy472Spec {
    const RESET_VALUE: u32 = 0;
}
