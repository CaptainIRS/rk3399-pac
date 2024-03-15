#[doc = "Register `IH_I2CM_STAT0` reader"]
pub type R = crate::R<IhI2cmStat0Spec>;
#[doc = "Register `IH_I2CM_STAT0` writer"]
pub type W = crate::W<IhI2cmStat0Spec>;
#[doc = "Field `I2CMASTERERROR` reader - I2C Master error indication"]
pub type I2cmastererrorR = crate::BitReader;
#[doc = "Field `I2CMASTERERROR` writer - I2C Master error indication"]
pub type I2cmastererrorW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `I2CMASTERDONE` reader - I2C Master done indication"]
pub type I2cmasterdoneR = crate::BitReader;
#[doc = "Field `I2CMASTERDONE` writer - I2C Master done indication"]
pub type I2cmasterdoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SCDC_READREQ` reader - I2C Master SCDC read request indication."]
pub type ScdcReadreqR = crate::BitReader;
#[doc = "Field `SCDC_READREQ` writer - I2C Master SCDC read request indication."]
pub type ScdcReadreqW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Master error indication"]
    #[inline(always)]
    pub fn i2cmastererror(&self) -> I2cmastererrorR {
        I2cmastererrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Master done indication"]
    #[inline(always)]
    pub fn i2cmasterdone(&self) -> I2cmasterdoneR {
        I2cmasterdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Master SCDC read request indication."]
    #[inline(always)]
    pub fn scdc_readreq(&self) -> ScdcReadreqR {
        ScdcReadreqR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Master error indication"]
    #[inline(always)]
    #[must_use]
    pub fn i2cmastererror(&mut self) -> I2cmastererrorW<IhI2cmStat0Spec> {
        I2cmastererrorW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Master done indication"]
    #[inline(always)]
    #[must_use]
    pub fn i2cmasterdone(&mut self) -> I2cmasterdoneW<IhI2cmStat0Spec> {
        I2cmasterdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - I2C Master SCDC read request indication."]
    #[inline(always)]
    #[must_use]
    pub fn scdc_readreq(&mut self) -> ScdcReadreqW<IhI2cmStat0Spec> {
        ScdcReadreqW::new(self, 2)
    }
}
#[doc = "I2C Master error indication\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_i2cm_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_i2cm_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhI2cmStat0Spec;
impl crate::RegisterSpec for IhI2cmStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_i2cm_stat0::R`](R) reader structure"]
impl crate::Readable for IhI2cmStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_i2cm_stat0::W`](W) writer structure"]
impl crate::Writable for IhI2cmStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x07;
}
#[doc = "`reset()` method sets IH_I2CM_STAT0 to value 0"]
impl crate::Resettable for IhI2cmStat0Spec {
    const RESET_VALUE: u8 = 0;
}
