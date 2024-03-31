#[doc = "Register `CLKGATE_CON30` reader"]
pub type R = crate::R<ClkgateCon30Spec>;
#[doc = "Register `CLKGATE_CON30` writer"]
pub type W = crate::W<ClkgateCon30Spec>;
#[doc = "Field `ACLK_USB3_NOC_EN` reader - aclk_usb3_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkUsb3NocEnR = crate::BitReader;
#[doc = "Field `ACLK_USB3_NOC_EN` writer - aclk_usb3_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkUsb3NocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_USB3OTG0_EN` reader - aclk_usb3otg0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkUsb3otg0EnR = crate::BitReader;
#[doc = "Field `ACLK_USB3OTG0_EN` writer - aclk_usb3otg0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkUsb3otg0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_USB3OTG1_EN` reader - aclk_usb3otg1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkUsb3otg1EnR = crate::BitReader;
#[doc = "Field `ACLK_USB3OTG1_EN` writer - aclk_usb3otg1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkUsb3otg1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_USB3_RKSOC_AXI_PERF_EN` reader - aclk_usb3_rksoc_axi_perf clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkUsb3RksocAxiPerfEnR = crate::BitReader;
#[doc = "Field `ACLK_USB3_RKSOC_AXI_PERF_EN` writer - aclk_usb3_rksoc_axi_perf clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkUsb3RksocAxiPerfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_USB3_GRF_EN` reader - aclk_usb3_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkUsb3GrfEnR = crate::BitReader;
#[doc = "Field `ACLK_USB3_GRF_EN` writer - aclk_usb3_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkUsb3GrfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_GPU_EN` reader - aclk_gpu clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGpuEnR = crate::BitReader;
#[doc = "Field `ACLK_GPU_EN` writer - aclk_gpu clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGpuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_PERF_GPU_EN` reader - aclk_perf_gpu clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkPerfGpuEnR = crate::BitReader;
#[doc = "Field `ACLK_PERF_GPU_EN` writer - aclk_perf_gpu clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkPerfGpuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_GPU_GRF_EN` reader - aclk_gpu_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkGpuGrfEnR = crate::BitReader;
#[doc = "Field `ACLK_GPU_GRF_EN` writer - aclk_gpu_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkGpuGrfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_usb3_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn aclk_usb3_noc_en(&self) -> AclkUsb3NocEnR {
        AclkUsb3NocEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_usb3otg0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_usb3otg0_en(&self) -> AclkUsb3otg0EnR {
        AclkUsb3otg0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aclk_usb3otg1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_usb3otg1_en(&self) -> AclkUsb3otg1EnR {
        AclkUsb3otg1EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - aclk_usb3_rksoc_axi_perf clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_usb3_rksoc_axi_perf_en(&self) -> AclkUsb3RksocAxiPerfEnR {
        AclkUsb3RksocAxiPerfEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - aclk_usb3_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn aclk_usb3_grf_en(&self) -> AclkUsb3GrfEnR {
        AclkUsb3GrfEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - aclk_gpu clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_gpu_en(&self) -> AclkGpuEnR {
        AclkGpuEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - aclk_perf_gpu clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_perf_gpu_en(&self) -> AclkPerfGpuEnR {
        AclkPerfGpuEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - aclk_gpu_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn aclk_gpu_grf_en(&self) -> AclkGpuGrfEnR {
        AclkGpuGrfEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_usb3_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_usb3_noc_en(&mut self) -> AclkUsb3NocEnW<ClkgateCon30Spec> {
        AclkUsb3NocEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_usb3otg0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_usb3otg0_en(&mut self) -> AclkUsb3otg0EnW<ClkgateCon30Spec> {
        AclkUsb3otg0EnW::new(self, 1)
    }
    #[doc = "Bit 2 - aclk_usb3otg1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_usb3otg1_en(&mut self) -> AclkUsb3otg1EnW<ClkgateCon30Spec> {
        AclkUsb3otg1EnW::new(self, 2)
    }
    #[doc = "Bit 3 - aclk_usb3_rksoc_axi_perf clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_usb3_rksoc_axi_perf_en(&mut self) -> AclkUsb3RksocAxiPerfEnW<ClkgateCon30Spec> {
        AclkUsb3RksocAxiPerfEnW::new(self, 3)
    }
    #[doc = "Bit 4 - aclk_usb3_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_usb3_grf_en(&mut self) -> AclkUsb3GrfEnW<ClkgateCon30Spec> {
        AclkUsb3GrfEnW::new(self, 4)
    }
    #[doc = "Bit 8 - aclk_gpu clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gpu_en(&mut self) -> AclkGpuEnW<ClkgateCon30Spec> {
        AclkGpuEnW::new(self, 8)
    }
    #[doc = "Bit 10 - aclk_perf_gpu clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perf_gpu_en(&mut self) -> AclkPerfGpuEnW<ClkgateCon30Spec> {
        AclkPerfGpuEnW::new(self, 10)
    }
    #[doc = "Bit 11 - aclk_gpu_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gpu_grf_en(&mut self) -> AclkGpuGrfEnW<ClkgateCon30Spec> {
        AclkGpuGrfEnW::new(self, 11)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon30Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon30Spec;
impl crate::RegisterSpec for ClkgateCon30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con30::R`](R) reader structure"]
impl crate::Readable for ClkgateCon30Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con30::W`](W) writer structure"]
impl crate::Writable for ClkgateCon30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON30 to value 0"]
impl crate::Resettable for ClkgateCon30Spec {
    const RESET_VALUE: u32 = 0;
}
