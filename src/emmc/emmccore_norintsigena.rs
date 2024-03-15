#[doc = "Register `EMMCCORE_NORINTSIGENA` reader"]
pub type R = crate::R<EmmccoreNorintsigenaSpec>;
#[doc = "Register `EMMCCORE_NORINTSIGENA` writer"]
pub type W = crate::W<EmmccoreNorintsigenaSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commandcomplete {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Commandcomplete> for bool {
    #[inline(always)]
    fn from(variant: Commandcomplete) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMMANDCOMPLETE` reader - "]
pub type CommandcompleteR = crate::BitReader<Commandcomplete>;
impl CommandcompleteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Commandcomplete {
        match self.bits {
            false => Commandcomplete::B0,
            true => Commandcomplete::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Commandcomplete::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Commandcomplete::B1
    }
}
#[doc = "Field `COMMANDCOMPLETE` writer - "]
pub type CommandcompleteW<'a, REG> = crate::BitWriter<'a, REG, Commandcomplete>;
impl<'a, REG> CommandcompleteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Commandcomplete::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Commandcomplete::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transfercomplete {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Transfercomplete> for bool {
    #[inline(always)]
    fn from(variant: Transfercomplete) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRANSFERCOMPLETE` reader - "]
pub type TransfercompleteR = crate::BitReader<Transfercomplete>;
impl TransfercompleteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Transfercomplete {
        match self.bits {
            false => Transfercomplete::B0,
            true => Transfercomplete::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Transfercomplete::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Transfercomplete::B1
    }
}
#[doc = "Field `TRANSFERCOMPLETE` writer - "]
pub type TransfercompleteW<'a, REG> = crate::BitWriter<'a, REG, Transfercomplete>;
impl<'a, REG> TransfercompleteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Transfercomplete::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Transfercomplete::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blockgapevent {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Blockgapevent> for bool {
    #[inline(always)]
    fn from(variant: Blockgapevent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKGAPEVENT` reader - "]
pub type BlockgapeventR = crate::BitReader<Blockgapevent>;
impl BlockgapeventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blockgapevent {
        match self.bits {
            false => Blockgapevent::B0,
            true => Blockgapevent::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Blockgapevent::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Blockgapevent::B1
    }
}
#[doc = "Field `BLOCKGAPEVENT` writer - "]
pub type BlockgapeventW<'a, REG> = crate::BitWriter<'a, REG, Blockgapevent>;
impl<'a, REG> BlockgapeventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Blockgapevent::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Blockgapevent::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmainterrupt {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Dmainterrupt> for bool {
    #[inline(always)]
    fn from(variant: Dmainterrupt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAINTERRUPT` reader - "]
pub type DmainterruptR = crate::BitReader<Dmainterrupt>;
impl DmainterruptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmainterrupt {
        match self.bits {
            false => Dmainterrupt::B0,
            true => Dmainterrupt::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dmainterrupt::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dmainterrupt::B1
    }
}
#[doc = "Field `DMAINTERRUPT` writer - "]
pub type DmainterruptW<'a, REG> = crate::BitWriter<'a, REG, Dmainterrupt>;
impl<'a, REG> DmainterruptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmainterrupt::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmainterrupt::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufferwriteready {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Bufferwriteready> for bool {
    #[inline(always)]
    fn from(variant: Bufferwriteready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFERWRITEREADY` reader - "]
pub type BufferwritereadyR = crate::BitReader<Bufferwriteready>;
impl BufferwritereadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufferwriteready {
        match self.bits {
            false => Bufferwriteready::B0,
            true => Bufferwriteready::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bufferwriteready::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bufferwriteready::B1
    }
}
#[doc = "Field `BUFFERWRITEREADY` writer - "]
pub type BufferwritereadyW<'a, REG> = crate::BitWriter<'a, REG, Bufferwriteready>;
impl<'a, REG> BufferwritereadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferwriteready::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferwriteready::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufferreadready {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Bufferreadready> for bool {
    #[inline(always)]
    fn from(variant: Bufferreadready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFERREADREADY` reader - "]
pub type BufferreadreadyR = crate::BitReader<Bufferreadready>;
impl BufferreadreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufferreadready {
        match self.bits {
            false => Bufferreadready::B0,
            true => Bufferreadready::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bufferreadready::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bufferreadready::B1
    }
}
#[doc = "Field `BUFFERREADREADY` writer - "]
pub type BufferreadreadyW<'a, REG> = crate::BitWriter<'a, REG, Bufferreadready>;
impl<'a, REG> BufferreadreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferreadready::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferreadready::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinsertion {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Cardinsertion> for bool {
    #[inline(always)]
    fn from(variant: Cardinsertion) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDINSERTION` reader - "]
pub type CardinsertionR = crate::BitReader<Cardinsertion>;
impl CardinsertionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardinsertion {
        match self.bits {
            false => Cardinsertion::B0,
            true => Cardinsertion::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cardinsertion::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cardinsertion::B1
    }
}
#[doc = "Field `CARDINSERTION` writer - "]
pub type CardinsertionW<'a, REG> = crate::BitWriter<'a, REG, Cardinsertion>;
impl<'a, REG> CardinsertionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinsertion::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinsertion::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardremoval {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Cardremoval> for bool {
    #[inline(always)]
    fn from(variant: Cardremoval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDREMOVAL` reader - "]
pub type CardremovalR = crate::BitReader<Cardremoval>;
impl CardremovalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardremoval {
        match self.bits {
            false => Cardremoval::B0,
            true => Cardremoval::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cardremoval::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cardremoval::B1
    }
}
#[doc = "Field `CARDREMOVAL` writer - "]
pub type CardremovalW<'a, REG> = crate::BitWriter<'a, REG, Cardremoval>;
impl<'a, REG> CardremovalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cardremoval::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cardremoval::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinterrupt {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Cardinterrupt> for bool {
    #[inline(always)]
    fn from(variant: Cardinterrupt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDINTERRUPT` reader - "]
pub type CardinterruptR = crate::BitReader<Cardinterrupt>;
impl CardinterruptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardinterrupt {
        match self.bits {
            false => Cardinterrupt::B0,
            true => Cardinterrupt::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cardinterrupt::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cardinterrupt::B1
    }
}
#[doc = "Field `CARDINTERRUPT` writer - "]
pub type CardinterruptW<'a, REG> = crate::BitWriter<'a, REG, Cardinterrupt>;
impl<'a, REG> CardinterruptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinterrupt::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinterrupt::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retuningevent {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Retuningevent> for bool {
    #[inline(always)]
    fn from(variant: Retuningevent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETUNINGEVENT` reader - "]
pub type RetuningeventR = crate::BitReader<Retuningevent>;
impl RetuningeventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Retuningevent {
        match self.bits {
            false => Retuningevent::B0,
            true => Retuningevent::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Retuningevent::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Retuningevent::B1
    }
}
#[doc = "Field `RETUNINGEVENT` writer - "]
pub type RetuningeventW<'a, REG> = crate::BitWriter<'a, REG, Retuningevent>;
impl<'a, REG> RetuningeventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningevent::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Retuningevent::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootackrcv {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Bootackrcv> for bool {
    #[inline(always)]
    fn from(variant: Bootackrcv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTACKRCV` reader - "]
pub type BootackrcvR = crate::BitReader<Bootackrcv>;
impl BootackrcvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootackrcv {
        match self.bits {
            false => Bootackrcv::B0,
            true => Bootackrcv::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bootackrcv::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bootackrcv::B1
    }
}
#[doc = "Field `BOOTACKRCV` writer - "]
pub type BootackrcvW<'a, REG> = crate::BitWriter<'a, REG, Bootackrcv>;
impl<'a, REG> BootackrcvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bootackrcv::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bootackrcv::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootterminateinterrupt {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Bootterminateinterrupt> for bool {
    #[inline(always)]
    fn from(variant: Bootterminateinterrupt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTTERMINATEINTERRUPT` reader - "]
pub type BootterminateinterruptR = crate::BitReader<Bootterminateinterrupt>;
impl BootterminateinterruptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootterminateinterrupt {
        match self.bits {
            false => Bootterminateinterrupt::B0,
            true => Bootterminateinterrupt::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bootterminateinterrupt::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bootterminateinterrupt::B1
    }
}
#[doc = "Field `BOOTTERMINATEINTERRUPT` writer - "]
pub type BootterminateinterruptW<'a, REG> = crate::BitWriter<'a, REG, Bootterminateinterrupt>;
impl<'a, REG> BootterminateinterruptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bootterminateinterrupt::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bootterminateinterrupt::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn commandcomplete(&self) -> CommandcompleteR {
        CommandcompleteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn transfercomplete(&self) -> TransfercompleteR {
        TransfercompleteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn blockgapevent(&self) -> BlockgapeventR {
        BlockgapeventR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dmainterrupt(&self) -> DmainterruptR {
        DmainterruptR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bufferwriteready(&self) -> BufferwritereadyR {
        BufferwritereadyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bufferreadready(&self) -> BufferreadreadyR {
        BufferreadreadyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cardinsertion(&self) -> CardinsertionR {
        CardinsertionR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cardremoval(&self) -> CardremovalR {
        CardremovalR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cardinterrupt(&self) -> CardinterruptR {
        CardinterruptR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn retuningevent(&self) -> RetuningeventR {
        RetuningeventR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn bootackrcv(&self) -> BootackrcvR {
        BootackrcvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn bootterminateinterrupt(&self) -> BootterminateinterruptR {
        BootterminateinterruptR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn commandcomplete(&mut self) -> CommandcompleteW<EmmccoreNorintsigenaSpec> {
        CommandcompleteW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn transfercomplete(&mut self) -> TransfercompleteW<EmmccoreNorintsigenaSpec> {
        TransfercompleteW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn blockgapevent(&mut self) -> BlockgapeventW<EmmccoreNorintsigenaSpec> {
        BlockgapeventW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dmainterrupt(&mut self) -> DmainterruptW<EmmccoreNorintsigenaSpec> {
        DmainterruptW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn bufferwriteready(&mut self) -> BufferwritereadyW<EmmccoreNorintsigenaSpec> {
        BufferwritereadyW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn bufferreadready(&mut self) -> BufferreadreadyW<EmmccoreNorintsigenaSpec> {
        BufferreadreadyW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cardinsertion(&mut self) -> CardinsertionW<EmmccoreNorintsigenaSpec> {
        CardinsertionW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cardremoval(&mut self) -> CardremovalW<EmmccoreNorintsigenaSpec> {
        CardremovalW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cardinterrupt(&mut self) -> CardinterruptW<EmmccoreNorintsigenaSpec> {
        CardinterruptW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn retuningevent(&mut self) -> RetuningeventW<EmmccoreNorintsigenaSpec> {
        RetuningeventW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn bootackrcv(&mut self) -> BootackrcvW<EmmccoreNorintsigenaSpec> {
        BootackrcvW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn bootterminateinterrupt(&mut self) -> BootterminateinterruptW<EmmccoreNorintsigenaSpec> {
        BootterminateinterruptW::new(self, 14)
    }
}
#[doc = "Normal interrupt signal enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_norintsigena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_norintsigena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreNorintsigenaSpec;
impl crate::RegisterSpec for EmmccoreNorintsigenaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmccore_norintsigena::R`](R) reader structure"]
impl crate::Readable for EmmccoreNorintsigenaSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_norintsigena::W`](W) writer structure"]
impl crate::Writable for EmmccoreNorintsigenaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_NORINTSIGENA to value 0"]
impl crate::Resettable for EmmccoreNorintsigenaSpec {
    const RESET_VALUE: u16 = 0;
}
