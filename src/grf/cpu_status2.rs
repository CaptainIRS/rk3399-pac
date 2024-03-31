#[doc = "Register `CPU_STATUS2` reader"]
pub type R = crate::R<CpuStatus2Spec>;
#[doc = "Register `CPU_STATUS2` writer"]
pub type W = crate::W<CpuStatus2Spec>;
#[doc = "Field `CCI_EVENT_BUS` reader - the status of cci_event_bus\\[31:0\\]"]
pub type CciEventBusR = crate::FieldReader<u32>;
#[doc = "Field `CCI_EVENT_BUS` writer - the status of cci_event_bus\\[31:0\\]"]
pub type CciEventBusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the status of cci_event_bus\\[31:0\\]"]
    #[inline(always)]
    pub fn cci_event_bus(&self) -> CciEventBusR {
        CciEventBusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the status of cci_event_bus\\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cci_event_bus(&mut self) -> CciEventBusW<CpuStatus2Spec> {
        CciEventBusW::new(self, 0)
    }
}
#[doc = "cpu status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_status2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_status2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuStatus2Spec;
impl crate::RegisterSpec for CpuStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_status2::R`](R) reader structure"]
impl crate::Readable for CpuStatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_status2::W`](W) writer structure"]
impl crate::Writable for CpuStatus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_STATUS2 to value 0"]
impl crate::Resettable for CpuStatus2Spec {
    const RESET_VALUE: u32 = 0;
}
