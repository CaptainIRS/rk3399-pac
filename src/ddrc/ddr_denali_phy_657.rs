#[doc = "Register `DDR_DENALI_PHY_657` reader"]
pub type R = crate::R<DdrDenaliPhy657Spec>;
#[doc = "Register `DDR_DENALI_PHY_657` writer"]
pub type W = crate::W<DdrDenaliPhy657Spec>;
#[doc = "Field `PHY_ADR_CALVL_RANK_CTRL_1` reader - Defines the CA training rank control bits for address slice 1."]
pub type PhyAdrCalvlRankCtrl1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_RANK_CTRL_1` writer - Defines the CA training rank control bits for address slice 1."]
pub type PhyAdrCalvlRankCtrl1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_CALVL_NUM_PATTERNS_1` reader - Sets the number of patterns to use during CA training for address slice 1."]
pub type PhyAdrCalvlNumPatterns1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_NUM_PATTERNS_1` writer - Sets the number of patterns to use during CA training for address slice 1."]
pub type PhyAdrCalvlNumPatterns1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_CALVL_CAPTURE_CNT_1` reader - Number of samples to take at each ADDR slave delay setting during CA training for address slice 1."]
pub type PhyAdrCalvlCaptureCnt1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_CAPTURE_CNT_1` writer - Number of samples to take at each ADDR slave delay setting during CA training for address slice 1."]
pub type PhyAdrCalvlCaptureCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADR_CALVL_RESP_WAIT_CNT_1` reader - Number of samples to wait for response during CA training for address slice 1."]
pub type PhyAdrCalvlRespWaitCnt1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_RESP_WAIT_CNT_1` writer - Number of samples to wait for response during CA training for address slice 1."]
pub type PhyAdrCalvlRespWaitCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Defines the CA training rank control bits for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_rank_ctrl_1(&self) -> PhyAdrCalvlRankCtrl1R {
        PhyAdrCalvlRankCtrl1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets the number of patterns to use during CA training for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_num_patterns_1(&self) -> PhyAdrCalvlNumPatterns1R {
        PhyAdrCalvlNumPatterns1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Number of samples to take at each ADDR slave delay setting during CA training for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_capture_cnt_1(&self) -> PhyAdrCalvlCaptureCnt1R {
        PhyAdrCalvlCaptureCnt1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Number of samples to wait for response during CA training for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_resp_wait_cnt_1(&self) -> PhyAdrCalvlRespWaitCnt1R {
        PhyAdrCalvlRespWaitCnt1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines the CA training rank control bits for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_rank_ctrl_1(&mut self) -> PhyAdrCalvlRankCtrl1W<DdrDenaliPhy657Spec> {
        PhyAdrCalvlRankCtrl1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Sets the number of patterns to use during CA training for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_num_patterns_1(
        &mut self,
    ) -> PhyAdrCalvlNumPatterns1W<DdrDenaliPhy657Spec> {
        PhyAdrCalvlNumPatterns1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Number of samples to take at each ADDR slave delay setting during CA training for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_capture_cnt_1(&mut self) -> PhyAdrCalvlCaptureCnt1W<DdrDenaliPhy657Spec> {
        PhyAdrCalvlCaptureCnt1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of samples to wait for response during CA training for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_resp_wait_cnt_1(
        &mut self,
    ) -> PhyAdrCalvlRespWaitCnt1W<DdrDenaliPhy657Spec> {
        PhyAdrCalvlRespWaitCnt1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_657::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_657::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy657Spec;
impl crate::RegisterSpec for DdrDenaliPhy657Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_657::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy657Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_657::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy657Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_657 to value 0"]
impl crate::Resettable for DdrDenaliPhy657Spec {
    const RESET_VALUE: u32 = 0;
}
