#[doc = "Register `DENALI_PHY_13` reader"]
pub type R = crate::R<DenaliPhy13Spec>;
#[doc = "Register `DENALI_PHY_13` writer"]
pub type W = crate::W<DenaliPhy13Spec>;
#[doc = "Field `PHY_GATE_SMPL2_SLAVE_DELAY_0` reader - Number of cycles to delay the read DQS gate signal to generate gate2 signal for on the fly read DQS training for slice 0."]
pub type PhyGateSmpl2SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GATE_SMPL2_SLAVE_DELAY_0` writer - Number of cycles to delay the read DQS gate signal to generate gate2 signal for on the fly read DQS training for slice 0."]
pub type PhyGateSmpl2SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ON_FLY_GATE_ADJUST_EN_0` reader - Control the on the fly gate adjustment for slice 0."]
pub type OnFlyGateAdjustEn0R = crate::FieldReader;
#[doc = "Field `ON_FLY_GATE_ADJUST_EN_0` writer - Control the on the fly gate adjustment for slice 0."]
pub type OnFlyGateAdjustEn0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:8 - Number of cycles to delay the read DQS gate signal to generate gate2 signal for on the fly read DQS training for slice 0."]
    #[inline(always)]
    pub fn phy_gate_smpl2_slave_delay_0(&self) -> PhyGateSmpl2SlaveDelay0R {
        PhyGateSmpl2SlaveDelay0R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - Control the on the fly gate adjustment for slice 0."]
    #[inline(always)]
    pub fn on_fly_gate_adjust_en_0(&self) -> OnFlyGateAdjustEn0R {
        OnFlyGateAdjustEn0R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Number of cycles to delay the read DQS gate signal to generate gate2 signal for on the fly read DQS training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gate_smpl2_slave_delay_0(&mut self) -> PhyGateSmpl2SlaveDelay0W<DenaliPhy13Spec> {
        PhyGateSmpl2SlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Control the on the fly gate adjustment for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn on_fly_gate_adjust_en_0(&mut self) -> OnFlyGateAdjustEn0W<DenaliPhy13Spec> {
        OnFlyGateAdjustEn0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy13Spec;
impl crate::RegisterSpec for DenaliPhy13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_13::R`](R) reader structure"]
impl crate::Readable for DenaliPhy13Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_13::W`](W) writer structure"]
impl crate::Writable for DenaliPhy13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_13 to value 0"]
impl crate::Resettable for DenaliPhy13Spec {
    const RESET_VALUE: u32 = 0;
}
