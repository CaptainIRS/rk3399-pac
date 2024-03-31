#[doc = "Register `DEP%sCMDPAR2` reader"]
pub type R = crate::R<Depcmdpar2Spec>;
#[doc = "Register `DEP%sCMDPAR2` writer"]
pub type W = crate::W<Depcmdpar2Spec>;
#[doc = "Field `PARAMETER` reader - PARAMETER\n\nThis register indicates the physical endpoint command Parameter\n\n2. It must be programmed before issuing the command."]
pub type ParameterR = crate::FieldReader<u32>;
#[doc = "Field `PARAMETER` writer - PARAMETER\n\nThis register indicates the physical endpoint command Parameter\n\n2. It must be programmed before issuing the command."]
pub type ParameterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PARAMETER\n\nThis register indicates the physical endpoint command Parameter\n\n2. It must be programmed before issuing the command."]
    #[inline(always)]
    pub fn parameter(&self) -> ParameterR {
        ParameterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PARAMETER\n\nThis register indicates the physical endpoint command Parameter\n\n2. It must be programmed before issuing the command."]
    #[inline(always)]
    #[must_use]
    pub fn parameter(&mut self) -> ParameterW<Depcmdpar2Spec> {
        ParameterW::new(self, 0)
    }
}
#[doc = "Device Physical Endpoint-n Command Parameter 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`depcmdpar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`depcmdpar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Depcmdpar2Spec;
impl crate::RegisterSpec for Depcmdpar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`depcmdpar2::R`](R) reader structure"]
impl crate::Readable for Depcmdpar2Spec {}
#[doc = "`write(|w| ..)` method takes [`depcmdpar2::W`](W) writer structure"]
impl crate::Writable for Depcmdpar2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEP%sCMDPAR2 to value 0"]
impl crate::Resettable for Depcmdpar2Spec {
    const RESET_VALUE: u32 = 0;
}
