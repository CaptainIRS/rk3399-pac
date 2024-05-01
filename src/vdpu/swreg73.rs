#[doc = "Register `SWREG73` reader"]
pub type R = crate::R<Swreg73Spec>;
#[doc = "Field `DEBUG_MB_CNT` reader - debug_mb_cnt"]
pub type DebugMbCntR = crate::FieldReader<u32>;
#[doc = "Field `DEBUG_REF1_REQ` reader - debug_ref1_req"]
pub type DebugRef1ReqR = crate::BitReader;
#[doc = "Field `DEBUG_REF0_REQ` reader - debug_ref0_req"]
pub type DebugRef0ReqR = crate::BitReader;
#[doc = "Field `DEBUG_FLT_REQ` reader - debug_flt_req"]
pub type DebugFltReqR = crate::BitReader;
#[doc = "Field `DEBUG_FRM_RDY` reader - debug_frm_rdy"]
pub type DebugFrmRdyR = crate::BitReader;
#[doc = "Field `DEBUG_STRM_DA_E` reader - debug_strm_da_e"]
pub type DebugStrmDaER = crate::BitReader;
#[doc = "Field `DEBUG_RES_C_REQ` reader - debug_res_c_req"]
pub type DebugResCReqR = crate::BitReader;
#[doc = "Field `DEBUG_RES_Y_REQ` reader - prtr_res_y_req signal value"]
pub type DebugResYReqR = crate::BitReader;
#[doc = "Field `DEBUG_RLC_REQ` reader - prtr_res_y_req signal value"]
pub type DebugRlcReqR = crate::BitReader;
#[doc = "Field `DEBUG_MV_REQ` reader - mvst_mv_req signal value"]
pub type DebugMvReqR = crate::BitReader;
impl R {
    #[doc = "Bits 0:20 - debug_mb_cnt"]
    #[inline(always)]
    pub fn debug_mb_cnt(&self) -> DebugMbCntR {
        DebugMbCntR::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bit 22 - debug_ref1_req"]
    #[inline(always)]
    pub fn debug_ref1_req(&self) -> DebugRef1ReqR {
        DebugRef1ReqR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - debug_ref0_req"]
    #[inline(always)]
    pub fn debug_ref0_req(&self) -> DebugRef0ReqR {
        DebugRef0ReqR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - debug_flt_req"]
    #[inline(always)]
    pub fn debug_flt_req(&self) -> DebugFltReqR {
        DebugFltReqR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - debug_frm_rdy"]
    #[inline(always)]
    pub fn debug_frm_rdy(&self) -> DebugFrmRdyR {
        DebugFrmRdyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - debug_strm_da_e"]
    #[inline(always)]
    pub fn debug_strm_da_e(&self) -> DebugStrmDaER {
        DebugStrmDaER::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - debug_res_c_req"]
    #[inline(always)]
    pub fn debug_res_c_req(&self) -> DebugResCReqR {
        DebugResCReqR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - prtr_res_y_req signal value"]
    #[inline(always)]
    pub fn debug_res_y_req(&self) -> DebugResYReqR {
        DebugResYReqR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - prtr_res_y_req signal value"]
    #[inline(always)]
    pub fn debug_rlc_req(&self) -> DebugRlcReqR {
        DebugRlcReqR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - mvst_mv_req signal value"]
    #[inline(always)]
    pub fn debug_mv_req(&self) -> DebugMvReqR {
        DebugMvReqR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg73::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg73Spec;
impl crate::RegisterSpec for Swreg73Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg73::R`](R) reader structure"]
impl crate::Readable for Swreg73Spec {}
#[doc = "`reset()` method sets SWREG73 to value 0"]
impl crate::Resettable for Swreg73Spec {
    const RESET_VALUE: u32 = 0;
}
