#[doc = "Register `DDR_DENALI_PHY_529` reader"]
pub type R = crate::R<DdrDenaliPhy529Spec>;
#[doc = "Register `DDR_DENALI_PHY_529` writer"]
pub type W = crate::W<DdrDenaliPhy529Spec>;
#[doc = "Field `PHY_ADR_CALVL_RANK_CTRL_0` reader - Defines the CA training rank control bits for address slice 0."]
pub type PhyAdrCalvlRankCtrl0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_RANK_CTRL_0` writer - Defines the CA training rank control bits for address slice 0."]
pub type PhyAdrCalvlRankCtrl0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_CALVL_NUM_PATTERNS_0` reader - Sets the number of patterns to use during CA training for address slice 0."]
pub type PhyAdrCalvlNumPatterns0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_NUM_PATTERNS_0` writer - Sets the number of patterns to use during CA training for address slice 0."]
pub type PhyAdrCalvlNumPatterns0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_CALVL_CAPTURE_CNT_0` reader - Number of samples to take at each ADDR slave delay setting during CA training for address slice 0."]
pub type PhyAdrCalvlCaptureCnt0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_CAPTURE_CNT_0` writer - Number of samples to take at each ADDR slave delay setting during CA training for address slice 0."]
pub type PhyAdrCalvlCaptureCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADR_CALVL_RESP_WAIT_CNT_0` reader - Number of samples to wait for response during CA training for address slice 0."]
pub type PhyAdrCalvlRespWaitCnt0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_RESP_WAIT_CNT_0` writer - Number of samples to wait for response during CA training for address slice 0."]
pub type PhyAdrCalvlRespWaitCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Defines the CA training rank control bits for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_rank_ctrl_0(&self) -> PhyAdrCalvlRankCtrl0R {
        PhyAdrCalvlRankCtrl0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets the number of patterns to use during CA training for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_num_patterns_0(&self) -> PhyAdrCalvlNumPatterns0R {
        PhyAdrCalvlNumPatterns0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Number of samples to take at each ADDR slave delay setting during CA training for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_capture_cnt_0(&self) -> PhyAdrCalvlCaptureCnt0R {
        PhyAdrCalvlCaptureCnt0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Number of samples to wait for response during CA training for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_resp_wait_cnt_0(&self) -> PhyAdrCalvlRespWaitCnt0R {
        PhyAdrCalvlRespWaitCnt0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines the CA training rank control bits for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_rank_ctrl_0(&mut self) -> PhyAdrCalvlRankCtrl0W<DdrDenaliPhy529Spec> {
        PhyAdrCalvlRankCtrl0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Sets the number of patterns to use during CA training for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_num_patterns_0(
        &mut self,
    ) -> PhyAdrCalvlNumPatterns0W<DdrDenaliPhy529Spec> {
        PhyAdrCalvlNumPatterns0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Number of samples to take at each ADDR slave delay setting during CA training for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_capture_cnt_0(&mut self) -> PhyAdrCalvlCaptureCnt0W<DdrDenaliPhy529Spec> {
        PhyAdrCalvlCaptureCnt0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of samples to wait for response during CA training for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_resp_wait_cnt_0(
        &mut self,
    ) -> PhyAdrCalvlRespWaitCnt0W<DdrDenaliPhy529Spec> {
        PhyAdrCalvlRespWaitCnt0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_529::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_529::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy529Spec;
impl crate::RegisterSpec for DdrDenaliPhy529Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_529::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy529Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_529::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy529Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_529 to value 0"]
impl crate::Resettable for DdrDenaliPhy529Spec {
    const RESET_VALUE: u32 = 0;
}
