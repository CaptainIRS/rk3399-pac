#[doc = "Register `DDR_DENALI_PHY_269` reader"]
pub type R = crate::R<DdrDenaliPhy269Spec>;
#[doc = "Register `DDR_DENALI_PHY_269` writer"]
pub type W = crate::W<DdrDenaliPhy269Spec>;
#[doc = "Field `PHY_GATE_SMPL2_SLAVE_DELAY_2` reader - Number of cycles to delay the read DQS gate signal to generate gate2 signal for on the fly read DQS training for slice 2."]
pub type PhyGateSmpl2SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GATE_SMPL2_SLAVE_DELAY_2` writer - Number of cycles to delay the read DQS gate signal to generate gate2 signal for on the fly read DQS training for slice 2."]
pub type PhyGateSmpl2SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ON_FLY_GATE_ADJUST_EN_2` reader - Control the on the fly gate adjustment for slice 2."]
pub type OnFlyGateAdjustEn2R = crate::FieldReader;
#[doc = "Field `ON_FLY_GATE_ADJUST_EN_2` writer - Control the on the fly gate adjustment for slice 2."]
pub type OnFlyGateAdjustEn2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:8 - Number of cycles to delay the read DQS gate signal to generate gate2 signal for on the fly read DQS training for slice 2."]
    #[inline(always)]
    pub fn phy_gate_smpl2_slave_delay_2(&self) -> PhyGateSmpl2SlaveDelay2R {
        PhyGateSmpl2SlaveDelay2R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - Control the on the fly gate adjustment for slice 2."]
    #[inline(always)]
    pub fn on_fly_gate_adjust_en_2(&self) -> OnFlyGateAdjustEn2R {
        OnFlyGateAdjustEn2R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Number of cycles to delay the read DQS gate signal to generate gate2 signal for on the fly read DQS training for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gate_smpl2_slave_delay_2(
        &mut self,
    ) -> PhyGateSmpl2SlaveDelay2W<DdrDenaliPhy269Spec> {
        PhyGateSmpl2SlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Control the on the fly gate adjustment for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn on_fly_gate_adjust_en_2(&mut self) -> OnFlyGateAdjustEn2W<DdrDenaliPhy269Spec> {
        OnFlyGateAdjustEn2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_269::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_269::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy269Spec;
impl crate::RegisterSpec for DdrDenaliPhy269Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_269::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy269Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_269::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy269Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_269 to value 0"]
impl crate::Resettable for DdrDenaliPhy269Spec {
    const RESET_VALUE: u32 = 0;
}
