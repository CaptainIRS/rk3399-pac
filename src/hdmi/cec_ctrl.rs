#[doc = "Register `CEC_CTRL` reader"]
pub type R = crate::R<CecCtrlSpec>;
#[doc = "Register `CEC_CTRL` writer"]
pub type W = crate::W<CecCtrlSpec>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameTyp {
    #[doc = "0: Illegal value. If software writes this value, hardware sets the value to the default 2'b01."]
    B00 = 0,
    #[doc = "1: Illegal value. If software writes this value, hardware sets the value to the default 2'b01."]
    B01 = 1,
    #[doc = "2: Illegal value. If software writes this value, hardware sets the value to the default 2'b01."]
    B10 = 2,
    #[doc = "3: Illegal value. If software writes this value, hardware sets the value to the default 2'b01."]
    B11 = 3,
}
impl From<FrameTyp> for u8 {
    #[inline(always)]
    fn from(variant: FrameTyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FrameTyp {
    type Ux = u8;
}
#[doc = "Field `FRAME_TYP` reader - "]
pub type FrameTypR = crate::FieldReader<FrameTyp>;
impl FrameTypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrameTyp {
        match self.bits {
            0 => FrameTyp::B00,
            1 => FrameTyp::B01,
            2 => FrameTyp::B10,
            3 => FrameTyp::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Illegal value. If software writes this value, hardware sets the value to the default 2'b01."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == FrameTyp::B00
    }
    #[doc = "Illegal value. If software writes this value, hardware sets the value to the default 2'b01."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == FrameTyp::B01
    }
    #[doc = "Illegal value. If software writes this value, hardware sets the value to the default 2'b01."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == FrameTyp::B10
    }
    #[doc = "Illegal value. If software writes this value, hardware sets the value to the default 2'b01."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == FrameTyp::B11
    }
}
#[doc = "Field `FRAME_TYP` writer - "]
pub type FrameTypW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FrameTyp>;
impl<'a, REG> FrameTypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Illegal value. If software writes this value, hardware sets the value to the default 2'b01."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(FrameTyp::B00)
    }
    #[doc = "Illegal value. If software writes this value, hardware sets the value to the default 2'b01."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(FrameTyp::B01)
    }
    #[doc = "Illegal value. If software writes this value, hardware sets the value to the default 2'b01."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(FrameTyp::B10)
    }
    #[doc = "Illegal value. If software writes this value, hardware sets the value to the default 2'b01."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(FrameTyp::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BcNack {
    #[doc = "1: Reset by software to ACK the received broadcast message."]
    B1 = 1,
    #[doc = "0: Reset by software to ACK the received broadcast message."]
    B0 = 0,
}
impl From<BcNack> for bool {
    #[inline(always)]
    fn from(variant: BcNack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC_NACK` reader - "]
pub type BcNackR = crate::BitReader<BcNack>;
impl BcNackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BcNack {
        match self.bits {
            true => BcNack::B1,
            false => BcNack::B0,
        }
    }
    #[doc = "Reset by software to ACK the received broadcast message."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BcNack::B1
    }
    #[doc = "Reset by software to ACK the received broadcast message."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BcNack::B0
    }
}
#[doc = "Field `BC_NACK` writer - "]
pub type BcNackW<'a, REG> = crate::BitWriter<'a, REG, BcNack>;
impl<'a, REG> BcNackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by software to ACK the received broadcast message."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BcNack::B1)
    }
    #[doc = "Reset by software to ACK the received broadcast message."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BcNack::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Standby {
    #[doc = "1: CEC controller responds the ACK to all messages."]
    B1 = 1,
    #[doc = "0: CEC controller responds the ACK to all messages."]
    B0 = 0,
}
impl From<Standby> for bool {
    #[inline(always)]
    fn from(variant: Standby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STANDBY` reader - "]
pub type StandbyR = crate::BitReader<Standby>;
impl StandbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Standby {
        match self.bits {
            true => Standby::B1,
            false => Standby::B0,
        }
    }
    #[doc = "CEC controller responds the ACK to all messages."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Standby::B1
    }
    #[doc = "CEC controller responds the ACK to all messages."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Standby::B0
    }
}
#[doc = "Field `STANDBY` writer - "]
pub type StandbyW<'a, REG> = crate::BitWriter<'a, REG, Standby>;
impl<'a, REG> StandbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CEC controller responds the ACK to all messages."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Standby::B1)
    }
    #[doc = "CEC controller responds the ACK to all messages."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Standby::B0)
    }
}
impl R {
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn frame_typ(&self) -> FrameTypR {
        FrameTypR::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bc_nack(&self) -> BcNackR {
        BcNackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn standby(&self) -> StandbyR {
        StandbyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn frame_typ(&mut self) -> FrameTypW<CecCtrlSpec> {
        FrameTypW::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn bc_nack(&mut self) -> BcNackW<CecCtrlSpec> {
        BcNackW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn standby(&mut self) -> StandbyW<CecCtrlSpec> {
        StandbyW::new(self, 4)
    }
}
#[doc = "2'b00: Signal Free Time = 3-bit periods. Previous attempt to send frame is unsuccessful. 2'b01: Signal Free Time = 5-bit periods. New initiator wants to send a frame. 2'b10: Signal Free Time = 7-bit periods. Present initiator wants to send another frame immediately after its previous frame. (specification CEC 9.1). 2'b11: Illegal value. If software writes this value, hardware sets the value to the default 2'b01.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CecCtrlSpec;
impl crate::RegisterSpec for CecCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cec_ctrl::R`](R) reader structure"]
impl crate::Readable for CecCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cec_ctrl::W`](W) writer structure"]
impl crate::Writable for CecCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CEC_CTRL to value 0x02"]
impl crate::Resettable for CecCtrlSpec {
    const RESET_VALUE: u8 = 0x02;
}
