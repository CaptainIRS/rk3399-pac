#[doc = "Register `SER` reader"]
pub type R = crate::R<SerSpec>;
#[doc = "Register `SER` writer"]
pub type W = crate::W<SerSpec>;
#[doc = "Field `SER0` reader - Slave Select Enable\n\n1’b1: Enable chip select 0\n\n1’b0: Disable chip select 0\n\nThis register is valid only when SPI is configured as a master\n\ndevice."]
pub type Ser0R = crate::BitReader;
#[doc = "Field `SER0` writer - Slave Select Enable\n\n1’b1: Enable chip select 0\n\n1’b0: Disable chip select 0\n\nThis register is valid only when SPI is configured as a master\n\ndevice."]
pub type Ser0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SER1` reader - Slave 1 Select Enable\n\n1’b1: Enable chip select 1\n\n1’b0: Disable chip select 1\n\nThis register is valid only when SPI is configured as a master\n\ndevice."]
pub type Ser1R = crate::BitReader;
#[doc = "Field `SER1` writer - Slave 1 Select Enable\n\n1’b1: Enable chip select 1\n\n1’b0: Disable chip select 1\n\nThis register is valid only when SPI is configured as a master\n\ndevice."]
pub type Ser1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Slave Select Enable\n\n1’b1: Enable chip select 0\n\n1’b0: Disable chip select 0\n\nThis register is valid only when SPI is configured as a master\n\ndevice."]
    #[inline(always)]
    pub fn ser0(&self) -> Ser0R {
        Ser0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave 1 Select Enable\n\n1’b1: Enable chip select 1\n\n1’b0: Disable chip select 1\n\nThis register is valid only when SPI is configured as a master\n\ndevice."]
    #[inline(always)]
    pub fn ser1(&self) -> Ser1R {
        Ser1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Select Enable\n\n1’b1: Enable chip select 0\n\n1’b0: Disable chip select 0\n\nThis register is valid only when SPI is configured as a master\n\ndevice."]
    #[inline(always)]
    #[must_use]
    pub fn ser0(&mut self) -> Ser0W<SerSpec> {
        Ser0W::new(self, 0)
    }
    #[doc = "Bit 1 - Slave 1 Select Enable\n\n1’b1: Enable chip select 1\n\n1’b0: Disable chip select 1\n\nThis register is valid only when SPI is configured as a master\n\ndevice."]
    #[inline(always)]
    #[must_use]
    pub fn ser1(&mut self) -> Ser1W<SerSpec> {
        Ser1W::new(self, 1)
    }
}
#[doc = "Slave Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ser::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ser::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SerSpec;
impl crate::RegisterSpec for SerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ser::R`](R) reader structure"]
impl crate::Readable for SerSpec {}
#[doc = "`write(|w| ..)` method takes [`ser::W`](W) writer structure"]
impl crate::Writable for SerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SER to value 0"]
impl crate::Resettable for SerSpec {
    const RESET_VALUE: u32 = 0;
}
