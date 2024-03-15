#[doc = "Register `SPI_ENR` reader"]
pub type R = crate::R<SpiEnrSpec>;
#[doc = "Register `SPI_ENR` writer"]
pub type W = crate::W<SpiEnrSpec>;
#[doc = "Field `ENR` reader - SPI Enable 1’b1: Enable all SPI operations. 1’b0: Disable all SPI operations Transmit and receive FIFO buffers are cleared when the device is disabled."]
pub type EnrR = crate::BitReader;
#[doc = "Field `ENR` writer - SPI Enable 1’b1: Enable all SPI operations. 1’b0: Disable all SPI operations Transmit and receive FIFO buffers are cleared when the device is disabled."]
pub type EnrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Enable 1’b1: Enable all SPI operations. 1’b0: Disable all SPI operations Transmit and receive FIFO buffers are cleared when the device is disabled."]
    #[inline(always)]
    pub fn enr(&self) -> EnrR {
        EnrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Enable 1’b1: Enable all SPI operations. 1’b0: Disable all SPI operations Transmit and receive FIFO buffers are cleared when the device is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn enr(&mut self) -> EnrW<SpiEnrSpec> {
        EnrW::new(self, 0)
    }
}
#[doc = "SPI Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiEnrSpec;
impl crate::RegisterSpec for SpiEnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_enr::R`](R) reader structure"]
impl crate::Readable for SpiEnrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_enr::W`](W) writer structure"]
impl crate::Writable for SpiEnrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_ENR to value 0"]
impl crate::Resettable for SpiEnrSpec {
    const RESET_VALUE: u32 = 0;
}
