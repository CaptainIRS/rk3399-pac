#[doc = "Register `ROLE_CONTROL` reader"]
pub type R = crate::R<RoleControlSpec>;
#[doc = "Register `ROLE_CONTROL` writer"]
pub type W = crate::W<RoleControlSpec>;
#[doc = "\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc1 {
    #[doc = "0: Ra"]
    B00 = 0,
    #[doc = "1: Rp (Use Rp definition in B5..4) 10b:Rd"]
    B01 = 1,
    #[doc = "3: Open (Disconnect or don’t care)"]
    B11 = 3,
}
impl From<Cc1> for u8 {
    #[inline(always)]
    fn from(variant: Cc1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc1 {
    type Ux = u8;
}
#[doc = "Field `CC1` reader - "]
pub type Cc1R = crate::FieldReader<Cc1>;
impl Cc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cc1> {
        match self.bits {
            0 => Some(Cc1::B00),
            1 => Some(Cc1::B01),
            3 => Some(Cc1::B11),
            _ => None,
        }
    }
    #[doc = "Ra"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Cc1::B00
    }
    #[doc = "Rp (Use Rp definition in B5..4) 10b:Rd"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Cc1::B01
    }
    #[doc = "Open (Disconnect or don’t care)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Cc1::B11
    }
}
#[doc = "Field `CC1` writer - "]
pub type Cc1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc1>;
impl<'a, REG> Cc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ra"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1::B00)
    }
    #[doc = "Rp (Use Rp definition in B5..4) 10b:Rd"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1::B01)
    }
    #[doc = "Open (Disconnect or don’t care)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1::B11)
    }
}
#[doc = "\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc2 {
    #[doc = "0: Ra"]
    B00 = 0,
    #[doc = "1: Rp (Use Rp definition in B5..4) 10b:Rd"]
    B01 = 1,
    #[doc = "3: Open (Disconnect or don’t care)"]
    B11 = 3,
}
impl From<Cc2> for u8 {
    #[inline(always)]
    fn from(variant: Cc2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc2 {
    type Ux = u8;
}
#[doc = "Field `CC2` reader - "]
pub type Cc2R = crate::FieldReader<Cc2>;
impl Cc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cc2> {
        match self.bits {
            0 => Some(Cc2::B00),
            1 => Some(Cc2::B01),
            3 => Some(Cc2::B11),
            _ => None,
        }
    }
    #[doc = "Ra"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Cc2::B00
    }
    #[doc = "Rp (Use Rp definition in B5..4) 10b:Rd"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Cc2::B01
    }
    #[doc = "Open (Disconnect or don’t care)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Cc2::B11
    }
}
#[doc = "Field `CC2` writer - "]
pub type Cc2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc2>;
impl<'a, REG> Cc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ra"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2::B00)
    }
    #[doc = "Rp (Use Rp definition in B5..4) 10b:Rd"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2::B01)
    }
    #[doc = "Open (Disconnect or don’t care)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2::B11)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RpValue {
    #[doc = "0: Rp default"]
    B00 = 0,
    #[doc = "1: Rp1.5A"]
    B01 = 1,
    #[doc = "2: Rp3.0A"]
    B10 = 2,
}
impl From<RpValue> for u8 {
    #[inline(always)]
    fn from(variant: RpValue) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RpValue {
    type Ux = u8;
}
#[doc = "Field `Rp_Value` reader - "]
pub type RpValueR = crate::FieldReader<RpValue>;
impl RpValueR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RpValue> {
        match self.bits {
            0 => Some(RpValue::B00),
            1 => Some(RpValue::B01),
            2 => Some(RpValue::B10),
            _ => None,
        }
    }
    #[doc = "Rp default"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == RpValue::B00
    }
    #[doc = "Rp1.5A"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == RpValue::B01
    }
    #[doc = "Rp3.0A"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == RpValue::B10
    }
}
#[doc = "Field `Rp_Value` writer - "]
pub type RpValueW<'a, REG> = crate::FieldWriter<'a, REG, 2, RpValue>;
impl<'a, REG> RpValueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rp default"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(RpValue::B00)
    }
    #[doc = "Rp1.5A"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(RpValue::B01)
    }
    #[doc = "Rp3.0A"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(RpValue::B10)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drp {
    #[doc = "0: No DRP. Bits B3..0 determine Rp/Rd/Ra or open settings"]
    B0 = 0,
    #[doc = "1: DRP The TCPC shall use the Rp value defined inB5..4 when a connection is resolved, ie. Upon entry to Potential_Connect_as_Src in Figure 4-11. TCPC State Machine before a Connection. The TCPC toggles CC1 &amp; CC2 after receiving COMMAND.Look4Connection and until connection is detected. Upon connection, the TCPC shall resolve to either an Rp or Rd and report the CC1/CC2 State in the CC_STATUS register. The CC pins shall stay in Potential_start_as_SRC or Potential_start_as_Sink until directedotherwise."]
    B1 = 1,
}
impl From<Drp> for bool {
    #[inline(always)]
    fn from(variant: Drp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRP` reader - "]
pub type DrpR = crate::BitReader<Drp>;
impl DrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drp {
        match self.bits {
            false => Drp::B0,
            true => Drp::B1,
        }
    }
    #[doc = "No DRP. Bits B3..0 determine Rp/Rd/Ra or open settings"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Drp::B0
    }
    #[doc = "DRP The TCPC shall use the Rp value defined inB5..4 when a connection is resolved, ie. Upon entry to Potential_Connect_as_Src in Figure 4-11. TCPC State Machine before a Connection. The TCPC toggles CC1 &amp; CC2 after receiving COMMAND.Look4Connection and until connection is detected. Upon connection, the TCPC shall resolve to either an Rp or Rd and report the CC1/CC2 State in the CC_STATUS register. The CC pins shall stay in Potential_start_as_SRC or Potential_start_as_Sink until directedotherwise."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Drp::B1
    }
}
#[doc = "Field `DRP` writer - "]
pub type DrpW<'a, REG> = crate::BitWriter<'a, REG, Drp>;
impl<'a, REG> DrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DRP. Bits B3..0 determine Rp/Rd/Ra or open settings"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Drp::B0)
    }
    #[doc = "DRP The TCPC shall use the Rp value defined inB5..4 when a connection is resolved, ie. Upon entry to Potential_Connect_as_Src in Figure 4-11. TCPC State Machine before a Connection. The TCPC toggles CC1 &amp; CC2 after receiving COMMAND.Look4Connection and until connection is detected. Upon connection, the TCPC shall resolve to either an Rp or Rd and report the CC1/CC2 State in the CC_STATUS register. The CC pins shall stay in Potential_start_as_SRC or Potential_start_as_Sink until directedotherwise."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Drp::B1)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn rp_value(&self) -> RpValueR {
        RpValueR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn drp(&self) -> DrpR {
        DrpR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> Cc1W<RoleControlSpec> {
        Cc1W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> Cc2W<RoleControlSpec> {
        Cc2W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn rp_value(&mut self) -> RpValueW<RoleControlSpec> {
        RpValueW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn drp(&mut self) -> DrpW<RoleControlSpec> {
        DrpW::new(self, 6)
    }
}
#[doc = "Role Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`role_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`role_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RoleControlSpec;
impl crate::RegisterSpec for RoleControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`role_control::R`](R) reader structure"]
impl crate::Readable for RoleControlSpec {}
#[doc = "`write(|w| ..)` method takes [`role_control::W`](W) writer structure"]
impl crate::Writable for RoleControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROLE_CONTROL to value 0x1f"]
impl crate::Resettable for RoleControlSpec {
    const RESET_VALUE: u32 = 0x1f;
}
