#[doc = "Register `SOFTRST_CON4` reader"]
pub type R = crate::R<SoftrstCon4Spec>;
#[doc = "Register `SOFTRST_CON4` writer"]
pub type W = crate::W<SoftrstCon4Spec>;
#[doc = "Field `ARESETN_CENTER_MAIN_NOC_REQ` reader - aresetn_center_main_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCenterMainNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_CENTER_MAIN_NOC_REQ` writer - aresetn_center_main_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCenterMainNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_CENTER_PERI_NOC_REQ` reader - aresetn_center_peri_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCenterPeriNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_CENTER_PERI_NOC_REQ` writer - aresetn_center_peri_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCenterPeriNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_CENTER_MAIN_REQ` reader - presetn_center_main request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnCenterMainReqR = crate::BitReader;
#[doc = "Field `PRESETN_CENTER_MAIN_REQ` writer - presetn_center_main request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnCenterMainReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_DDRMON_REQ` reader - presetn_ddrmon request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnDdrmonReqR = crate::BitReader;
#[doc = "Field `PRESETN_DDRMON_REQ` writer - presetn_ddrmon request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnDdrmonReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_CIC_REQ` reader - presetn_cic request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnCicReqR = crate::BitReader;
#[doc = "Field `PRESETN_CIC_REQ` writer - presetn_cic request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnCicReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_CENTER_SGRF_REQ` reader - presetn_center_sgrf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnCenterSgrfReqR = crate::BitReader;
#[doc = "Field `PRESETN_CENTER_SGRF_REQ` writer - presetn_center_sgrf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnCenterSgrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DDR0_MSCH_REQ` reader - resetn_ddr0_msch request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdr0MschReqR = crate::BitReader;
#[doc = "Field `RESETN_DDR0_MSCH_REQ` writer - resetn_ddr0_msch request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdr0MschReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DDRCFG0_MSCH_REQ` reader - resetn_ddrcfg0_msch request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdrcfg0MschReqR = crate::BitReader;
#[doc = "Field `RESETN_DDRCFG0_MSCH_REQ` writer - resetn_ddrcfg0_msch request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdrcfg0MschReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DDR0_REQ` reader - resetn_ddr0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdr0ReqR = crate::BitReader;
#[doc = "Field `RESETN_DDR0_REQ` writer - resetn_ddr0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdr0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DDRPHY0_REQ` reader - resetn_ddrphy0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdrphy0ReqR = crate::BitReader;
#[doc = "Field `RESETN_DDRPHY0_REQ` writer - resetn_ddrphy0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdrphy0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DDR1_MSCH_REQ` reader - resetn_ddr1_msch request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdr1MschReqR = crate::BitReader;
#[doc = "Field `RESETN_DDR1_MSCH_REQ` writer - resetn_ddr1_msch request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdr1MschReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DDRCFG1_MSCH_REQ` reader - resetn_ddrcfg1_msch request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdrcfg1MschReqR = crate::BitReader;
#[doc = "Field `RESETN_DDRCFG1_MSCH_REQ` writer - resetn_ddrcfg1_msch request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdrcfg1MschReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DDR1_REQ` reader - resetn_ddr1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdr1ReqR = crate::BitReader;
#[doc = "Field `RESETN_DDR1_REQ` writer - resetn_ddr1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdr1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DDRPHY1_REQ` reader - resetn_ddrphy1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdrphy1ReqR = crate::BitReader;
#[doc = "Field `RESETN_DDRPHY1_REQ` writer - resetn_ddrphy1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdrphy1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DDR_CIC_REQ` reader - resetn_ddr_cic request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdrCicReqR = crate::BitReader;
#[doc = "Field `RESETN_DDR_CIC_REQ` writer - resetn_ddr_cic request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDdrCicReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_PVTM_DDR_REQ` reader - resetn_pvtm_ddr request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPvtmDdrReqR = crate::BitReader;
#[doc = "Field `RESETN_PVTM_DDR_REQ` writer - resetn_pvtm_ddr request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnPvtmDdrReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aresetn_center_main_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_center_main_noc_req(&self) -> AresetnCenterMainNocReqR {
        AresetnCenterMainNocReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aresetn_center_peri_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_center_peri_noc_req(&self) -> AresetnCenterPeriNocReqR {
        AresetnCenterPeriNocReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - presetn_center_main request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_center_main_req(&self) -> PresetnCenterMainReqR {
        PresetnCenterMainReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - presetn_ddrmon request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_ddrmon_req(&self) -> PresetnDdrmonReqR {
        PresetnDdrmonReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - presetn_cic request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_cic_req(&self) -> PresetnCicReqR {
        PresetnCicReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - presetn_center_sgrf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_center_sgrf_req(&self) -> PresetnCenterSgrfReqR {
        PresetnCenterSgrfReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - resetn_ddr0_msch request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_ddr0_msch_req(&self) -> ResetnDdr0MschReqR {
        ResetnDdr0MschReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - resetn_ddrcfg0_msch request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_ddrcfg0_msch_req(&self) -> ResetnDdrcfg0MschReqR {
        ResetnDdrcfg0MschReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - resetn_ddr0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_ddr0_req(&self) -> ResetnDdr0ReqR {
        ResetnDdr0ReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - resetn_ddrphy0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_ddrphy0_req(&self) -> ResetnDdrphy0ReqR {
        ResetnDdrphy0ReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - resetn_ddr1_msch request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_ddr1_msch_req(&self) -> ResetnDdr1MschReqR {
        ResetnDdr1MschReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - resetn_ddrcfg1_msch request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_ddrcfg1_msch_req(&self) -> ResetnDdrcfg1MschReqR {
        ResetnDdrcfg1MschReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - resetn_ddr1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_ddr1_req(&self) -> ResetnDdr1ReqR {
        ResetnDdr1ReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - resetn_ddrphy1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_ddrphy1_req(&self) -> ResetnDdrphy1ReqR {
        ResetnDdrphy1ReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - resetn_ddr_cic request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_ddr_cic_req(&self) -> ResetnDdrCicReqR {
        ResetnDdrCicReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - resetn_pvtm_ddr request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_pvtm_ddr_req(&self) -> ResetnPvtmDdrReqR {
        ResetnPvtmDdrReqR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aresetn_center_main_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_center_main_noc_req(&mut self) -> AresetnCenterMainNocReqW<SoftrstCon4Spec> {
        AresetnCenterMainNocReqW::new(self, 0)
    }
    #[doc = "Bit 1 - aresetn_center_peri_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_center_peri_noc_req(&mut self) -> AresetnCenterPeriNocReqW<SoftrstCon4Spec> {
        AresetnCenterPeriNocReqW::new(self, 1)
    }
    #[doc = "Bit 2 - presetn_center_main request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_center_main_req(&mut self) -> PresetnCenterMainReqW<SoftrstCon4Spec> {
        PresetnCenterMainReqW::new(self, 2)
    }
    #[doc = "Bit 3 - presetn_ddrmon request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_ddrmon_req(&mut self) -> PresetnDdrmonReqW<SoftrstCon4Spec> {
        PresetnDdrmonReqW::new(self, 3)
    }
    #[doc = "Bit 4 - presetn_cic request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_cic_req(&mut self) -> PresetnCicReqW<SoftrstCon4Spec> {
        PresetnCicReqW::new(self, 4)
    }
    #[doc = "Bit 5 - presetn_center_sgrf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_center_sgrf_req(&mut self) -> PresetnCenterSgrfReqW<SoftrstCon4Spec> {
        PresetnCenterSgrfReqW::new(self, 5)
    }
    #[doc = "Bit 6 - resetn_ddr0_msch request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_ddr0_msch_req(&mut self) -> ResetnDdr0MschReqW<SoftrstCon4Spec> {
        ResetnDdr0MschReqW::new(self, 6)
    }
    #[doc = "Bit 7 - resetn_ddrcfg0_msch request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_ddrcfg0_msch_req(&mut self) -> ResetnDdrcfg0MschReqW<SoftrstCon4Spec> {
        ResetnDdrcfg0MschReqW::new(self, 7)
    }
    #[doc = "Bit 8 - resetn_ddr0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_ddr0_req(&mut self) -> ResetnDdr0ReqW<SoftrstCon4Spec> {
        ResetnDdr0ReqW::new(self, 8)
    }
    #[doc = "Bit 9 - resetn_ddrphy0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_ddrphy0_req(&mut self) -> ResetnDdrphy0ReqW<SoftrstCon4Spec> {
        ResetnDdrphy0ReqW::new(self, 9)
    }
    #[doc = "Bit 10 - resetn_ddr1_msch request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_ddr1_msch_req(&mut self) -> ResetnDdr1MschReqW<SoftrstCon4Spec> {
        ResetnDdr1MschReqW::new(self, 10)
    }
    #[doc = "Bit 11 - resetn_ddrcfg1_msch request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_ddrcfg1_msch_req(&mut self) -> ResetnDdrcfg1MschReqW<SoftrstCon4Spec> {
        ResetnDdrcfg1MschReqW::new(self, 11)
    }
    #[doc = "Bit 12 - resetn_ddr1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_ddr1_req(&mut self) -> ResetnDdr1ReqW<SoftrstCon4Spec> {
        ResetnDdr1ReqW::new(self, 12)
    }
    #[doc = "Bit 13 - resetn_ddrphy1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_ddrphy1_req(&mut self) -> ResetnDdrphy1ReqW<SoftrstCon4Spec> {
        ResetnDdrphy1ReqW::new(self, 13)
    }
    #[doc = "Bit 14 - resetn_ddr_cic request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_ddr_cic_req(&mut self) -> ResetnDdrCicReqW<SoftrstCon4Spec> {
        ResetnDdrCicReqW::new(self, 14)
    }
    #[doc = "Bit 15 - resetn_pvtm_ddr request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_pvtm_ddr_req(&mut self) -> ResetnPvtmDdrReqW<SoftrstCon4Spec> {
        ResetnPvtmDdrReqW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<SoftrstCon4Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftrstCon4Spec;
impl crate::RegisterSpec for SoftrstCon4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softrst_con4::R`](R) reader structure"]
impl crate::Readable for SoftrstCon4Spec {}
#[doc = "`write(|w| ..)` method takes [`softrst_con4::W`](W) writer structure"]
impl crate::Writable for SoftrstCon4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTRST_CON4 to value 0"]
impl crate::Resettable for SoftrstCon4Spec {
    const RESET_VALUE: u32 = 0;
}
