#[doc = "Register `DENALI_PHY_52` reader"]
pub type R = crate::R<DenaliPhy52Spec>;
#[doc = "Register `DENALI_PHY_52` writer"]
pub type W = crate::W<DenaliPhy52Spec>;
#[doc = "Field `SC_PHY_RX_CAL_START_0` writer - Manual RX Calibration start for slice 0. WRITE-ONLY"]
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
    #[doc = "Bit 0 - Manual RX Calibration start for slice 0. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_rx_cal_start_0(&mut self) -> ScPhyRxCalStart0W<DenaliPhy52Spec> {
        ScPhyRxCalStart0W::new(self, 0)
    }
    #[doc = "Bit 8 - Manual setting of RX Calibration enable for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_override_0(&mut self) -> PhyRxCalOverride0W<DenaliPhy52Spec> {
        PhyRxCalOverride0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - RX Calibration state machine wait count for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_sample_wait_0(&mut self) -> PhyRxCalSampleWait0W<DenaliPhy52Spec> {
        PhyRxCalSampleWait0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy52Spec;
impl crate::RegisterSpec for DenaliPhy52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_52::R`](R) reader structure"]
impl crate::Readable for DenaliPhy52Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_52::W`](W) writer structure"]
impl crate::Writable for DenaliPhy52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_52 to value 0"]
impl crate::Resettable for DenaliPhy52Spec {
    const RESET_VALUE: u32 = 0;
}
