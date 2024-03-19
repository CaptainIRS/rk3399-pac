#[doc = "Register `DDR_DENALI_PHY_944` reader"]
pub type R = crate::R<DdrDenaliPhy944Spec>;
#[doc = "Register `DDR_DENALI_PHY_944` writer"]
pub type W = crate::W<DdrDenaliPhy944Spec>;
#[doc = "Field `PHY_TST_CLK_PAD_CTRL3` reader - PHY test clock pad additional controls."]
pub type PhyTstClkPadCtrl3R = crate::FieldReader<u32>;
#[doc = "Field `PHY_TST_CLK_PAD_CTRL3` writer - PHY test clock pad additional controls."]
pub type PhyTstClkPadCtrl3W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `PHY_CAL_MODE_0` reader - Pad calibration mode bits for block 0. Bit (0) disables pad calibration upon initialization. Bit (1) enables automatic interval based calibration. Bits (3:2) set the base interval for the interval counter. Bits (7:4) are direct connections to pad control signals."]
pub type PhyCalMode0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_MODE_0` writer - Pad calibration mode bits for block 0. Bit (0) disables pad calibration upon initialization. Bit (1) enables automatic interval based calibration. Bits (3:2) set the base interval for the interval counter. Bits (7:4) are direct connections to pad control signals."]
pub type PhyCalMode0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:20 - PHY test clock pad additional controls."]
    #[inline(always)]
    pub fn phy_tst_clk_pad_ctrl3(&self) -> PhyTstClkPadCtrl3R {
        PhyTstClkPadCtrl3R::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bits 24:31 - Pad calibration mode bits for block 0. Bit (0) disables pad calibration upon initialization. Bit (1) enables automatic interval based calibration. Bits (3:2) set the base interval for the interval counter. Bits (7:4) are direct connections to pad control signals."]
    #[inline(always)]
    pub fn phy_cal_mode_0(&self) -> PhyCalMode0R {
        PhyCalMode0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:20 - PHY test clock pad additional controls."]
    #[inline(always)]
    #[must_use]
    pub fn phy_tst_clk_pad_ctrl3(&mut self) -> PhyTstClkPadCtrl3W<DdrDenaliPhy944Spec> {
        PhyTstClkPadCtrl3W::new(self, 0)
    }
    #[doc = "Bits 24:31 - Pad calibration mode bits for block 0. Bit (0) disables pad calibration upon initialization. Bit (1) enables automatic interval based calibration. Bits (3:2) set the base interval for the interval counter. Bits (7:4) are direct connections to pad control signals."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_mode_0(&mut self) -> PhyCalMode0W<DdrDenaliPhy944Spec> {
        PhyCalMode0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_944::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_944::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy944Spec;
impl crate::RegisterSpec for DdrDenaliPhy944Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_944::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy944Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_944::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy944Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_944 to value 0"]
impl crate::Resettable for DdrDenaliPhy944Spec {
    const RESET_VALUE: u32 = 0;
}
