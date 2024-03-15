#[doc = "Register `USB3_DEP%sCMDPAR2` reader"]
pub type R = crate::R<Usb3Depcmdpar2Spec>;
#[doc = "Register `USB3_DEP%sCMDPAR2` writer"]
pub type W = crate::W<Usb3Depcmdpar2Spec>;
#[doc = "Field `PARAMETER` reader - PARAMETER This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command."]
pub type ParameterR = crate::FieldReader<u32>;
#[doc = "Field `PARAMETER` writer - PARAMETER This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command."]
pub type ParameterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PARAMETER This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command."]
    #[inline(always)]
    pub fn parameter(&self) -> ParameterR {
        ParameterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PARAMETER This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command."]
    #[inline(always)]
    #[must_use]
    pub fn parameter(&mut self) -> ParameterW<Usb3Depcmdpar2Spec> {
        ParameterW::new(self, 0)
    }
}
#[doc = "Device Physical Endpoint-n Command Parameter 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_depcmdpar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_depcmdpar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3Depcmdpar2Spec;
impl crate::RegisterSpec for Usb3Depcmdpar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_depcmdpar2::R`](R) reader structure"]
impl crate::Readable for Usb3Depcmdpar2Spec {}
#[doc = "`write(|w| ..)` method takes [`usb3_depcmdpar2::W`](W) writer structure"]
impl crate::Writable for Usb3Depcmdpar2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_DEP%sCMDPAR2 to value 0"]
impl crate::Resettable for Usb3Depcmdpar2Spec {
    const RESET_VALUE: u32 = 0;
}
