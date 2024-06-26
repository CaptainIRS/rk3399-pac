#[doc = "Register `USB3_PERF_WORKING_CNT` reader"]
pub type R = crate::R<Usb3PerfWorkingCntSpec>;
#[doc = "Register `USB3_PERF_WORKING_CNT` writer"]
pub type W = crate::W<Usb3PerfWorkingCntSpec>;
#[doc = "Field `WORKING_CNT_R` reader - working counter"]
pub type WorkingCntRR = crate::FieldReader<u32>;
#[doc = "Field `WORKING_CNT_R` writer - working counter"]
pub type WorkingCntRW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - working counter"]
    #[inline(always)]
    pub fn working_cnt_r(&self) -> WorkingCntRR {
        WorkingCntRR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - working counter"]
    #[inline(always)]
    #[must_use]
    pub fn working_cnt_r(&mut self) -> WorkingCntRW<Usb3PerfWorkingCntSpec> {
        WorkingCntRW::new(self, 0)
    }
}
#[doc = "usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_working_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_working_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3PerfWorkingCntSpec;
impl crate::RegisterSpec for Usb3PerfWorkingCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_perf_working_cnt::R`](R) reader structure"]
impl crate::Readable for Usb3PerfWorkingCntSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_perf_working_cnt::W`](W) writer structure"]
impl crate::Writable for Usb3PerfWorkingCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_PERF_WORKING_CNT to value 0"]
impl crate::Resettable for Usb3PerfWorkingCntSpec {
    const RESET_VALUE: u32 = 0;
}
