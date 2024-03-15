#[doc = "Register `GRF_GPIO2D_P` reader"]
pub type R = crate::R<GrfGpio2dPSpec>;
#[doc = "Register `GRF_GPIO2D_P` writer"]
pub type W = crate::W<GrfGpio2dPSpec>;
#[doc = "GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d0P {
    #[doc = "0: weak 1(pull-up);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 1(pull-up);"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2d0P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d0P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d0P {
    type Ux = u8;
}
#[doc = "Field `GPIO2D0_P` reader - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio2d0PR = crate::FieldReader<Gpio2d0P>;
impl Gpio2d0PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d0P {
        match self.bits {
            0 => Gpio2d0P::B00,
            1 => Gpio2d0P::B01,
            2 => Gpio2d0P::B10,
            3 => Gpio2d0P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d0P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d0P::B01
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d0P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d0P::B11
    }
}
#[doc = "Field `GPIO2D0_P` writer - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio2d0PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d0P>;
impl<'a, REG> Gpio2d0PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d0P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d0P::B01)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d0P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d0P::B11)
    }
}
#[doc = "GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d1P {
    #[doc = "0: weak 1(pull-up);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 1(pull-up);"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2d1P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d1P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d1P {
    type Ux = u8;
}
#[doc = "Field `GPIO2D1_P` reader - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio2d1PR = crate::FieldReader<Gpio2d1P>;
impl Gpio2d1PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d1P {
        match self.bits {
            0 => Gpio2d1P::B00,
            1 => Gpio2d1P::B01,
            2 => Gpio2d1P::B10,
            3 => Gpio2d1P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d1P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d1P::B01
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d1P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d1P::B11
    }
}
#[doc = "Field `GPIO2D1_P` writer - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio2d1PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d1P>;
impl<'a, REG> Gpio2d1PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d1P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d1P::B01)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d1P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d1P::B11)
    }
}
#[doc = "GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d2P {
    #[doc = "0: weak 1(pull-up);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 1(pull-up);"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2d2P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d2P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d2P {
    type Ux = u8;
}
#[doc = "Field `GPIO2D2_P` reader - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio2d2PR = crate::FieldReader<Gpio2d2P>;
impl Gpio2d2PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d2P {
        match self.bits {
            0 => Gpio2d2P::B00,
            1 => Gpio2d2P::B01,
            2 => Gpio2d2P::B10,
            3 => Gpio2d2P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d2P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d2P::B01
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d2P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d2P::B11
    }
}
#[doc = "Field `GPIO2D2_P` writer - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio2d2PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d2P>;
impl<'a, REG> Gpio2d2PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d2P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d2P::B01)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d2P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d2P::B11)
    }
}
#[doc = "GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d3P {
    #[doc = "0: weak 1(pull-up);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 1(pull-up);"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2d3P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d3P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d3P {
    type Ux = u8;
}
#[doc = "Field `GPIO2D3_P` reader - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio2d3PR = crate::FieldReader<Gpio2d3P>;
impl Gpio2d3PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d3P {
        match self.bits {
            0 => Gpio2d3P::B00,
            1 => Gpio2d3P::B01,
            2 => Gpio2d3P::B10,
            3 => Gpio2d3P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d3P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d3P::B01
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d3P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d3P::B11
    }
}
#[doc = "Field `GPIO2D3_P` writer - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio2d3PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d3P>;
impl<'a, REG> Gpio2d3PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d3P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d3P::B01)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d3P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d3P::B11)
    }
}
#[doc = "GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d4P {
    #[doc = "0: weak 1(pull-up);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 1(pull-up);"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2d4P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d4P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d4P {
    type Ux = u8;
}
#[doc = "Field `GPIO2D4_P` reader - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio2d4PR = crate::FieldReader<Gpio2d4P>;
impl Gpio2d4PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d4P {
        match self.bits {
            0 => Gpio2d4P::B00,
            1 => Gpio2d4P::B01,
            2 => Gpio2d4P::B10,
            3 => Gpio2d4P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d4P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d4P::B01
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d4P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d4P::B11
    }
}
#[doc = "Field `GPIO2D4_P` writer - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio2d4PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d4P>;
impl<'a, REG> Gpio2d4PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d4P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d4P::B01)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d4P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d4P::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d0_p(&self) -> Gpio2d0PR {
        Gpio2d0PR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d1_p(&self) -> Gpio2d1PR {
        Gpio2d1PR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d2_p(&self) -> Gpio2d2PR {
        Gpio2d2PR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d3_p(&self) -> Gpio2d3PR {
        Gpio2d3PR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d4_p(&self) -> Gpio2d4PR {
        Gpio2d4PR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d0_p(&mut self) -> Gpio2d0PW<GrfGpio2dPSpec> {
        Gpio2d0PW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d1_p(&mut self) -> Gpio2d1PW<GrfGpio2dPSpec> {
        Gpio2d1PW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d2_p(&mut self) -> Gpio2d2PW<GrfGpio2dPSpec> {
        Gpio2d2PW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d3_p(&mut self) -> Gpio2d3PW<GrfGpio2dPSpec> {
        Gpio2d3PW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO2D PE/PS programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d4_p(&mut self) -> Gpio2d4PW<GrfGpio2dPSpec> {
        Gpio2d4PW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio2dPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2D PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2d_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2d_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio2dPSpec;
impl crate::RegisterSpec for GrfGpio2dPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio2d_p::R`](R) reader structure"]
impl crate::Readable for GrfGpio2dPSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio2d_p::W`](W) writer structure"]
impl crate::Writable for GrfGpio2dPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO2D_P to value 0x7f"]
impl crate::Resettable for GrfGpio2dPSpec {
    const RESET_VALUE: u32 = 0x7f;
}
