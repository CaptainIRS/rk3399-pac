#[doc = "Register `ENR` reader"]
pub type R = crate::R<EnrSpec>;
#[doc = "Register `ENR` writer"]
pub type W = crate::W<EnrSpec>;
#[doc = "Field `ENR` reader - SPI Enable\n\n1’b1: Enable all SPI operations.\n\n1’b0: Disable all SPI operations\n\nTransmit and receive FIFO buffers are cleared when the device is\n\ndisabled."]
pub type EnrR = crate::BitReader;
#[doc = "Field `ENR` writer - SPI Enable\n\n1’b1: Enable all SPI operations.\n\n1’b0: Disable all SPI operations\n\nTransmit and receive FIFO buffers are cleared when the device is\n\ndisabled."]
pub type EnrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Enable\n\n1’b1: Enable all SPI operations.\n\n1’b0: Disable all SPI operations\n\nTransmit and receive FIFO buffers are cleared when the device is\n\ndisabled."]
    #[inline(always)]
    pub fn enr(&self) -> EnrR {
        EnrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Enable\n\n1’b1: Enable all SPI operations.\n\n1’b0: Disable all SPI operations\n\nTransmit and receive FIFO buffers are cleared when the device is\n\ndisabled."]
    #[inline(always)]
    #[must_use]
    pub fn enr(&mut self) -> EnrW<EnrSpec> {
        EnrW::new(self, 0)
    }
}
#[doc = "SPI Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnrSpec;
impl crate::RegisterSpec for EnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enr::R`](R) reader structure"]
impl crate::Readable for EnrSpec {}
#[doc = "`write(|w| ..)` method takes [`enr::W`](W) writer structure"]
impl crate::Writable for EnrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENR to value 0"]
impl crate::Resettable for EnrSpec {
    const RESET_VALUE: u32 = 0;
}
