#[doc = "Register `DDR_DENALI_PHY_52` reader"]
pub type R = crate::R<DdrDenaliPhy52Spec>;
#[doc = "Register `DDR_DENALI_PHY_52` writer"]
pub type W = crate::W<DdrDenaliPhy52Spec>;
#[doc = "Field `SC_PHY_RX_CAL_START_0` writer - Manual RX Calibration start for slice 0."]
pub type ScPhyRxCalStart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RX_CAL_OVERRIDE_0` reader - Manual setting of RX Calibration enable for slice 0."]
pub type PhyRxCalOverride0R = crate::BitReader;
#[doc = "Field `PHY_RX_CAL_OVERRIDE_0` writer - Manual setting of RX Calibration enable for slice 0."]
pub type PhyRxCalOverride0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RX_CAL_SAMPLE_WAIT_0` reader - RX Calibration state machine wait count for slice 0."]
pub type PhyRxCalSampleWait0R = crate::FieldReader;
#[doc = "Field `PHY_RX_CAL_SAMPLE_WAIT_0` writer - RX Calibration state machine wait count for slice 0."]
pub type PhyRxCalSampleWait0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 8 - Manual setting of RX Calibration enable for slice 0."]
    #[inline(always)]
    pub fn phy_rx_cal_override_0(&self) -> PhyRxCalOverride0R {
        PhyRxCalOverride0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - RX Calibration state machine wait count for slice 0."]
    #[inline(always)]
    pub fn phy_rx_cal_sample_wait_0(&self) -> PhyRxCalSampleWait0R {
        PhyRxCalSampleWait0R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Manual RX Calibration start for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_rx_cal_start_0(&mut self) -> ScPhyRxCalStart0W<DdrDenaliPhy52Spec> {
        ScPhyRxCalStart0W::new(self, 0)
    }
    #[doc = "Bit 8 - Manual setting of RX Calibration enable for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_override_0(&mut self) -> PhyRxCalOverride0W<DdrDenaliPhy52Spec> {
        PhyRxCalOverride0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - RX Calibration state machine wait count for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_sample_wait_0(&mut self) -> PhyRxCalSampleWait0W<DdrDenaliPhy52Spec> {
        PhyRxCalSampleWait0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy52Spec;
impl crate::RegisterSpec for DdrDenaliPhy52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_52::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy52Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_52::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_52 to value 0"]
impl crate::Resettable for DdrDenaliPhy52Spec {
    const RESET_VALUE: u32 = 0;
}
