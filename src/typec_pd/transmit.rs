#[doc = "Register `TRANSMIT` reader"]
pub type R = crate::R<TransmitSpec>;
#[doc = "Register `TRANSMIT` writer"]
pub type W = crate::W<TransmitSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TransmitSopMessage {
    #[doc = "0: Transmit SOP"]
    B000 = 0,
    #[doc = "1: Transmit SOP’"]
    B001 = 1,
    #[doc = "2: Transmit SOP’’"]
    B010 = 2,
    #[doc = "3: Transmit SOP_DBG’"]
    B011 = 3,
    #[doc = "4: Transmit SOP_DBG’’"]
    B100 = 4,
    #[doc = "5: Transmit Hard Reset"]
    B101 = 5,
    #[doc = "6: Transmit CableReset"]
    B110 = 6,
    #[doc = "7: Transmit BIST Carrier Mode 2 (TCPC shall exit the BIST mode no later than tBISTContModemax)"]
    B111 = 7,
}
impl From<TransmitSopMessage> for u8 {
    #[inline(always)]
    fn from(variant: TransmitSopMessage) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TransmitSopMessage {
    type Ux = u8;
}
#[doc = "Field `Transmit_SOP_message` reader - "]
pub type TransmitSopMessageR = crate::FieldReader<TransmitSopMessage>;
impl TransmitSopMessageR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TransmitSopMessage {
        match self.bits {
            0 => TransmitSopMessage::B000,
            1 => TransmitSopMessage::B001,
            2 => TransmitSopMessage::B010,
            3 => TransmitSopMessage::B011,
            4 => TransmitSopMessage::B100,
            5 => TransmitSopMessage::B101,
            6 => TransmitSopMessage::B110,
            7 => TransmitSopMessage::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "Transmit SOP"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == TransmitSopMessage::B000
    }
    #[doc = "Transmit SOP’"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == TransmitSopMessage::B001
    }
    #[doc = "Transmit SOP’’"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == TransmitSopMessage::B010
    }
    #[doc = "Transmit SOP_DBG’"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == TransmitSopMessage::B011
    }
    #[doc = "Transmit SOP_DBG’’"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == TransmitSopMessage::B100
    }
    #[doc = "Transmit Hard Reset"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == TransmitSopMessage::B101
    }
    #[doc = "Transmit CableReset"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == TransmitSopMessage::B110
    }
    #[doc = "Transmit BIST Carrier Mode 2 (TCPC shall exit the BIST mode no later than tBISTContModemax)"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == TransmitSopMessage::B111
    }
}
#[doc = "Field `Transmit_SOP_message` writer - "]
pub type TransmitSopMessageW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TransmitSopMessage>;
impl<'a, REG> TransmitSopMessageW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmit SOP"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessage::B000)
    }
    #[doc = "Transmit SOP’"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessage::B001)
    }
    #[doc = "Transmit SOP’’"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessage::B010)
    }
    #[doc = "Transmit SOP_DBG’"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessage::B011)
    }
    #[doc = "Transmit SOP_DBG’’"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessage::B100)
    }
    #[doc = "Transmit Hard Reset"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessage::B101)
    }
    #[doc = "Transmit CableReset"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessage::B110)
    }
    #[doc = "Transmit BIST Carrier Mode 2 (TCPC shall exit the BIST mode no later than tBISTContModemax)"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(TransmitSopMessage::B111)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RetryCounter {
    #[doc = "0: No message retry is required"]
    B00 = 0,
    #[doc = "1: Automatically retry message transmission once"]
    B01 = 1,
    #[doc = "2: Automatically retry message transmission twice"]
    B10 = 2,
    #[doc = "3: Automatically retry message transmission three times"]
    B11 = 3,
}
impl From<RetryCounter> for u8 {
    #[inline(always)]
    fn from(variant: RetryCounter) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RetryCounter {
    type Ux = u8;
}
#[doc = "Field `Retry_Counter` reader - "]
pub type RetryCounterR = crate::FieldReader<RetryCounter>;
impl RetryCounterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RetryCounter {
        match self.bits {
            0 => RetryCounter::B00,
            1 => RetryCounter::B01,
            2 => RetryCounter::B10,
            3 => RetryCounter::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "No message retry is required"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == RetryCounter::B00
    }
    #[doc = "Automatically retry message transmission once"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == RetryCounter::B01
    }
    #[doc = "Automatically retry message transmission twice"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == RetryCounter::B10
    }
    #[doc = "Automatically retry message transmission three times"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == RetryCounter::B11
    }
}
#[doc = "Field `Retry_Counter` writer - "]
pub type RetryCounterW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RetryCounter>;
impl<'a, REG> RetryCounterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No message retry is required"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(RetryCounter::B00)
    }
    #[doc = "Automatically retry message transmission once"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(RetryCounter::B01)
    }
    #[doc = "Automatically retry message transmission twice"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(RetryCounter::B10)
    }
    #[doc = "Automatically retry message transmission three times"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(RetryCounter::B11)
    }
}
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore read \n\nvalue."]
pub type NotUsedR = crate::FieldReader<u32>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore read \n\nvalue."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn transmit_sop_message(&self) -> TransmitSopMessageR {
        TransmitSopMessageR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn retry_counter(&self) -> RetryCounterR {
        RetryCounterR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_sop_message(&mut self) -> TransmitSopMessageW<TransmitSpec> {
        TransmitSopMessageW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn retry_counter(&mut self) -> RetryCounterW<TransmitSpec> {
        RetryCounterW::new(self, 4)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<TransmitSpec> {
        NotUsedW::new(self, 8)
    }
}
#[doc = "Transmit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TransmitSpec;
impl crate::RegisterSpec for TransmitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transmit::R`](R) reader structure"]
impl crate::Readable for TransmitSpec {}
#[doc = "`write(|w| ..)` method takes [`transmit::W`](W) writer structure"]
impl crate::Writable for TransmitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRANSMIT to value 0"]
impl crate::Resettable for TransmitSpec {
    const RESET_VALUE: u32 = 0;
}
