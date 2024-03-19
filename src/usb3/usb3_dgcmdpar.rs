#[doc = "Register `USB3_DGCMDPAR` reader"]
pub type R = crate::R<Usb3DgcmdparSpec>;
#[doc = "Register `USB3_DGCMDPAR` writer"]
pub type W = crate::W<Usb3DgcmdparSpec>;
#[doc = "Field `PARAMETER` reader - PARAMETER\n\nThis register indicates the device command parameter. This must\n\nbe programmed before or along with the device command. The\n\navailable device commands are listed in DGCMD register."]
pub type ParameterR = crate::FieldReader<u32>;
#[doc = "Field `PARAMETER` writer - PARAMETER\n\nThis register indicates the device command parameter. This must\n\nbe programmed before or along with the device command. The\n\navailable device commands are listed in DGCMD register."]
pub type ParameterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PARAMETER\n\nThis register indicates the device command parameter. This must\n\nbe programmed before or along with the device command. The\n\navailable device commands are listed in DGCMD register."]
    #[inline(always)]
    pub fn parameter(&self) -> ParameterR {
        ParameterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PARAMETER\n\nThis register indicates the device command parameter. This must\n\nbe programmed before or along with the device command. The\n\navailable device commands are listed in DGCMD register."]
    #[inline(always)]
    #[must_use]
    pub fn parameter(&mut self) -> ParameterW<Usb3DgcmdparSpec> {
        ParameterW::new(self, 0)
    }
}
#[doc = "Device Generic Command Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_dgcmdpar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_dgcmdpar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3DgcmdparSpec;
impl crate::RegisterSpec for Usb3DgcmdparSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_dgcmdpar::R`](R) reader structure"]
impl crate::Readable for Usb3DgcmdparSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_dgcmdpar::W`](W) writer structure"]
impl crate::Writable for Usb3DgcmdparSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_DGCMDPAR to value 0"]
impl crate::Resettable for Usb3DgcmdparSpec {
    const RESET_VALUE: u32 = 0;
}
