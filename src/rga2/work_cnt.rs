#[doc = "Register `WORK_CNT` reader"]
pub type R = crate::R<WorkCntSpec>;
#[doc = "Field `SW_WORK_CNT` reader - working counter register\n\nRGA total working counter"]
pub type SwWorkCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:26 - working counter register\n\nRGA total working counter"]
    #[inline(always)]
    pub fn sw_work_cnt(&self) -> SwWorkCntR {
        SwWorkCntR::new(self.bits & 0x07ff_ffff)
    }
}
#[doc = "work counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`work_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WorkCntSpec;
impl crate::RegisterSpec for WorkCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`work_cnt::R`](R) reader structure"]
impl crate::Readable for WorkCntSpec {}
#[doc = "`reset()` method sets WORK_CNT to value 0"]
impl crate::Resettable for WorkCntSpec {
    const RESET_VALUE: u32 = 0;
}
