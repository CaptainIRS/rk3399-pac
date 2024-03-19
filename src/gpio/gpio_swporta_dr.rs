#[doc = "Register `GPIO_SWPORTA_DR` reader"]
pub type R = crate::R<GpioSwportaDrSpec>;
#[doc = "Register `GPIO_SWPORTA_DR` writer"]
pub type W = crate::W<GpioSwportaDrSpec>;
#[doc = "Field `GPIO_SWPORTA_DR` reader - Values written to this register are output on the I/O signals for\n\nPort A if the corresponding data direction bits for Port A are set to\n\nOutput mode. The value read back is equal to the last value\n\nwritten to this register."]
pub type GpioSwportaDrR = crate::FieldReader<u32>;
#[doc = "Field `GPIO_SWPORTA_DR` writer - Values written to this register are output on the I/O signals for\n\nPort A if the corresponding data direction bits for Port A are set to\n\nOutput mode. The value read back is equal to the last value\n\nwritten to this register."]
pub type GpioSwportaDrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Values written to this register are output on the I/O signals for\n\nPort A if the corresponding data direction bits for Port A are set to\n\nOutput mode. The value read back is equal to the last value\n\nwritten to this register."]
    #[inline(always)]
    pub fn gpio_swporta_dr(&self) -> GpioSwportaDrR {
        GpioSwportaDrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Values written to this register are output on the I/O signals for\n\nPort A if the corresponding data direction bits for Port A are set to\n\nOutput mode. The value read back is equal to the last value\n\nwritten to this register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_swporta_dr(&mut self) -> GpioSwportaDrW<GpioSwportaDrSpec> {
        GpioSwportaDrW::new(self, 0)
    }
}
#[doc = "Port A data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_swporta_dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_swporta_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioSwportaDrSpec;
impl crate::RegisterSpec for GpioSwportaDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_swporta_dr::R`](R) reader structure"]
impl crate::Readable for GpioSwportaDrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_swporta_dr::W`](W) writer structure"]
impl crate::Writable for GpioSwportaDrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_SWPORTA_DR to value 0"]
impl crate::Resettable for GpioSwportaDrSpec {
    const RESET_VALUE: u32 = 0;
}
