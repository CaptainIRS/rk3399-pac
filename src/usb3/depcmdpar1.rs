#[doc = "Register `DEP%sCMDPAR1` reader"]
pub type R = crate::R<Depcmdpar1Spec>;
#[doc = "Register `DEP%sCMDPAR1` writer"]
pub type W = crate::W<Depcmdpar1Spec>;
#[doc = "Field `PARAMETER` reader - PARAMETER\n\nThis register indicates the physical endpoint command Parameter\n\n1. It must be programmed before issuing the command."]
pub type ParameterR = crate::FieldReader<u32>;
#[doc = "Field `PARAMETER` writer - PARAMETER\n\nThis register indicates the physical endpoint command Parameter\n\n1. It must be programmed before issuing the command."]
pub type ParameterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PARAMETER\n\nThis register indicates the physical endpoint command Parameter\n\n1. It must be programmed before issuing the command."]
    #[inline(always)]
    pub fn parameter(&self) -> ParameterR {
        ParameterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PARAMETER\n\nThis register indicates the physical endpoint command Parameter\n\n1. It must be programmed before issuing the command."]
    #[inline(always)]
    #[must_use]
    pub fn parameter(&mut self) -> ParameterW<Depcmdpar1Spec> {
        ParameterW::new(self, 0)
    }
}
#[doc = "Device Physical Endpoint-n Command Parameter 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`depcmdpar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`depcmdpar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Depcmdpar1Spec;
impl crate::RegisterSpec for Depcmdpar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`depcmdpar1::R`](R) reader structure"]
impl crate::Readable for Depcmdpar1Spec {}
#[doc = "`write(|w| ..)` method takes [`depcmdpar1::W`](W) writer structure"]
impl crate::Writable for Depcmdpar1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEP%sCMDPAR1 to value 0"]
impl crate::Resettable for Depcmdpar1Spec {
    const RESET_VALUE: u32 = 0;
}
