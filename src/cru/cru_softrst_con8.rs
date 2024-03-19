#[doc = "Register `CRU_SOFTRST_CON8` reader"]
pub type R = crate::R<CruSoftrstCon8Spec>;
#[doc = "Register `CRU_SOFTRST_CON8` writer"]
pub type W = crate::W<CruSoftrstCon8Spec>;
#[doc = "Field `ARESETN_PCIE_REQ` reader - aresetn_pcie request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnPcieReqR = crate::BitReader;
#[doc = "Field `ARESETN_PCIE_REQ` writer - aresetn_pcie request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnPcieReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_PCIE_REQ` reader - presetn_pcie request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPcieReqR = crate::BitReader;
#[doc = "Field `PRESETN_PCIE_REQ` writer - presetn_pcie request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnPcieReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_PCIE_CORE_REQ` reader - resetn_pcie_core request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPcieCoreReqR = crate::BitReader;
#[doc = "Field `RESETN_PCIE_CORE_REQ` writer - resetn_pcie_core request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPcieCoreReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_PCIE_MGMT_REQ` reader - resetn_pcie_mgmt request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPcieMgmtReqR = crate::BitReader;
#[doc = "Field `RESETN_PCIE_MGMT_REQ` writer - resetn_pcie_mgmt request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPcieMgmtReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_PCIE_MGMT_STICKY_REQ` reader - resetn_pcie_mgmt_sticky request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPcieMgmtStickyReqR = crate::BitReader;
#[doc = "Field `RESETN_PCIE_MGMT_STICKY_REQ` writer - resetn_pcie_mgmt_sticky request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPcieMgmtStickyReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_PCIE_PIPE_REQ` reader - resetn_pcie_pipe request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPciePipeReqR = crate::BitReader;
#[doc = "Field `RESETN_PCIE_PIPE_REQ` writer - resetn_pcie_pipe request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPciePipeReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_PCIE_PM_REQ` reader - resetn_pcie_pm request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPciePmReqR = crate::BitReader;
#[doc = "Field `RESETN_PCIE_PM_REQ` writer - resetn_pcie_pm request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPciePmReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_PCIEPHY_REQ` reader - resetn_pciephy request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPciephyReqR = crate::BitReader;
#[doc = "Field `RESETN_PCIEPHY_REQ` writer - resetn_pciephy request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPciephyReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_GMAC_NOC_REQ` reader - aresetn_gmac_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnGmacNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_GMAC_NOC_REQ` writer - aresetn_gmac_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnGmacNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_GMAC_REQ` reader - aresetn_gmac request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnGmacReqR = crate::BitReader;
#[doc = "Field `ARESETN_GMAC_REQ` writer - aresetn_gmac request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnGmacReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_GMAC_NOC_REQ` reader - presetn_gmac_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGmacNocReqR = crate::BitReader;
#[doc = "Field `PRESETN_GMAC_NOC_REQ` writer - presetn_gmac_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGmacNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_GMAC_GRF_REQ` reader - presetn_gmac_grf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGmacGrfReqR = crate::BitReader;
#[doc = "Field `PRESETN_GMAC_GRF_REQ` writer - presetn_gmac_grf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnGmacGrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSICPHY_POR_RSTN_REQ` reader - hsicphy_por_rstn request bit\n\nWhen HIGH, reset relative logic"]
pub type HsicphyPorRstnReqR = crate::BitReader;
#[doc = "Field `HSICPHY_POR_RSTN_REQ` writer - hsicphy_por_rstn request bit\n\nWhen HIGH, reset relative logic"]
pub type HsicphyPorRstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSICPHY_UTMI_RSTN_REQ` reader - hsicphy_utmi_rstn request bit\n\nWhen HIGH, reset relative logic"]
pub type HsicphyUtmiRstnReqR = crate::BitReader;
#[doc = "Field `HSICPHY_UTMI_RSTN_REQ` writer - hsicphy_utmi_rstn request bit\n\nWhen HIGH, reset relative logic"]
pub type HsicphyUtmiRstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aresetn_pcie request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_pcie_req(&self) -> AresetnPcieReqR {
        AresetnPcieReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - presetn_pcie request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_pcie_req(&self) -> PresetnPcieReqR {
        PresetnPcieReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - resetn_pcie_core request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_pcie_core_req(&self) -> ResetnPcieCoreReqR {
        ResetnPcieCoreReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - resetn_pcie_mgmt request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_pcie_mgmt_req(&self) -> ResetnPcieMgmtReqR {
        ResetnPcieMgmtReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - resetn_pcie_mgmt_sticky request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_pcie_mgmt_sticky_req(&self) -> ResetnPcieMgmtStickyReqR {
        ResetnPcieMgmtStickyReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - resetn_pcie_pipe request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_pcie_pipe_req(&self) -> ResetnPciePipeReqR {
        ResetnPciePipeReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - resetn_pcie_pm request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_pcie_pm_req(&self) -> ResetnPciePmReqR {
        ResetnPciePmReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - resetn_pciephy request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_pciephy_req(&self) -> ResetnPciephyReqR {
        ResetnPciephyReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - aresetn_gmac_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_gmac_noc_req(&self) -> AresetnGmacNocReqR {
        AresetnGmacNocReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - aresetn_gmac request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_gmac_req(&self) -> AresetnGmacReqR {
        AresetnGmacReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - presetn_gmac_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_gmac_noc_req(&self) -> PresetnGmacNocReqR {
        PresetnGmacNocReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - presetn_gmac_grf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_gmac_grf_req(&self) -> PresetnGmacGrfReqR {
        PresetnGmacGrfReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - hsicphy_por_rstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hsicphy_por_rstn_req(&self) -> HsicphyPorRstnReqR {
        HsicphyPorRstnReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - hsicphy_utmi_rstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hsicphy_utmi_rstn_req(&self) -> HsicphyUtmiRstnReqR {
        HsicphyUtmiRstnReqR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aresetn_pcie request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_pcie_req(&mut self) -> AresetnPcieReqW<CruSoftrstCon8Spec> {
        AresetnPcieReqW::new(self, 0)
    }
    #[doc = "Bit 1 - presetn_pcie request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_pcie_req(&mut self) -> PresetnPcieReqW<CruSoftrstCon8Spec> {
        PresetnPcieReqW::new(self, 1)
    }
    #[doc = "Bit 2 - resetn_pcie_core request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_pcie_core_req(&mut self) -> ResetnPcieCoreReqW<CruSoftrstCon8Spec> {
        ResetnPcieCoreReqW::new(self, 2)
    }
    #[doc = "Bit 3 - resetn_pcie_mgmt request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_pcie_mgmt_req(&mut self) -> ResetnPcieMgmtReqW<CruSoftrstCon8Spec> {
        ResetnPcieMgmtReqW::new(self, 3)
    }
    #[doc = "Bit 4 - resetn_pcie_mgmt_sticky request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_pcie_mgmt_sticky_req(&mut self) -> ResetnPcieMgmtStickyReqW<CruSoftrstCon8Spec> {
        ResetnPcieMgmtStickyReqW::new(self, 4)
    }
    #[doc = "Bit 5 - resetn_pcie_pipe request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_pcie_pipe_req(&mut self) -> ResetnPciePipeReqW<CruSoftrstCon8Spec> {
        ResetnPciePipeReqW::new(self, 5)
    }
    #[doc = "Bit 6 - resetn_pcie_pm request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_pcie_pm_req(&mut self) -> ResetnPciePmReqW<CruSoftrstCon8Spec> {
        ResetnPciePmReqW::new(self, 6)
    }
    #[doc = "Bit 7 - resetn_pciephy request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_pciephy_req(&mut self) -> ResetnPciephyReqW<CruSoftrstCon8Spec> {
        ResetnPciephyReqW::new(self, 7)
    }
    #[doc = "Bit 8 - aresetn_gmac_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_gmac_noc_req(&mut self) -> AresetnGmacNocReqW<CruSoftrstCon8Spec> {
        AresetnGmacNocReqW::new(self, 8)
    }
    #[doc = "Bit 9 - aresetn_gmac request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_gmac_req(&mut self) -> AresetnGmacReqW<CruSoftrstCon8Spec> {
        AresetnGmacReqW::new(self, 9)
    }
    #[doc = "Bit 10 - presetn_gmac_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_gmac_noc_req(&mut self) -> PresetnGmacNocReqW<CruSoftrstCon8Spec> {
        PresetnGmacNocReqW::new(self, 10)
    }
    #[doc = "Bit 12 - presetn_gmac_grf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_gmac_grf_req(&mut self) -> PresetnGmacGrfReqW<CruSoftrstCon8Spec> {
        PresetnGmacGrfReqW::new(self, 12)
    }
    #[doc = "Bit 14 - hsicphy_por_rstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hsicphy_por_rstn_req(&mut self) -> HsicphyPorRstnReqW<CruSoftrstCon8Spec> {
        HsicphyPorRstnReqW::new(self, 14)
    }
    #[doc = "Bit 15 - hsicphy_utmi_rstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hsicphy_utmi_rstn_req(&mut self) -> HsicphyUtmiRstnReqW<CruSoftrstCon8Spec> {
        HsicphyUtmiRstnReqW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSoftrstCon8Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSoftrstCon8Spec;
impl crate::RegisterSpec for CruSoftrstCon8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_softrst_con8::R`](R) reader structure"]
impl crate::Readable for CruSoftrstCon8Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_softrst_con8::W`](W) writer structure"]
impl crate::Writable for CruSoftrstCon8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SOFTRST_CON8 to value 0xbc"]
impl crate::Resettable for CruSoftrstCon8Spec {
    const RESET_VALUE: u32 = 0xbc;
}
