#[doc = "Register `IH_MUTE_I2CMPHY_STAT0` reader"]
pub type R = crate::R<IhMuteI2cmphyStat0Spec>;
#[doc = "Register `IH_MUTE_I2CMPHY_STAT0` writer"]
pub type W = crate::W<IhMuteI2cmphyStat0Spec>;
#[doc = "Field `I2CMPHYERROR` reader - When set to 1, mutes ih_i2cmphy_stat0\\[0\\]"]
pub type I2cmphyerrorR = crate::BitReader;
#[doc = "Field `I2CMPHYERROR` writer - When set to 1, mutes ih_i2cmphy_stat0\\[0\\]"]
pub type I2cmphyerrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CMPHYDONE` reader - When set to 1, mutes ih_i2cmphy_stat0\\[1\\]"]
pub type I2cmphydoneR = crate::BitReader;
#[doc = "Field `I2CMPHYDONE` writer - When set to 1, mutes ih_i2cmphy_stat0\\[1\\]"]
pub type I2cmphydoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, mutes ih_i2cmphy_stat0\\[0\\]"]
    #[inline(always)]
    pub fn i2cmphyerror(&self) -> I2cmphyerrorR {
        I2cmphyerrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_i2cmphy_stat0\\[1\\]"]
    #[inline(always)]
    pub fn i2cmphydone(&self) -> I2cmphydoneR {
        I2cmphydoneR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, mutes ih_i2cmphy_stat0\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn i2cmphyerror(&mut self) -> I2cmphyerrorW<IhMuteI2cmphyStat0Spec> {
        I2cmphyerrorW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_i2cmphy_stat0\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn i2cmphydone(&mut self) -> I2cmphydoneW<IhMuteI2cmphyStat0Spec> {
        I2cmphydoneW::new(self, 1)
    }
}
#[doc = "When set to 1, mutes ih_i2cmphy_stat0\\[0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_i2cmphy_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_i2cmphy_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhMuteI2cmphyStat0Spec;
impl crate::RegisterSpec for IhMuteI2cmphyStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_mute_i2cmphy_stat0::R`](R) reader structure"]
impl crate::Readable for IhMuteI2cmphyStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_mute_i2cmphy_stat0::W`](W) writer structure"]
impl crate::Writable for IhMuteI2cmphyStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IH_MUTE_I2CMPHY_STAT0 to value 0"]
impl crate::Resettable for IhMuteI2cmphyStat0Spec {
    const RESET_VALUE: u8 = 0;
}
