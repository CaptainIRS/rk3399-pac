#[doc = "Register `MCU_BYPASS_PORT` reader"]
pub type R = crate::R<McuBypassPortSpec>;
#[doc = "Register `MCU_BYPASS_PORT` writer"]
pub type W = crate::W<McuBypassPortSpec>;
#[doc = "Field `FIELD0000_ABSTRACT` reader - Field0000 Description"]
pub type Field0000AbstractR = crate::BitReader;
#[doc = "Field `FIELD0000_ABSTRACT` writer - Field0000 Description"]
pub type Field0000AbstractW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Field0000 Description"]
    #[inline(always)]
    pub fn field0000_abstract(&self) -> Field0000AbstractR {
        Field0000AbstractR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Field0000 Description"]
    #[inline(always)]
    #[must_use]
    pub fn field0000_abstract(&mut self) -> Field0000AbstractW<McuBypassPortSpec> {
        Field0000AbstractW::new(self, 0)
    }
}
#[doc = "MCU bypass port\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu_bypass_port::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu_bypass_port::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McuBypassPortSpec;
impl crate::RegisterSpec for McuBypassPortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu_bypass_port::R`](R) reader structure"]
impl crate::Readable for McuBypassPortSpec {}
#[doc = "`write(|w| ..)` method takes [`mcu_bypass_port::W`](W) writer structure"]
impl crate::Writable for McuBypassPortSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCU_BYPASS_PORT to value 0"]
impl crate::Resettable for McuBypassPortSpec {
    const RESET_VALUE: u32 = 0;
}
