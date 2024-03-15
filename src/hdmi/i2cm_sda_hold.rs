#[doc = "Register `I2CM_SDA_HOLD` reader"]
pub type R = crate::R<I2cmSdaHoldSpec>;
#[doc = "Register `I2CM_SDA_HOLD` writer"]
pub type W = crate::W<I2cmSdaHoldSpec>;
#[doc = "Field `OSDA_HOLD` reader - Defines the number of SFR clock cycles to meet tHD;DAT (300 ns) osda_hold = round_to_high_integer (300 ns / (1 / isfrclk_frequency))"]
pub type OsdaHoldR = crate::FieldReader;
#[doc = "Field `OSDA_HOLD` writer - Defines the number of SFR clock cycles to meet tHD;DAT (300 ns) osda_hold = round_to_high_integer (300 ns / (1 / isfrclk_frequency))"]
pub type OsdaHoldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Defines the number of SFR clock cycles to meet tHD;DAT (300 ns) osda_hold = round_to_high_integer (300 ns / (1 / isfrclk_frequency))"]
    #[inline(always)]
    pub fn osda_hold(&self) -> OsdaHoldR {
        OsdaHoldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the number of SFR clock cycles to meet tHD;DAT (300 ns) osda_hold = round_to_high_integer (300 ns / (1 / isfrclk_frequency))"]
    #[inline(always)]
    #[must_use]
    pub fn osda_hold(&mut self) -> OsdaHoldW<I2cmSdaHoldSpec> {
        OsdaHoldW::new(self, 0)
    }
}
#[doc = "Defines the number of SFR clock cycles to meet tHD;DAT (300 ns) osda_hold = round_to_high_integer (300 ns / (1 / isfrclk_frequency))\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_sda_hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_sda_hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmSdaHoldSpec;
impl crate::RegisterSpec for I2cmSdaHoldSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_sda_hold::R`](R) reader structure"]
impl crate::Readable for I2cmSdaHoldSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cm_sda_hold::W`](W) writer structure"]
impl crate::Writable for I2cmSdaHoldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_SDA_HOLD to value 0x09"]
impl crate::Resettable for I2cmSdaHoldSpec {
    const RESET_VALUE: u8 = 0x09;
}
