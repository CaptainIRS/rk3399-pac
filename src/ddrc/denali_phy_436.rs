#[doc = "Register `DENALI_PHY_436` reader"]
pub type R = crate::R<DenaliPhy436Spec>;
#[doc = "Register `DENALI_PHY_436` writer"]
pub type W = crate::W<DenaliPhy436Spec>;
#[doc = "Field `SC_PHY_RX_CAL_START_3` writer - Manual RX Calibration start for slice 3."]
pub type ScPhyRxCalStart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RX_CAL_OVERRIDE_3` reader - Manual setting of RX Calibration enable for slice 3."]
pub type PhyRxCalOverride3R = crate::BitReader;
#[doc = "Field `PHY_RX_CAL_OVERRIDE_3` writer - Manual setting of RX Calibration enable for slice 3."]
pub type PhyRxCalOverride3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RX_CAL_SAMPLE_WAIT_3` reader - RX Calibration state machine wait count for slice 3."]
pub type PhyRxCalSampleWait3R = crate::FieldReader;
#[doc = "Field `PHY_RX_CAL_SAMPLE_WAIT_3` writer - RX Calibration state machine wait count for slice 3."]
pub type PhyRxCalSampleWait3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 8 - Manual setting of RX Calibration enable for slice 3."]
    #[inline(always)]
    pub fn phy_rx_cal_override_3(&self) -> PhyRxCalOverride3R {
        PhyRxCalOverride3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - RX Calibration state machine wait count for slice 3."]
    #[inline(always)]
    pub fn phy_rx_cal_sample_wait_3(&self) -> PhyRxCalSampleWait3R {
        PhyRxCalSampleWait3R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Manual RX Calibration start for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_rx_cal_start_3(&mut self) -> ScPhyRxCalStart3W<DenaliPhy436Spec> {
        ScPhyRxCalStart3W::new(self, 0)
    }
    #[doc = "Bit 8 - Manual setting of RX Calibration enable for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_override_3(&mut self) -> PhyRxCalOverride3W<DenaliPhy436Spec> {
        PhyRxCalOverride3W::new(self, 8)
    }
    #[doc = "Bits 16:23 - RX Calibration state machine wait count for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_sample_wait_3(&mut self) -> PhyRxCalSampleWait3W<DenaliPhy436Spec> {
        PhyRxCalSampleWait3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_436::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_436::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy436Spec;
impl crate::RegisterSpec for DenaliPhy436Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_436::R`](R) reader structure"]
impl crate::Readable for DenaliPhy436Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_436::W`](W) writer structure"]
impl crate::Writable for DenaliPhy436Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_436 to value 0"]
impl crate::Resettable for DenaliPhy436Spec {
    const RESET_VALUE: u32 = 0;
}
