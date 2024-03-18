#[doc = "Register `EMMCCORE_CQTDBN` reader"]
pub type R = crate::R<EmmccoreCqtdbnSpec>;
#[doc = "Register `EMMCCORE_CQTDBN` writer"]
pub type W = crate::W<EmmccoreCqtdbnSpec>;
#[doc = "Field `TCN` reader - Task Complete Notification CQE shall set bit n of this register (at the same time it clears bit n of CQTDBR) when a task execution is completed (with success or error). When receiving interrupt for task completion, software may read this register to know which tasks have finished. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
pub type TcnR = crate::FieldReader<u32>;
#[doc = "Field `TCN` writer - Task Complete Notification CQE shall set bit n of this register (at the same time it clears bit n of CQTDBR) when a task execution is completed (with success or error). When receiving interrupt for task completion, software may read this register to know which tasks have finished. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
pub type TcnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Task Complete Notification CQE shall set bit n of this register (at the same time it clears bit n of CQTDBR) when a task execution is completed (with success or error). When receiving interrupt for task completion, software may read this register to know which tasks have finished. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
    #[inline(always)]
    pub fn tcn(&self) -> TcnR {
        TcnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Task Complete Notification CQE shall set bit n of this register (at the same time it clears bit n of CQTDBR) when a task execution is completed (with success or error). When receiving interrupt for task completion, software may read this register to know which tasks have finished. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
    #[inline(always)]
    #[must_use]
    pub fn tcn(&mut self) -> TcnW<EmmccoreCqtdbnSpec> {
        TcnW::new(self, 0)
    }
}
#[doc = "Command queueing task doorbell notification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqtdbn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqtdbn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqtdbnSpec;
impl crate::RegisterSpec for EmmccoreCqtdbnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqtdbn::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqtdbnSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_cqtdbn::W`](W) writer structure"]
impl crate::Writable for EmmccoreCqtdbnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CQTDBN to value 0"]
impl crate::Resettable for EmmccoreCqtdbnSpec {
    const RESET_VALUE: u32 = 0;
}
