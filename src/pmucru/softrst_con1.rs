#[doc = "Register `SOFTRST_CON1` reader"]
pub type R = crate::R<SoftrstCon1Spec>;
#[doc = "Register `SOFTRST_CON1` writer"]
pub type W = crate::W<SoftrstCon1Spec>;
#[doc = "Field `PRESETN_I2C0_REQ` reader - presetn_i2c0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c0ReqR = crate::BitReader;
#[doc = "Field `PRESETN_I2C0_REQ` writer - presetn_i2c0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_I2C4_REQ` reader - presetn_i2c4 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c4ReqR = crate::BitReader;
#[doc = "Field `PRESETN_I2C4_REQ` writer - presetn_i2c4 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c4ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_I2C8_REQ` reader - presetn_i2c8 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c8ReqR = crate::BitReader;
#[doc = "Field `PRESETN_I2C8_REQ` writer - presetn_i2c8 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c8ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_MAILBOX_PMU_REQ` reader - presetn_mailbox_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnMailboxPmuReqR = crate::BitReader;
#[doc = "Field `PRESETN_MAILBOX_PMU_REQ` writer - presetn_mailbox_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnMailboxPmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_RKPWM_PMU_REQ` reader - presetn_rkpwm_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnRkpwmPmuReqR = crate::BitReader;
#[doc = "Field `PRESETN_RKPWM_PMU_REQ` writer - presetn_rkpwm_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnRkpwmPmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_PMUGRF_REQ` reader - presetn_pmugrf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPmugrfReqR = crate::BitReader;
#[doc = "Field `PRESETN_PMUGRF_REQ` writer - presetn_pmugrf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPmugrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_SGRF_REQ` reader - presetn_sgrf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSgrfReqR = crate::BitReader;
#[doc = "Field `PRESETN_SGRF_REQ` writer - presetn_sgrf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSgrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_GPIO0_REQ` reader - presetn_gpio0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGpio0ReqR = crate::BitReader;
#[doc = "Field `PRESETN_GPIO0_REQ` writer - presetn_gpio0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGpio0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_GPIO1_REQ` reader - presetn_gpio1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGpio1ReqR = crate::BitReader;
#[doc = "Field `PRESETN_GPIO1_REQ` writer - presetn_gpio1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGpio1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_CRU_PMU_REQ` reader - presetn_cru_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnCruPmuReqR = crate::BitReader;
#[doc = "Field `PRESETN_CRU_PMU_REQ` writer - presetn_cru_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnCruPmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_INTR_ARB_REQ` reader - presetn_intr_arb request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnIntrArbReqR = crate::BitReader;
#[doc = "Field `PRESETN_INTR_ARB_REQ` writer - presetn_intr_arb request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnIntrArbReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_PVTM_PMU_REQ` reader - resetn_pvtm_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPvtmPmuReqR = crate::BitReader;
#[doc = "Field `RESETN_PVTM_PMU_REQ` writer - resetn_pvtm_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPvtmPmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2C0_REQ` reader - resetn_i2c0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnI2c0ReqR = crate::BitReader;
#[doc = "Field `RESETN_I2C0_REQ` writer - resetn_i2c0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnI2c0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2C4_REQ` reader - resetn_i2c4 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnI2c4ReqR = crate::BitReader;
#[doc = "Field `RESETN_I2C4_REQ` writer - resetn_i2c4 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnI2c4ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2C8_REQ` reader - resetn_i2c8 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnI2c8ReqR = crate::BitReader;
#[doc = "Field `RESETN_I2C8_REQ` writer - resetn_i2c8 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnI2c8ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - presetn_i2c0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_i2c0_req(&self) -> PresetnI2c0ReqR {
        PresetnI2c0ReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - presetn_i2c4 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_i2c4_req(&self) -> PresetnI2c4ReqR {
        PresetnI2c4ReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - presetn_i2c8 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_i2c8_req(&self) -> PresetnI2c8ReqR {
        PresetnI2c8ReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - presetn_mailbox_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_mailbox_pmu_req(&self) -> PresetnMailboxPmuReqR {
        PresetnMailboxPmuReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - presetn_rkpwm_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_rkpwm_pmu_req(&self) -> PresetnRkpwmPmuReqR {
        PresetnRkpwmPmuReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - presetn_pmugrf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_pmugrf_req(&self) -> PresetnPmugrfReqR {
        PresetnPmugrfReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - presetn_sgrf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_sgrf_req(&self) -> PresetnSgrfReqR {
        PresetnSgrfReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - presetn_gpio0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_gpio0_req(&self) -> PresetnGpio0ReqR {
        PresetnGpio0ReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - presetn_gpio1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_gpio1_req(&self) -> PresetnGpio1ReqR {
        PresetnGpio1ReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - presetn_cru_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_cru_pmu_req(&self) -> PresetnCruPmuReqR {
        PresetnCruPmuReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - presetn_intr_arb request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_intr_arb_req(&self) -> PresetnIntrArbReqR {
        PresetnIntrArbReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - resetn_pvtm_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_pvtm_pmu_req(&self) -> ResetnPvtmPmuReqR {
        ResetnPvtmPmuReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - resetn_i2c0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_i2c0_req(&self) -> ResetnI2c0ReqR {
        ResetnI2c0ReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - resetn_i2c4 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_i2c4_req(&self) -> ResetnI2c4ReqR {
        ResetnI2c4ReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - resetn_i2c8 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_i2c8_req(&self) -> ResetnI2c8ReqR {
        ResetnI2c8ReqR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - presetn_i2c0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_i2c0_req(&mut self) -> PresetnI2c0ReqW<SoftrstCon1Spec> {
        PresetnI2c0ReqW::new(self, 0)
    }
    #[doc = "Bit 1 - presetn_i2c4 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_i2c4_req(&mut self) -> PresetnI2c4ReqW<SoftrstCon1Spec> {
        PresetnI2c4ReqW::new(self, 1)
    }
    #[doc = "Bit 2 - presetn_i2c8 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_i2c8_req(&mut self) -> PresetnI2c8ReqW<SoftrstCon1Spec> {
        PresetnI2c8ReqW::new(self, 2)
    }
    #[doc = "Bit 3 - presetn_mailbox_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_mailbox_pmu_req(&mut self) -> PresetnMailboxPmuReqW<SoftrstCon1Spec> {
        PresetnMailboxPmuReqW::new(self, 3)
    }
    #[doc = "Bit 4 - presetn_rkpwm_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_rkpwm_pmu_req(&mut self) -> PresetnRkpwmPmuReqW<SoftrstCon1Spec> {
        PresetnRkpwmPmuReqW::new(self, 4)
    }
    #[doc = "Bit 5 - presetn_pmugrf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_pmugrf_req(&mut self) -> PresetnPmugrfReqW<SoftrstCon1Spec> {
        PresetnPmugrfReqW::new(self, 5)
    }
    #[doc = "Bit 6 - presetn_sgrf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_sgrf_req(&mut self) -> PresetnSgrfReqW<SoftrstCon1Spec> {
        PresetnSgrfReqW::new(self, 6)
    }
    #[doc = "Bit 7 - presetn_gpio0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_gpio0_req(&mut self) -> PresetnGpio0ReqW<SoftrstCon1Spec> {
        PresetnGpio0ReqW::new(self, 7)
    }
    #[doc = "Bit 8 - presetn_gpio1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_gpio1_req(&mut self) -> PresetnGpio1ReqW<SoftrstCon1Spec> {
        PresetnGpio1ReqW::new(self, 8)
    }
    #[doc = "Bit 9 - presetn_cru_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_cru_pmu_req(&mut self) -> PresetnCruPmuReqW<SoftrstCon1Spec> {
        PresetnCruPmuReqW::new(self, 9)
    }
    #[doc = "Bit 10 - presetn_intr_arb request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_intr_arb_req(&mut self) -> PresetnIntrArbReqW<SoftrstCon1Spec> {
        PresetnIntrArbReqW::new(self, 10)
    }
    #[doc = "Bit 11 - resetn_pvtm_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_pvtm_pmu_req(&mut self) -> ResetnPvtmPmuReqW<SoftrstCon1Spec> {
        ResetnPvtmPmuReqW::new(self, 11)
    }
    #[doc = "Bit 12 - resetn_i2c0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2c0_req(&mut self) -> ResetnI2c0ReqW<SoftrstCon1Spec> {
        ResetnI2c0ReqW::new(self, 12)
    }
    #[doc = "Bit 13 - resetn_i2c4 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2c4_req(&mut self) -> ResetnI2c4ReqW<SoftrstCon1Spec> {
        ResetnI2c4ReqW::new(self, 13)
    }
    #[doc = "Bit 14 - resetn_i2c8 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2c8_req(&mut self) -> ResetnI2c8ReqW<SoftrstCon1Spec> {
        ResetnI2c8ReqW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<SoftrstCon1Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftrstCon1Spec;
impl crate::RegisterSpec for SoftrstCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softrst_con1::R`](R) reader structure"]
impl crate::Readable for SoftrstCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`softrst_con1::W`](W) writer structure"]
impl crate::Writable for SoftrstCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTRST_CON1 to value 0"]
impl crate::Resettable for SoftrstCon1Spec {
    const RESET_VALUE: u32 = 0;
}
