#[doc = "Register `RSTNHOLD_CON1` reader"]
pub type R = crate::R<RstnholdCon1Spec>;
#[doc = "Register `RSTNHOLD_CON1` writer"]
pub type W = crate::W<RstnholdCon1Spec>;
#[doc = "Field `PRESETN_I2C0_HOLD` reader - presetn_i2c0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnI2c0HoldR = crate::BitReader;
#[doc = "Field `PRESETN_I2C0_HOLD` writer - presetn_i2c0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnI2c0HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_I2C4_HOLD` reader - presetn_i2c4_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnI2c4HoldR = crate::BitReader;
#[doc = "Field `PRESETN_I2C4_HOLD` writer - presetn_i2c4_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnI2c4HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_I2C8_HOLD` reader - presetn_i2c8_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnI2c8HoldR = crate::BitReader;
#[doc = "Field `PRESETN_I2C8_HOLD` writer - presetn_i2c8_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnI2c8HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_MAILBOX_PMU_HOLD` reader - presetn_mailbox_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnMailboxPmuHoldR = crate::BitReader;
#[doc = "Field `PRESETN_MAILBOX_PMU_HOLD` writer - presetn_mailbox_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnMailboxPmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_RKPWM_PMU_HOLD` reader - presetn_rkpwm_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnRkpwmPmuHoldR = crate::BitReader;
#[doc = "Field `PRESETN_RKPWM_PMU_HOLD` writer - presetn_rkpwm_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnRkpwmPmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_PMUGRF_HOLD` reader - presetn_pmugrf_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnPmugrfHoldR = crate::BitReader;
#[doc = "Field `PRESETN_PMUGRF_HOLD` writer - presetn_pmugrf_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnPmugrfHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_SGRF_HOLD` reader - presetn_sgrf_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnSgrfHoldR = crate::BitReader;
#[doc = "Field `PRESETN_SGRF_HOLD` writer - presetn_sgrf_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnSgrfHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_GPIO0_HOLD` reader - presetn_gpio0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnGpio0HoldR = crate::BitReader;
#[doc = "Field `PRESETN_GPIO0_HOLD` writer - presetn_gpio0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnGpio0HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_GPIO1_HOLD` reader - presetn_gpio1_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnGpio1HoldR = crate::BitReader;
#[doc = "Field `PRESETN_GPIO1_HOLD` writer - presetn_gpio1_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnGpio1HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_CRU_PMU_HOLD` reader - presetn_cru_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnCruPmuHoldR = crate::BitReader;
#[doc = "Field `PRESETN_CRU_PMU_HOLD` writer - presetn_cru_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnCruPmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_INTR_ARB_HOLD` reader - presetn_intr_arb_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnIntrArbHoldR = crate::BitReader;
#[doc = "Field `PRESETN_INTR_ARB_HOLD` writer - presetn_intr_arb_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnIntrArbHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_PVTM_PMU_HOLD` reader - resetn_pvtm_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnPvtmPmuHoldR = crate::BitReader;
#[doc = "Field `RESETN_PVTM_PMU_HOLD` writer - resetn_pvtm_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnPvtmPmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2C0_HOLD` reader - resetn_i2c0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnI2c0HoldR = crate::BitReader;
#[doc = "Field `RESETN_I2C0_HOLD` writer - resetn_i2c0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnI2c0HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2C4_HOLD` reader - resetn_i2c4_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnI2c4HoldR = crate::BitReader;
#[doc = "Field `RESETN_I2C4_HOLD` writer - resetn_i2c4_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnI2c4HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2C8_HOLD` reader - resetn_i2c8_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnI2c8HoldR = crate::BitReader;
#[doc = "Field `RESETN_I2C8_HOLD` writer - resetn_i2c8_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnI2c8HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - presetn_i2c0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_i2c0_hold(&self) -> PresetnI2c0HoldR {
        PresetnI2c0HoldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - presetn_i2c4_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_i2c4_hold(&self) -> PresetnI2c4HoldR {
        PresetnI2c4HoldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - presetn_i2c8_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_i2c8_hold(&self) -> PresetnI2c8HoldR {
        PresetnI2c8HoldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - presetn_mailbox_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_mailbox_pmu_hold(&self) -> PresetnMailboxPmuHoldR {
        PresetnMailboxPmuHoldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - presetn_rkpwm_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_rkpwm_pmu_hold(&self) -> PresetnRkpwmPmuHoldR {
        PresetnRkpwmPmuHoldR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - presetn_pmugrf_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_pmugrf_hold(&self) -> PresetnPmugrfHoldR {
        PresetnPmugrfHoldR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - presetn_sgrf_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_sgrf_hold(&self) -> PresetnSgrfHoldR {
        PresetnSgrfHoldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - presetn_gpio0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_gpio0_hold(&self) -> PresetnGpio0HoldR {
        PresetnGpio0HoldR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - presetn_gpio1_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_gpio1_hold(&self) -> PresetnGpio1HoldR {
        PresetnGpio1HoldR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - presetn_cru_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_cru_pmu_hold(&self) -> PresetnCruPmuHoldR {
        PresetnCruPmuHoldR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - presetn_intr_arb_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_intr_arb_hold(&self) -> PresetnIntrArbHoldR {
        PresetnIntrArbHoldR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - resetn_pvtm_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn resetn_pvtm_pmu_hold(&self) -> ResetnPvtmPmuHoldR {
        ResetnPvtmPmuHoldR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - resetn_i2c0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn resetn_i2c0_hold(&self) -> ResetnI2c0HoldR {
        ResetnI2c0HoldR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - resetn_i2c4_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn resetn_i2c4_hold(&self) -> ResetnI2c4HoldR {
        ResetnI2c4HoldR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - resetn_i2c8_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn resetn_i2c8_hold(&self) -> ResetnI2c8HoldR {
        ResetnI2c8HoldR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - presetn_i2c0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_i2c0_hold(&mut self) -> PresetnI2c0HoldW<RstnholdCon1Spec> {
        PresetnI2c0HoldW::new(self, 0)
    }
    #[doc = "Bit 1 - presetn_i2c4_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_i2c4_hold(&mut self) -> PresetnI2c4HoldW<RstnholdCon1Spec> {
        PresetnI2c4HoldW::new(self, 1)
    }
    #[doc = "Bit 2 - presetn_i2c8_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_i2c8_hold(&mut self) -> PresetnI2c8HoldW<RstnholdCon1Spec> {
        PresetnI2c8HoldW::new(self, 2)
    }
    #[doc = "Bit 3 - presetn_mailbox_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_mailbox_pmu_hold(&mut self) -> PresetnMailboxPmuHoldW<RstnholdCon1Spec> {
        PresetnMailboxPmuHoldW::new(self, 3)
    }
    #[doc = "Bit 4 - presetn_rkpwm_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_rkpwm_pmu_hold(&mut self) -> PresetnRkpwmPmuHoldW<RstnholdCon1Spec> {
        PresetnRkpwmPmuHoldW::new(self, 4)
    }
    #[doc = "Bit 5 - presetn_pmugrf_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_pmugrf_hold(&mut self) -> PresetnPmugrfHoldW<RstnholdCon1Spec> {
        PresetnPmugrfHoldW::new(self, 5)
    }
    #[doc = "Bit 6 - presetn_sgrf_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_sgrf_hold(&mut self) -> PresetnSgrfHoldW<RstnholdCon1Spec> {
        PresetnSgrfHoldW::new(self, 6)
    }
    #[doc = "Bit 7 - presetn_gpio0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_gpio0_hold(&mut self) -> PresetnGpio0HoldW<RstnholdCon1Spec> {
        PresetnGpio0HoldW::new(self, 7)
    }
    #[doc = "Bit 8 - presetn_gpio1_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_gpio1_hold(&mut self) -> PresetnGpio1HoldW<RstnholdCon1Spec> {
        PresetnGpio1HoldW::new(self, 8)
    }
    #[doc = "Bit 9 - presetn_cru_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_cru_pmu_hold(&mut self) -> PresetnCruPmuHoldW<RstnholdCon1Spec> {
        PresetnCruPmuHoldW::new(self, 9)
    }
    #[doc = "Bit 10 - presetn_intr_arb_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_intr_arb_hold(&mut self) -> PresetnIntrArbHoldW<RstnholdCon1Spec> {
        PresetnIntrArbHoldW::new(self, 10)
    }
    #[doc = "Bit 11 - resetn_pvtm_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_pvtm_pmu_hold(&mut self) -> ResetnPvtmPmuHoldW<RstnholdCon1Spec> {
        ResetnPvtmPmuHoldW::new(self, 11)
    }
    #[doc = "Bit 12 - resetn_i2c0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2c0_hold(&mut self) -> ResetnI2c0HoldW<RstnholdCon1Spec> {
        ResetnI2c0HoldW::new(self, 12)
    }
    #[doc = "Bit 13 - resetn_i2c4_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2c4_hold(&mut self) -> ResetnI2c4HoldW<RstnholdCon1Spec> {
        ResetnI2c4HoldW::new(self, 13)
    }
    #[doc = "Bit 14 - resetn_i2c8_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2c8_hold(&mut self) -> ResetnI2c8HoldW<RstnholdCon1Spec> {
        ResetnI2c8HoldW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<RstnholdCon1Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal reset hold control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstnhold_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstnhold_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstnholdCon1Spec;
impl crate::RegisterSpec for RstnholdCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstnhold_con1::R`](R) reader structure"]
impl crate::Readable for RstnholdCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`rstnhold_con1::W`](W) writer structure"]
impl crate::Writable for RstnholdCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTNHOLD_CON1 to value 0"]
impl crate::Resettable for RstnholdCon1Spec {
    const RESET_VALUE: u32 = 0;
}
