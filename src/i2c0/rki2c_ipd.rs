#[doc = "Register `RKI2C_IPD` reader"]
pub type R = crate::R<Rki2cIpdSpec>;
#[doc = "Register `RKI2C_IPD` writer"]
pub type W = crate::W<Rki2cIpdSpec>;
#[doc = "byte tx finished interrupt pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Btfipd {
    #[doc = "0: byte tx finished interrupt appear, write 1 to clear"]
    B0 = 0,
    #[doc = "1: byte tx finished interrupt appear, write 1 to clear"]
    B1 = 1,
}
impl From<Btfipd> for bool {
    #[inline(always)]
    fn from(variant: Btfipd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTFIPD` reader - byte tx finished interrupt pending bit"]
pub type BtfipdR = crate::BitReader<Btfipd>;
impl BtfipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Btfipd {
        match self.bits {
            false => Btfipd::B0,
            true => Btfipd::B1,
        }
    }
    #[doc = "byte tx finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Btfipd::B0
    }
    #[doc = "byte tx finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Btfipd::B1
    }
}
#[doc = "Field `BTFIPD` writer - byte tx finished interrupt pending bit"]
pub type BtfipdW<'a, REG> = crate::BitWriter1C<'a, REG, Btfipd>;
impl<'a, REG> BtfipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "byte tx finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Btfipd::B0)
    }
    #[doc = "byte tx finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Btfipd::B1)
    }
}
#[doc = "byte rx finished interrupt pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brfipd {
    #[doc = "0: byte rx finished interrupt appear, write 1 to clear"]
    B0 = 0,
    #[doc = "1: byte rx finished interrupt appear, write 1 to clear"]
    B1 = 1,
}
impl From<Brfipd> for bool {
    #[inline(always)]
    fn from(variant: Brfipd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRFIPD` reader - byte rx finished interrupt pending bit"]
pub type BrfipdR = crate::BitReader<Brfipd>;
impl BrfipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brfipd {
        match self.bits {
            false => Brfipd::B0,
            true => Brfipd::B1,
        }
    }
    #[doc = "byte rx finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Brfipd::B0
    }
    #[doc = "byte rx finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Brfipd::B1
    }
}
#[doc = "Field `BRFIPD` writer - byte rx finished interrupt pending bit"]
pub type BrfipdW<'a, REG> = crate::BitWriter1C<'a, REG, Brfipd>;
impl<'a, REG> BrfipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "byte rx finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Brfipd::B0)
    }
    #[doc = "byte rx finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Brfipd::B1)
    }
}
#[doc = "MTXCNT data transfer finished interrupt pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbtfipd {
    #[doc = "0: MTXCNT data transfer finished interrupt appear, write 1 to clear"]
    B0 = 0,
    #[doc = "1: MTXCNT data transfer finished interrupt appear, write 1 to clear"]
    B1 = 1,
}
impl From<Mbtfipd> for bool {
    #[inline(always)]
    fn from(variant: Mbtfipd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBTFIPD` reader - MTXCNT data transfer finished interrupt pending bit"]
pub type MbtfipdR = crate::BitReader<Mbtfipd>;
impl MbtfipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbtfipd {
        match self.bits {
            false => Mbtfipd::B0,
            true => Mbtfipd::B1,
        }
    }
    #[doc = "MTXCNT data transfer finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Mbtfipd::B0
    }
    #[doc = "MTXCNT data transfer finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Mbtfipd::B1
    }
}
#[doc = "Field `MBTFIPD` writer - MTXCNT data transfer finished interrupt pending bit"]
pub type MbtfipdW<'a, REG> = crate::BitWriter1C<'a, REG, Mbtfipd>;
impl<'a, REG> MbtfipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MTXCNT data transfer finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Mbtfipd::B0)
    }
    #[doc = "MTXCNT data transfer finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Mbtfipd::B1)
    }
}
#[doc = "MRXCNT data received finished interrupt pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbrfipd {
    #[doc = "0: MRXCNT data received finished interrupt appear, write 1 to clear"]
    B0 = 0,
    #[doc = "1: MRXCNT data received finished interrupt appear, write 1 to clear"]
    B1 = 1,
}
impl From<Mbrfipd> for bool {
    #[inline(always)]
    fn from(variant: Mbrfipd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBRFIPD` reader - MRXCNT data received finished interrupt pending bit"]
pub type MbrfipdR = crate::BitReader<Mbrfipd>;
impl MbrfipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbrfipd {
        match self.bits {
            false => Mbrfipd::B0,
            true => Mbrfipd::B1,
        }
    }
    #[doc = "MRXCNT data received finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Mbrfipd::B0
    }
    #[doc = "MRXCNT data received finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Mbrfipd::B1
    }
}
#[doc = "Field `MBRFIPD` writer - MRXCNT data received finished interrupt pending bit"]
pub type MbrfipdW<'a, REG> = crate::BitWriter1C<'a, REG, Mbrfipd>;
impl<'a, REG> MbrfipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MRXCNT data received finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Mbrfipd::B0)
    }
    #[doc = "MRXCNT data received finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Mbrfipd::B1)
    }
}
#[doc = "start operation finished interrupt pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Startipd {
    #[doc = "0: start operation finished interrupt appear, write 1 to clear"]
    B0 = 0,
    #[doc = "1: start operation finished interrupt appear, write 1 to clear"]
    B1 = 1,
}
impl From<Startipd> for bool {
    #[inline(always)]
    fn from(variant: Startipd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTIPD` reader - start operation finished interrupt pending bit"]
pub type StartipdR = crate::BitReader<Startipd>;
impl StartipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startipd {
        match self.bits {
            false => Startipd::B0,
            true => Startipd::B1,
        }
    }
    #[doc = "start operation finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Startipd::B0
    }
    #[doc = "start operation finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Startipd::B1
    }
}
#[doc = "Field `STARTIPD` writer - start operation finished interrupt pending bit"]
pub type StartipdW<'a, REG> = crate::BitWriter1C<'a, REG, Startipd>;
impl<'a, REG> StartipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "start operation finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Startipd::B0)
    }
    #[doc = "start operation finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Startipd::B1)
    }
}
#[doc = "stop operation finished interrupt pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopipd {
    #[doc = "0: stop operation finished interrupt appear, write 1 to clear"]
    B0 = 0,
    #[doc = "1: stop operation finished interrupt appear, write 1 to clear"]
    B1 = 1,
}
impl From<Stopipd> for bool {
    #[inline(always)]
    fn from(variant: Stopipd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPIPD` reader - stop operation finished interrupt pending bit"]
pub type StopipdR = crate::BitReader<Stopipd>;
impl StopipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopipd {
        match self.bits {
            false => Stopipd::B0,
            true => Stopipd::B1,
        }
    }
    #[doc = "stop operation finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stopipd::B0
    }
    #[doc = "stop operation finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stopipd::B1
    }
}
#[doc = "Field `STOPIPD` writer - stop operation finished interrupt pending bit"]
pub type StopipdW<'a, REG> = crate::BitWriter1C<'a, REG, Stopipd>;
impl<'a, REG> StopipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "stop operation finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stopipd::B0)
    }
    #[doc = "stop operation finished interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stopipd::B1)
    }
}
#[doc = "NAK handshake received interrupt pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nakrcvipd {
    #[doc = "0: NAK handshake received interrupt appear, write 1 to clear"]
    B0 = 0,
    #[doc = "1: NAK handshake received interrupt appear, write 1 to clear"]
    B1 = 1,
}
impl From<Nakrcvipd> for bool {
    #[inline(always)]
    fn from(variant: Nakrcvipd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAKRCVIPD` reader - NAK handshake received interrupt pending bit"]
pub type NakrcvipdR = crate::BitReader<Nakrcvipd>;
impl NakrcvipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nakrcvipd {
        match self.bits {
            false => Nakrcvipd::B0,
            true => Nakrcvipd::B1,
        }
    }
    #[doc = "NAK handshake received interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Nakrcvipd::B0
    }
    #[doc = "NAK handshake received interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Nakrcvipd::B1
    }
}
#[doc = "Field `NAKRCVIPD` writer - NAK handshake received interrupt pending bit"]
pub type NakrcvipdW<'a, REG> = crate::BitWriter1C<'a, REG, Nakrcvipd>;
impl<'a, REG> NakrcvipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NAK handshake received interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Nakrcvipd::B0)
    }
    #[doc = "NAK handshake received interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Nakrcvipd::B1)
    }
}
#[doc = "slave hold scl interrupt pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slavehdsclipd {
    #[doc = "0: slave hold scl interrupt appear, write 1 to clear"]
    B0 = 0,
    #[doc = "1: slave hold scl interrupt appear, write 1 to clear"]
    B1 = 1,
}
impl From<Slavehdsclipd> for bool {
    #[inline(always)]
    fn from(variant: Slavehdsclipd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVEHDSCLIPD` reader - slave hold scl interrupt pending bit"]
pub type SlavehdsclipdR = crate::BitReader<Slavehdsclipd>;
impl SlavehdsclipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slavehdsclipd {
        match self.bits {
            false => Slavehdsclipd::B0,
            true => Slavehdsclipd::B1,
        }
    }
    #[doc = "slave hold scl interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Slavehdsclipd::B0
    }
    #[doc = "slave hold scl interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Slavehdsclipd::B1
    }
}
#[doc = "Field `SLAVEHDSCLIPD` writer - slave hold scl interrupt pending bit"]
pub type SlavehdsclipdW<'a, REG> = crate::BitWriter<'a, REG, Slavehdsclipd>;
impl<'a, REG> SlavehdsclipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "slave hold scl interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Slavehdsclipd::B0)
    }
    #[doc = "slave hold scl interrupt appear, write 1 to clear"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Slavehdsclipd::B1)
    }
}
impl R {
    #[doc = "Bit 0 - byte tx finished interrupt pending bit"]
    #[inline(always)]
    pub fn btfipd(&self) -> BtfipdR {
        BtfipdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - byte rx finished interrupt pending bit"]
    #[inline(always)]
    pub fn brfipd(&self) -> BrfipdR {
        BrfipdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MTXCNT data transfer finished interrupt pending bit"]
    #[inline(always)]
    pub fn mbtfipd(&self) -> MbtfipdR {
        MbtfipdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MRXCNT data received finished interrupt pending bit"]
    #[inline(always)]
    pub fn mbrfipd(&self) -> MbrfipdR {
        MbrfipdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - start operation finished interrupt pending bit"]
    #[inline(always)]
    pub fn startipd(&self) -> StartipdR {
        StartipdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - stop operation finished interrupt pending bit"]
    #[inline(always)]
    pub fn stopipd(&self) -> StopipdR {
        StopipdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NAK handshake received interrupt pending bit"]
    #[inline(always)]
    pub fn nakrcvipd(&self) -> NakrcvipdR {
        NakrcvipdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - slave hold scl interrupt pending bit"]
    #[inline(always)]
    pub fn slavehdsclipd(&self) -> SlavehdsclipdR {
        SlavehdsclipdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - byte tx finished interrupt pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn btfipd(&mut self) -> BtfipdW<Rki2cIpdSpec> {
        BtfipdW::new(self, 0)
    }
    #[doc = "Bit 1 - byte rx finished interrupt pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn brfipd(&mut self) -> BrfipdW<Rki2cIpdSpec> {
        BrfipdW::new(self, 1)
    }
    #[doc = "Bit 2 - MTXCNT data transfer finished interrupt pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn mbtfipd(&mut self) -> MbtfipdW<Rki2cIpdSpec> {
        MbtfipdW::new(self, 2)
    }
    #[doc = "Bit 3 - MRXCNT data received finished interrupt pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn mbrfipd(&mut self) -> MbrfipdW<Rki2cIpdSpec> {
        MbrfipdW::new(self, 3)
    }
    #[doc = "Bit 4 - start operation finished interrupt pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn startipd(&mut self) -> StartipdW<Rki2cIpdSpec> {
        StartipdW::new(self, 4)
    }
    #[doc = "Bit 5 - stop operation finished interrupt pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn stopipd(&mut self) -> StopipdW<Rki2cIpdSpec> {
        StopipdW::new(self, 5)
    }
    #[doc = "Bit 6 - NAK handshake received interrupt pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn nakrcvipd(&mut self) -> NakrcvipdW<Rki2cIpdSpec> {
        NakrcvipdW::new(self, 6)
    }
    #[doc = "Bit 7 - slave hold scl interrupt pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn slavehdsclipd(&mut self) -> SlavehdsclipdW<Rki2cIpdSpec> {
        SlavehdsclipdW::new(self, 7)
    }
}
#[doc = "interrupt pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_ipd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_ipd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cIpdSpec;
impl crate::RegisterSpec for Rki2cIpdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_ipd::R`](R) reader structure"]
impl crate::Readable for Rki2cIpdSpec {}
#[doc = "`write(|w| ..)` method takes [`rki2c_ipd::W`](W) writer structure"]
impl crate::Writable for Rki2cIpdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7f;
}
#[doc = "`reset()` method sets RKI2C_IPD to value 0"]
impl crate::Resettable for Rki2cIpdSpec {
    const RESET_VALUE: u32 = 0;
}
