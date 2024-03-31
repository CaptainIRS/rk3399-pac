#[doc = "Register `DENALI_PHY_785` reader"]
pub type R = crate::R<DenaliPhy785Spec>;
#[doc = "Register `DENALI_PHY_785` writer"]
pub type W = crate::W<DenaliPhy785Spec>;
#[doc = "Field `PHY_ADR_CALVL_RANK_CTRL_2` reader - Defines the CA training rank control bits for address slice 2."]
pub type PhyAdrCalvlRankCtrl2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_RANK_CTRL_2` writer - Defines the CA training rank control bits for address slice 2."]
pub type PhyAdrCalvlRankCtrl2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_CALVL_NUM_PATTERNS_2` reader - Sets the number of patterns to use during CA training for address slice 2."]
pub type PhyAdrCalvlNumPatterns2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_NUM_PATTERNS_2` writer - Sets the number of patterns to use during CA training for address slice 2."]
pub type PhyAdrCalvlNumPatterns2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_CALVL_CAPTURE_CNT_2` reader - Number of samples to take at each ADDR slave delay setting during CA training for address slice 2."]
pub type PhyAdrCalvlCaptureCnt2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_CAPTURE_CNT_2` writer - Number of samples to take at each ADDR slave delay setting during CA training for address slice 2."]
pub type PhyAdrCalvlCaptureCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADR_CALVL_RESP_WAIT_CNT_2` reader - Number of samples to wait for response during CA training for address slice 2."]
pub type PhyAdrCalvlRespWaitCnt2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_RESP_WAIT_CNT_2` writer - Number of samples to wait for response during CA training for address slice 2."]
pub type PhyAdrCalvlRespWaitCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Defines the CA training rank control bits for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_rank_ctrl_2(&self) -> PhyAdrCalvlRankCtrl2R {
        PhyAdrCalvlRankCtrl2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets the number of patterns to use during CA training for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_num_patterns_2(&self) -> PhyAdrCalvlNumPatterns2R {
        PhyAdrCalvlNumPatterns2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Number of samples to take at each ADDR slave delay setting during CA training for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_capture_cnt_2(&self) -> PhyAdrCalvlCaptureCnt2R {
        PhyAdrCalvlCaptureCnt2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Number of samples to wait for response during CA training for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_resp_wait_cnt_2(&self) -> PhyAdrCalvlRespWaitCnt2R {
        PhyAdrCalvlRespWaitCnt2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines the CA training rank control bits for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_rank_ctrl_2(&mut self) -> PhyAdrCalvlRankCtrl2W<DenaliPhy785Spec> {
        PhyAdrCalvlRankCtrl2W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Sets the number of patterns to use during CA training for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_num_patterns_2(&mut self) -> PhyAdrCalvlNumPatterns2W<DenaliPhy785Spec> {
        PhyAdrCalvlNumPatterns2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Number of samples to take at each ADDR slave delay setting during CA training for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_capture_cnt_2(&mut self) -> PhyAdrCalvlCaptureCnt2W<DenaliPhy785Spec> {
        PhyAdrCalvlCaptureCnt2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of samples to wait for response during CA training for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_resp_wait_cnt_2(&mut self) -> PhyAdrCalvlRespWaitCnt2W<DenaliPhy785Spec> {
        PhyAdrCalvlRespWaitCnt2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_785::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_785::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy785Spec;
impl crate::RegisterSpec for DenaliPhy785Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_785::R`](R) reader structure"]
impl crate::Readable for DenaliPhy785Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_785::W`](W) writer structure"]
impl crate::Writable for DenaliPhy785Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_785 to value 0"]
impl crate::Resettable for DenaliPhy785Spec {
    const RESET_VALUE: u32 = 0;
}
