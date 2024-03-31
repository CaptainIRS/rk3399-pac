#[doc = "Register `DENALI_PHY_308` reader"]
pub type R = crate::R<DenaliPhy308Spec>;
#[doc = "Register `DENALI_PHY_308` writer"]
pub type W = crate::W<DenaliPhy308Spec>;
#[doc = "Field `SC_PHY_RX_CAL_START_2` writer - Manual RX Calibration start for slice 2."]
pub type ScPhyRxCalStart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RX_CAL_OVERRIDE_2` reader - Manual setting of RX Calibration enable for slice 2."]
pub type PhyRxCalOverride2R = crate::BitReader;
#[doc = "Field `PHY_RX_CAL_OVERRIDE_2` writer - Manual setting of RX Calibration enable for slice 2."]
pub type PhyRxCalOverride2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RX_CAL_SAMPLE_WAIT_2` reader - RX Calibration state machine wait count for slice 2."]
pub type PhyRxCalSampleWait2R = crate::FieldReader;
#[doc = "Field `PHY_RX_CAL_SAMPLE_WAIT_2` writer - RX Calibration state machine wait count for slice 2."]
pub type PhyRxCalSampleWait2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 8 - Manual setting of RX Calibration enable for slice 2."]
    #[inline(always)]
    pub fn phy_rx_cal_override_2(&self) -> PhyRxCalOverride2R {
        PhyRxCalOverride2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - RX Calibration state machine wait count for slice 2."]
    #[inline(always)]
    pub fn phy_rx_cal_sample_wait_2(&self) -> PhyRxCalSampleWait2R {
        PhyRxCalSampleWait2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Manual RX Calibration start for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_rx_cal_start_2(&mut self) -> ScPhyRxCalStart2W<DenaliPhy308Spec> {
        ScPhyRxCalStart2W::new(self, 0)
    }
    #[doc = "Bit 8 - Manual setting of RX Calibration enable for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_override_2(&mut self) -> PhyRxCalOverride2W<DenaliPhy308Spec> {
        PhyRxCalOverride2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - RX Calibration state machine wait count for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_sample_wait_2(&mut self) -> PhyRxCalSampleWait2W<DenaliPhy308Spec> {
        PhyRxCalSampleWait2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_308::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_308::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy308Spec;
impl crate::RegisterSpec for DenaliPhy308Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_308::R`](R) reader structure"]
impl crate::Readable for DenaliPhy308Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_308::W`](W) writer structure"]
impl crate::Writable for DenaliPhy308Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_308 to value 0"]
impl crate::Resettable for DenaliPhy308Spec {
    const RESET_VALUE: u32 = 0;
}
