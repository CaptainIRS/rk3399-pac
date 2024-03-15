#[doc = "Register `GRF_CPU_STATUS4` reader"]
pub type R = crate::R<GrfCpuStatus4Spec>;
#[doc = "Register `GRF_CPU_STATUS4` writer"]
pub type W = crate::W<GrfCpuStatus4Spec>;
#[doc = "Field `CCI_EVENT_BUS` reader - the status of cci_event_bus\\[93:64\\]"]
pub type CciEventBusR = crate::FieldReader<u32>;
#[doc = "Field `CCI_EVENT_BUS` writer - the status of cci_event_bus\\[93:64\\]"]
pub type CciEventBusW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - the status of cci_event_bus\\[93:64\\]"]
    #[inline(always)]
    pub fn cci_event_bus(&self) -> CciEventBusR {
        CciEventBusR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - the status of cci_event_bus\\[93:64\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cci_event_bus(&mut self) -> CciEventBusW<GrfCpuStatus4Spec> {
        CciEventBusW::new(self, 0)
    }
}
#[doc = "cpu status register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_status4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_status4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfCpuStatus4Spec;
impl crate::RegisterSpec for GrfCpuStatus4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_cpu_status4::R`](R) reader structure"]
impl crate::Readable for GrfCpuStatus4Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_cpu_status4::W`](W) writer structure"]
impl crate::Writable for GrfCpuStatus4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_CPU_STATUS4 to value 0"]
impl crate::Resettable for GrfCpuStatus4Spec {
    const RESET_VALUE: u32 = 0;
}
