#[doc = "Register `RKI2C_IEN` reader"]
pub type R = crate::R<Rki2cIenSpec>;
#[doc = "Register `RKI2C_IEN` writer"]
pub type W = crate::W<Rki2cIenSpec>;
#[doc = "byte tx finished interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Btfien {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Btfien> for bool {
    #[inline(always)]
    fn from(variant: Btfien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTFIEN` reader - byte tx finished interrupt enable"]
pub type BtfienR = crate::BitReader<Btfien>;
impl BtfienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Btfien {
        match self.bits {
            false => Btfien::B0,
            true => Btfien::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Btfien::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Btfien::B1
    }
}
#[doc = "Field `BTFIEN` writer - byte tx finished interrupt enable"]
pub type BtfienW<'a, REG> = crate::BitWriter<'a, REG, Btfien>;
impl<'a, REG> BtfienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Btfien::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Btfien::B1)
    }
}
#[doc = "byte rx finished interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brfien {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Brfien> for bool {
    #[inline(always)]
    fn from(variant: Brfien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRFIEN` reader - byte rx finished interrupt enable"]
pub type BrfienR = crate::BitReader<Brfien>;
impl BrfienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brfien {
        match self.bits {
            false => Brfien::B0,
            true => Brfien::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Brfien::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Brfien::B1
    }
}
#[doc = "Field `BRFIEN` writer - byte rx finished interrupt enable"]
pub type BrfienW<'a, REG> = crate::BitWriter<'a, REG, Brfien>;
impl<'a, REG> BrfienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Brfien::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Brfien::B1)
    }
}
#[doc = "MTXCNT data transfer finished interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbtfien {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Mbtfien> for bool {
    #[inline(always)]
    fn from(variant: Mbtfien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBTFIEN` reader - MTXCNT data transfer finished interrupt enable"]
pub type MbtfienR = crate::BitReader<Mbtfien>;
impl MbtfienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbtfien {
        match self.bits {
            false => Mbtfien::B0,
            true => Mbtfien::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Mbtfien::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Mbtfien::B1
    }
}
#[doc = "Field `MBTFIEN` writer - MTXCNT data transfer finished interrupt enable"]
pub type MbtfienW<'a, REG> = crate::BitWriter<'a, REG, Mbtfien>;
impl<'a, REG> MbtfienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Mbtfien::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Mbtfien::B1)
    }
}
#[doc = "MRXCNT data received finished interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbrfien {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Mbrfien> for bool {
    #[inline(always)]
    fn from(variant: Mbrfien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBRFIEN` reader - MRXCNT data received finished interrupt enable"]
pub type MbrfienR = crate::BitReader<Mbrfien>;
impl MbrfienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbrfien {
        match self.bits {
            false => Mbrfien::B0,
            true => Mbrfien::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Mbrfien::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Mbrfien::B1
    }
}
#[doc = "Field `MBRFIEN` writer - MRXCNT data received finished interrupt enable"]
pub type MbrfienW<'a, REG> = crate::BitWriter<'a, REG, Mbrfien>;
impl<'a, REG> MbrfienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Mbrfien::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Mbrfien::B1)
    }
}
#[doc = "start operation finished interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Startien {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Startien> for bool {
    #[inline(always)]
    fn from(variant: Startien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTIEN` reader - start operation finished interrupt enable"]
pub type StartienR = crate::BitReader<Startien>;
impl StartienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startien {
        match self.bits {
            false => Startien::B0,
            true => Startien::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Startien::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Startien::B1
    }
}
#[doc = "Field `STARTIEN` writer - start operation finished interrupt enable"]
pub type StartienW<'a, REG> = crate::BitWriter<'a, REG, Startien>;
impl<'a, REG> StartienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Startien::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Startien::B1)
    }
}
#[doc = "stop operation finished interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopien {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Stopien> for bool {
    #[inline(always)]
    fn from(variant: Stopien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPIEN` reader - stop operation finished interrupt enable"]
pub type StopienR = crate::BitReader<Stopien>;
impl StopienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopien {
        match self.bits {
            false => Stopien::B0,
            true => Stopien::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stopien::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stopien::B1
    }
}
#[doc = "Field `STOPIEN` writer - stop operation finished interrupt enable"]
pub type StopienW<'a, REG> = crate::BitWriter<'a, REG, Stopien>;
impl<'a, REG> StopienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stopien::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stopien::B1)
    }
}
#[doc = "NAK handshake received interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nakrcvien {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Nakrcvien> for bool {
    #[inline(always)]
    fn from(variant: Nakrcvien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAKRCVIEN` reader - NAK handshake received interrupt enable"]
pub type NakrcvienR = crate::BitReader<Nakrcvien>;
impl NakrcvienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nakrcvien {
        match self.bits {
            false => Nakrcvien::B0,
            true => Nakrcvien::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Nakrcvien::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Nakrcvien::B1
    }
}
#[doc = "Field `NAKRCVIEN` writer - NAK handshake received interrupt enable"]
pub type NakrcvienW<'a, REG> = crate::BitWriter<'a, REG, Nakrcvien>;
impl<'a, REG> NakrcvienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Nakrcvien::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Nakrcvien::B1)
    }
}
#[doc = "slave hold scl interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slavehdsclen {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Slavehdsclen> for bool {
    #[inline(always)]
    fn from(variant: Slavehdsclen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVEHDSCLEN` reader - slave hold scl interrupt enable"]
pub type SlavehdsclenR = crate::BitReader<Slavehdsclen>;
impl SlavehdsclenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slavehdsclen {
        match self.bits {
            false => Slavehdsclen::B0,
            true => Slavehdsclen::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Slavehdsclen::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Slavehdsclen::B1
    }
}
#[doc = "Field `SLAVEHDSCLEN` writer - slave hold scl interrupt enable"]
pub type SlavehdsclenW<'a, REG> = crate::BitWriter<'a, REG, Slavehdsclen>;
impl<'a, REG> SlavehdsclenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Slavehdsclen::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Slavehdsclen::B1)
    }
}
impl R {
    #[doc = "Bit 0 - byte tx finished interrupt enable"]
    #[inline(always)]
    pub fn btfien(&self) -> BtfienR {
        BtfienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - byte rx finished interrupt enable"]
    #[inline(always)]
    pub fn brfien(&self) -> BrfienR {
        BrfienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MTXCNT data transfer finished interrupt enable"]
    #[inline(always)]
    pub fn mbtfien(&self) -> MbtfienR {
        MbtfienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MRXCNT data received finished interrupt enable"]
    #[inline(always)]
    pub fn mbrfien(&self) -> MbrfienR {
        MbrfienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - start operation finished interrupt enable"]
    #[inline(always)]
    pub fn startien(&self) -> StartienR {
        StartienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - stop operation finished interrupt enable"]
    #[inline(always)]
    pub fn stopien(&self) -> StopienR {
        StopienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NAK handshake received interrupt enable"]
    #[inline(always)]
    pub fn nakrcvien(&self) -> NakrcvienR {
        NakrcvienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - slave hold scl interrupt enable"]
    #[inline(always)]
    pub fn slavehdsclen(&self) -> SlavehdsclenR {
        SlavehdsclenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - byte tx finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn btfien(&mut self) -> BtfienW<Rki2cIenSpec> {
        BtfienW::new(self, 0)
    }
    #[doc = "Bit 1 - byte rx finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn brfien(&mut self) -> BrfienW<Rki2cIenSpec> {
        BrfienW::new(self, 1)
    }
    #[doc = "Bit 2 - MTXCNT data transfer finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn mbtfien(&mut self) -> MbtfienW<Rki2cIenSpec> {
        MbtfienW::new(self, 2)
    }
    #[doc = "Bit 3 - MRXCNT data received finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn mbrfien(&mut self) -> MbrfienW<Rki2cIenSpec> {
        MbrfienW::new(self, 3)
    }
    #[doc = "Bit 4 - start operation finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn startien(&mut self) -> StartienW<Rki2cIenSpec> {
        StartienW::new(self, 4)
    }
    #[doc = "Bit 5 - stop operation finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stopien(&mut self) -> StopienW<Rki2cIenSpec> {
        StopienW::new(self, 5)
    }
    #[doc = "Bit 6 - NAK handshake received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakrcvien(&mut self) -> NakrcvienW<Rki2cIenSpec> {
        NakrcvienW::new(self, 6)
    }
    #[doc = "Bit 7 - slave hold scl interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn slavehdsclen(&mut self) -> SlavehdsclenW<Rki2cIenSpec> {
        SlavehdsclenW::new(self, 7)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cIenSpec;
impl crate::RegisterSpec for Rki2cIenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_ien::R`](R) reader structure"]
impl crate::Readable for Rki2cIenSpec {}
#[doc = "`write(|w| ..)` method takes [`rki2c_ien::W`](W) writer structure"]
impl crate::Writable for Rki2cIenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RKI2C_IEN to value 0"]
impl crate::Resettable for Rki2cIenSpec {
    const RESET_VALUE: u32 = 0;
}
