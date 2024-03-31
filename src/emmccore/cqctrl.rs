#[doc = "Register `CQCTRL` reader"]
pub type R = crate::R<CqctrlSpec>;
#[doc = "Register `CQCTRL` writer"]
pub type W = crate::W<CqctrlSpec>;
#[doc = "Field `HALT` reader - Host software shall write 1 to the bit when it wants to acquire\n\nsoftware control over the eMMC bus and disable CQE from issuing\n\ncommands on the bus.\n\nFor example, issuing a Discard Task command\n\n(CMDQ_TASK_MGMT).\n\nWhen software writes 1, CQE shall complete the ongoing task if\n\nsuch a task is in progress.\n\nOnce the task is completed and CQE is in idle state, CQE shall not\n\nissue new commands and shall indicate so to software by setting\n\nthis bit to 1.\n\nSoftware may poll on this bit until it is set to 1, and may only\n\nthen send commands on the eMMC bus.\n\nIn order to exit halt state (i.e. resume CQE activity), software\n\nshall clear this bit (write 0). Writing 0 when the value is already 0\n\nshall have no effect."]
pub type HaltR = crate::BitReader;
#[doc = "Field `HALT` writer - Host software shall write 1 to the bit when it wants to acquire\n\nsoftware control over the eMMC bus and disable CQE from issuing\n\ncommands on the bus.\n\nFor example, issuing a Discard Task command\n\n(CMDQ_TASK_MGMT).\n\nWhen software writes 1, CQE shall complete the ongoing task if\n\nsuch a task is in progress.\n\nOnce the task is completed and CQE is in idle state, CQE shall not\n\nissue new commands and shall indicate so to software by setting\n\nthis bit to 1.\n\nSoftware may poll on this bit until it is set to 1, and may only\n\nthen send commands on the eMMC bus.\n\nIn order to exit halt state (i.e. resume CQE activity), software\n\nshall clear this bit (write 0). Writing 0 when the value is already 0\n\nshall have no effect."]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARALLTASKS` reader - Software shall write 1 this bit when it wants to clear all the tasks\n\nsent to the device.\n\nThis bit can only be written when CQE is in halt state (i.e. Halt bit\n\nis 1).\n\nWhen software writes 1, the value of the register is updated to 1,\n\nand CQE shall reset CQTDBR register and all other context\n\ninformation for all unfinished tasks.\n\nThen CQE will clear this bit.\n\nSoftware should poll on this bit until it is set to back 0 and may\n\nthen resume normal operation, by clearing the Halt bit.\n\nCQE does not communicate to the device that the tasks were\n\ncleared. It is software's responsibility to order the device to\n\ndiscard the tasks in its queue using CMDQ_TASK_MGMT\n\ncommand.\n\nWriting 0 to this register shall have no effect."]
pub type ClearalltasksR = crate::BitReader;
#[doc = "Field `CLEARALLTASKS` writer - Software shall write 1 this bit when it wants to clear all the tasks\n\nsent to the device.\n\nThis bit can only be written when CQE is in halt state (i.e. Halt bit\n\nis 1).\n\nWhen software writes 1, the value of the register is updated to 1,\n\nand CQE shall reset CQTDBR register and all other context\n\ninformation for all unfinished tasks.\n\nThen CQE will clear this bit.\n\nSoftware should poll on this bit until it is set to back 0 and may\n\nthen resume normal operation, by clearing the Halt bit.\n\nCQE does not communicate to the device that the tasks were\n\ncleared. It is software's responsibility to order the device to\n\ndiscard the tasks in its queue using CMDQ_TASK_MGMT\n\ncommand.\n\nWriting 0 to this register shall have no effect."]
pub type ClearalltasksW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host software shall write 1 to the bit when it wants to acquire\n\nsoftware control over the eMMC bus and disable CQE from issuing\n\ncommands on the bus.\n\nFor example, issuing a Discard Task command\n\n(CMDQ_TASK_MGMT).\n\nWhen software writes 1, CQE shall complete the ongoing task if\n\nsuch a task is in progress.\n\nOnce the task is completed and CQE is in idle state, CQE shall not\n\nissue new commands and shall indicate so to software by setting\n\nthis bit to 1.\n\nSoftware may poll on this bit until it is set to 1, and may only\n\nthen send commands on the eMMC bus.\n\nIn order to exit halt state (i.e. resume CQE activity), software\n\nshall clear this bit (write 0). Writing 0 when the value is already 0\n\nshall have no effect."]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Software shall write 1 this bit when it wants to clear all the tasks\n\nsent to the device.\n\nThis bit can only be written when CQE is in halt state (i.e. Halt bit\n\nis 1).\n\nWhen software writes 1, the value of the register is updated to 1,\n\nand CQE shall reset CQTDBR register and all other context\n\ninformation for all unfinished tasks.\n\nThen CQE will clear this bit.\n\nSoftware should poll on this bit until it is set to back 0 and may\n\nthen resume normal operation, by clearing the Halt bit.\n\nCQE does not communicate to the device that the tasks were\n\ncleared. It is software's responsibility to order the device to\n\ndiscard the tasks in its queue using CMDQ_TASK_MGMT\n\ncommand.\n\nWriting 0 to this register shall have no effect."]
    #[inline(always)]
    pub fn clearalltasks(&self) -> ClearalltasksR {
        ClearalltasksR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host software shall write 1 to the bit when it wants to acquire\n\nsoftware control over the eMMC bus and disable CQE from issuing\n\ncommands on the bus.\n\nFor example, issuing a Discard Task command\n\n(CMDQ_TASK_MGMT).\n\nWhen software writes 1, CQE shall complete the ongoing task if\n\nsuch a task is in progress.\n\nOnce the task is completed and CQE is in idle state, CQE shall not\n\nissue new commands and shall indicate so to software by setting\n\nthis bit to 1.\n\nSoftware may poll on this bit until it is set to 1, and may only\n\nthen send commands on the eMMC bus.\n\nIn order to exit halt state (i.e. resume CQE activity), software\n\nshall clear this bit (write 0). Writing 0 when the value is already 0\n\nshall have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<CqctrlSpec> {
        HaltW::new(self, 0)
    }
    #[doc = "Bit 8 - Software shall write 1 this bit when it wants to clear all the tasks\n\nsent to the device.\n\nThis bit can only be written when CQE is in halt state (i.e. Halt bit\n\nis 1).\n\nWhen software writes 1, the value of the register is updated to 1,\n\nand CQE shall reset CQTDBR register and all other context\n\ninformation for all unfinished tasks.\n\nThen CQE will clear this bit.\n\nSoftware should poll on this bit until it is set to back 0 and may\n\nthen resume normal operation, by clearing the Halt bit.\n\nCQE does not communicate to the device that the tasks were\n\ncleared. It is software's responsibility to order the device to\n\ndiscard the tasks in its queue using CMDQ_TASK_MGMT\n\ncommand.\n\nWriting 0 to this register shall have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn clearalltasks(&mut self) -> ClearalltasksW<CqctrlSpec> {
        ClearalltasksW::new(self, 8)
    }
}
#[doc = "Command queueing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqctrlSpec;
impl crate::RegisterSpec for CqctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqctrl::R`](R) reader structure"]
impl crate::Readable for CqctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cqctrl::W`](W) writer structure"]
impl crate::Writable for CqctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQCTRL to value 0"]
impl crate::Resettable for CqctrlSpec {
    const RESET_VALUE: u32 = 0;
}
