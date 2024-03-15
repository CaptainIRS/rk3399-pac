#[doc = "Register `EMMCCORE_CQTCLR` reader"]
pub type R = crate::R<EmmccoreCqtclrSpec>;
#[doc = "Register `EMMCCORE_CQTCLR` writer"]
pub type W = crate::W<EmmccoreCqtclrSpec>;
#[doc = "Field `CQTC` reader - Command Queueing Task Clear Writing 1 to bit n of this register orders CQE to clear a task which software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once clear operation is complete. Software should poll on the CQTCLR until it is cleared to verify clear operation was complete. Writing to this register only clears the task in the CQE and does not have impact on the device. In order to discard the task in the device, host softwareshall send CMDQ_TASK _MGMT while CQE is still in Halt state. Host driver is not allowed to use this register to clear multiple tasks at the same time. Clearing multiple tasks can be done using CQCTL register. Writing 0 to a register bit shall have no impact."]
pub type CqtcR = crate::FieldReader<u32>;
#[doc = "Field `CQTC` writer - Command Queueing Task Clear Writing 1 to bit n of this register orders CQE to clear a task which software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once clear operation is complete. Software should poll on the CQTCLR until it is cleared to verify clear operation was complete. Writing to this register only clears the task in the CQE and does not have impact on the device. In order to discard the task in the device, host softwareshall send CMDQ_TASK _MGMT while CQE is still in Halt state. Host driver is not allowed to use this register to clear multiple tasks at the same time. Clearing multiple tasks can be done using CQCTL register. Writing 0 to a register bit shall have no impact."]
pub type CqtcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Queueing Task Clear Writing 1 to bit n of this register orders CQE to clear a task which software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once clear operation is complete. Software should poll on the CQTCLR until it is cleared to verify clear operation was complete. Writing to this register only clears the task in the CQE and does not have impact on the device. In order to discard the task in the device, host softwareshall send CMDQ_TASK _MGMT while CQE is still in Halt state. Host driver is not allowed to use this register to clear multiple tasks at the same time. Clearing multiple tasks can be done using CQCTL register. Writing 0 to a register bit shall have no impact."]
    #[inline(always)]
    pub fn cqtc(&self) -> CqtcR {
        CqtcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Queueing Task Clear Writing 1 to bit n of this register orders CQE to clear a task which software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once clear operation is complete. Software should poll on the CQTCLR until it is cleared to verify clear operation was complete. Writing to this register only clears the task in the CQE and does not have impact on the device. In order to discard the task in the device, host softwareshall send CMDQ_TASK _MGMT while CQE is still in Halt state. Host driver is not allowed to use this register to clear multiple tasks at the same time. Clearing multiple tasks can be done using CQCTL register. Writing 0 to a register bit shall have no impact."]
    #[inline(always)]
    #[must_use]
    pub fn cqtc(&mut self) -> CqtcW<EmmccoreCqtclrSpec> {
        CqtcW::new(self, 0)
    }
}
#[doc = "Command queueing task clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqtclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqtclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqtclrSpec;
impl crate::RegisterSpec for EmmccoreCqtclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqtclr::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqtclrSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_cqtclr::W`](W) writer structure"]
impl crate::Writable for EmmccoreCqtclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CQTCLR to value 0"]
impl crate::Resettable for EmmccoreCqtclrSpec {
    const RESET_VALUE: u32 = 0;
}
