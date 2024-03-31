#[doc = "Register `CQDPT` reader"]
pub type R = crate::R<CqdptSpec>;
#[doc = "Field `DPT` reader - Device Pending Tasks\n\nBit n of this register is set if and only if QUEUED_TASK_PARAMS\n\n(CMD44) and QUEUED_TASK_ADDRESS (CMD45) were sent for\n\nthis specific task and if this task hasn’t been executed yet.\n\nCQE shall set this bit after receiving a successful response for\n\nCMD45. CQE shall clear this bit after the task has completed\n\nexecution.\n\nSoftware needs to read this register in the task-discard\n\nprocedure, when the controlleris halted, to determine if the task\n\nis queued in the device. If the task is queued, the driver sends a\n\nCMDQ_TASK_MGMT (CMD48) to the device ordering it to discard\n\nthe task. Then software clears the task in the CQE. Only then the\n\nsoftware orders CQE to resume its operation using CQCTL\n\nregister."]
pub type DptR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Device Pending Tasks\n\nBit n of this register is set if and only if QUEUED_TASK_PARAMS\n\n(CMD44) and QUEUED_TASK_ADDRESS (CMD45) were sent for\n\nthis specific task and if this task hasn’t been executed yet.\n\nCQE shall set this bit after receiving a successful response for\n\nCMD45. CQE shall clear this bit after the task has completed\n\nexecution.\n\nSoftware needs to read this register in the task-discard\n\nprocedure, when the controlleris halted, to determine if the task\n\nis queued in the device. If the task is queued, the driver sends a\n\nCMDQ_TASK_MGMT (CMD48) to the device ordering it to discard\n\nthe task. Then software clears the task in the CQE. Only then the\n\nsoftware orders CQE to resume its operation using CQCTL\n\nregister."]
    #[inline(always)]
    pub fn dpt(&self) -> DptR {
        DptR::new(self.bits)
    }
}
#[doc = "Command queueing device pending tasks register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqdpt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqdptSpec;
impl crate::RegisterSpec for CqdptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqdpt::R`](R) reader structure"]
impl crate::Readable for CqdptSpec {}
#[doc = "`reset()` method sets CQDPT to value 0"]
impl crate::Resettable for CqdptSpec {
    const RESET_VALUE: u32 = 0;
}
