#[doc = "Register `PME_SERVICE_TIMEOUT_DELAY` reader"]
pub type R = crate::R<PmeServiceTimeoutDelaySpec>;
#[doc = "Register `PME_SERVICE_TIMEOUT_DELAY` writer"]
pub type W = crate::W<PmeServiceTimeoutDelaySpec>;
#[doc = "Field `PSTD` reader - PME Service Timeout Delay \\[PSTD\\]
Specifies the timeout delay for retransmission of PM_PME messages. The value is in units of microseconds. The actual time elapsed has a +1 microseconds tolerance from the value programmed."]
pub type PstdR = crate::FieldReader<u32>;
#[doc = "Field `PSTD` writer - PME Service Timeout Delay \\[PSTD\\]
Specifies the timeout delay for retransmission of PM_PME messages. The value is in units of microseconds. The actual time elapsed has a +1 microseconds tolerance from the value programmed."]
pub type PstdW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DPMOPS` reader - Disable PME message on PM Status \\[DPMOPS\\]
When this bit is set, core will not automatically send a PME message, when PM Status bit in PMCSR register is set"]
pub type DpmopsR = crate::BitReader;
#[doc = "Field `DPMOPS` writer - Disable PME message on PM Status \\[DPMOPS\\]
When this bit is set, core will not automatically send a PME message, when PM Status bit in PMCSR register is set"]
pub type DpmopsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R21` reader - Reserved \\[R21\\]
Reserved"]
pub type R21R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:19 - PME Service Timeout Delay \\[PSTD\\]
Specifies the timeout delay for retransmission of PM_PME messages. The value is in units of microseconds. The actual time elapsed has a +1 microseconds tolerance from the value programmed."]
    #[inline(always)]
    pub fn pstd(&self) -> PstdR {
        PstdR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - Disable PME message on PM Status \\[DPMOPS\\]
When this bit is set, core will not automatically send a PME message, when PM Status bit in PMCSR register is set"]
    #[inline(always)]
    pub fn dpmops(&self) -> DpmopsR {
        DpmopsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - Reserved \\[R21\\]
Reserved"]
    #[inline(always)]
    pub fn r21(&self) -> R21R {
        R21R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - PME Service Timeout Delay \\[PSTD\\]
Specifies the timeout delay for retransmission of PM_PME messages. The value is in units of microseconds. The actual time elapsed has a +1 microseconds tolerance from the value programmed."]
    #[inline(always)]
    #[must_use]
    pub fn pstd(&mut self) -> PstdW<PmeServiceTimeoutDelaySpec> {
        PstdW::new(self, 0)
    }
    #[doc = "Bit 20 - Disable PME message on PM Status \\[DPMOPS\\]
When this bit is set, core will not automatically send a PME message, when PM Status bit in PMCSR register is set"]
    #[inline(always)]
    #[must_use]
    pub fn dpmops(&mut self) -> DpmopsW<PmeServiceTimeoutDelaySpec> {
        DpmopsW::new(self, 20)
    }
}
#[doc = "PME Service Timeout Delay Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pme_service_timeout_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pme_service_timeout_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmeServiceTimeoutDelaySpec;
impl crate::RegisterSpec for PmeServiceTimeoutDelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pme_service_timeout_delay::R`](R) reader structure"]
impl crate::Readable for PmeServiceTimeoutDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`pme_service_timeout_delay::W`](W) writer structure"]
impl crate::Writable for PmeServiceTimeoutDelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PME_SERVICE_TIMEOUT_DELAY to value 0x0001_86a0"]
impl crate::Resettable for PmeServiceTimeoutDelaySpec {
    const RESET_VALUE: u32 = 0x0001_86a0;
}
