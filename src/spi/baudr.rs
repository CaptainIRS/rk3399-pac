#[doc = "Register `BAUDR` reader"]
pub type R = crate::R<BaudrSpec>;
#[doc = "Register `BAUDR` writer"]
pub type W = crate::W<BaudrSpec>;
#[doc = "Field `BAUDR` reader - Baud Rate Select\n\nSPI Clock Divider.\n\nThis register is valid only when the SPI is configured as a master\n\ndevice.\n\nThe LSB for this field is always set to 0 and is unaffected by a\n\nwrite operation, which ensures an even value is held in this\n\nregister.\n\nIf the value is 0, the serial output clock (sclk_out) is disabled.\n\nThe frequency of the sclk_out is derived from the following\n\nequation:\n\nFsclk_out = Fspi_clk/ SCKDV\n\nWhere SCKDV is any even value between 2 and 65534.\n\nFor example:\n\nfor Fspi_clk = 3.6864MHz and SCKDV =2\n\nFsclk_out = 3.6864/2= 1.8432MHz"]
pub type BaudrR = crate::FieldReader<u16>;
#[doc = "Field `BAUDR` writer - Baud Rate Select\n\nSPI Clock Divider.\n\nThis register is valid only when the SPI is configured as a master\n\ndevice.\n\nThe LSB for this field is always set to 0 and is unaffected by a\n\nwrite operation, which ensures an even value is held in this\n\nregister.\n\nIf the value is 0, the serial output clock (sclk_out) is disabled.\n\nThe frequency of the sclk_out is derived from the following\n\nequation:\n\nFsclk_out = Fspi_clk/ SCKDV\n\nWhere SCKDV is any even value between 2 and 65534.\n\nFor example:\n\nfor Fspi_clk = 3.6864MHz and SCKDV =2\n\nFsclk_out = 3.6864/2= 1.8432MHz"]
pub type BaudrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Baud Rate Select\n\nSPI Clock Divider.\n\nThis register is valid only when the SPI is configured as a master\n\ndevice.\n\nThe LSB for this field is always set to 0 and is unaffected by a\n\nwrite operation, which ensures an even value is held in this\n\nregister.\n\nIf the value is 0, the serial output clock (sclk_out) is disabled.\n\nThe frequency of the sclk_out is derived from the following\n\nequation:\n\nFsclk_out = Fspi_clk/ SCKDV\n\nWhere SCKDV is any even value between 2 and 65534.\n\nFor example:\n\nfor Fspi_clk = 3.6864MHz and SCKDV =2\n\nFsclk_out = 3.6864/2= 1.8432MHz"]
    #[inline(always)]
    pub fn baudr(&self) -> BaudrR {
        BaudrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Baud Rate Select\n\nSPI Clock Divider.\n\nThis register is valid only when the SPI is configured as a master\n\ndevice.\n\nThe LSB for this field is always set to 0 and is unaffected by a\n\nwrite operation, which ensures an even value is held in this\n\nregister.\n\nIf the value is 0, the serial output clock (sclk_out) is disabled.\n\nThe frequency of the sclk_out is derived from the following\n\nequation:\n\nFsclk_out = Fspi_clk/ SCKDV\n\nWhere SCKDV is any even value between 2 and 65534.\n\nFor example:\n\nfor Fspi_clk = 3.6864MHz and SCKDV =2\n\nFsclk_out = 3.6864/2= 1.8432MHz"]
    #[inline(always)]
    #[must_use]
    pub fn baudr(&mut self) -> BaudrW<BaudrSpec> {
        BaudrW::new(self, 0)
    }
}
#[doc = "Baud Rate Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baudr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baudr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudrSpec;
impl crate::RegisterSpec for BaudrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baudr::R`](R) reader structure"]
impl crate::Readable for BaudrSpec {}
#[doc = "`write(|w| ..)` method takes [`baudr::W`](W) writer structure"]
impl crate::Writable for BaudrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BAUDR to value 0"]
impl crate::Resettable for BaudrSpec {
    const RESET_VALUE: u32 = 0;
}
