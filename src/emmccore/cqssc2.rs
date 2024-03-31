#[doc = "Register `CQSSC2` reader"]
pub type R = crate::R<Cqssc2Spec>;
#[doc = "Register `CQSSC2` writer"]
pub type W = crate::W<Cqssc2Spec>;
#[doc = "Field `SQRCA` reader - Send Queue RCA\n\nThis field provides CQE with the contents of the 16-bit RCA field\n\nin SEND_QUEUE_ STATUS (CMD13) command. argument.\n\nCQE shall copy this field to bits 31:16 of the argument when\n\ntransmitting SEND_ QUEUE_STATUS (CMD13) command"]
pub type SqrcaR = crate::FieldReader<u16>;
#[doc = "Field `SQRCA` writer - Send Queue RCA\n\nThis field provides CQE with the contents of the 16-bit RCA field\n\nin SEND_QUEUE_ STATUS (CMD13) command. argument.\n\nCQE shall copy this field to bits 31:16 of the argument when\n\ntransmitting SEND_ QUEUE_STATUS (CMD13) command"]
pub type SqrcaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Send Queue RCA\n\nThis field provides CQE with the contents of the 16-bit RCA field\n\nin SEND_QUEUE_ STATUS (CMD13) command. argument.\n\nCQE shall copy this field to bits 31:16 of the argument when\n\ntransmitting SEND_ QUEUE_STATUS (CMD13) command"]
    #[inline(always)]
    pub fn sqrca(&self) -> SqrcaR {
        SqrcaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Send Queue RCA\n\nThis field provides CQE with the contents of the 16-bit RCA field\n\nin SEND_QUEUE_ STATUS (CMD13) command. argument.\n\nCQE shall copy this field to bits 31:16 of the argument when\n\ntransmitting SEND_ QUEUE_STATUS (CMD13) command"]
    #[inline(always)]
    #[must_use]
    pub fn sqrca(&mut self) -> SqrcaW<Cqssc2Spec> {
        SqrcaW::new(self, 0)
    }
}
#[doc = "Command queueing send status configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqssc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqssc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cqssc2Spec;
impl crate::RegisterSpec for Cqssc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqssc2::R`](R) reader structure"]
impl crate::Readable for Cqssc2Spec {}
#[doc = "`write(|w| ..)` method takes [`cqssc2::W`](W) writer structure"]
impl crate::Writable for Cqssc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQSSC2 to value 0"]
impl crate::Resettable for Cqssc2Spec {
    const RESET_VALUE: u32 = 0;
}
