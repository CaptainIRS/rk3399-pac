#[doc = "Register `CRU_SOFTRST_CON7` reader"]
pub type R = crate::R<CruSoftrstCon7Spec>;
#[doc = "Register `CRU_SOFTRST_CON7` writer"]
pub type W = crate::W<CruSoftrstCon7Spec>;
#[doc = "Field `ARESETN_PERIHP_NOC_REQ` reader - aresetn_perihp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnPerihpNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_PERIHP_NOC_REQ` writer - aresetn_perihp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnPerihpNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_PERIHP_GRF_REQ` reader - presetn_perihp_grf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPerihpGrfReqR = crate::BitReader;
#[doc = "Field `PRESETN_PERIHP_GRF_REQ` writer - presetn_perihp_grf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPerihpGrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_PERIHP_NOC_REQ` reader - hresetn_perihp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnPerihpNocReqR = crate::BitReader;
#[doc = "Field `HRESETN_PERIHP_NOC_REQ` writer - hresetn_perihp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnPerihpNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_USBHOST0_REQ` reader - hresetn_usbhost0 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnUsbhost0ReqR = crate::BitReader;
#[doc = "Field `HRESETN_USBHOST0_REQ` writer - hresetn_usbhost0 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnUsbhost0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_HOSTC0_AUX_REQ` reader - hresetn_hostc0_aux request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHostc0AuxReqR = crate::BitReader;
#[doc = "Field `HRESETN_HOSTC0_AUX_REQ` writer - hresetn_hostc0_aux request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHostc0AuxReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_HOST0_ARB_REQ` reader - hresetn_host0_arb request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHost0ArbReqR = crate::BitReader;
#[doc = "Field `HRESETN_HOST0_ARB_REQ` writer - hresetn_host0_arb request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHost0ArbReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_USBHOST1_REQ` reader - hresetn_usbhost1 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnUsbhost1ReqR = crate::BitReader;
#[doc = "Field `HRESETN_USBHOST1_REQ` writer - hresetn_usbhost1 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnUsbhost1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_HOSTC1_AUX_REQ` reader - hresetn_hostc1_aux request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHostc1AuxReqR = crate::BitReader;
#[doc = "Field `HRESETN_HOSTC1_AUX_REQ` writer - hresetn_hostc1_aux request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHostc1AuxReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_HOST1_ARB_REQ` reader - hresetn_host1_arb request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHost1ArbReqR = crate::BitReader;
#[doc = "Field `HRESETN_HOST1_ARB_REQ` writer - hresetn_host1_arb request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHost1ArbReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_SDIO0_REQ` reader - hresetn_sdio0 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnSdio0ReqR = crate::BitReader;
#[doc = "Field `HRESETN_SDIO0_REQ` writer - hresetn_sdio0 request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnSdio0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_SDMMC_REQ` reader - hresetn_sdmmc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnSdmmcReqR = crate::BitReader;
#[doc = "Field `HRESETN_SDMMC_REQ` writer - hresetn_sdmmc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnSdmmcReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_HSIC_REQ` reader - hresetn_hsic request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHsicReqR = crate::BitReader;
#[doc = "Field `HRESETN_HSIC_REQ` writer - hresetn_hsic request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHsicReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_HSIC_AUX_REQ` reader - hresetn_hsic_aux request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHsicAuxReqR = crate::BitReader;
#[doc = "Field `HRESETN_HSIC_AUX_REQ` writer - hresetn_hsic_aux request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHsicAuxReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_AHB1TOM_REQ` reader - hresetn_ahb1tom request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnAhb1tomReqR = crate::BitReader;
#[doc = "Field `HRESETN_AHB1TOM_REQ` writer - hresetn_ahb1tom request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnAhb1tomReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_PERIHP_NOC_REQ` reader - presetn_perihp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPerihpNocReqR = crate::BitReader;
#[doc = "Field `PRESETN_PERIHP_NOC_REQ` writer - presetn_perihp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPerihpNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_HSICPHY_REQ` reader - presetn_hsicphy request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnHsicphyReqR = crate::BitReader;
#[doc = "Field `PRESETN_HSICPHY_REQ` writer - presetn_hsicphy request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnHsicphyReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aresetn_perihp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_perihp_noc_req(&self) -> AresetnPerihpNocReqR {
        AresetnPerihpNocReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - presetn_perihp_grf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_perihp_grf_req(&self) -> PresetnPerihpGrfReqR {
        PresetnPerihpGrfReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - hresetn_perihp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_perihp_noc_req(&self) -> HresetnPerihpNocReqR {
        HresetnPerihpNocReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hresetn_usbhost0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_usbhost0_req(&self) -> HresetnUsbhost0ReqR {
        HresetnUsbhost0ReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hresetn_hostc0_aux request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_hostc0_aux_req(&self) -> HresetnHostc0AuxReqR {
        HresetnHostc0AuxReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - hresetn_host0_arb request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_host0_arb_req(&self) -> HresetnHost0ArbReqR {
        HresetnHost0ArbReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - hresetn_usbhost1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_usbhost1_req(&self) -> HresetnUsbhost1ReqR {
        HresetnUsbhost1ReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - hresetn_hostc1_aux request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_hostc1_aux_req(&self) -> HresetnHostc1AuxReqR {
        HresetnHostc1AuxReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - hresetn_host1_arb request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_host1_arb_req(&self) -> HresetnHost1ArbReqR {
        HresetnHost1ArbReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - hresetn_sdio0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_sdio0_req(&self) -> HresetnSdio0ReqR {
        HresetnSdio0ReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - hresetn_sdmmc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_sdmmc_req(&self) -> HresetnSdmmcReqR {
        HresetnSdmmcReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - hresetn_hsic request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_hsic_req(&self) -> HresetnHsicReqR {
        HresetnHsicReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - hresetn_hsic_aux request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_hsic_aux_req(&self) -> HresetnHsicAuxReqR {
        HresetnHsicAuxReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - hresetn_ahb1tom request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_ahb1tom_req(&self) -> HresetnAhb1tomReqR {
        HresetnAhb1tomReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - presetn_perihp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_perihp_noc_req(&self) -> PresetnPerihpNocReqR {
        PresetnPerihpNocReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - presetn_hsicphy request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_hsicphy_req(&self) -> PresetnHsicphyReqR {
        PresetnHsicphyReqR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aresetn_perihp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_perihp_noc_req(&mut self) -> AresetnPerihpNocReqW<CruSoftrstCon7Spec> {
        AresetnPerihpNocReqW::new(self, 0)
    }
    #[doc = "Bit 1 - presetn_perihp_grf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_perihp_grf_req(&mut self) -> PresetnPerihpGrfReqW<CruSoftrstCon7Spec> {
        PresetnPerihpGrfReqW::new(self, 1)
    }
    #[doc = "Bit 2 - hresetn_perihp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_perihp_noc_req(&mut self) -> HresetnPerihpNocReqW<CruSoftrstCon7Spec> {
        HresetnPerihpNocReqW::new(self, 2)
    }
    #[doc = "Bit 3 - hresetn_usbhost0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_usbhost0_req(&mut self) -> HresetnUsbhost0ReqW<CruSoftrstCon7Spec> {
        HresetnUsbhost0ReqW::new(self, 3)
    }
    #[doc = "Bit 4 - hresetn_hostc0_aux request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_hostc0_aux_req(&mut self) -> HresetnHostc0AuxReqW<CruSoftrstCon7Spec> {
        HresetnHostc0AuxReqW::new(self, 4)
    }
    #[doc = "Bit 5 - hresetn_host0_arb request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_host0_arb_req(&mut self) -> HresetnHost0ArbReqW<CruSoftrstCon7Spec> {
        HresetnHost0ArbReqW::new(self, 5)
    }
    #[doc = "Bit 6 - hresetn_usbhost1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_usbhost1_req(&mut self) -> HresetnUsbhost1ReqW<CruSoftrstCon7Spec> {
        HresetnUsbhost1ReqW::new(self, 6)
    }
    #[doc = "Bit 7 - hresetn_hostc1_aux request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_hostc1_aux_req(&mut self) -> HresetnHostc1AuxReqW<CruSoftrstCon7Spec> {
        HresetnHostc1AuxReqW::new(self, 7)
    }
    #[doc = "Bit 8 - hresetn_host1_arb request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_host1_arb_req(&mut self) -> HresetnHost1ArbReqW<CruSoftrstCon7Spec> {
        HresetnHost1ArbReqW::new(self, 8)
    }
    #[doc = "Bit 9 - hresetn_sdio0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_sdio0_req(&mut self) -> HresetnSdio0ReqW<CruSoftrstCon7Spec> {
        HresetnSdio0ReqW::new(self, 9)
    }
    #[doc = "Bit 10 - hresetn_sdmmc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_sdmmc_req(&mut self) -> HresetnSdmmcReqW<CruSoftrstCon7Spec> {
        HresetnSdmmcReqW::new(self, 10)
    }
    #[doc = "Bit 11 - hresetn_hsic request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_hsic_req(&mut self) -> HresetnHsicReqW<CruSoftrstCon7Spec> {
        HresetnHsicReqW::new(self, 11)
    }
    #[doc = "Bit 12 - hresetn_hsic_aux request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_hsic_aux_req(&mut self) -> HresetnHsicAuxReqW<CruSoftrstCon7Spec> {
        HresetnHsicAuxReqW::new(self, 12)
    }
    #[doc = "Bit 13 - hresetn_ahb1tom request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_ahb1tom_req(&mut self) -> HresetnAhb1tomReqW<CruSoftrstCon7Spec> {
        HresetnAhb1tomReqW::new(self, 13)
    }
    #[doc = "Bit 14 - presetn_perihp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_perihp_noc_req(&mut self) -> PresetnPerihpNocReqW<CruSoftrstCon7Spec> {
        PresetnPerihpNocReqW::new(self, 14)
    }
    #[doc = "Bit 15 - presetn_hsicphy request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_hsicphy_req(&mut self) -> PresetnHsicphyReqW<CruSoftrstCon7Spec> {
        PresetnHsicphyReqW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSoftrstCon7Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSoftrstCon7Spec;
impl crate::RegisterSpec for CruSoftrstCon7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_softrst_con7::R`](R) reader structure"]
impl crate::Readable for CruSoftrstCon7Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_softrst_con7::W`](W) writer structure"]
impl crate::Writable for CruSoftrstCon7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SOFTRST_CON7 to value 0"]
impl crate::Resettable for CruSoftrstCon7Spec {
    const RESET_VALUE: u32 = 0;
}
