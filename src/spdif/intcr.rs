#[doc = "Register `INTCR` reader"]
pub type R = crate::R<IntcrSpec>;
#[doc = "Register `INTCR` writer"]
pub type W = crate::W<IntcrSpec>;
#[doc = "User Data Interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Udtie {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable If enabled, an interrupt will be asserted when the content of the user data register is fed into the corresponding shadow register"]
    B1 = 1,
}
impl From<Udtie> for bool {
    #[inline(always)]
    fn from(variant: Udtie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDTIE` reader - User Data Interrupt"]
pub type UdtieR = crate::BitReader<Udtie>;
impl UdtieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Udtie {
        match self.bits {
            false => Udtie::B0,
            true => Udtie::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Udtie::B0
    }
    #[doc = "enable If enabled, an interrupt will be asserted when the content of the user data register is fed into the corresponding shadow register"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Udtie::B1
    }
}
#[doc = "Field `UDTIE` writer - User Data Interrupt"]
pub type UdtieW<'a, REG> = crate::BitWriter<'a, REG, Udtie>;
impl<'a, REG> UdtieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Udtie::B0)
    }
    #[doc = "enable If enabled, an interrupt will be asserted when the content of the user data register is fed into the corresponding shadow register"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Udtie::B1)
    }
}
#[doc = "Block transfer/repetition period end interrupt enable\n\nWhen enabled, an interrupt will be asserted when the block\n\ntransfer is finished if the channel conveys linear PCM or when the\n\nrepetition period is reached if the channel conveys non-linear\n\nPCM.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bttie {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Bttie> for bool {
    #[inline(always)]
    fn from(variant: Bttie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTTIE` reader - Block transfer/repetition period end interrupt enable\n\nWhen enabled, an interrupt will be asserted when the block\n\ntransfer is finished if the channel conveys linear PCM or when the\n\nrepetition period is reached if the channel conveys non-linear\n\nPCM."]
pub type BttieR = crate::BitReader<Bttie>;
impl BttieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bttie {
        match self.bits {
            false => Bttie::B0,
            true => Bttie::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bttie::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bttie::B1
    }
}
#[doc = "Field `BTTIE` writer - Block transfer/repetition period end interrupt enable\n\nWhen enabled, an interrupt will be asserted when the block\n\ntransfer is finished if the channel conveys linear PCM or when the\n\nrepetition period is reached if the channel conveys non-linear\n\nPCM."]
pub type BttieW<'a, REG> = crate::BitWriter<'a, REG, Bttie>;
impl<'a, REG> BttieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bttie::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bttie::B1)
    }
}
#[doc = "Sample Date Buffer empty interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdbeie {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Sdbeie> for bool {
    #[inline(always)]
    fn from(variant: Sdbeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDBEIE` reader - Sample Date Buffer empty interrupt enable"]
pub type SdbeieR = crate::BitReader<Sdbeie>;
impl SdbeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdbeie {
        match self.bits {
            false => Sdbeie::B0,
            true => Sdbeie::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sdbeie::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sdbeie::B1
    }
}
#[doc = "Field `SDBEIE` writer - Sample Date Buffer empty interrupt enable"]
pub type SdbeieW<'a, REG> = crate::BitWriter<'a, REG, Sdbeie>;
impl<'a, REG> SdbeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbeie::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbeie::B1)
    }
}
#[doc = "Field `SDBT` reader - Sample Date Buffer Threshold\n\nSample Date Buffer Threshold for empty interrupt"]
pub type SdbtR = crate::FieldReader;
#[doc = "Field `SDBT` writer - Sample Date Buffer Threshold\n\nSample Date Buffer Threshold for empty interrupt"]
pub type SdbtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BTTIC` reader - Block/Data burst transfer finish interrupt clear\n\nWrite 1 to clear the interrupt."]
pub type BtticR = crate::BitReader;
#[doc = "Field `BTTIC` writer - Block/Data burst transfer finish interrupt clear\n\nWrite 1 to clear the interrupt."]
pub type BtticW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UDTIC` reader - User Data Interrupt Clear\n\nWrite '1' to clear the user data interrupt."]
pub type UdticR = crate::BitReader;
#[doc = "Field `UDTIC` writer - User Data Interrupt Clear\n\nWrite '1' to clear the user data interrupt."]
pub type UdticW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 2 - User Data Interrupt"]
    #[inline(always)]
    pub fn udtie(&self) -> UdtieR {
        UdtieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Block transfer/repetition period end interrupt enable\n\nWhen enabled, an interrupt will be asserted when the block\n\ntransfer is finished if the channel conveys linear PCM or when the\n\nrepetition period is reached if the channel conveys non-linear\n\nPCM."]
    #[inline(always)]
    pub fn bttie(&self) -> BttieR {
        BttieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sample Date Buffer empty interrupt enable"]
    #[inline(always)]
    pub fn sdbeie(&self) -> SdbeieR {
        SdbeieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:9 - Sample Date Buffer Threshold\n\nSample Date Buffer Threshold for empty interrupt"]
    #[inline(always)]
    pub fn sdbt(&self) -> SdbtR {
        SdbtR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Block/Data burst transfer finish interrupt clear\n\nWrite 1 to clear the interrupt."]
    #[inline(always)]
    pub fn bttic(&self) -> BtticR {
        BtticR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - User Data Interrupt Clear\n\nWrite '1' to clear the user data interrupt."]
    #[inline(always)]
    pub fn udtic(&self) -> UdticR {
        UdticR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - User Data Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn udtie(&mut self) -> UdtieW<IntcrSpec> {
        UdtieW::new(self, 2)
    }
    #[doc = "Bit 3 - Block transfer/repetition period end interrupt enable\n\nWhen enabled, an interrupt will be asserted when the block\n\ntransfer is finished if the channel conveys linear PCM or when the\n\nrepetition period is reached if the channel conveys non-linear\n\nPCM."]
    #[inline(always)]
    #[must_use]
    pub fn bttie(&mut self) -> BttieW<IntcrSpec> {
        BttieW::new(self, 3)
    }
    #[doc = "Bit 4 - Sample Date Buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdbeie(&mut self) -> SdbeieW<IntcrSpec> {
        SdbeieW::new(self, 4)
    }
    #[doc = "Bits 5:9 - Sample Date Buffer Threshold\n\nSample Date Buffer Threshold for empty interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sdbt(&mut self) -> SdbtW<IntcrSpec> {
        SdbtW::new(self, 5)
    }
    #[doc = "Bit 16 - Block/Data burst transfer finish interrupt clear\n\nWrite 1 to clear the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn bttic(&mut self) -> BtticW<IntcrSpec> {
        BtticW::new(self, 16)
    }
    #[doc = "Bit 17 - User Data Interrupt Clear\n\nWrite '1' to clear the user data interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn udtic(&mut self) -> UdticW<IntcrSpec> {
        UdticW::new(self, 17)
    }
}
#[doc = "Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcrSpec;
impl crate::RegisterSpec for IntcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intcr::R`](R) reader structure"]
impl crate::Readable for IntcrSpec {}
#[doc = "`write(|w| ..)` method takes [`intcr::W`](W) writer structure"]
impl crate::Writable for IntcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0003_0000;
}
#[doc = "`reset()` method sets INTCR to value 0"]
impl crate::Resettable for IntcrSpec {
    const RESET_VALUE: u32 = 0;
}
