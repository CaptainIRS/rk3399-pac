#[doc = "Register `RKI2C_CON` reader"]
pub type R = crate::R<Rki2cConSpec>;
#[doc = "Register `RKI2C_CON` writer"]
pub type W = crate::W<Rki2cConSpec>;
#[doc = "i2c module enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cEn {
    #[doc = "0: not enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<I2cEn> for bool {
    #[inline(always)]
    fn from(variant: I2cEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_EN` reader - i2c module enable"]
pub type I2cEnR = crate::BitReader<I2cEn>;
impl I2cEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cEn {
        match self.bits {
            false => I2cEn::B0,
            true => I2cEn::B1,
        }
    }
    #[doc = "not enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == I2cEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == I2cEn::B1
    }
}
#[doc = "Field `I2C_EN` writer - i2c module enable"]
pub type I2cEnW<'a, REG> = crate::BitWriter<'a, REG, I2cEn>;
impl<'a, REG> I2cEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(I2cEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(I2cEn::B1)
    }
}
#[doc = "i2c mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2cMode {
    #[doc = "0: transmit only"]
    B00 = 0,
    #[doc = "1: transmit address (device + register address) --> restart - -> transmit address -> receive only"]
    B01 = 1,
    #[doc = "2: receive only"]
    B10 = 2,
    #[doc = "3: transmit address (device + register address, write/read bit is 1) --> restart --> transmit address (device address) --> receive data"]
    B11 = 3,
}
impl From<I2cMode> for u8 {
    #[inline(always)]
    fn from(variant: I2cMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2cMode {
    type Ux = u8;
}
#[doc = "Field `I2C_MODE` reader - i2c mode select"]
pub type I2cModeR = crate::FieldReader<I2cMode>;
impl I2cModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cMode {
        match self.bits {
            0 => I2cMode::B00,
            1 => I2cMode::B01,
            2 => I2cMode::B10,
            3 => I2cMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "transmit only"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == I2cMode::B00
    }
    #[doc = "transmit address (device + register address) --> restart - -> transmit address -> receive only"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == I2cMode::B01
    }
    #[doc = "receive only"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == I2cMode::B10
    }
    #[doc = "transmit address (device + register address, write/read bit is 1) --> restart --> transmit address (device address) --> receive data"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == I2cMode::B11
    }
}
#[doc = "Field `I2C_MODE` writer - i2c mode select"]
pub type I2cModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, I2cMode>;
impl<'a, REG> I2cModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "transmit only"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(I2cMode::B00)
    }
    #[doc = "transmit address (device + register address) --> restart - -> transmit address -> receive only"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(I2cMode::B01)
    }
    #[doc = "receive only"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(I2cMode::B10)
    }
    #[doc = "transmit address (device + register address, write/read bit is 1) --> restart --> transmit address (device address) --> receive data"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(I2cMode::B11)
    }
}
#[doc = "Field `START` reader - start enable\n\nstart enable, when this bit is written to 1, I2C will generate start\n\nsignal."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - start enable\n\nstart enable, when this bit is written to 1, I2C will generate start\n\nsignal."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - stop enable\n\nstop enable, when this bit is written to 1, I2C will generate stop\n\nsignal."]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - stop enable\n\nstop enable, when this bit is written to 1, I2C will generate stop\n\nsignal."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "last byte acknowledge control in master receive mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ack {
    #[doc = "0: ACK"]
    B0 = 0,
    #[doc = "1: NAK"]
    B1 = 1,
}
impl From<Ack> for bool {
    #[inline(always)]
    fn from(variant: Ack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACK` reader - last byte acknowledge control in master receive mode"]
pub type AckR = crate::BitReader<Ack>;
impl AckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ack {
        match self.bits {
            false => Ack::B0,
            true => Ack::B1,
        }
    }
    #[doc = "ACK"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ack::B0
    }
    #[doc = "NAK"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ack::B1
    }
}
#[doc = "Field `ACK` writer - last byte acknowledge control in master receive mode"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG, Ack>;
impl<'a, REG> AckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACK"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ack::B0)
    }
    #[doc = "NAK"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ack::B1)
    }
}
#[doc = "operation when NAK handshake is received\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act2nak {
    #[doc = "0: ignored"]
    B0 = 0,
    #[doc = "1: stop transaction"]
    B1 = 1,
}
impl From<Act2nak> for bool {
    #[inline(always)]
    fn from(variant: Act2nak) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACT2NAK` reader - operation when NAK handshake is received"]
pub type Act2nakR = crate::BitReader<Act2nak>;
impl Act2nakR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Act2nak {
        match self.bits {
            false => Act2nak::B0,
            true => Act2nak::B1,
        }
    }
    #[doc = "ignored"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Act2nak::B0
    }
    #[doc = "stop transaction"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Act2nak::B1
    }
}
#[doc = "Field `ACT2NAK` writer - operation when NAK handshake is received"]
pub type Act2nakW<'a, REG> = crate::BitWriter<'a, REG, Act2nak>;
impl<'a, REG> Act2nakW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ignored"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Act2nak::B0)
    }
    #[doc = "stop transaction"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Act2nak::B1)
    }
}
#[doc = "Field `DATA_UPD_ST` reader - SDA update point config\n\nUsed to config sda change state when scl is low, used to adjust\n\nsetup/hold time\n\n4'bn:Thold = (n + 1) * Tclk_i2c\n\nNote: 0 &lt;= n &lt;= 5"]
pub type DataUpdStR = crate::FieldReader;
#[doc = "Field `DATA_UPD_ST` writer - SDA update point config\n\nUsed to config sda change state when scl is low, used to adjust\n\nsetup/hold time\n\n4'bn:Thold = (n + 1) * Tclk_i2c\n\nNote: 0 &lt;= n &lt;= 5"]
pub type DataUpdStW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `START_SETUP` reader - start setup config\n\nTSU;sta = (start_setup + 1) * T(SCL_HIGH) + Tclk_i2c\n\nTHD;sta = (start_setup + 2) * T(SCL_HIGH) - Tclk_i2c"]
pub type StartSetupR = crate::FieldReader;
#[doc = "Field `START_SETUP` writer - start setup config\n\nTSU;sta = (start_setup + 1) * T(SCL_HIGH) + Tclk_i2c\n\nTHD;sta = (start_setup + 2) * T(SCL_HIGH) - Tclk_i2c"]
pub type StartSetupW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP_SETUP` reader - staop setup config\n\nTSU;sto = (stop_setup + 1) * T(SCL_HIGH) + Tclk_i2c"]
pub type StopSetupR = crate::FieldReader;
#[doc = "Field `STOP_SETUP` writer - staop setup config\n\nTSU;sto = (stop_setup + 1) * T(SCL_HIGH) + Tclk_i2c"]
pub type StopSetupW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VERSION` reader - rki2c version\n\nversion information"]
pub type VersionR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - i2c module enable"]
    #[inline(always)]
    pub fn i2c_en(&self) -> I2cEnR {
        I2cEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - i2c mode select"]
    #[inline(always)]
    pub fn i2c_mode(&self) -> I2cModeR {
        I2cModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - start enable\n\nstart enable, when this bit is written to 1, I2C will generate start\n\nsignal."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - stop enable\n\nstop enable, when this bit is written to 1, I2C will generate stop\n\nsignal."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - last byte acknowledge control in master receive mode"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - operation when NAK handshake is received"]
    #[inline(always)]
    pub fn act2nak(&self) -> Act2nakR {
        Act2nakR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SDA update point config\n\nUsed to config sda change state when scl is low, used to adjust\n\nsetup/hold time\n\n4'bn:Thold = (n + 1) * Tclk_i2c\n\nNote: 0 &lt;= n &lt;= 5"]
    #[inline(always)]
    pub fn data_upd_st(&self) -> DataUpdStR {
        DataUpdStR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - start setup config\n\nTSU;sta = (start_setup + 1) * T(SCL_HIGH) + Tclk_i2c\n\nTHD;sta = (start_setup + 2) * T(SCL_HIGH) - Tclk_i2c"]
    #[inline(always)]
    pub fn start_setup(&self) -> StartSetupR {
        StartSetupR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - staop setup config\n\nTSU;sto = (stop_setup + 1) * T(SCL_HIGH) + Tclk_i2c"]
    #[inline(always)]
    pub fn stop_setup(&self) -> StopSetupR {
        StopSetupR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - rki2c version\n\nversion information"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - i2c module enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_en(&mut self) -> I2cEnW<Rki2cConSpec> {
        I2cEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - i2c mode select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mode(&mut self) -> I2cModeW<Rki2cConSpec> {
        I2cModeW::new(self, 1)
    }
    #[doc = "Bit 3 - start enable\n\nstart enable, when this bit is written to 1, I2C will generate start\n\nsignal."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<Rki2cConSpec> {
        StartW::new(self, 3)
    }
    #[doc = "Bit 4 - stop enable\n\nstop enable, when this bit is written to 1, I2C will generate stop\n\nsignal."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<Rki2cConSpec> {
        StopW::new(self, 4)
    }
    #[doc = "Bit 5 - last byte acknowledge control in master receive mode"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<Rki2cConSpec> {
        AckW::new(self, 5)
    }
    #[doc = "Bit 6 - operation when NAK handshake is received"]
    #[inline(always)]
    #[must_use]
    pub fn act2nak(&mut self) -> Act2nakW<Rki2cConSpec> {
        Act2nakW::new(self, 6)
    }
    #[doc = "Bits 8:10 - SDA update point config\n\nUsed to config sda change state when scl is low, used to adjust\n\nsetup/hold time\n\n4'bn:Thold = (n + 1) * Tclk_i2c\n\nNote: 0 &lt;= n &lt;= 5"]
    #[inline(always)]
    #[must_use]
    pub fn data_upd_st(&mut self) -> DataUpdStW<Rki2cConSpec> {
        DataUpdStW::new(self, 8)
    }
    #[doc = "Bits 12:13 - start setup config\n\nTSU;sta = (start_setup + 1) * T(SCL_HIGH) + Tclk_i2c\n\nTHD;sta = (start_setup + 2) * T(SCL_HIGH) - Tclk_i2c"]
    #[inline(always)]
    #[must_use]
    pub fn start_setup(&mut self) -> StartSetupW<Rki2cConSpec> {
        StartSetupW::new(self, 12)
    }
    #[doc = "Bits 14:15 - staop setup config\n\nTSU;sto = (stop_setup + 1) * T(SCL_HIGH) + Tclk_i2c"]
    #[inline(always)]
    #[must_use]
    pub fn stop_setup(&mut self) -> StopSetupW<Rki2cConSpec> {
        StopSetupW::new(self, 14)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cConSpec;
impl crate::RegisterSpec for Rki2cConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_con::R`](R) reader structure"]
impl crate::Readable for Rki2cConSpec {}
#[doc = "`write(|w| ..)` method takes [`rki2c_con::W`](W) writer structure"]
impl crate::Writable for Rki2cConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RKI2C_CON to value 0"]
impl crate::Resettable for Rki2cConSpec {
    const RESET_VALUE: u32 = 0;
}
