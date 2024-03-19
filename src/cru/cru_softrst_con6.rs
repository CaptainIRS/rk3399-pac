#[doc = "Register `CRU_SOFTRST_CON6` reader"]
pub type R = crate::R<CruSoftrstCon6Spec>;
#[doc = "Register `CRU_SOFTRST_CON6` writer"]
pub type W = crate::W<CruSoftrstCon6Spec>;
#[doc = "Field `ARESETN_IEP_NOC_REQ` reader - aresetn_iep_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnIepNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_IEP_NOC_REQ` writer - aresetn_iep_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnIepNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_VOP_IEP_REQ` reader - aresetn_vop_iep request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVopIepReqR = crate::BitReader;
#[doc = "Field `ARESETN_VOP_IEP_REQ` writer - aresetn_vop_iep request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVopIepReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_IEP_REQ` reader - aresetn_iep request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnIepReqR = crate::BitReader;
#[doc = "Field `ARESETN_IEP_REQ` writer - aresetn_iep request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnIepReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_IEP_NOC_REQ` reader - hresetn_iep_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnIepNocReqR = crate::BitReader;
#[doc = "Field `HRESETN_IEP_NOC_REQ` writer - hresetn_iep_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnIepNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_IEP_REQ` reader - hresetn_iep request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnIepReqR = crate::BitReader;
#[doc = "Field `HRESETN_IEP_REQ` writer - hresetn_iep request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnIepReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_RGA_NOC_REQ` reader - aresetn_rga_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnRgaNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_RGA_NOC_REQ` writer - aresetn_rga_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnRgaNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_RGA_REQ` reader - aresetn_rga request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnRgaReqR = crate::BitReader;
#[doc = "Field `ARESETN_RGA_REQ` writer - aresetn_rga request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnRgaReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_RGA_NOC_REQ` reader - hresetn_rga_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnRgaNocReqR = crate::BitReader;
#[doc = "Field `HRESETN_RGA_NOC_REQ` writer - hresetn_rga_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnRgaNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_RGA_REQ` reader - hresetn_rga request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnRgaReqR = crate::BitReader;
#[doc = "Field `HRESETN_RGA_REQ` writer - hresetn_rga request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnRgaReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_RGA_CORE_REQ` reader - resetn_rga_core request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnRgaCoreReqR = crate::BitReader;
#[doc = "Field `RESETN_RGA_CORE_REQ` writer - resetn_rga_core request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnRgaCoreReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_EMMC_NOC_REQ` reader - aresetn_emmc_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnEmmcNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_EMMC_NOC_REQ` writer - aresetn_emmc_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnEmmcNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_EMMC_REQ` reader - aresetn_emmc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnEmmcReqR = crate::BitReader;
#[doc = "Field `ARESETN_EMMC_REQ` writer - aresetn_emmc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnEmmcReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_EMMC_GRF_REQ` reader - aresetn_emmc_grf request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnEmmcGrfReqR = crate::BitReader;
#[doc = "Field `ARESETN_EMMC_GRF_REQ` writer - aresetn_emmc_grf request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnEmmcGrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aresetn_iep_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_iep_noc_req(&self) -> AresetnIepNocReqR {
        AresetnIepNocReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aresetn_vop_iep request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_vop_iep_req(&self) -> AresetnVopIepReqR {
        AresetnVopIepReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aresetn_iep request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_iep_req(&self) -> AresetnIepReqR {
        AresetnIepReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hresetn_iep_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_iep_noc_req(&self) -> HresetnIepNocReqR {
        HresetnIepNocReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hresetn_iep request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_iep_req(&self) -> HresetnIepReqR {
        HresetnIepReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - aresetn_rga_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_rga_noc_req(&self) -> AresetnRgaNocReqR {
        AresetnRgaNocReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - aresetn_rga request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_rga_req(&self) -> AresetnRgaReqR {
        AresetnRgaReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - hresetn_rga_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_rga_noc_req(&self) -> HresetnRgaNocReqR {
        HresetnRgaNocReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - hresetn_rga request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_rga_req(&self) -> HresetnRgaReqR {
        HresetnRgaReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - resetn_rga_core request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_rga_core_req(&self) -> ResetnRgaCoreReqR {
        ResetnRgaCoreReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - aresetn_emmc_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_emmc_noc_req(&self) -> AresetnEmmcNocReqR {
        AresetnEmmcNocReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - aresetn_emmc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_emmc_req(&self) -> AresetnEmmcReqR {
        AresetnEmmcReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - aresetn_emmc_grf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_emmc_grf_req(&self) -> AresetnEmmcGrfReqR {
        AresetnEmmcGrfReqR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aresetn_iep_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_iep_noc_req(&mut self) -> AresetnIepNocReqW<CruSoftrstCon6Spec> {
        AresetnIepNocReqW::new(self, 0)
    }
    #[doc = "Bit 1 - aresetn_vop_iep request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_vop_iep_req(&mut self) -> AresetnVopIepReqW<CruSoftrstCon6Spec> {
        AresetnVopIepReqW::new(self, 1)
    }
    #[doc = "Bit 2 - aresetn_iep request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_iep_req(&mut self) -> AresetnIepReqW<CruSoftrstCon6Spec> {
        AresetnIepReqW::new(self, 2)
    }
    #[doc = "Bit 3 - hresetn_iep_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_iep_noc_req(&mut self) -> HresetnIepNocReqW<CruSoftrstCon6Spec> {
        HresetnIepNocReqW::new(self, 3)
    }
    #[doc = "Bit 4 - hresetn_iep request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_iep_req(&mut self) -> HresetnIepReqW<CruSoftrstCon6Spec> {
        HresetnIepReqW::new(self, 4)
    }
    #[doc = "Bit 6 - aresetn_rga_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_rga_noc_req(&mut self) -> AresetnRgaNocReqW<CruSoftrstCon6Spec> {
        AresetnRgaNocReqW::new(self, 6)
    }
    #[doc = "Bit 7 - aresetn_rga request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_rga_req(&mut self) -> AresetnRgaReqW<CruSoftrstCon6Spec> {
        AresetnRgaReqW::new(self, 7)
    }
    #[doc = "Bit 8 - hresetn_rga_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_rga_noc_req(&mut self) -> HresetnRgaNocReqW<CruSoftrstCon6Spec> {
        HresetnRgaNocReqW::new(self, 8)
    }
    #[doc = "Bit 9 - hresetn_rga request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_rga_req(&mut self) -> HresetnRgaReqW<CruSoftrstCon6Spec> {
        HresetnRgaReqW::new(self, 9)
    }
    #[doc = "Bit 10 - resetn_rga_core request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_rga_core_req(&mut self) -> ResetnRgaCoreReqW<CruSoftrstCon6Spec> {
        ResetnRgaCoreReqW::new(self, 10)
    }
    #[doc = "Bit 12 - aresetn_emmc_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_emmc_noc_req(&mut self) -> AresetnEmmcNocReqW<CruSoftrstCon6Spec> {
        AresetnEmmcNocReqW::new(self, 12)
    }
    #[doc = "Bit 13 - aresetn_emmc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_emmc_req(&mut self) -> AresetnEmmcReqW<CruSoftrstCon6Spec> {
        AresetnEmmcReqW::new(self, 13)
    }
    #[doc = "Bit 14 - aresetn_emmc_grf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_emmc_grf_req(&mut self) -> AresetnEmmcGrfReqW<CruSoftrstCon6Spec> {
        AresetnEmmcGrfReqW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSoftrstCon6Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSoftrstCon6Spec;
impl crate::RegisterSpec for CruSoftrstCon6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_softrst_con6::R`](R) reader structure"]
impl crate::Readable for CruSoftrstCon6Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_softrst_con6::W`](W) writer structure"]
impl crate::Writable for CruSoftrstCon6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SOFTRST_CON6 to value 0"]
impl crate::Resettable for CruSoftrstCon6Spec {
    const RESET_VALUE: u32 = 0;
}
