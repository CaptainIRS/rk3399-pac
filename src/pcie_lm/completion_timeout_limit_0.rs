#[doc = "Register `COMPLETION_TIMEOUT_LIMIT_0` reader"]
pub type R = crate::R<CompletionTimeoutLimit0Spec>;
#[doc = "Register `COMPLETION_TIMEOUT_LIMIT_0` writer"]
pub type W = crate::W<CompletionTimeoutLimit0Spec>;
#[doc = "Field `CTL` reader - Completion Timeout Limit \\[CTL\\]
Timeout limit for completion timers (in 4 ns cycles)."]
pub type CtlR = crate::FieldReader<u32>;
#[doc = "Field `CTL` writer - Completion Timeout Limit \\[CTL\\]
Timeout limit for completion timers (in 4 ns cycles)."]
pub type CtlW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `R5` reader - Reserved \\[R5\\]
Reserved"]
pub type R5R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - Completion Timeout Limit \\[CTL\\]
Timeout limit for completion timers (in 4 ns cycles)."]
    #[inline(always)]
    pub fn ctl(&self) -> CtlR {
        CtlR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Reserved \\[R5\\]
Reserved"]
    #[inline(always)]
    pub fn r5(&self) -> R5R {
        R5R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Completion Timeout Limit \\[CTL\\]
Timeout limit for completion timers (in 4 ns cycles)."]
    #[inline(always)]
    #[must_use]
    pub fn ctl(&mut self) -> CtlW<CompletionTimeoutLimit0Spec> {
        CtlW::new(self, 0)
    }
}
#[doc = "Completion Timeout Limit Register 0 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`completion_timeout_limit_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`completion_timeout_limit_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompletionTimeoutLimit0Spec;
impl crate::RegisterSpec for CompletionTimeoutLimit0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`completion_timeout_limit_0::R`](R) reader structure"]
impl crate::Readable for CompletionTimeoutLimit0Spec {}
#[doc = "`write(|w| ..)` method takes [`completion_timeout_limit_0::W`](W) writer structure"]
impl crate::Writable for CompletionTimeoutLimit0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMPLETION_TIMEOUT_LIMIT_0 to value 0x00be_bc20"]
impl crate::Resettable for CompletionTimeoutLimit0Spec {
    const RESET_VALUE: u32 = 0x00be_bc20;
}
