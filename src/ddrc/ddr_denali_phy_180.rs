#[doc = "Register `DDR_DENALI_PHY_180` reader"]
pub type R = crate::R<DdrDenaliPhy180Spec>;
#[doc = "Register `DDR_DENALI_PHY_180` writer"]
pub type W = crate::W<DdrDenaliPhy180Spec>;
#[doc = "Field `SC_PHY_RX_CAL_START_1` writer - Manual RX Calibration start for slice 1."]
pub type ScPhyRxCalStart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RX_CAL_OVERRIDE_1` reader - Manual setting of RX Calibration enable for slice 1."]
pub type PhyRxCalOverride1R = crate::BitReader;
#[doc = "Field `PHY_RX_CAL_OVERRIDE_1` writer - Manual setting of RX Calibration enable for slice 1."]
pub type PhyRxCalOverride1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RX_CAL_SAMPLE_WAIT_1` reader - RX Calibration state machine wait count for slice 1."]
pub type PhyRxCalSampleWait1R = crate::FieldReader;
#[doc = "Field `PHY_RX_CAL_SAMPLE_WAIT_1` writer - RX Calibration state machine wait count for slice 1."]
pub type PhyRxCalSampleWait1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 8 - Manual setting of RX Calibration enable for slice 1."]
    #[inline(always)]
    pub fn phy_rx_cal_override_1(&self) -> PhyRxCalOverride1R {
        PhyRxCalOverride1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - RX Calibration state machine wait count for slice 1."]
    #[inline(always)]
    pub fn phy_rx_cal_sample_wait_1(&self) -> PhyRxCalSampleWait1R {
        PhyRxCalSampleWait1R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Manual RX Calibration start for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_rx_cal_start_1(&mut self) -> ScPhyRxCalStart1W<DdrDenaliPhy180Spec> {
        ScPhyRxCalStart1W::new(self, 0)
    }
    #[doc = "Bit 8 - Manual setting of RX Calibration enable for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_override_1(&mut self) -> PhyRxCalOverride1W<DdrDenaliPhy180Spec> {
        PhyRxCalOverride1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - RX Calibration state machine wait count for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_sample_wait_1(&mut self) -> PhyRxCalSampleWait1W<DdrDenaliPhy180Spec> {
        PhyRxCalSampleWait1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_180::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_180::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy180Spec;
impl crate::RegisterSpec for DdrDenaliPhy180Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_180::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy180Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_180::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy180Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_180 to value 0"]
impl crate::Resettable for DdrDenaliPhy180Spec {
    const RESET_VALUE: u32 = 0;
}
