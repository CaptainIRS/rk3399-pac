#[doc = "Register `DDR_DENALI_PHY_947` reader"]
pub type R = crate::R<DdrDenaliPhy947Spec>;
#[doc = "Register `DDR_DENALI_PHY_947` writer"]
pub type W = crate::W<DdrDenaliPhy947Spec>;
#[doc = "Field `PHY_CAL_SAMPLE_WAIT_0` reader - Pad calibration state machine wait count in pad clock cycles for block 0."]
pub type PhyCalSampleWait0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_SAMPLE_WAIT_0` writer - Pad calibration state machine wait count in pad clock cycles for block 0."]
pub type PhyCalSampleWait0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_CAL_CLK_SELECT_0` reader - Pad calibration pad clock frequency select setting for block 0."]
pub type PhyCalClkSelect0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_CLK_SELECT_0` writer - Pad calibration pad clock frequency select setting for block 0."]
pub type PhyCalClkSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Pad calibration state machine wait count in pad clock cycles for block 0."]
    #[inline(always)]
    pub fn phy_cal_sample_wait_0(&self) -> PhyCalSampleWait0R {
        PhyCalSampleWait0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Pad calibration pad clock frequency select setting for block 0."]
    #[inline(always)]
    pub fn phy_cal_clk_select_0(&self) -> PhyCalClkSelect0R {
        PhyCalClkSelect0R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pad calibration state machine wait count in pad clock cycles for block 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_sample_wait_0(&mut self) -> PhyCalSampleWait0W<DdrDenaliPhy947Spec> {
        PhyCalSampleWait0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Pad calibration pad clock frequency select setting for block 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_clk_select_0(&mut self) -> PhyCalClkSelect0W<DdrDenaliPhy947Spec> {
        PhyCalClkSelect0W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_947::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_947::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy947Spec;
impl crate::RegisterSpec for DdrDenaliPhy947Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_947::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy947Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_947::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy947Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_947 to value 0"]
impl crate::Resettable for DdrDenaliPhy947Spec {
    const RESET_VALUE: u32 = 0;
}
