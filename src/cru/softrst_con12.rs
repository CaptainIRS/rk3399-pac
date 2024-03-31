#[doc = "Register `SOFTRST_CON12` reader"]
pub type R = crate::R<SoftrstCon12Spec>;
#[doc = "Register `SOFTRST_CON12` writer"]
pub type W = crate::W<SoftrstCon12Spec>;
#[doc = "Field `HRESETN_PERILP1_REQ` reader - hresetn_perilp1 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnPerilp1ReqR = crate::BitReader;
#[doc = "Field `HRESETN_PERILP1_REQ` writer - hresetn_perilp1 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnPerilp1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_PERILP1_NOC_REQ` reader - hresetn_perilp1_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnPerilp1NocReqR = crate::BitReader;
#[doc = "Field `HRESETN_PERILP1_NOC_REQ` writer - hresetn_perilp1_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnPerilp1NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_I2S0_REQ` reader - hresetn_i2s0 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnI2s0ReqR = crate::BitReader;
#[doc = "Field `HRESETN_I2S0_REQ` writer - hresetn_i2s0 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnI2s0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_I2S1_REQ` reader - hresetn_i2s1 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnI2s1ReqR = crate::BitReader;
#[doc = "Field `HRESETN_I2S1_REQ` writer - hresetn_i2s1 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnI2s1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_I2S2_REQ` reader - hresetn_i2s2 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnI2s2ReqR = crate::BitReader;
#[doc = "Field `HRESETN_I2S2_REQ` writer - hresetn_i2s2 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnI2s2ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_SPDIF_8CH_REQ` reader - hresetn_spdif_8ch request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnSpdif8chReqR = crate::BitReader;
#[doc = "Field `HRESETN_SPDIF_8CH_REQ` writer - hresetn_spdif_8ch request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnSpdif8chReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_PERILP1_NOC_REQ` reader - presetn_perilp1_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPerilp1NocReqR = crate::BitReader;
#[doc = "Field `PRESETN_PERILP1_NOC_REQ` writer - presetn_perilp1_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPerilp1NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_EFUSE_1024_REQ` reader - presetn_efuse_1024 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnEfuse1024ReqR = crate::BitReader;
#[doc = "Field `PRESETN_EFUSE_1024_REQ` writer - presetn_efuse_1024 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnEfuse1024ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_EFUSE_1024S_REQ` reader - presetn_efuse_1024s request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnEfuse1024sReqR = crate::BitReader;
#[doc = "Field `PRESETN_EFUSE_1024S_REQ` writer - presetn_efuse_1024s request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnEfuse1024sReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_I2C1_REQ` reader - presetn_i2c1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c1ReqR = crate::BitReader;
#[doc = "Field `PRESETN_I2C1_REQ` writer - presetn_i2c1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_I2C5_REQ` reader - presetn_i2c5 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c5ReqR = crate::BitReader;
#[doc = "Field `PRESETN_I2C5_REQ` writer - presetn_i2c5 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c5ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_I2C2_REQ` reader - presetn_i2c2 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c2ReqR = crate::BitReader;
#[doc = "Field `PRESETN_I2C2_REQ` writer - presetn_i2c2 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c2ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_I2C6_REQ` reader - presetn_i2c6 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c6ReqR = crate::BitReader;
#[doc = "Field `PRESETN_I2C6_REQ` writer - presetn_i2c6 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c6ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_I2C3_REQ` reader - presetn_i2c3 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c3ReqR = crate::BitReader;
#[doc = "Field `PRESETN_I2C3_REQ` writer - presetn_i2c3 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c3ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_I2C7_REQ` reader - presetn_i2c7 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c7ReqR = crate::BitReader;
#[doc = "Field `PRESETN_I2C7_REQ` writer - presetn_i2c7 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnI2c7ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_MAILBOX0_REQ` reader - presetn_mailbox0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnMailbox0ReqR = crate::BitReader;
#[doc = "Field `PRESETN_MAILBOX0_REQ` writer - presetn_mailbox0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnMailbox0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - hresetn_perilp1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_perilp1_req(&self) -> HresetnPerilp1ReqR {
        HresetnPerilp1ReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - hresetn_perilp1_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_perilp1_noc_req(&self) -> HresetnPerilp1NocReqR {
        HresetnPerilp1NocReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - hresetn_i2s0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_i2s0_req(&self) -> HresetnI2s0ReqR {
        HresetnI2s0ReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hresetn_i2s1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_i2s1_req(&self) -> HresetnI2s1ReqR {
        HresetnI2s1ReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hresetn_i2s2 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_i2s2_req(&self) -> HresetnI2s2ReqR {
        HresetnI2s2ReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - hresetn_spdif_8ch request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_spdif_8ch_req(&self) -> HresetnSpdif8chReqR {
        HresetnSpdif8chReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - presetn_perilp1_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_perilp1_noc_req(&self) -> PresetnPerilp1NocReqR {
        PresetnPerilp1NocReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - presetn_efuse_1024 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_efuse_1024_req(&self) -> PresetnEfuse1024ReqR {
        PresetnEfuse1024ReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - presetn_efuse_1024s request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_efuse_1024s_req(&self) -> PresetnEfuse1024sReqR {
        PresetnEfuse1024sReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - presetn_i2c1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_i2c1_req(&self) -> PresetnI2c1ReqR {
        PresetnI2c1ReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - presetn_i2c5 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_i2c5_req(&self) -> PresetnI2c5ReqR {
        PresetnI2c5ReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - presetn_i2c2 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_i2c2_req(&self) -> PresetnI2c2ReqR {
        PresetnI2c2ReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - presetn_i2c6 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_i2c6_req(&self) -> PresetnI2c6ReqR {
        PresetnI2c6ReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - presetn_i2c3 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_i2c3_req(&self) -> PresetnI2c3ReqR {
        PresetnI2c3ReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - presetn_i2c7 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_i2c7_req(&self) -> PresetnI2c7ReqR {
        PresetnI2c7ReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - presetn_mailbox0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_mailbox0_req(&self) -> PresetnMailbox0ReqR {
        PresetnMailbox0ReqR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - hresetn_perilp1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_perilp1_req(&mut self) -> HresetnPerilp1ReqW<SoftrstCon12Spec> {
        HresetnPerilp1ReqW::new(self, 0)
    }
    #[doc = "Bit 1 - hresetn_perilp1_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_perilp1_noc_req(&mut self) -> HresetnPerilp1NocReqW<SoftrstCon12Spec> {
        HresetnPerilp1NocReqW::new(self, 1)
    }
    #[doc = "Bit 2 - hresetn_i2s0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_i2s0_req(&mut self) -> HresetnI2s0ReqW<SoftrstCon12Spec> {
        HresetnI2s0ReqW::new(self, 2)
    }
    #[doc = "Bit 3 - hresetn_i2s1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_i2s1_req(&mut self) -> HresetnI2s1ReqW<SoftrstCon12Spec> {
        HresetnI2s1ReqW::new(self, 3)
    }
    #[doc = "Bit 4 - hresetn_i2s2 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_i2s2_req(&mut self) -> HresetnI2s2ReqW<SoftrstCon12Spec> {
        HresetnI2s2ReqW::new(self, 4)
    }
    #[doc = "Bit 5 - hresetn_spdif_8ch request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_spdif_8ch_req(&mut self) -> HresetnSpdif8chReqW<SoftrstCon12Spec> {
        HresetnSpdif8chReqW::new(self, 5)
    }
    #[doc = "Bit 6 - presetn_perilp1_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_perilp1_noc_req(&mut self) -> PresetnPerilp1NocReqW<SoftrstCon12Spec> {
        PresetnPerilp1NocReqW::new(self, 6)
    }
    #[doc = "Bit 7 - presetn_efuse_1024 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_efuse_1024_req(&mut self) -> PresetnEfuse1024ReqW<SoftrstCon12Spec> {
        PresetnEfuse1024ReqW::new(self, 7)
    }
    #[doc = "Bit 8 - presetn_efuse_1024s request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_efuse_1024s_req(&mut self) -> PresetnEfuse1024sReqW<SoftrstCon12Spec> {
        PresetnEfuse1024sReqW::new(self, 8)
    }
    #[doc = "Bit 9 - presetn_i2c1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_i2c1_req(&mut self) -> PresetnI2c1ReqW<SoftrstCon12Spec> {
        PresetnI2c1ReqW::new(self, 9)
    }
    #[doc = "Bit 10 - presetn_i2c5 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_i2c5_req(&mut self) -> PresetnI2c5ReqW<SoftrstCon12Spec> {
        PresetnI2c5ReqW::new(self, 10)
    }
    #[doc = "Bit 11 - presetn_i2c2 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_i2c2_req(&mut self) -> PresetnI2c2ReqW<SoftrstCon12Spec> {
        PresetnI2c2ReqW::new(self, 11)
    }
    #[doc = "Bit 12 - presetn_i2c6 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_i2c6_req(&mut self) -> PresetnI2c6ReqW<SoftrstCon12Spec> {
        PresetnI2c6ReqW::new(self, 12)
    }
    #[doc = "Bit 13 - presetn_i2c3 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_i2c3_req(&mut self) -> PresetnI2c3ReqW<SoftrstCon12Spec> {
        PresetnI2c3ReqW::new(self, 13)
    }
    #[doc = "Bit 14 - presetn_i2c7 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_i2c7_req(&mut self) -> PresetnI2c7ReqW<SoftrstCon12Spec> {
        PresetnI2c7ReqW::new(self, 14)
    }
    #[doc = "Bit 15 - presetn_mailbox0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_mailbox0_req(&mut self) -> PresetnMailbox0ReqW<SoftrstCon12Spec> {
        PresetnMailbox0ReqW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<SoftrstCon12Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftrstCon12Spec;
impl crate::RegisterSpec for SoftrstCon12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softrst_con12::R`](R) reader structure"]
impl crate::Readable for SoftrstCon12Spec {}
#[doc = "`write(|w| ..)` method takes [`softrst_con12::W`](W) writer structure"]
impl crate::Writable for SoftrstCon12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTRST_CON12 to value 0"]
impl crate::Resettable for SoftrstCon12Spec {
    const RESET_VALUE: u32 = 0;
}
