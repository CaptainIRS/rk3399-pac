#[doc = "Register `ERRLOG_FaultEn` reader"]
pub type R = crate::R<ErrlogFaultEnSpec>;
#[doc = "Register `ERRLOG_FaultEn` writer"]
pub type W = crate::W<ErrlogFaultEnSpec>;
#[doc = "Field `FAULTEN` reader - When set to 1, enables error reporting output signal Fault. Fault is\n\nasserted when register ErrVld is set to 1, and driven to 0 when\n\nFaultEn is cleared to 0."]
pub type FaultenR = crate::BitReader;
#[doc = "Field `FAULTEN` writer - When set to 1, enables error reporting output signal Fault. Fault is\n\nasserted when register ErrVld is set to 1, and driven to 0 when\n\nFaultEn is cleared to 0."]
pub type FaultenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, enables error reporting output signal Fault. Fault is\n\nasserted when register ErrVld is set to 1, and driven to 0 when\n\nFaultEn is cleared to 0."]
    #[inline(always)]
    pub fn faulten(&self) -> FaultenR {
        FaultenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, enables error reporting output signal Fault. Fault is\n\nasserted when register ErrVld is set to 1, and driven to 0 when\n\nFaultEn is cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn faulten(&mut self) -> FaultenW<ErrlogFaultEnSpec> {
        FaultenW::new(self, 0)
    }
}
#[doc = "Error interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_fault_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errlog_fault_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrlogFaultEnSpec;
impl crate::RegisterSpec for ErrlogFaultEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errlog_fault_en::R`](R) reader structure"]
impl crate::Readable for ErrlogFaultEnSpec {}
#[doc = "`write(|w| ..)` method takes [`errlog_fault_en::W`](W) writer structure"]
impl crate::Writable for ErrlogFaultEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRLOG_FaultEn to value 0"]
impl crate::Resettable for ErrlogFaultEnSpec {
    const RESET_VALUE: u32 = 0;
}
