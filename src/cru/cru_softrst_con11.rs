#[doc = "Register `CRU_SOFTRST_CON11` reader"]
pub type R = crate::R<CruSoftrstCon11Spec>;
#[doc = "Register `CRU_SOFTRST_CON11` writer"]
pub type W = crate::W<CruSoftrstCon11Spec>;
#[doc = "Field `PRESETN_DCF_REQ` reader - presetn_dcf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnDcfReqR = crate::BitReader;
#[doc = "Field `PRESETN_DCF_REQ` writer - presetn_dcf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnDcfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_CM0S_NOC_REQ` reader - hresetn_cm0s_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnCm0sNocReqR = crate::BitReader;
#[doc = "Field `HRESETN_CM0S_NOC_REQ` writer - hresetn_cm0s_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnCm0sNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_CM0S_REQ` reader - hresetn_cm0s request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnCm0sReqR = crate::BitReader;
#[doc = "Field `HRESETN_CM0S_REQ` writer - hresetn_cm0s request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnCm0sReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGRESETN_CM0S_REQ` reader - dbgresetn_cm0s request bit\n\nWhen HIGH, reset relative logic"]
pub type DbgresetnCm0sReqR = crate::BitReader;
#[doc = "Field `DBGRESETN_CM0S_REQ` writer - dbgresetn_cm0s request bit\n\nWhen HIGH, reset relative logic"]
pub type DbgresetnCm0sReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORESETN_CM0S_REQ` reader - poresetn_cm0s request bit\n\nWhen HIGH, reset relative logic"]
pub type PoresetnCm0sReqR = crate::BitReader;
#[doc = "Field `PORESETN_CM0S_REQ` writer - poresetn_cm0s request bit\n\nWhen HIGH, reset relative logic"]
pub type PoresetnCm0sReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_CRYPTO0_REQ` reader - resetn_crypto0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnCrypto0ReqR = crate::BitReader;
#[doc = "Field `RESETN_CRYPTO0_REQ` writer - resetn_crypto0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnCrypto0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_PERILP1_SGRF_REQ` reader - presetn_perilp1_sgrf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPerilp1SgrfReqR = crate::BitReader;
#[doc = "Field `PRESETN_PERILP1_SGRF_REQ` writer - presetn_perilp1_sgrf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPerilp1SgrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_PERILP1_GRF_REQ` reader - presetn_perilp1_grf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPerilp1GrfReqR = crate::BitReader;
#[doc = "Field `PRESETN_PERILP1_GRF_REQ` writer - presetn_perilp1_grf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPerilp1GrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_CRYPTO1_S_REQ` reader - hresetn_crypto1_s request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnCrypto1SReqR = crate::BitReader;
#[doc = "Field `HRESETN_CRYPTO1_S_REQ` writer - hresetn_crypto1_s request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnCrypto1SReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_CRYPTO1_M_REQ` reader - hresetn_crypto1_m request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnCrypto1MReqR = crate::BitReader;
#[doc = "Field `HRESETN_CRYPTO1_M_REQ` writer - hresetn_crypto1_m request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnCrypto1MReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_CRYPTO1_REQ` reader - resetn_crypto1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnCrypto1ReqR = crate::BitReader;
#[doc = "Field `RESETN_CRYPTO1_REQ` writer - resetn_crypto1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnCrypto1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_GIC_NOC_REQ` reader - aresetn_gic_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnGicNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_GIC_NOC_REQ` writer - aresetn_gic_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnGicNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_SD_NOC_REQ` reader - hresetn_sd_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnSdNocReqR = crate::BitReader;
#[doc = "Field `HRESETN_SD_NOC_REQ` writer - hresetn_sd_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnSdNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_SDIOAUDIO_BRG_REQ` reader - hresetn_sdioaudio_brg request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnSdioaudioBrgReqR = crate::BitReader;
#[doc = "Field `HRESETN_SDIOAUDIO_BRG_REQ` writer - hresetn_sdioaudio_brg request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnSdioaudioBrgReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - presetn_dcf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_dcf_req(&self) -> PresetnDcfReqR {
        PresetnDcfReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - hresetn_cm0s_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_cm0s_noc_req(&self) -> HresetnCm0sNocReqR {
        HresetnCm0sNocReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - hresetn_cm0s request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_cm0s_req(&self) -> HresetnCm0sReqR {
        HresetnCm0sReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - dbgresetn_cm0s request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn dbgresetn_cm0s_req(&self) -> DbgresetnCm0sReqR {
        DbgresetnCm0sReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - poresetn_cm0s request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn poresetn_cm0s_req(&self) -> PoresetnCm0sReqR {
        PoresetnCm0sReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - resetn_crypto0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_crypto0_req(&self) -> ResetnCrypto0ReqR {
        ResetnCrypto0ReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - presetn_perilp1_sgrf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_perilp1_sgrf_req(&self) -> PresetnPerilp1SgrfReqR {
        PresetnPerilp1SgrfReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - presetn_perilp1_grf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_perilp1_grf_req(&self) -> PresetnPerilp1GrfReqR {
        PresetnPerilp1GrfReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - hresetn_crypto1_s request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_crypto1_s_req(&self) -> HresetnCrypto1SReqR {
        HresetnCrypto1SReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - hresetn_crypto1_m request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_crypto1_m_req(&self) -> HresetnCrypto1MReqR {
        HresetnCrypto1MReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - resetn_crypto1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_crypto1_req(&self) -> ResetnCrypto1ReqR {
        ResetnCrypto1ReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - aresetn_gic_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_gic_noc_req(&self) -> AresetnGicNocReqR {
        AresetnGicNocReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - hresetn_sd_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_sd_noc_req(&self) -> HresetnSdNocReqR {
        HresetnSdNocReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - hresetn_sdioaudio_brg request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_sdioaudio_brg_req(&self) -> HresetnSdioaudioBrgReqR {
        HresetnSdioaudioBrgReqR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - presetn_dcf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_dcf_req(&mut self) -> PresetnDcfReqW<CruSoftrstCon11Spec> {
        PresetnDcfReqW::new(self, 0)
    }
    #[doc = "Bit 1 - hresetn_cm0s_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_cm0s_noc_req(&mut self) -> HresetnCm0sNocReqW<CruSoftrstCon11Spec> {
        HresetnCm0sNocReqW::new(self, 1)
    }
    #[doc = "Bit 2 - hresetn_cm0s request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_cm0s_req(&mut self) -> HresetnCm0sReqW<CruSoftrstCon11Spec> {
        HresetnCm0sReqW::new(self, 2)
    }
    #[doc = "Bit 3 - dbgresetn_cm0s request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn dbgresetn_cm0s_req(&mut self) -> DbgresetnCm0sReqW<CruSoftrstCon11Spec> {
        DbgresetnCm0sReqW::new(self, 3)
    }
    #[doc = "Bit 4 - poresetn_cm0s request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn poresetn_cm0s_req(&mut self) -> PoresetnCm0sReqW<CruSoftrstCon11Spec> {
        PoresetnCm0sReqW::new(self, 4)
    }
    #[doc = "Bit 5 - resetn_crypto0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_crypto0_req(&mut self) -> ResetnCrypto0ReqW<CruSoftrstCon11Spec> {
        ResetnCrypto0ReqW::new(self, 5)
    }
    #[doc = "Bit 6 - presetn_perilp1_sgrf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_perilp1_sgrf_req(&mut self) -> PresetnPerilp1SgrfReqW<CruSoftrstCon11Spec> {
        PresetnPerilp1SgrfReqW::new(self, 6)
    }
    #[doc = "Bit 7 - presetn_perilp1_grf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_perilp1_grf_req(&mut self) -> PresetnPerilp1GrfReqW<CruSoftrstCon11Spec> {
        PresetnPerilp1GrfReqW::new(self, 7)
    }
    #[doc = "Bit 8 - hresetn_crypto1_s request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_crypto1_s_req(&mut self) -> HresetnCrypto1SReqW<CruSoftrstCon11Spec> {
        HresetnCrypto1SReqW::new(self, 8)
    }
    #[doc = "Bit 9 - hresetn_crypto1_m request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_crypto1_m_req(&mut self) -> HresetnCrypto1MReqW<CruSoftrstCon11Spec> {
        HresetnCrypto1MReqW::new(self, 9)
    }
    #[doc = "Bit 10 - resetn_crypto1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_crypto1_req(&mut self) -> ResetnCrypto1ReqW<CruSoftrstCon11Spec> {
        ResetnCrypto1ReqW::new(self, 10)
    }
    #[doc = "Bit 12 - aresetn_gic_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_gic_noc_req(&mut self) -> AresetnGicNocReqW<CruSoftrstCon11Spec> {
        AresetnGicNocReqW::new(self, 12)
    }
    #[doc = "Bit 13 - hresetn_sd_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_sd_noc_req(&mut self) -> HresetnSdNocReqW<CruSoftrstCon11Spec> {
        HresetnSdNocReqW::new(self, 13)
    }
    #[doc = "Bit 14 - hresetn_sdioaudio_brg request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_sdioaudio_brg_req(&mut self) -> HresetnSdioaudioBrgReqW<CruSoftrstCon11Spec> {
        HresetnSdioaudioBrgReqW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSoftrstCon11Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSoftrstCon11Spec;
impl crate::RegisterSpec for CruSoftrstCon11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_softrst_con11::R`](R) reader structure"]
impl crate::Readable for CruSoftrstCon11Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_softrst_con11::W`](W) writer structure"]
impl crate::Writable for CruSoftrstCon11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SOFTRST_CON11 to value 0x14"]
impl crate::Resettable for CruSoftrstCon11Spec {
    const RESET_VALUE: u32 = 0x14;
}
