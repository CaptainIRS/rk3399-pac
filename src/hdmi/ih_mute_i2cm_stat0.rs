#[doc = "Register `IH_MUTE_I2CM_STAT0` reader"]
pub type R = crate::R<IhMuteI2cmStat0Spec>;
#[doc = "Register `IH_MUTE_I2CM_STAT0` writer"]
pub type W = crate::W<IhMuteI2cmStat0Spec>;
#[doc = "Field `I2CMASTERERROR` reader - When set to 1, mutes ih_i2cm_stat0\\[0\\]"]
pub type I2cmastererrorR = crate::BitReader;
#[doc = "Field `I2CMASTERERROR` writer - When set to 1, mutes ih_i2cm_stat0\\[0\\]"]
pub type I2cmastererrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CMASTERDONE` reader - When set to 1, mutes ih_i2cm_stat0\\[1\\]"]
pub type I2cmasterdoneR = crate::BitReader;
#[doc = "Field `I2CMASTERDONE` writer - When set to 1, mutes ih_i2cm_stat0\\[1\\]"]
pub type I2cmasterdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCDC_READREQ` reader - When set to 1, mutes ih_i2cm_stat0\\[2\\]"]
pub type ScdcReadreqR = crate::BitReader;
#[doc = "Field `SCDC_READREQ` writer - When set to 1, mutes ih_i2cm_stat0\\[2\\]"]
pub type ScdcReadreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, mutes ih_i2cm_stat0\\[0\\]"]
    #[inline(always)]
    pub fn i2cmastererror(&self) -> I2cmastererrorR {
        I2cmastererrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_i2cm_stat0\\[1\\]"]
    #[inline(always)]
    pub fn i2cmasterdone(&self) -> I2cmasterdoneR {
        I2cmasterdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_i2cm_stat0\\[2\\]"]
    #[inline(always)]
    pub fn scdc_readreq(&self) -> ScdcReadreqR {
        ScdcReadreqR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, mutes ih_i2cm_stat0\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn i2cmastererror(&mut self) -> I2cmastererrorW<IhMuteI2cmStat0Spec> {
        I2cmastererrorW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_i2cm_stat0\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn i2cmasterdone(&mut self) -> I2cmasterdoneW<IhMuteI2cmStat0Spec> {
        I2cmasterdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_i2cm_stat0\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn scdc_readreq(&mut self) -> ScdcReadreqW<IhMuteI2cmStat0Spec> {
        ScdcReadreqW::new(self, 2)
    }
}
#[doc = "E-DDC I2C Master Interrupt Mute Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_i2cm_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_i2cm_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhMuteI2cmStat0Spec;
impl crate::RegisterSpec for IhMuteI2cmStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_mute_i2cm_stat0::R`](R) reader structure"]
impl crate::Readable for IhMuteI2cmStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_mute_i2cm_stat0::W`](W) writer structure"]
impl crate::Writable for IhMuteI2cmStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IH_MUTE_I2CM_STAT0 to value 0x04"]
impl crate::Resettable for IhMuteI2cmStat0Spec {
    const RESET_VALUE: u8 = 0x04;
}
