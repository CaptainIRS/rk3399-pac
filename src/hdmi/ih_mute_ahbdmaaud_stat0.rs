#[doc = "Register `IH_MUTE_AHBDMAAUD_STAT0` reader"]
pub type R = crate::R<IhMuteAhbdmaaudStat0Spec>;
#[doc = "Register `IH_MUTE_AHBDMAAUD_STAT0` writer"]
pub type W = crate::W<IhMuteAhbdmaaudStat0Spec>;
#[doc = "Field `AHBDMAAUD_INTBUFFEMPTY` reader - When set to 1, mutes ih_ahbdmaaud_stat0\\[0\\]"]
pub type AhbdmaaudIntbuffemptyR = crate::BitReader;
#[doc = "Field `AHBDMAAUD_INTBUFFEMPTY` writer - When set to 1, mutes ih_ahbdmaaud_stat0\\[0\\]"]
pub type AhbdmaaudIntbuffemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBDMAAUD_INTBUFFFULL` reader - When set to 1, mutes ih_ahbdmaaud_stat0\\[1\\]"]
pub type AhbdmaaudIntbufffullR = crate::BitReader;
#[doc = "Field `AHBDMAAUD_INTBUFFFULL` writer - When set to 1, mutes ih_ahbdmaaud_stat0\\[1\\]"]
pub type AhbdmaaudIntbufffullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBDMAAUD_INTDONE` reader - When set to 1, mutes ih_ahbdmaaud_stat0\\[2\\]"]
pub type AhbdmaaudIntdoneR = crate::BitReader;
#[doc = "Field `AHBDMAAUD_INTDONE` writer - When set to 1, mutes ih_ahbdmaaud_stat0\\[2\\]"]
pub type AhbdmaaudIntdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBDMAAUD_INTRETRYSPLIT` reader - When set to 1, mutes ih_ahbdmaaud_stat0\\[3\\]"]
pub type AhbdmaaudIntretrysplitR = crate::BitReader;
#[doc = "Field `AHBDMAAUD_INTRETRYSPLIT` writer - When set to 1, mutes ih_ahbdmaaud_stat0\\[3\\]"]
pub type AhbdmaaudIntretrysplitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBDMAAUD_INTLOSTOWNERSHIP` reader - When set to 1, mutes ih_ahbdmaaud_stat0\\[4\\]"]
pub type AhbdmaaudIntlostownershipR = crate::BitReader;
#[doc = "Field `AHBDMAAUD_INTLOSTOWNERSHIP` writer - When set to 1, mutes ih_ahbdmaaud_stat0\\[4\\]"]
pub type AhbdmaaudIntlostownershipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBDMAAUD_INTERROR` reader - When set to 1, mutes ih_ahbdmaaud_stat0\\[5\\]"]
pub type AhbdmaaudInterrorR = crate::BitReader;
#[doc = "Field `AHBDMAAUD_INTERROR` writer - When set to 1, mutes ih_ahbdmaaud_stat0\\[5\\]"]
pub type AhbdmaaudInterrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBDMAAUD_INTBUFFOVERRUN` reader - When set to 1, mutes ih_ahbdmaaud_stat0\\[6\\]"]
pub type AhbdmaaudIntbuffoverrunR = crate::BitReader;
#[doc = "Field `AHBDMAAUD_INTBUFFOVERRUN` writer - When set to 1, mutes ih_ahbdmaaud_stat0\\[6\\]"]
pub type AhbdmaaudIntbuffoverrunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, mutes ih_ahbdmaaud_stat0\\[0\\]"]
    #[inline(always)]
    pub fn ahbdmaaud_intbuffempty(&self) -> AhbdmaaudIntbuffemptyR {
        AhbdmaaudIntbuffemptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_ahbdmaaud_stat0\\[1\\]"]
    #[inline(always)]
    pub fn ahbdmaaud_intbufffull(&self) -> AhbdmaaudIntbufffullR {
        AhbdmaaudIntbufffullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_ahbdmaaud_stat0\\[2\\]"]
    #[inline(always)]
    pub fn ahbdmaaud_intdone(&self) -> AhbdmaaudIntdoneR {
        AhbdmaaudIntdoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_ahbdmaaud_stat0\\[3\\]"]
    #[inline(always)]
    pub fn ahbdmaaud_intretrysplit(&self) -> AhbdmaaudIntretrysplitR {
        AhbdmaaudIntretrysplitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_ahbdmaaud_stat0\\[4\\]"]
    #[inline(always)]
    pub fn ahbdmaaud_intlostownership(&self) -> AhbdmaaudIntlostownershipR {
        AhbdmaaudIntlostownershipR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set to 1, mutes ih_ahbdmaaud_stat0\\[5\\]"]
    #[inline(always)]
    pub fn ahbdmaaud_interror(&self) -> AhbdmaaudInterrorR {
        AhbdmaaudInterrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set to 1, mutes ih_ahbdmaaud_stat0\\[6\\]"]
    #[inline(always)]
    pub fn ahbdmaaud_intbuffoverrun(&self) -> AhbdmaaudIntbuffoverrunR {
        AhbdmaaudIntbuffoverrunR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, mutes ih_ahbdmaaud_stat0\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdmaaud_intbuffempty(&mut self) -> AhbdmaaudIntbuffemptyW<IhMuteAhbdmaaudStat0Spec> {
        AhbdmaaudIntbuffemptyW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_ahbdmaaud_stat0\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdmaaud_intbufffull(&mut self) -> AhbdmaaudIntbufffullW<IhMuteAhbdmaaudStat0Spec> {
        AhbdmaaudIntbufffullW::new(self, 1)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_ahbdmaaud_stat0\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdmaaud_intdone(&mut self) -> AhbdmaaudIntdoneW<IhMuteAhbdmaaudStat0Spec> {
        AhbdmaaudIntdoneW::new(self, 2)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_ahbdmaaud_stat0\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdmaaud_intretrysplit(&mut self) -> AhbdmaaudIntretrysplitW<IhMuteAhbdmaaudStat0Spec> {
        AhbdmaaudIntretrysplitW::new(self, 3)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_ahbdmaaud_stat0\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdmaaud_intlostownership(
        &mut self,
    ) -> AhbdmaaudIntlostownershipW<IhMuteAhbdmaaudStat0Spec> {
        AhbdmaaudIntlostownershipW::new(self, 4)
    }
    #[doc = "Bit 5 - When set to 1, mutes ih_ahbdmaaud_stat0\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdmaaud_interror(&mut self) -> AhbdmaaudInterrorW<IhMuteAhbdmaaudStat0Spec> {
        AhbdmaaudInterrorW::new(self, 5)
    }
    #[doc = "Bit 6 - When set to 1, mutes ih_ahbdmaaud_stat0\\[6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdmaaud_intbuffoverrun(
        &mut self,
    ) -> AhbdmaaudIntbuffoverrunW<IhMuteAhbdmaaudStat0Spec> {
        AhbdmaaudIntbuffoverrunW::new(self, 6)
    }
}
#[doc = "AHB Audio DMA Interrupt Mute Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_ahbdmaaud_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_ahbdmaaud_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhMuteAhbdmaaudStat0Spec;
impl crate::RegisterSpec for IhMuteAhbdmaaudStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_mute_ahbdmaaud_stat0::R`](R) reader structure"]
impl crate::Readable for IhMuteAhbdmaaudStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_mute_ahbdmaaud_stat0::W`](W) writer structure"]
impl crate::Writable for IhMuteAhbdmaaudStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IH_MUTE_AHBDMAAUD_STAT0 to value 0x40"]
impl crate::Resettable for IhMuteAhbdmaaudStat0Spec {
    const RESET_VALUE: u8 = 0x40;
}
