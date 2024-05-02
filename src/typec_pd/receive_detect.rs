#[doc = "Register `RECEIVE_DETECT` reader"]
pub type R = crate::R<ReceiveDetectSpec>;
#[doc = "Register `RECEIVE_DETECT` writer"]
pub type W = crate::W<ReceiveDetectSpec>;
#[doc = "Field `Enable_SOP_message` reader - 0b: \n\nTCPC \n\ndoes \n\nnot \n\ndetect \n\nSOP \n\nmessage (default) 1b: TCPC detects \n\nSOP message"]
pub type EnableSopMessageR = crate::BitReader;
#[doc = "Field `Enable_SOP_message` writer - 0b: \n\nTCPC \n\ndoes \n\nnot \n\ndetect \n\nSOP \n\nmessage (default) 1b: TCPC detects \n\nSOP message"]
pub type EnableSopMessageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Enable_SOP_p_message` reader - 0b: \n\nTCPC \n\ndoes \n\nnot \n\ndetect \n\nSOP’ \n\nmessage (default) 1b: TCPC detects \n\nSOP’ message"]
pub type EnableSopPMessageR = crate::BitReader;
#[doc = "Field `Enable_SOP_p_message` writer - 0b: \n\nTCPC \n\ndoes \n\nnot \n\ndetect \n\nSOP’ \n\nmessage (default) 1b: TCPC detects \n\nSOP’ message"]
pub type EnableSopPMessageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Enable_SOP_b_message` reader - 0b: \n\nTCPC \n\ndoes \n\nnot \n\ndetect SOP’’ \n\nmessage (default) 1b: TCPC detects \n\nSOP’’ message"]
pub type EnableSopBMessageR = crate::BitReader;
#[doc = "Field `Enable_SOP_b_message` writer - 0b: \n\nTCPC \n\ndoes \n\nnot \n\ndetect SOP’’ \n\nmessage (default) 1b: TCPC detects \n\nSOP’’ message"]
pub type EnableSopBMessageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnableSopDbgPMessage {
    #[doc = "0: TCPC does not detect SOP_DBG’ message (default)"]
    B0 = 0,
    #[doc = "1: TCPC detects SOP_DBG“ message"]
    B1 = 1,
}
impl From<EnableSopDbgPMessage> for bool {
    #[inline(always)]
    fn from(variant: EnableSopDbgPMessage) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Enable_SOP_DBG_p_message` reader - "]
pub type EnableSopDbgPMessageR = crate::BitReader<EnableSopDbgPMessage>;
impl EnableSopDbgPMessageR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnableSopDbgPMessage {
        match self.bits {
            false => EnableSopDbgPMessage::B0,
            true => EnableSopDbgPMessage::B1,
        }
    }
    #[doc = "TCPC does not detect SOP_DBG’ message (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EnableSopDbgPMessage::B0
    }
    #[doc = "TCPC detects SOP_DBG“ message"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EnableSopDbgPMessage::B1
    }
}
#[doc = "Field `Enable_SOP_DBG_p_message` writer - "]
pub type EnableSopDbgPMessageW<'a, REG> = crate::BitWriter<'a, REG, EnableSopDbgPMessage>;
impl<'a, REG> EnableSopDbgPMessageW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TCPC does not detect SOP_DBG’ message (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EnableSopDbgPMessage::B0)
    }
    #[doc = "TCPC detects SOP_DBG“ message"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EnableSopDbgPMessage::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnableSopDbgBMessage {
    #[doc = "0: TCPC does not detect SOP_DBG’’ message (default)"]
    B0 = 0,
    #[doc = "1: TCPC detects SOP_DBG’’ message"]
    B1 = 1,
}
impl From<EnableSopDbgBMessage> for bool {
    #[inline(always)]
    fn from(variant: EnableSopDbgBMessage) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Enable_SOP_DBG_b_message` reader - "]
pub type EnableSopDbgBMessageR = crate::BitReader<EnableSopDbgBMessage>;
impl EnableSopDbgBMessageR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnableSopDbgBMessage {
        match self.bits {
            false => EnableSopDbgBMessage::B0,
            true => EnableSopDbgBMessage::B1,
        }
    }
    #[doc = "TCPC does not detect SOP_DBG’’ message (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EnableSopDbgBMessage::B0
    }
    #[doc = "TCPC detects SOP_DBG’’ message"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EnableSopDbgBMessage::B1
    }
}
#[doc = "Field `Enable_SOP_DBG_b_message` writer - "]
pub type EnableSopDbgBMessageW<'a, REG> = crate::BitWriter<'a, REG, EnableSopDbgBMessage>;
impl<'a, REG> EnableSopDbgBMessageW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TCPC does not detect SOP_DBG’’ message (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EnableSopDbgBMessage::B0)
    }
    #[doc = "TCPC detects SOP_DBG’’ message"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EnableSopDbgBMessage::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnableHardReset {
    #[doc = "0: TCPC does not detect Hard Reset signaling (default)"]
    B0 = 0,
    #[doc = "1: TCPC detects Hard Reset signaling"]
    B1 = 1,
}
impl From<EnableHardReset> for bool {
    #[inline(always)]
    fn from(variant: EnableHardReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Enable_Hard_Reset` reader - "]
pub type EnableHardResetR = crate::BitReader<EnableHardReset>;
impl EnableHardResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnableHardReset {
        match self.bits {
            false => EnableHardReset::B0,
            true => EnableHardReset::B1,
        }
    }
    #[doc = "TCPC does not detect Hard Reset signaling (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EnableHardReset::B0
    }
    #[doc = "TCPC detects Hard Reset signaling"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EnableHardReset::B1
    }
}
#[doc = "Field `Enable_Hard_Reset` writer - "]
pub type EnableHardResetW<'a, REG> = crate::BitWriter<'a, REG, EnableHardReset>;
impl<'a, REG> EnableHardResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TCPC does not detect Hard Reset signaling (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EnableHardReset::B0)
    }
    #[doc = "TCPC detects Hard Reset signaling"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EnableHardReset::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnableCableReset {
    #[doc = "0: TCPC does not detect Cable Reset signaling (default)"]
    B0 = 0,
    #[doc = "1: TCPC detects Cable Reset signaling"]
    B1 = 1,
}
impl From<EnableCableReset> for bool {
    #[inline(always)]
    fn from(variant: EnableCableReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Enable_Cable_Reset` reader - "]
pub type EnableCableResetR = crate::BitReader<EnableCableReset>;
impl EnableCableResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnableCableReset {
        match self.bits {
            false => EnableCableReset::B0,
            true => EnableCableReset::B1,
        }
    }
    #[doc = "TCPC does not detect Cable Reset signaling (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EnableCableReset::B0
    }
    #[doc = "TCPC detects Cable Reset signaling"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EnableCableReset::B1
    }
}
#[doc = "Field `Enable_Cable_Reset` writer - "]
pub type EnableCableResetW<'a, REG> = crate::BitWriter<'a, REG, EnableCableReset>;
impl<'a, REG> EnableCableResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TCPC does not detect Cable Reset signaling (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EnableCableReset::B0)
    }
    #[doc = "TCPC detects Cable Reset signaling"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EnableCableReset::B1)
    }
}
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedR = crate::FieldReader<u32>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0b: \n\nTCPC \n\ndoes \n\nnot \n\ndetect \n\nSOP \n\nmessage (default) 1b: TCPC detects \n\nSOP message"]
    #[inline(always)]
    pub fn enable_sop_message(&self) -> EnableSopMessageR {
        EnableSopMessageR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0b: \n\nTCPC \n\ndoes \n\nnot \n\ndetect \n\nSOP’ \n\nmessage (default) 1b: TCPC detects \n\nSOP’ message"]
    #[inline(always)]
    pub fn enable_sop_p_message(&self) -> EnableSopPMessageR {
        EnableSopPMessageR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0b: \n\nTCPC \n\ndoes \n\nnot \n\ndetect SOP’’ \n\nmessage (default) 1b: TCPC detects \n\nSOP’’ message"]
    #[inline(always)]
    pub fn enable_sop_b_message(&self) -> EnableSopBMessageR {
        EnableSopBMessageR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn enable_sop_dbg_p_message(&self) -> EnableSopDbgPMessageR {
        EnableSopDbgPMessageR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn enable_sop_dbg_b_message(&self) -> EnableSopDbgBMessageR {
        EnableSopDbgBMessageR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn enable_hard_reset(&self) -> EnableHardResetR {
        EnableHardResetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn enable_cable_reset(&self) -> EnableCableResetR {
        EnableCableResetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore \n\nread value."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0b: \n\nTCPC \n\ndoes \n\nnot \n\ndetect \n\nSOP \n\nmessage (default) 1b: TCPC detects \n\nSOP message"]
    #[inline(always)]
    #[must_use]
    pub fn enable_sop_message(&mut self) -> EnableSopMessageW<ReceiveDetectSpec> {
        EnableSopMessageW::new(self, 0)
    }
    #[doc = "Bit 1 - 0b: \n\nTCPC \n\ndoes \n\nnot \n\ndetect \n\nSOP’ \n\nmessage (default) 1b: TCPC detects \n\nSOP’ message"]
    #[inline(always)]
    #[must_use]
    pub fn enable_sop_p_message(&mut self) -> EnableSopPMessageW<ReceiveDetectSpec> {
        EnableSopPMessageW::new(self, 1)
    }
    #[doc = "Bit 2 - 0b: \n\nTCPC \n\ndoes \n\nnot \n\ndetect SOP’’ \n\nmessage (default) 1b: TCPC detects \n\nSOP’’ message"]
    #[inline(always)]
    #[must_use]
    pub fn enable_sop_b_message(&mut self) -> EnableSopBMessageW<ReceiveDetectSpec> {
        EnableSopBMessageW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn enable_sop_dbg_p_message(&mut self) -> EnableSopDbgPMessageW<ReceiveDetectSpec> {
        EnableSopDbgPMessageW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn enable_sop_dbg_b_message(&mut self) -> EnableSopDbgBMessageW<ReceiveDetectSpec> {
        EnableSopDbgBMessageW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn enable_hard_reset(&mut self) -> EnableHardResetW<ReceiveDetectSpec> {
        EnableHardResetW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn enable_cable_reset(&mut self) -> EnableCableResetW<ReceiveDetectSpec> {
        EnableCableResetW::new(self, 6)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore \n\nread value."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<ReceiveDetectSpec> {
        NotUsedW::new(self, 8)
    }
}
#[doc = "Receive Detect Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_detect::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_detect::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReceiveDetectSpec;
impl crate::RegisterSpec for ReceiveDetectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_detect::R`](R) reader structure"]
impl crate::Readable for ReceiveDetectSpec {}
#[doc = "`write(|w| ..)` method takes [`receive_detect::W`](W) writer structure"]
impl crate::Writable for ReceiveDetectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECEIVE_DETECT to value 0"]
impl crate::Resettable for ReceiveDetectSpec {
    const RESET_VALUE: u32 = 0;
}
