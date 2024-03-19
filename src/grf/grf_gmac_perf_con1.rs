#[doc = "Register `GRF_GMAC_PERF_CON1` reader"]
pub type R = crate::R<GrfGmacPerfCon1Spec>;
#[doc = "Register `GRF_GMAC_PERF_CON1` writer"]
pub type W = crate::W<GrfGmacPerfCon1Spec>;
#[doc = "Field `GMAC_SW_RD_LATENCY_THR` reader - Axi read channel id for latency\n\nAXI_PERFormance test"]
pub type GmacSwRdLatencyThrR = crate::FieldReader<u16>;
#[doc = "Field `GMAC_SW_RD_LATENCY_THR` writer - Axi read channel id for latency\n\nAXI_PERFormance test"]
pub type GmacSwRdLatencyThrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11 - Axi read channel id for latency\n\nAXI_PERFormance test"]
    #[inline(always)]
    pub fn gmac_sw_rd_latency_thr(&self) -> GmacSwRdLatencyThrR {
        GmacSwRdLatencyThrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Axi read channel id for latency\n\nAXI_PERFormance test"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_sw_rd_latency_thr(&mut self) -> GmacSwRdLatencyThrW<GrfGmacPerfCon1Spec> {
        GmacSwRdLatencyThrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGmacPerfCon1Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "gmac performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gmac_perf_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gmac_perf_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGmacPerfCon1Spec;
impl crate::RegisterSpec for GrfGmacPerfCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gmac_perf_con1::R`](R) reader structure"]
impl crate::Readable for GrfGmacPerfCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_gmac_perf_con1::W`](W) writer structure"]
impl crate::Writable for GrfGmacPerfCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GMAC_PERF_CON1 to value 0"]
impl crate::Resettable for GrfGmacPerfCon1Spec {
    const RESET_VALUE: u32 = 0;
}
