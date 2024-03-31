#[doc = "Register `CQTDBN` reader"]
pub type R = crate::R<CqtdbnSpec>;
#[doc = "Register `CQTDBN` writer"]
pub type W = crate::W<CqtdbnSpec>;
#[doc = "Field `TCN` reader - Task Complete Notification\n\nCQE shall set bit n of this register (at the same time it clears bit n\n\nof CQTDBR) when a task execution is completed (with success or\n\nerror).\n\nWhen receiving interrupt for task completion, software may read\n\nthis register to know which tasks have finished.\n\nAfter reading this register, software may clear the relevant bit\n\nfields by writing 1 to the corresponding bits."]
pub type TcnR = crate::FieldReader<u32>;
#[doc = "Field `TCN` writer - Task Complete Notification\n\nCQE shall set bit n of this register (at the same time it clears bit n\n\nof CQTDBR) when a task execution is completed (with success or\n\nerror).\n\nWhen receiving interrupt for task completion, software may read\n\nthis register to know which tasks have finished.\n\nAfter reading this register, software may clear the relevant bit\n\nfields by writing 1 to the corresponding bits."]
pub type TcnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Task Complete Notification\n\nCQE shall set bit n of this register (at the same time it clears bit n\n\nof CQTDBR) when a task execution is completed (with success or\n\nerror).\n\nWhen receiving interrupt for task completion, software may read\n\nthis register to know which tasks have finished.\n\nAfter reading this register, software may clear the relevant bit\n\nfields by writing 1 to the corresponding bits."]
    #[inline(always)]
    pub fn tcn(&self) -> TcnR {
        TcnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Task Complete Notification\n\nCQE shall set bit n of this register (at the same time it clears bit n\n\nof CQTDBR) when a task execution is completed (with success or\n\nerror).\n\nWhen receiving interrupt for task completion, software may read\n\nthis register to know which tasks have finished.\n\nAfter reading this register, software may clear the relevant bit\n\nfields by writing 1 to the corresponding bits."]
    #[inline(always)]
    #[must_use]
    pub fn tcn(&mut self) -> TcnW<CqtdbnSpec> {
        TcnW::new(self, 0)
    }
}
#[doc = "Command queueing task doorbell notification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtdbn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqtdbn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqtdbnSpec;
impl crate::RegisterSpec for CqtdbnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqtdbn::R`](R) reader structure"]
impl crate::Readable for CqtdbnSpec {}
#[doc = "`write(|w| ..)` method takes [`cqtdbn::W`](W) writer structure"]
impl crate::Writable for CqtdbnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQTDBN to value 0"]
impl crate::Resettable for CqtdbnSpec {
    const RESET_VALUE: u32 = 0;
}
