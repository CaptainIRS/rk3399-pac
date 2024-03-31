#[doc = "Register `SOFTRST_CON3` reader"]
pub type R = crate::R<SoftrstCon3Spec>;
#[doc = "Register `SOFTRST_CON3` writer"]
pub type W = crate::W<SoftrstCon3Spec>;
#[doc = "Field `ARESETN_CCI_REQ_T` reader - aresetn_cci request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCciReqTR = crate::BitReader;
#[doc = "Field `ARESETN_CCI_REQ_T` writer - aresetn_cci request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCciReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_CCIM0_NOC_REQ_T` reader - aresetn_ccim0_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCcim0NocReqTR = crate::BitReader;
#[doc = "Field `ARESETN_CCIM0_NOC_REQ_T` writer - aresetn_ccim0_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCcim0NocReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_CCIM1_NOC_REQ_T` reader - aresetn_ccim1_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCcim1NocReqTR = crate::BitReader;
#[doc = "Field `ARESETN_ADB400M_PD_CORE_B_REQ_T` reader - aresetn_adb400m_pd_core_b request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnAdb400mPdCoreBReqTR = crate::BitReader;
#[doc = "Field `ARESETN_ADB400M_PD_CORE_B_REQ_T` writer - aresetn_adb400m_pd_core_b request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnAdb400mPdCoreBReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_ADB400M_PD_CORE_L_REQ_T` reader - aresetn_adb400m_pd_core_l request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnAdb400mPdCoreLReqTR = crate::BitReader;
#[doc = "Field `ARESETN_ADB400M_PD_CORE_L_REQ_T` writer - aresetn_adb400m_pd_core_l request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnAdb400mPdCoreLReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DBG_NOC_REQ_T` reader - resetn_dbg_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDbgNocReqTR = crate::BitReader;
#[doc = "Field `RESETN_DBG_NOC_REQ_T` writer - resetn_dbg_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDbgNocReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DBG_CXCS_REQ` reader - resetn_dbg_cxcs request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDbgCxcsReqR = crate::BitReader;
#[doc = "Field `RESETN_DBG_CXCS_REQ` writer - resetn_dbg_cxcs request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDbgCxcsReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_CCI_TRACE_REQ` reader - resetn_cci_trace request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnCciTraceReqR = crate::BitReader;
#[doc = "Field `RESETN_CCI_TRACE_REQ` writer - resetn_cci_trace request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnCciTraceReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_CCI_GRF_REQ` reader - presetn_cci_grf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnCciGrfReqR = crate::BitReader;
#[doc = "Field `PRESETN_CCI_GRF_REQ` writer - presetn_cci_grf request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnCciGrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 2 - aresetn_cci request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_cci_req_t(&self) -> AresetnCciReqTR {
        AresetnCciReqTR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - aresetn_ccim0_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_ccim0_noc_req_t(&self) -> AresetnCcim0NocReqTR {
        AresetnCcim0NocReqTR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - aresetn_ccim1_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_ccim1_noc_req_t(&self) -> AresetnCcim1NocReqTR {
        AresetnCcim1NocReqTR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - aresetn_adb400m_pd_core_b request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_adb400m_pd_core_b_req_t(&self) -> AresetnAdb400mPdCoreBReqTR {
        AresetnAdb400mPdCoreBReqTR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - aresetn_adb400m_pd_core_l request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_adb400m_pd_core_l_req_t(&self) -> AresetnAdb400mPdCoreLReqTR {
        AresetnAdb400mPdCoreLReqTR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - resetn_dbg_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_dbg_noc_req_t(&self) -> ResetnDbgNocReqTR {
        ResetnDbgNocReqTR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - resetn_dbg_cxcs request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_dbg_cxcs_req(&self) -> ResetnDbgCxcsReqR {
        ResetnDbgCxcsReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - resetn_cci_trace request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_cci_trace_req(&self) -> ResetnCciTraceReqR {
        ResetnCciTraceReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - presetn_cci_grf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_cci_grf_req(&self) -> PresetnCciGrfReqR {
        PresetnCciGrfReqR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - aresetn_cci request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_cci_req_t(&mut self) -> AresetnCciReqTW<SoftrstCon3Spec> {
        AresetnCciReqTW::new(self, 2)
    }
    #[doc = "Bit 3 - aresetn_ccim0_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_ccim0_noc_req_t(&mut self) -> AresetnCcim0NocReqTW<SoftrstCon3Spec> {
        AresetnCcim0NocReqTW::new(self, 3)
    }
    #[doc = "Bit 5 - aresetn_adb400m_pd_core_b request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_adb400m_pd_core_b_req_t(
        &mut self,
    ) -> AresetnAdb400mPdCoreBReqTW<SoftrstCon3Spec> {
        AresetnAdb400mPdCoreBReqTW::new(self, 5)
    }
    #[doc = "Bit 6 - aresetn_adb400m_pd_core_l request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_adb400m_pd_core_l_req_t(
        &mut self,
    ) -> AresetnAdb400mPdCoreLReqTW<SoftrstCon3Spec> {
        AresetnAdb400mPdCoreLReqTW::new(self, 6)
    }
    #[doc = "Bit 7 - resetn_dbg_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_dbg_noc_req_t(&mut self) -> ResetnDbgNocReqTW<SoftrstCon3Spec> {
        ResetnDbgNocReqTW::new(self, 7)
    }
    #[doc = "Bit 8 - resetn_dbg_cxcs request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_dbg_cxcs_req(&mut self) -> ResetnDbgCxcsReqW<SoftrstCon3Spec> {
        ResetnDbgCxcsReqW::new(self, 8)
    }
    #[doc = "Bit 9 - resetn_cci_trace request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_cci_trace_req(&mut self) -> ResetnCciTraceReqW<SoftrstCon3Spec> {
        ResetnCciTraceReqW::new(self, 9)
    }
    #[doc = "Bit 10 - presetn_cci_grf request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_cci_grf_req(&mut self) -> PresetnCciGrfReqW<SoftrstCon3Spec> {
        PresetnCciGrfReqW::new(self, 10)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<SoftrstCon3Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftrstCon3Spec;
impl crate::RegisterSpec for SoftrstCon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softrst_con3::R`](R) reader structure"]
impl crate::Readable for SoftrstCon3Spec {}
#[doc = "`write(|w| ..)` method takes [`softrst_con3::W`](W) writer structure"]
impl crate::Writable for SoftrstCon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTRST_CON3 to value 0x10"]
impl crate::Resettable for SoftrstCon3Spec {
    const RESET_VALUE: u32 = 0x10;
}
