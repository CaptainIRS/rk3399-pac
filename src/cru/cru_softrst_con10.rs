#[doc = "Register `CRU_SOFTRST_CON10` reader"]
pub type R = crate::R<CruSoftrstCon10Spec>;
#[doc = "Register `CRU_SOFTRST_CON10` writer"]
pub type W = crate::W<CruSoftrstCon10Spec>;
#[doc = "Field `ARESETN_PERILP0_NOC_REQ` reader - aresetn_perilp0_noc request bit When HIGH, reset relative logic"]
pub type AresetnPerilp0NocReqR = crate::BitReader;
#[doc = "Field `ARESETN_PERILP0_NOC_REQ` writer - aresetn_perilp0_noc request bit When HIGH, reset relative logic"]
pub type AresetnPerilp0NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_DCF_REQ` reader - aresetn_dcf request bit When HIGH, reset relative logic"]
pub type AresetnDcfReqR = crate::BitReader;
#[doc = "Field `ARESETN_DCF_REQ` writer - aresetn_dcf request bit When HIGH, reset relative logic"]
pub type AresetnDcfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_GIC500_REQ` reader - aresetn_gic500 request bit When HIGH, reset relative logic"]
pub type AresetnGic500ReqR = crate::BitReader;
#[doc = "Field `ARESETN_GIC500_REQ` writer - aresetn_gic500 request bit When HIGH, reset relative logic"]
pub type AresetnGic500ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_DMAC0_PERILP0_REQ` reader - aresetn_dmac0_perilp0 request bit When HIGH, reset relative logic"]
pub type AresetnDmac0Perilp0ReqR = crate::BitReader;
#[doc = "Field `ARESETN_DMAC0_PERILP0_REQ` writer - aresetn_dmac0_perilp0 request bit When HIGH, reset relative logic"]
pub type AresetnDmac0Perilp0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_DMAC1_PERILP0_REQ` reader - aresetn_dmac1_perilp0 request bit When HIGH, reset relative logic"]
pub type AresetnDmac1Perilp0ReqR = crate::BitReader;
#[doc = "Field `ARESETN_DMAC1_PERILP0_REQ` writer - aresetn_dmac1_perilp0 request bit When HIGH, reset relative logic"]
pub type AresetnDmac1Perilp0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_TZMA_REQ` reader - aresetn_tzma request bit When HIGH, reset relative logic"]
pub type AresetnTzmaReqR = crate::BitReader;
#[doc = "Field `ARESETN_TZMA_REQ` writer - aresetn_tzma request bit When HIGH, reset relative logic"]
pub type AresetnTzmaReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_INTMEM_REQ` reader - aresetn_intmem request bit When HIGH, reset relative logic"]
pub type AresetnIntmemReqR = crate::BitReader;
#[doc = "Field `ARESETN_INTMEM_REQ` writer - aresetn_intmem request bit When HIGH, reset relative logic"]
pub type AresetnIntmemReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_ADB400_MST0_REQ` reader - aresetn_adb400_mst0 request bit When HIGH, reset relative logic"]
pub type AresetnAdb400Mst0ReqR = crate::BitReader;
#[doc = "Field `ARESETN_ADB400_MST0_REQ` writer - aresetn_adb400_mst0 request bit When HIGH, reset relative logic"]
pub type AresetnAdb400Mst0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_ADB400_MST1_REQ` reader - aresetn_adb400_mst1 request bit When HIGH, reset relative logic"]
pub type AresetnAdb400Mst1ReqR = crate::BitReader;
#[doc = "Field `ARESETN_ADB400_MST1_REQ` writer - aresetn_adb400_mst1 request bit When HIGH, reset relative logic"]
pub type AresetnAdb400Mst1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_ADB400_SLV0_REQ` reader - aresetn_adb400_slv0 request bit When HIGH, reset relative logic"]
pub type AresetnAdb400Slv0ReqR = crate::BitReader;
#[doc = "Field `ARESETN_ADB400_SLV0_REQ` writer - aresetn_adb400_slv0 request bit When HIGH, reset relative logic"]
pub type AresetnAdb400Slv0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_ADB400_SLV1_REQ` reader - aresetn_adb400_slv1 request bit When HIGH, reset relative logic"]
pub type AresetnAdb400Slv1ReqR = crate::BitReader;
#[doc = "Field `ARESETN_ADB400_SLV1_REQ` writer - aresetn_adb400_slv1 request bit When HIGH, reset relative logic"]
pub type AresetnAdb400Slv1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_PERILP0_REQ` reader - hresetn_perilp0 request bit When HIGH, reset relative logic"]
pub type HresetnPerilp0ReqR = crate::BitReader;
#[doc = "Field `HRESETN_PERILP0_REQ` writer - hresetn_perilp0 request bit When HIGH, reset relative logic"]
pub type HresetnPerilp0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_PERILP0_NOC_REQ` reader - hresetn_perilp0_noc request bit When HIGH, reset relative logic"]
pub type HresetnPerilp0NocReqR = crate::BitReader;
#[doc = "Field `HRESETN_PERILP0_NOC_REQ` writer - hresetn_perilp0_noc request bit When HIGH, reset relative logic"]
pub type HresetnPerilp0NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_ROM_REQ` reader - hresetn_rom request bit When HIGH, reset relative logic"]
pub type HresetnRomReqR = crate::BitReader;
#[doc = "Field `HRESETN_ROM_REQ` writer - hresetn_rom request bit When HIGH, reset relative logic"]
pub type HresetnRomReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_CRYPTO0_S_REQ` reader - hresetn_crypto0_s request bit When HIGH, reset relative logic"]
pub type HresetnCrypto0SReqR = crate::BitReader;
#[doc = "Field `HRESETN_CRYPTO0_S_REQ` writer - hresetn_crypto0_s request bit When HIGH, reset relative logic"]
pub type HresetnCrypto0SReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_CRYPTO0_M_REQ` reader - hresetn_crypto0_m request bit When HIGH, reset relative logic"]
pub type HresetnCrypto0MReqR = crate::BitReader;
#[doc = "Field `HRESETN_CRYPTO0_M_REQ` writer - hresetn_crypto0_m request bit When HIGH, reset relative logic"]
pub type HresetnCrypto0MReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aresetn_perilp0_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_perilp0_noc_req(&self) -> AresetnPerilp0NocReqR {
        AresetnPerilp0NocReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aresetn_dcf request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_dcf_req(&self) -> AresetnDcfReqR {
        AresetnDcfReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aresetn_gic500 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_gic500_req(&self) -> AresetnGic500ReqR {
        AresetnGic500ReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - aresetn_dmac0_perilp0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_dmac0_perilp0_req(&self) -> AresetnDmac0Perilp0ReqR {
        AresetnDmac0Perilp0ReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - aresetn_dmac1_perilp0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_dmac1_perilp0_req(&self) -> AresetnDmac1Perilp0ReqR {
        AresetnDmac1Perilp0ReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - aresetn_tzma request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_tzma_req(&self) -> AresetnTzmaReqR {
        AresetnTzmaReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - aresetn_intmem request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_intmem_req(&self) -> AresetnIntmemReqR {
        AresetnIntmemReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - aresetn_adb400_mst0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_adb400_mst0_req(&self) -> AresetnAdb400Mst0ReqR {
        AresetnAdb400Mst0ReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - aresetn_adb400_mst1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_adb400_mst1_req(&self) -> AresetnAdb400Mst1ReqR {
        AresetnAdb400Mst1ReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - aresetn_adb400_slv0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_adb400_slv0_req(&self) -> AresetnAdb400Slv0ReqR {
        AresetnAdb400Slv0ReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - aresetn_adb400_slv1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_adb400_slv1_req(&self) -> AresetnAdb400Slv1ReqR {
        AresetnAdb400Slv1ReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - hresetn_perilp0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_perilp0_req(&self) -> HresetnPerilp0ReqR {
        HresetnPerilp0ReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - hresetn_perilp0_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_perilp0_noc_req(&self) -> HresetnPerilp0NocReqR {
        HresetnPerilp0NocReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - hresetn_rom request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_rom_req(&self) -> HresetnRomReqR {
        HresetnRomReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - hresetn_crypto0_s request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_crypto0_s_req(&self) -> HresetnCrypto0SReqR {
        HresetnCrypto0SReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - hresetn_crypto0_m request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_crypto0_m_req(&self) -> HresetnCrypto0MReqR {
        HresetnCrypto0MReqR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aresetn_perilp0_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_perilp0_noc_req(&mut self) -> AresetnPerilp0NocReqW<CruSoftrstCon10Spec> {
        AresetnPerilp0NocReqW::new(self, 0)
    }
    #[doc = "Bit 1 - aresetn_dcf request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_dcf_req(&mut self) -> AresetnDcfReqW<CruSoftrstCon10Spec> {
        AresetnDcfReqW::new(self, 1)
    }
    #[doc = "Bit 2 - aresetn_gic500 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_gic500_req(&mut self) -> AresetnGic500ReqW<CruSoftrstCon10Spec> {
        AresetnGic500ReqW::new(self, 2)
    }
    #[doc = "Bit 3 - aresetn_dmac0_perilp0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_dmac0_perilp0_req(&mut self) -> AresetnDmac0Perilp0ReqW<CruSoftrstCon10Spec> {
        AresetnDmac0Perilp0ReqW::new(self, 3)
    }
    #[doc = "Bit 4 - aresetn_dmac1_perilp0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_dmac1_perilp0_req(&mut self) -> AresetnDmac1Perilp0ReqW<CruSoftrstCon10Spec> {
        AresetnDmac1Perilp0ReqW::new(self, 4)
    }
    #[doc = "Bit 5 - aresetn_tzma request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_tzma_req(&mut self) -> AresetnTzmaReqW<CruSoftrstCon10Spec> {
        AresetnTzmaReqW::new(self, 5)
    }
    #[doc = "Bit 6 - aresetn_intmem request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_intmem_req(&mut self) -> AresetnIntmemReqW<CruSoftrstCon10Spec> {
        AresetnIntmemReqW::new(self, 6)
    }
    #[doc = "Bit 7 - aresetn_adb400_mst0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_adb400_mst0_req(&mut self) -> AresetnAdb400Mst0ReqW<CruSoftrstCon10Spec> {
        AresetnAdb400Mst0ReqW::new(self, 7)
    }
    #[doc = "Bit 8 - aresetn_adb400_mst1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_adb400_mst1_req(&mut self) -> AresetnAdb400Mst1ReqW<CruSoftrstCon10Spec> {
        AresetnAdb400Mst1ReqW::new(self, 8)
    }
    #[doc = "Bit 9 - aresetn_adb400_slv0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_adb400_slv0_req(&mut self) -> AresetnAdb400Slv0ReqW<CruSoftrstCon10Spec> {
        AresetnAdb400Slv0ReqW::new(self, 9)
    }
    #[doc = "Bit 10 - aresetn_adb400_slv1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_adb400_slv1_req(&mut self) -> AresetnAdb400Slv1ReqW<CruSoftrstCon10Spec> {
        AresetnAdb400Slv1ReqW::new(self, 10)
    }
    #[doc = "Bit 11 - hresetn_perilp0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_perilp0_req(&mut self) -> HresetnPerilp0ReqW<CruSoftrstCon10Spec> {
        HresetnPerilp0ReqW::new(self, 11)
    }
    #[doc = "Bit 12 - hresetn_perilp0_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_perilp0_noc_req(&mut self) -> HresetnPerilp0NocReqW<CruSoftrstCon10Spec> {
        HresetnPerilp0NocReqW::new(self, 12)
    }
    #[doc = "Bit 13 - hresetn_rom request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_rom_req(&mut self) -> HresetnRomReqW<CruSoftrstCon10Spec> {
        HresetnRomReqW::new(self, 13)
    }
    #[doc = "Bit 14 - hresetn_crypto0_s request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_crypto0_s_req(&mut self) -> HresetnCrypto0SReqW<CruSoftrstCon10Spec> {
        HresetnCrypto0SReqW::new(self, 14)
    }
    #[doc = "Bit 15 - hresetn_crypto0_m request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_crypto0_m_req(&mut self) -> HresetnCrypto0MReqW<CruSoftrstCon10Spec> {
        HresetnCrypto0MReqW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSoftrstCon10Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSoftrstCon10Spec;
impl crate::RegisterSpec for CruSoftrstCon10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_softrst_con10::R`](R) reader structure"]
impl crate::Readable for CruSoftrstCon10Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_softrst_con10::W`](W) writer structure"]
impl crate::Writable for CruSoftrstCon10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SOFTRST_CON10 to value 0"]
impl crate::Resettable for CruSoftrstCon10Spec {
    const RESET_VALUE: u32 = 0;
}
