#[doc = "Register `CCI500_STATUS` reader"]
pub type R = crate::R<Cci500StatusSpec>;
#[doc = "Register `CCI500_STATUS` writer"]
pub type W = crate::W<Cci500StatusSpec>;
#[doc = "Indicates whether any changes to the Snoop\n\nControl Registers or the Control Override\n\nRegister are pending in the CCI-500:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangePending {
    #[doc = "0: No change pending"]
    B0 = 0,
    #[doc = "1: Change pending"]
    B1 = 1,
}
impl From<ChangePending> for bool {
    #[inline(always)]
    fn from(variant: ChangePending) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANGE_PENDING` reader - Indicates whether any changes to the Snoop\n\nControl Registers or the Control Override\n\nRegister are pending in the CCI-500:"]
pub type ChangePendingR = crate::BitReader<ChangePending>;
impl ChangePendingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChangePending {
        match self.bits {
            false => ChangePending::B0,
            true => ChangePending::B1,
        }
    }
    #[doc = "No change pending"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ChangePending::B0
    }
    #[doc = "Change pending"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ChangePending::B1
    }
}
#[doc = "Field `CHANGE_PENDING` writer - Indicates whether any changes to the Snoop\n\nControl Registers or the Control Override\n\nRegister are pending in the CCI-500:"]
pub type ChangePendingW<'a, REG> = crate::BitWriter<'a, REG, ChangePending>;
impl<'a, REG> ChangePendingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change pending"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ChangePending::B0)
    }
    #[doc = "Change pending"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ChangePending::B1)
    }
}
#[doc = "Indicates when the snoop filter RAM is\n\ninitialized. Shareable requests are not\n\nserviced during this period.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfRamInitialization {
    #[doc = "0: Snoop filter RAM initialization is complete."]
    B0 = 0,
    #[doc = "1: Snoop filter RAM initialization is in progress."]
    B1 = 1,
}
impl From<SfRamInitialization> for bool {
    #[inline(always)]
    fn from(variant: SfRamInitialization) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SF_RAM_INITIALIZATION` reader - Indicates when the snoop filter RAM is\n\ninitialized. Shareable requests are not\n\nserviced during this period."]
pub type SfRamInitializationR = crate::BitReader<SfRamInitialization>;
impl SfRamInitializationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SfRamInitialization {
        match self.bits {
            false => SfRamInitialization::B0,
            true => SfRamInitialization::B1,
        }
    }
    #[doc = "Snoop filter RAM initialization is complete."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SfRamInitialization::B0
    }
    #[doc = "Snoop filter RAM initialization is in progress."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SfRamInitialization::B1
    }
}
#[doc = "Field `SF_RAM_INITIALIZATION` writer - Indicates when the snoop filter RAM is\n\ninitialized. Shareable requests are not\n\nserviced during this period."]
pub type SfRamInitializationW<'a, REG> = crate::BitWriter<'a, REG, SfRamInitialization>;
impl<'a, REG> SfRamInitializationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Snoop filter RAM initialization is complete."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SfRamInitialization::B0)
    }
    #[doc = "Snoop filter RAM initialization is in progress."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SfRamInitialization::B1)
    }
}
#[doc = "The snoop filter RAM power states are\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SfRamState {
    #[doc = "0: Off."]
    B000 = 0,
    #[doc = "1: Static snoop filter RAM retention."]
    B001 = 1,
    #[doc = "2: Reserved."]
    B010 = 2,
    #[doc = "3: Dynamic snoop filter RAM retention."]
    B011 = 3,
    #[doc = "4: On. 0b101-0b111 Reserved"]
    B100 = 4,
}
impl From<SfRamState> for u8 {
    #[inline(always)]
    fn from(variant: SfRamState) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SfRamState {
    type Ux = u8;
}
#[doc = "Field `SF_RAM_STATE` reader - The snoop filter RAM power states are"]
pub type SfRamStateR = crate::FieldReader<SfRamState>;
impl SfRamStateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SfRamState> {
        match self.bits {
            0 => Some(SfRamState::B000),
            1 => Some(SfRamState::B001),
            2 => Some(SfRamState::B010),
            3 => Some(SfRamState::B011),
            4 => Some(SfRamState::B100),
            _ => None,
        }
    }
    #[doc = "Off."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == SfRamState::B000
    }
    #[doc = "Static snoop filter RAM retention."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == SfRamState::B001
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == SfRamState::B010
    }
    #[doc = "Dynamic snoop filter RAM retention."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == SfRamState::B011
    }
    #[doc = "On. 0b101-0b111 Reserved"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == SfRamState::B100
    }
}
#[doc = "Field `SF_RAM_STATE` writer - The snoop filter RAM power states are"]
pub type SfRamStateW<'a, REG> = crate::FieldWriter<'a, REG, 3, SfRamState>;
impl<'a, REG> SfRamStateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off."]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(SfRamState::B000)
    }
    #[doc = "Static snoop filter RAM retention."]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(SfRamState::B001)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(SfRamState::B010)
    }
    #[doc = "Dynamic snoop filter RAM retention."]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(SfRamState::B011)
    }
    #[doc = "On. 0b101-0b111 Reserved"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(SfRamState::B100)
    }
}
#[doc = "Field `SF_RAM_STATE_REQUEST` reader - This is the last requested power state of the\n\nsnoop filter RAMs Encoding as SF_RAM_state."]
pub type SfRamStateRequestR = crate::FieldReader;
#[doc = "Field `SF_RAM_STATE_REQUEST` writer - This is the last requested power state of the\n\nsnoop filter RAMs Encoding as SF_RAM_state."]
pub type SfRamStateRequestW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Snoop filter RAM power state change pending.\n\nThis bit reads back the PREQ input.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfRamStateChangePending {
    #[doc = "0: No change pending, any previous requests have been accepted or denied."]
    B0 = 0,
    #[doc = "1: State change is pending and might be accepted or denied"]
    B1 = 1,
}
impl From<SfRamStateChangePending> for bool {
    #[inline(always)]
    fn from(variant: SfRamStateChangePending) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SF_RAM_STATE_CHANGE_PENDING` reader - Snoop filter RAM power state change pending.\n\nThis bit reads back the PREQ input."]
pub type SfRamStateChangePendingR = crate::BitReader<SfRamStateChangePending>;
impl SfRamStateChangePendingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SfRamStateChangePending {
        match self.bits {
            false => SfRamStateChangePending::B0,
            true => SfRamStateChangePending::B1,
        }
    }
    #[doc = "No change pending, any previous requests have been accepted or denied."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SfRamStateChangePending::B0
    }
    #[doc = "State change is pending and might be accepted or denied"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SfRamStateChangePending::B1
    }
}
#[doc = "Field `SF_RAM_STATE_CHANGE_PENDING` writer - Snoop filter RAM power state change pending.\n\nThis bit reads back the PREQ input."]
pub type SfRamStateChangePendingW<'a, REG> = crate::BitWriter<'a, REG, SfRamStateChangePending>;
impl<'a, REG> SfRamStateChangePendingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change pending, any previous requests have been accepted or denied."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SfRamStateChangePending::B0)
    }
    #[doc = "State change is pending and might be accepted or denied"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SfRamStateChangePending::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether any changes to the Snoop\n\nControl Registers or the Control Override\n\nRegister are pending in the CCI-500:"]
    #[inline(always)]
    pub fn change_pending(&self) -> ChangePendingR {
        ChangePendingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates when the snoop filter RAM is\n\ninitialized. Shareable requests are not\n\nserviced during this period."]
    #[inline(always)]
    pub fn sf_ram_initialization(&self) -> SfRamInitializationR {
        SfRamInitializationR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - The snoop filter RAM power states are"]
    #[inline(always)]
    pub fn sf_ram_state(&self) -> SfRamStateR {
        SfRamStateR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - This is the last requested power state of the\n\nsnoop filter RAMs Encoding as SF_RAM_state."]
    #[inline(always)]
    pub fn sf_ram_state_request(&self) -> SfRamStateRequestR {
        SfRamStateRequestR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Snoop filter RAM power state change pending.\n\nThis bit reads back the PREQ input."]
    #[inline(always)]
    pub fn sf_ram_state_change_pending(&self) -> SfRamStateChangePendingR {
        SfRamStateChangePendingR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether any changes to the Snoop\n\nControl Registers or the Control Override\n\nRegister are pending in the CCI-500:"]
    #[inline(always)]
    #[must_use]
    pub fn change_pending(&mut self) -> ChangePendingW<Cci500StatusSpec> {
        ChangePendingW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates when the snoop filter RAM is\n\ninitialized. Shareable requests are not\n\nserviced during this period."]
    #[inline(always)]
    #[must_use]
    pub fn sf_ram_initialization(&mut self) -> SfRamInitializationW<Cci500StatusSpec> {
        SfRamInitializationW::new(self, 1)
    }
    #[doc = "Bits 2:4 - The snoop filter RAM power states are"]
    #[inline(always)]
    #[must_use]
    pub fn sf_ram_state(&mut self) -> SfRamStateW<Cci500StatusSpec> {
        SfRamStateW::new(self, 2)
    }
    #[doc = "Bits 5:7 - This is the last requested power state of the\n\nsnoop filter RAMs Encoding as SF_RAM_state."]
    #[inline(always)]
    #[must_use]
    pub fn sf_ram_state_request(&mut self) -> SfRamStateRequestW<Cci500StatusSpec> {
        SfRamStateRequestW::new(self, 5)
    }
    #[doc = "Bit 8 - Snoop filter RAM power state change pending.\n\nThis bit reads back the PREQ input."]
    #[inline(always)]
    #[must_use]
    pub fn sf_ram_state_change_pending(&mut self) -> SfRamStateChangePendingW<Cci500StatusSpec> {
        SfRamStateChangePendingW::new(self, 8)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cci500StatusSpec;
impl crate::RegisterSpec for Cci500StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci500_status::R`](R) reader structure"]
impl crate::Readable for Cci500StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cci500_status::W`](W) writer structure"]
impl crate::Writable for Cci500StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCI500_STATUS to value 0"]
impl crate::Resettable for Cci500StatusSpec {
    const RESET_VALUE: u32 = 0;
}
