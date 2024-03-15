#[doc = "Register `GRF_HDCP22_PERF_RD_LATENCY_ACC_NUM` reader"]
pub type R = crate::R<GrfHdcp22PerfRdLatencyAccNumSpec>;
#[doc = "Register `GRF_HDCP22_PERF_RD_LATENCY_ACC_NUM` writer"]
pub type W = crate::W<GrfHdcp22PerfRdLatencyAccNumSpec>;
#[doc = "Field `RD_LATENCY_ACC_CNT_R` reader - AXI read latency (>sw_rd_latency_thr) total number"]
pub type RdLatencyAccCntRR = crate::FieldReader<u32>;
#[doc = "Field `RD_LATENCY_ACC_CNT_R` writer - AXI read latency (>sw_rd_latency_thr) total number"]
pub type RdLatencyAccCntRW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AXI read latency (>sw_rd_latency_thr) total number"]
    #[inline(always)]
    pub fn rd_latency_acc_cnt_r(&self) -> RdLatencyAccCntRR {
        RdLatencyAccCntRR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AXI read latency (>sw_rd_latency_thr) total number"]
    #[inline(always)]
    #[must_use]
    pub fn rd_latency_acc_cnt_r(&mut self) -> RdLatencyAccCntRW<GrfHdcp22PerfRdLatencyAccNumSpec> {
        RdLatencyAccCntRW::new(self, 0)
    }
}
#[doc = "hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hdcp22_perf_rd_latency_acc_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hdcp22_perf_rd_latency_acc_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfHdcp22PerfRdLatencyAccNumSpec;
impl crate::RegisterSpec for GrfHdcp22PerfRdLatencyAccNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_hdcp22_perf_rd_latency_acc_num::R`](R) reader structure"]
impl crate::Readable for GrfHdcp22PerfRdLatencyAccNumSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_hdcp22_perf_rd_latency_acc_num::W`](W) writer structure"]
impl crate::Writable for GrfHdcp22PerfRdLatencyAccNumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_HDCP22_PERF_RD_LATENCY_ACC_NUM to value 0"]
impl crate::Resettable for GrfHdcp22PerfRdLatencyAccNumSpec {
    const RESET_VALUE: u32 = 0;
}
