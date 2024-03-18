#[doc = "Register `SPI_SER` reader"]
pub type R = crate::R<SpiSerSpec>;
#[doc = "Register `SPI_SER` writer"]
pub type W = crate::W<SpiSerSpec>;
#[doc = "Field `SER0` reader - Slave Select Enable 1’b1: Enable chip select 0 1’b0: Disable chip select 0 This register is valid only when SPI is configured as a master device."]
pub type Ser0R = crate::BitReader;
#[doc = "Field `SER0` writer - Slave Select Enable 1’b1: Enable chip select 0 1’b0: Disable chip select 0 This register is valid only when SPI is configured as a master device."]
pub type Ser0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SER1` reader - Slave 1 Select Enable 1’b1: Enable chip select 1 1’b0: Disable chip select 1 This register is valid only when SPI is configured as a master device."]
pub type Ser1R = crate::BitReader;
#[doc = "Field `SER1` writer - Slave 1 Select Enable 1’b1: Enable chip select 1 1’b0: Disable chip select 1 This register is valid only when SPI is configured as a master device."]
pub type Ser1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Slave Select Enable 1’b1: Enable chip select 0 1’b0: Disable chip select 0 This register is valid only when SPI is configured as a master device."]
    #[inline(always)]
    pub fn ser0(&self) -> Ser0R {
        Ser0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave 1 Select Enable 1’b1: Enable chip select 1 1’b0: Disable chip select 1 This register is valid only when SPI is configured as a master device."]
    #[inline(always)]
    pub fn ser1(&self) -> Ser1R {
        Ser1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Select Enable 1’b1: Enable chip select 0 1’b0: Disable chip select 0 This register is valid only when SPI is configured as a master device."]
    #[inline(always)]
    #[must_use]
    pub fn ser0(&mut self) -> Ser0W<SpiSerSpec> {
        Ser0W::new(self, 0)
    }
    #[doc = "Bit 1 - Slave 1 Select Enable 1’b1: Enable chip select 1 1’b0: Disable chip select 1 This register is valid only when SPI is configured as a master device."]
    #[inline(always)]
    #[must_use]
    pub fn ser1(&mut self) -> Ser1W<SpiSerSpec> {
        Ser1W::new(self, 1)
    }
}
#[doc = "Slave Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ser::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ser::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSerSpec;
impl crate::RegisterSpec for SpiSerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ser::R`](R) reader structure"]
impl crate::Readable for SpiSerSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_ser::W`](W) writer structure"]
impl crate::Writable for SpiSerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_SER to value 0"]
impl crate::Resettable for SpiSerSpec {
    const RESET_VALUE: u32 = 0;
}
