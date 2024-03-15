#[doc = "Register `CRU_SOFTRST_CON18` reader"]
pub type R = crate::R<CruSoftrstCon18Spec>;
#[doc = "Register `CRU_SOFTRST_CON18` writer"]
pub type W = crate::W<CruSoftrstCon18Spec>;
#[doc = "Field `ARESETN_GPU_REQ` reader - aresetn_gpu request bit When HIGH, reset relative logic"]
pub type AresetnGpuReqR = crate::BitReader;
#[doc = "Field `ARESETN_GPU_REQ` writer - aresetn_gpu request bit When HIGH, reset relative logic"]
pub type AresetnGpuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_GPU_NOC_REQ` reader - aresetn_gpu_noc request bit When HIGH, reset relative logic"]
pub type AresetnGpuNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_GPU_NOC_REQ` writer - aresetn_gpu_noc request bit When HIGH, reset relative logic"]
pub type AresetnGpuNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_GPU_GRF_REQ` reader - aresetn_gpu_grf request bit When HIGH, reset relative logic"]
pub type AresetnGpuGrfReqR = crate::BitReader;
#[doc = "Field `ARESETN_GPU_GRF_REQ` writer - aresetn_gpu_grf request bit When HIGH, reset relative logic"]
pub type AresetnGpuGrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_PVTM_GPU_REQ` reader - resetn_pvtm_gpu request bit When HIGH, reset relative logic"]
pub type ResetnPvtmGpuReqR = crate::BitReader;
#[doc = "Field `RESETN_PVTM_GPU_REQ` writer - resetn_pvtm_gpu request bit When HIGH, reset relative logic"]
pub type ResetnPvtmGpuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_USB3_NOC_REQ` reader - aresetn_usb3_noc request bit When HIGH, reset relative logic"]
pub type AresetnUsb3NocReqR = crate::BitReader;
#[doc = "Field `ARESETN_USB3_NOC_REQ` writer - aresetn_usb3_noc request bit When HIGH, reset relative logic"]
pub type AresetnUsb3NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_USB3_OTG0_REQ` reader - aresetn_usb3_otg0 request bit When HIGH, reset relative logic"]
pub type AresetnUsb3Otg0ReqR = crate::BitReader;
#[doc = "Field `ARESETN_USB3_OTG0_REQ` writer - aresetn_usb3_otg0 request bit When HIGH, reset relative logic"]
pub type AresetnUsb3Otg0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_USB3_OTG1_REQ` reader - aresetn_usb3_otg1 request bit When HIGH, reset relative logic"]
pub type AresetnUsb3Otg1ReqR = crate::BitReader;
#[doc = "Field `ARESETN_USB3_OTG1_REQ` writer - aresetn_usb3_otg1 request bit When HIGH, reset relative logic"]
pub type AresetnUsb3Otg1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_USB3_GRF_REQ` reader - aresetn_usb3_grf request bit When HIGH, reset relative logic"]
pub type AresetnUsb3GrfReqR = crate::BitReader;
#[doc = "Field `ARESETN_USB3_GRF_REQ` writer - aresetn_usb3_grf request bit When HIGH, reset relative logic"]
pub type AresetnUsb3GrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_SRSTN_REQ` reader - pmu_srstn request bit When HIGH, reset relative logic"]
pub type PmuSrstnReqR = crate::BitReader;
#[doc = "Field `PMU_SRSTN_REQ` writer - pmu_srstn request bit When HIGH, reset relative logic"]
pub type PmuSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aresetn_gpu request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_gpu_req(&self) -> AresetnGpuReqR {
        AresetnGpuReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aresetn_gpu_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_gpu_noc_req(&self) -> AresetnGpuNocReqR {
        AresetnGpuNocReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aresetn_gpu_grf request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_gpu_grf_req(&self) -> AresetnGpuGrfReqR {
        AresetnGpuGrfReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - resetn_pvtm_gpu request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_pvtm_gpu_req(&self) -> ResetnPvtmGpuReqR {
        ResetnPvtmGpuReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - aresetn_usb3_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_usb3_noc_req(&self) -> AresetnUsb3NocReqR {
        AresetnUsb3NocReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - aresetn_usb3_otg0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_usb3_otg0_req(&self) -> AresetnUsb3Otg0ReqR {
        AresetnUsb3Otg0ReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - aresetn_usb3_otg1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_usb3_otg1_req(&self) -> AresetnUsb3Otg1ReqR {
        AresetnUsb3Otg1ReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - aresetn_usb3_grf request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_usb3_grf_req(&self) -> AresetnUsb3GrfReqR {
        AresetnUsb3GrfReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pmu_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn pmu_srstn_req(&self) -> PmuSrstnReqR {
        PmuSrstnReqR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aresetn_gpu request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_gpu_req(&mut self) -> AresetnGpuReqW<CruSoftrstCon18Spec> {
        AresetnGpuReqW::new(self, 0)
    }
    #[doc = "Bit 1 - aresetn_gpu_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_gpu_noc_req(&mut self) -> AresetnGpuNocReqW<CruSoftrstCon18Spec> {
        AresetnGpuNocReqW::new(self, 1)
    }
    #[doc = "Bit 2 - aresetn_gpu_grf request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_gpu_grf_req(&mut self) -> AresetnGpuGrfReqW<CruSoftrstCon18Spec> {
        AresetnGpuGrfReqW::new(self, 2)
    }
    #[doc = "Bit 3 - resetn_pvtm_gpu request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_pvtm_gpu_req(&mut self) -> ResetnPvtmGpuReqW<CruSoftrstCon18Spec> {
        ResetnPvtmGpuReqW::new(self, 3)
    }
    #[doc = "Bit 4 - aresetn_usb3_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_usb3_noc_req(&mut self) -> AresetnUsb3NocReqW<CruSoftrstCon18Spec> {
        AresetnUsb3NocReqW::new(self, 4)
    }
    #[doc = "Bit 5 - aresetn_usb3_otg0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_usb3_otg0_req(&mut self) -> AresetnUsb3Otg0ReqW<CruSoftrstCon18Spec> {
        AresetnUsb3Otg0ReqW::new(self, 5)
    }
    #[doc = "Bit 6 - aresetn_usb3_otg1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_usb3_otg1_req(&mut self) -> AresetnUsb3Otg1ReqW<CruSoftrstCon18Spec> {
        AresetnUsb3Otg1ReqW::new(self, 6)
    }
    #[doc = "Bit 7 - aresetn_usb3_grf request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_usb3_grf_req(&mut self) -> AresetnUsb3GrfReqW<CruSoftrstCon18Spec> {
        AresetnUsb3GrfReqW::new(self, 7)
    }
    #[doc = "Bit 8 - pmu_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_srstn_req(&mut self) -> PmuSrstnReqW<CruSoftrstCon18Spec> {
        PmuSrstnReqW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSoftrstCon18Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSoftrstCon18Spec;
impl crate::RegisterSpec for CruSoftrstCon18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_softrst_con18::R`](R) reader structure"]
impl crate::Readable for CruSoftrstCon18Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_softrst_con18::W`](W) writer structure"]
impl crate::Writable for CruSoftrstCon18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SOFTRST_CON18 to value 0"]
impl crate::Resettable for CruSoftrstCon18Spec {
    const RESET_VALUE: u32 = 0;
}
