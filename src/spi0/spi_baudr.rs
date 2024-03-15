#[doc = "Register `SPI_BAUDR` reader"]
pub type R = crate::R<SpiBaudrSpec>;
#[doc = "Register `SPI_BAUDR` writer"]
pub type W = crate::W<SpiBaudrSpec>;
#[doc = "Field `BAUDR` reader - Baud Rate Select SPI Clock Divider. This register is valid only when the SPI is configured as a master device. The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (sclk_out) is disabled. The frequency of the sclk_out is derived from the following equation: Fsclk_out = Fspi_clk/ SCKDV Where SCKDV is any even value between 2 and 65534. For example: for Fspi_clk = 3.6864MHz and SCKDV =2 Fsclk_out = 3.6864/2= 1.8432MHz"]
pub type BaudrR = crate::FieldReader<u16>;
#[doc = "Field `BAUDR` writer - Baud Rate Select SPI Clock Divider. This register is valid only when the SPI is configured as a master device. The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (sclk_out) is disabled. The frequency of the sclk_out is derived from the following equation: Fsclk_out = Fspi_clk/ SCKDV Where SCKDV is any even value between 2 and 65534. For example: for Fspi_clk = 3.6864MHz and SCKDV =2 Fsclk_out = 3.6864/2= 1.8432MHz"]
pub type BaudrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Baud Rate Select SPI Clock Divider. This register is valid only when the SPI is configured as a master device. The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (sclk_out) is disabled. The frequency of the sclk_out is derived from the following equation: Fsclk_out = Fspi_clk/ SCKDV Where SCKDV is any even value between 2 and 65534. For example: for Fspi_clk = 3.6864MHz and SCKDV =2 Fsclk_out = 3.6864/2= 1.8432MHz"]
    #[inline(always)]
    pub fn baudr(&self) -> BaudrR {
        BaudrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Baud Rate Select SPI Clock Divider. This register is valid only when the SPI is configured as a master device. The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (sclk_out) is disabled. The frequency of the sclk_out is derived from the following equation: Fsclk_out = Fspi_clk/ SCKDV Where SCKDV is any even value between 2 and 65534. For example: for Fspi_clk = 3.6864MHz and SCKDV =2 Fsclk_out = 3.6864/2= 1.8432MHz"]
    #[inline(always)]
    #[must_use]
    pub fn baudr(&mut self) -> BaudrW<SpiBaudrSpec> {
        BaudrW::new(self, 0)
    }
}
#[doc = "Baud Rate Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_baudr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_baudr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiBaudrSpec;
impl crate::RegisterSpec for SpiBaudrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_baudr::R`](R) reader structure"]
impl crate::Readable for SpiBaudrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_baudr::W`](W) writer structure"]
impl crate::Writable for SpiBaudrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_BAUDR to value 0"]
impl crate::Resettable for SpiBaudrSpec {
    const RESET_VALUE: u32 = 0;
}
