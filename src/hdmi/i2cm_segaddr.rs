#[doc = "Register `I2CM_SEGADDR` reader"]
pub type R = crate::R<I2cmSegaddrSpec>;
#[doc = "Register `I2CM_SEGADDR` writer"]
pub type W = crate::W<I2cmSegaddrSpec>;
#[doc = "Field `SEG_ADDR` reader - I2C DDC Segment Address Configuration Register"]
pub type SegAddrR = crate::FieldReader;
#[doc = "Field `SEG_ADDR` writer - I2C DDC Segment Address Configuration Register"]
pub type SegAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - I2C DDC Segment Address Configuration Register"]
    #[inline(always)]
    pub fn seg_addr(&self) -> SegAddrR {
        SegAddrR::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - I2C DDC Segment Address Configuration Register"]
    #[inline(always)]
    #[must_use]
    pub fn seg_addr(&mut self) -> SegAddrW<I2cmSegaddrSpec> {
        SegAddrW::new(self, 0)
    }
}
#[doc = "I2C DDC Segment Address Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_segaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_segaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmSegaddrSpec;
impl crate::RegisterSpec for I2cmSegaddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_segaddr::R`](R) reader structure"]
impl crate::Readable for I2cmSegaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cm_segaddr::W`](W) writer structure"]
impl crate::Writable for I2cmSegaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_SEGADDR to value 0"]
impl crate::Resettable for I2cmSegaddrSpec {
    const RESET_VALUE: u8 = 0;
}
