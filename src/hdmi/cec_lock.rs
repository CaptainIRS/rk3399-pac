#[doc = "Register `CEC_LOCK` reader"]
pub type R = crate::R<CecLockSpec>;
#[doc = "Register `CEC_LOCK` writer"]
pub type W = crate::W<CecLockSpec>;
#[doc = "Field `LOCKED_BUFFER` reader - When a frame is received, this bit would be active. The CEC controller answers to all the messages with NACK until the CPU writes it to '0'."]
pub type LockedBufferR = crate::BitReader;
#[doc = "Field `LOCKED_BUFFER` writer - When a frame is received, this bit would be active. The CEC controller answers to all the messages with NACK until the CPU writes it to '0'."]
pub type LockedBufferW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When a frame is received, this bit would be active. The CEC controller answers to all the messages with NACK until the CPU writes it to '0'."]
    #[inline(always)]
    pub fn locked_buffer(&self) -> LockedBufferR {
        LockedBufferR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When a frame is received, this bit would be active. The CEC controller answers to all the messages with NACK until the CPU writes it to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn locked_buffer(&mut self) -> LockedBufferW<CecLockSpec> {
        LockedBufferW::new(self, 0)
    }
}
#[doc = "When a frame is received, this bit would be active. The CEC controller answers to all the messages with NACK until the CPU writes it to '0'.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CecLockSpec;
impl crate::RegisterSpec for CecLockSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cec_lock::R`](R) reader structure"]
impl crate::Readable for CecLockSpec {}
#[doc = "`write(|w| ..)` method takes [`cec_lock::W`](W) writer structure"]
impl crate::Writable for CecLockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CEC_LOCK to value 0"]
impl crate::Resettable for CecLockSpec {
    const RESET_VALUE: u8 = 0;
}
