#[doc = "Register `DDR_DENALI_PHY_344` reader"]
pub type R = crate::R<DdrDenaliPhy344Spec>;
#[doc = "Register `DDR_DENALI_PHY_344` writer"]
pub type W = crate::W<DdrDenaliPhy344Spec>;
#[doc = "Field `PHY_WRLVL_RESP_WAIT_CNT_2` reader - Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 2."]
pub type PhyWrlvlRespWaitCnt2R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_RESP_WAIT_CNT_2` writer - Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 2."]
pub type PhyWrlvlRespWaitCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_GTLVL_DLY_STEP_2` reader - DQS slave delay step size during gate training for slice 2."]
pub type PhyGtlvlDlyStep2R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_DLY_STEP_2` writer - DQS slave delay step size during gate training for slice 2."]
pub type PhyGtlvlDlyStep2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_GTLVL_RESP_WAIT_CNT_2` reader - Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 2. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlRespWaitCnt2R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_RESP_WAIT_CNT_2` writer - Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 2. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlRespWaitCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 2."]
    #[inline(always)]
    pub fn phy_wrlvl_resp_wait_cnt_2(&self) -> PhyWrlvlRespWaitCnt2R {
        PhyWrlvlRespWaitCnt2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - DQS slave delay step size during gate training for slice 2."]
    #[inline(always)]
    pub fn phy_gtlvl_dly_step_2(&self) -> PhyGtlvlDlyStep2R {
        PhyGtlvlDlyStep2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 2. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    pub fn phy_gtlvl_resp_wait_cnt_2(&self) -> PhyGtlvlRespWaitCnt2R {
        PhyGtlvlRespWaitCnt2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_resp_wait_cnt_2(&mut self) -> PhyWrlvlRespWaitCnt2W<DdrDenaliPhy344Spec> {
        PhyWrlvlRespWaitCnt2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQS slave delay step size during gate training for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_dly_step_2(&mut self) -> PhyGtlvlDlyStep2W<DdrDenaliPhy344Spec> {
        PhyGtlvlDlyStep2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 2. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_resp_wait_cnt_2(&mut self) -> PhyGtlvlRespWaitCnt2W<DdrDenaliPhy344Spec> {
        PhyGtlvlRespWaitCnt2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_344::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_344::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy344Spec;
impl crate::RegisterSpec for DdrDenaliPhy344Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_344::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy344Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_344::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy344Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_344 to value 0"]
impl crate::Resettable for DdrDenaliPhy344Spec {
    const RESET_VALUE: u32 = 0;
}
